use std::sync::Arc;

use yixuan_logic::debug::{GMCmd, GMInput};
use yixuan_service::{CreatableServiceModule, ServiceModule};

use crate::cluster::{PlayerLogicCluster, PlayerLogicClusterManager};

use tokio::sync::{mpsc, oneshot};
use tracing::{error, info, warn};
use yixuan_proto::{
    PlayerLoginCsReq, PlayerLoginScRsp,
    head::PacketHead,
    server_only::{
        ClientPerformNotify, ExecuteClientCommandReq, ExecuteClientCommandRsp, GmTalkByMuipReq,
        GmTalkByMuipRsp, NetCommand, PlayerGetDataReq, PlayerGetDataRsp, StopPlayerLogicReq,
        StopPlayerLogicRsp,
    },
};
use yixuan_service::{
    ServiceContext, ServiceScope,
    config::ServiceType,
    network::{
        NetworkEntityManager, NetworkEventListener,
        client::NetworkClient,
        entity::NetworkEntity,
        packet::{GetProtoError, NetPacket},
    },
};

pub struct PlayerSession {
    pub player_uid: u32,
    pub gate_session_id: u64,
    pub cluster: PlayerLogicCluster,
}

#[derive(Default)]
pub struct PlayerSessionManager {
    pub session_map: scc::HashMap<u32, Arc<PlayerSession>>,
}

impl ServiceModule for PlayerSessionManager {
    fn run(
        self: std::sync::Arc<Self>,
        _service: std::sync::Arc<yixuan_service::ServiceContext>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl CreatableServiceModule for PlayerSessionManager {
    fn new(_context: &yixuan_service::ServiceContext) -> Self {
        Self::default()
    }
}

struct NetworkListener(mpsc::UnboundedSender<(u64, NetPacket)>);

impl NetworkEventListener for NetworkListener {
    fn on_receive(&self, entity_id: u64, packet: NetPacket) {
        self.0.send((entity_id, packet)).unwrap();
    }

    fn on_disconnect(&self, _entity_id: u64) {}
}

pub fn start_handler_task() -> (
    oneshot::Sender<Arc<ServiceContext>>,
    impl NetworkEventListener,
) {
    let (sv_tx, sv_rx) = oneshot::channel();
    let (tx, rx) = mpsc::unbounded_channel();

    tokio::spawn(handler_loop(sv_rx, rx));
    (sv_tx, NetworkListener(tx))
}

async fn handler_loop(
    lazy_service: oneshot::Receiver<Arc<ServiceContext>>,
    mut rx: mpsc::UnboundedReceiver<(u64, NetPacket)>,
) {
    let service = lazy_service.await.unwrap();

    while let Some((id, packet)) = rx.recv().await {
        tokio::spawn(process_cmd(Arc::clone(&service), id, packet));
    }
}

async fn process_cmd(service: Arc<ServiceContext>, entity_id: u64, packet: NetPacket) {
    if let Some(entity) = service.resolve::<NetworkEntityManager>().get(entity_id) {
        let scope = service.new_scope().with_variable(entity).build();

        if let Err(err) = handle_cmd(scope.as_ref(), packet).await {
            error!("failed to decode client cmd: {err}");
        }
    } else {
        warn!("process_cmd: client entity with id {entity_id} not found");
    }
}

handlers! {
    PlayerLoginCsReq;
    ExecuteClientCommandReq;
    StopPlayerLogicReq;
    GmTalkByMuipReq;
}

async fn handle_player_login_cs_req(
    scope: &ServiceScope,
    head: PacketHead,
    _request: PlayerLoginCsReq,
) -> PlayerLoginScRsp {
    let session_manager = scope.resolve::<PlayerSessionManager>();
    let cluster_manager = scope.resolve::<PlayerLogicClusterManager>();

    if session_manager
        .session_map
        .remove(&head.player_uid)
        .is_some()
    {
        // TODO: proper save and kick notify
    }

    let rsp = scope
        .resolve::<NetworkClient>()
        .send_request::<_, PlayerGetDataRsp>(
            ServiceType::Dbgate,
            head,
            PlayerGetDataReq {
                player_uid: head.player_uid,
            },
        )
        .await
        .unwrap();

    if rsp.retcode != 0 {
        error!(
            "PlayerGetData failed, retcode: {}, uid: {}",
            rsp.retcode, head.player_uid
        );

        return PlayerLoginScRsp {
            retcode: -1,
            ..Default::default()
        };
    }

    info!("PlayerGetData success, data: {:?}", rsp.player_data);

    let cluster = cluster_manager.get_least_loaded_cluster();
    cluster.create_player_slot(head.player_uid, rsp.player_data.unwrap());

    let _ = session_manager.session_map.insert(
        head.player_uid,
        Arc::new(PlayerSession {
            player_uid: head.player_uid,
            gate_session_id: head.gate_session_id,
            cluster,
        }),
    );

    PlayerLoginScRsp {
        retcode: 0,
        ..Default::default()
    }
}

async fn handle_execute_client_command_req(
    scope: &ServiceScope,
    head: PacketHead,
    request: ExecuteClientCommandReq,
) -> ExecuteClientCommandRsp {
    let session_manager = scope.resolve::<PlayerSessionManager>();
    if let Some(session) = session_manager
        .session_map
        .read(&head.player_uid, |_, session| Arc::clone(session))
    {
        let command = request.command.unwrap();
        let result = session
            .cluster
            .handle_player_command(session.player_uid, command.cmd_id as u16, command.body)
            .await;

        ExecuteClientCommandRsp {
            retcode: 0,
            notify_list: result
                .notifies
                .into_iter()
                .map(|(cmd_id, body)| NetCommand {
                    cmd_id: cmd_id as u32,
                    body,
                })
                .collect(),
            response: result.response.map(|(cmd_id, body)| NetCommand {
                cmd_id: cmd_id as u32,
                body,
            }),
        }
    } else {
        ExecuteClientCommandRsp {
            retcode: -1,
            ..Default::default()
        }
    }
}

async fn handle_stop_player_logic_req(
    scope: &ServiceScope,
    _head: PacketHead,
    request: StopPlayerLogicReq,
) -> StopPlayerLogicRsp {
    let session_manager = scope.resolve::<PlayerSessionManager>();

    if let Some((_, session)) = session_manager.session_map.remove(&request.player_uid) {
        session.cluster.remove_player(session.player_uid);
        StopPlayerLogicRsp { retcode: 0 }
    } else {
        StopPlayerLogicRsp { retcode: 1 }
    }
}

async fn handle_gm_talk_by_muip_req(
    scope: &ServiceScope,
    head: PacketHead,
    request: GmTalkByMuipReq,
) -> GmTalkByMuipRsp {
    let session_manager = scope.resolve::<PlayerSessionManager>();

    if let Some(session) = session_manager
        .session_map
        .read(&head.player_uid, |_, session| Arc::clone(session))
    {
        let cmd = match GMCmd::from_str(&request.msg) {
            Ok(cmd) => cmd,
            Err(err) => {
                return GmTalkByMuipRsp {
                    retcode: 2,
                    retmsg: format!("failed to parse GM Command: {err}"),
                };
            }
        };

        let mut notifies = session
            .cluster
            .push_gm_command(session.player_uid, cmd)
            .await;

        scope
            .resolve::<NetworkClient>()
            .send_notify(
                ServiceType::Gate,
                PacketHead {
                    gate_session_id: session.gate_session_id,
                    player_uid: session.player_uid,
                    ..Default::default()
                },
                ClientPerformNotify {
                    notify_list: notifies
                        .drain()
                        .into_iter()
                        .map(|(cmd_id, body)| NetCommand {
                            cmd_id: cmd_id as u32,
                            body,
                        })
                        .collect(),
                },
            )
            .await;

        GmTalkByMuipRsp {
            retcode: 0,
            retmsg: String::from("OK"),
        }
    } else {
        GmTalkByMuipRsp {
            retcode: 1,
            retmsg: String::from("GMTalk failed: player is offline"),
        }
    }
}

macro_rules! handlers {
    ($($name:ident;)*) => {
        async fn handle_cmd(scope: &ServiceScope, packet: NetPacket) -> Result<(), GetProtoError> {
            ::paste::paste! {
                use yixuan_proto::*;
                use yixuan_proto::server_only::*;

                match packet.cmd_id {
                    $($name::CMD_ID => {
                        let response = [<handle_ $name:snake>](scope, packet.head, packet.get_proto()?).await;

                        let entity = scope.fetch::<Arc<NetworkEntity>>().unwrap();
                        entity.send(NetPacket::new(
                            PacketHead {
                                player_uid: packet.head.player_uid,
                                session_id: packet.head.session_id,
                                ack_packet_id: packet.head.packet_id,
                                ..Default::default()
                            },
                            response,
                        ));
                    })*
                    _ => warn!("unhandled: {}", packet.cmd_id),
                }
            }

            Ok(())
        }
    };
}

use handlers;

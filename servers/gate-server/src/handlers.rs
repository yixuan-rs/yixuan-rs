use std::{net::SocketAddr, sync::Arc};

use tokio::sync::{mpsc, oneshot};
use tracing::{debug, error, warn};
use yixuan_proto::{
    KeepAliveNotify, NetCmd, PlayerGetTokenCsReq, PlayerGetTokenScRsp, PlayerLoginCsReq,
    PlayerLoginScRsp, PlayerLogoutCsReq,
    head::PacketHead,
    server_only::{ClientPerformNotify, StopPlayerLogicReq, StopPlayerLogicRsp},
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

use crate::{encryption::SecurityModule, session::PlayerSessionManager};

struct NetworkListener(mpsc::UnboundedSender<NetworkEvent>);

enum NetworkEvent {
    Receive(u64, NetPacket),
    Disconnect(u64),
}

struct InternalAddr(SocketAddr);

impl NetworkEventListener for NetworkListener {
    fn on_receive(&self, entity_id: u64, packet: NetPacket) {
        self.0
            .send(NetworkEvent::Receive(entity_id, packet))
            .unwrap();
    }

    fn on_disconnect(&self, entity_id: u64) {
        debug!("entity with id {entity_id} disconnected");
        self.0.send(NetworkEvent::Disconnect(entity_id)).unwrap();
    }
}

pub fn start_handler_task(
    internal_addr: SocketAddr,
) -> (
    oneshot::Sender<Arc<ServiceContext>>,
    impl NetworkEventListener,
) {
    let (sv_tx, sv_rx) = oneshot::channel();
    let (tx, rx) = mpsc::unbounded_channel();

    tokio::spawn(handler_loop(sv_rx, internal_addr, rx));
    (sv_tx, NetworkListener(tx))
}

async fn handler_loop(
    lazy_service: oneshot::Receiver<Arc<ServiceContext>>,
    internal_addr: SocketAddr,
    mut rx: mpsc::UnboundedReceiver<NetworkEvent>,
) {
    let service = lazy_service.await.unwrap();

    while let Some(event) = rx.recv().await {
        match event {
            NetworkEvent::Receive(id, packet) => {
                tokio::spawn(process_cmd(Arc::clone(&service), id, internal_addr, packet));
            }
            NetworkEvent::Disconnect(id) => {
                tokio::spawn(process_disconnect(Arc::clone(&service), id));
            }
        }
    }
}

async fn process_cmd(
    service: Arc<ServiceContext>,
    entity_id: u64,
    internal_addr: SocketAddr,
    mut packet: NetPacket,
) {
    if let Some(entity) = service.resolve::<NetworkEntityManager>().get(entity_id) {
        packet.head.session_id = entity_id;
        let scope = service
            .new_scope()
            .with_variable(InternalAddr(internal_addr))
            .with_variable(entity)
            .build();

        if let Err(err) = handle_cmd(scope.as_ref(), packet).await {
            error!("failed to decode client cmd: {err}");
        }
    } else {
        warn!("process_cmd: client entity with id {entity_id} not found");
    }
}

async fn process_disconnect(service: Arc<ServiceContext>, entity_id: u64) {
    let entity_manager = service.resolve::<NetworkEntityManager>();
    entity_manager.on_disconnect(entity_id);

    if let Some(session) = service.resolve::<PlayerSessionManager>().remove(entity_id) {
        if session.is_logged_in() && !session.is_logged_out() {
            let _ = service
                .resolve::<NetworkClient>()
                .send_request::<_, StopPlayerLogicRsp>(
                    ServiceType::Game,
                    PacketHead {
                        player_uid: session.uid(),
                        session_id: entity_id,
                        ..Default::default()
                    },
                    StopPlayerLogicReq {
                        player_uid: session.uid(),
                    },
                )
                .await;
        }
    }
}

#[allow(dead_code)]
pub fn bytes_as_hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    bytes.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{b:02x}");
        output
    })
}

async fn handle_cmd(scope: &ServiceScope, packet: NetPacket) -> Result<(), GetProtoError> {
    debug!(
        "handle_cmd - cmd_id: {}, size: {}",
        packet.cmd_id,
        packet.body.len()
    );
    match packet.cmd_id {
        PlayerGetTokenCsReq::CMD_ID => {
            handle_player_get_token(scope, packet.head, packet.get_proto()?).await
        }
        PlayerLoginCsReq::CMD_ID => {
            handle_player_login(scope, packet.head, packet.get_proto()?).await
        }
        PlayerLogoutCsReq::CMD_ID => {
            handle_player_logout(scope, packet.head, packet.get_proto()?).await
        }
        KeepAliveNotify::CMD_ID => (),
        ClientPerformNotify::CMD_ID => {
            handle_client_perform(scope, packet.head, packet.get_proto()?).await
        }
        cmd_id if cmd_id < 10000 => {
            if let Some(session) = scope
                .resolve::<PlayerSessionManager>()
                .get(packet.head.session_id)
            {
                if session.is_logged_in() {
                    session.enqueue(packet).await;
                } else {
                    warn!("unhandled [session not logged in]: {}", packet.cmd_id);
                    debug!("{}", bytes_as_hex(packet.body.as_slice()));
                }
            } else {
                warn!("unhandled [missing session]: {}", packet.cmd_id);
                debug!("{}", bytes_as_hex(packet.body.as_slice()));
            }
        }
        _ => {
            warn!("unhandled: {}", packet.cmd_id);
            debug!("{}", bytes_as_hex(packet.body.as_slice()));
        }
    }

    Ok(())
}

async fn handle_player_get_token(
    scope: &ServiceScope,
    head: PacketHead,
    mut request: PlayerGetTokenCsReq,
) {
    debug!("{request:?}");

    let session = scope
        .resolve::<PlayerSessionManager>()
        .get_or_insert(head.session_id);

    if session.is_player_token_got() {
        debug!(
            "PlayerGetTokenCsReq received twice! (player uid: {}, login failed status: {})",
            session.uid(),
            session.login_failed_status()
        );
        return;
    }

    let client_rsa_ver = request.rsa_ver;
    let client_rand_key = std::mem::take(&mut request.client_rand_key);

    let client = scope.resolve::<NetworkClient>();
    let mut rsp = client
        .send_request::<_, PlayerGetTokenScRsp>(ServiceType::Dbgate, head, request)
        .await
        .unwrap();

    session.on_player_token_got(&rsp);

    let security_module = scope.resolve::<SecurityModule>();
    let rand_key = match security_module.gen_rand_key(&mut rsp, client_rsa_ver, &client_rand_key) {
        Ok(rand_key) => rand_key,
        Err(err) => {
            error!("gen_rand_key failed: {err}");
            return;
        }
    };

    let entity = scope.fetch::<Arc<NetworkEntity>>().unwrap();

    entity.send(NetPacket::new(
        PacketHead {
            packet_id: 0,
            ack_packet_id: head.packet_id,
            ..Default::default()
        },
        rsp,
    ));

    entity.set_session_key(yixuan_encryption::mersenne_twister::generate_xorpad(
        rand_key, true,
    ));
}

async fn handle_player_login(scope: &ServiceScope, head: PacketHead, request: PlayerLoginCsReq) {
    debug!("{request:?}");

    let session = scope
        .resolve::<PlayerSessionManager>()
        .get_or_insert(head.session_id);

    if !session.is_player_token_got() {
        debug!("received PlayerLoginCsReq before PlayerGetTokenCsReq");
        return;
    } else if session.is_login_failed() {
        debug!(
            "received PlayerLoginCsReq but session failed to login already! uid: {}, login failed status: {}",
            session.uid(),
            session.login_failed_status()
        );
        return;
    } else if session.is_logged_out() {
        debug!(
            "received PlayerLoginCsReq, but player is already logged out, uid: {}",
            session.uid(),
        );
        return;
    } else if session.is_logged_in() {
        debug!("PlayerLoginCsReq received twice, uid: {}", session.uid(),);
        return;
    }

    let client = scope.resolve::<NetworkClient>();
    let rsp = client
        .send_request::<_, PlayerLoginScRsp>(
            ServiceType::Game,
            PacketHead {
                player_uid: session.uid(),
                session_id: head.session_id,
                gate_session_id: head.session_id,
                ..Default::default()
            },
            request,
        )
        .await
        .unwrap();

    if rsp.retcode == 0 {
        session.logged_in();
        session.init_queue_worker(
            Arc::clone(scope.service()),
            Arc::clone(scope.fetch::<Arc<NetworkEntity>>().unwrap()),
        );
    } else {
        session.login_failed(rsp.retcode);
    }

    scope
        .fetch::<Arc<NetworkEntity>>()
        .unwrap()
        .send(NetPacket::new(
            PacketHead {
                packet_id: 1,
                player_uid: session.uid(),
                session_id: head.session_id,
                ack_packet_id: head.packet_id,
                ..Default::default()
            },
            rsp,
        ));
}

async fn handle_player_logout(scope: &ServiceScope, head: PacketHead, _request: PlayerLogoutCsReq) {
    if let Some(session) = scope.resolve::<PlayerSessionManager>().get(head.session_id) {
        if session.is_logged_in() && !session.is_logged_out() {
            session.logged_out();

            let _ = scope
                .resolve::<NetworkClient>()
                .send_request::<_, StopPlayerLogicRsp>(
                    ServiceType::Game,
                    PacketHead {
                        player_uid: session.uid(),
                        gate_session_id: head.session_id,
                        ..Default::default()
                    },
                    StopPlayerLogicReq {
                        player_uid: session.uid(),
                    },
                )
                .await;

            scope.fetch::<Arc<NetworkEntity>>().unwrap().disconnect();
        }
    }
}

async fn handle_client_perform(
    scope: &ServiceScope,
    head: PacketHead,
    notify: ClientPerformNotify,
) {
    let InternalAddr(internal_addr) = scope.fetch().unwrap();
    let entity = scope.fetch::<Arc<NetworkEntity>>().unwrap();

    if Some(internal_addr) != entity.local_addr.as_ref() {
        warn!("received server-only packet from client!");
        return;
    }

    if let Some(session) = scope.resolve::<PlayerSessionManager>().get(head.gate_session_id) {
        debug!("pushing notifies: {:?}", notify.notify_list);

        session
            .push_notifies(
                notify
                    .notify_list
                    .into_iter()
                    .map(|notify| (notify.cmd_id as u16, notify.body))
                    .collect(),
            )
            .await;
    } else {
        debug!("no session with id {}", head.session_id);
    }
}

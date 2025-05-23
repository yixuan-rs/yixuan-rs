use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use tokio::sync::mpsc;
use tracing::{debug, error, warn};
use vivian_proto::{
    PlayerGetTokenScRsp,
    head::PacketHead,
    server_only::{ExecuteClientCommandReq, ExecuteClientCommandRsp, NetCommand},
};
use vivian_service::{
    CreatableServiceModule, ServiceContext, ServiceModule,
    config::ServiceType,
    network::{client::NetworkClient, entity::NetworkEntity, packet::NetPacket},
};

#[derive(Default)]
pub struct PlayerSessionManager(scc::HashMap<u64, Arc<PlayerSession>>);

pub struct PlayerSession {
    uid: OnceLock<u32>,
    login_failed_status: OnceLock<i32>,
    is_logged_in: OnceLock<()>,
    is_logged_out: OnceLock<()>,
    packet_tx: OnceLock<mpsc::Sender<NetPacket>>,
}

impl PlayerSessionManager {
    pub fn get_or_insert(&self, id: u64) -> Arc<PlayerSession> {
        if let Some(session) = self.get(id) {
            session
        } else {
            let _ = self.0.insert(id, Arc::new(PlayerSession::new()));
            self.get(id).unwrap()
        }
    }

    pub fn get(&self, id: u64) -> Option<Arc<PlayerSession>> {
        self.0.read(&id, |_, session| Arc::clone(session))
    }

    pub fn remove(&self, id: u64) -> Option<Arc<PlayerSession>> {
        self.0.remove(&id).map(|(_, v)| v)
    }
}

impl PlayerSession {
    pub fn new() -> Self {
        Self {
            uid: OnceLock::new(),
            login_failed_status: OnceLock::new(),
            is_logged_in: OnceLock::new(),
            is_logged_out: OnceLock::new(),
            packet_tx: OnceLock::new(),
        }
    }

    pub async fn enqueue(&self, packet: NetPacket) {
        if let Some(tx) = self.packet_tx.get() {
            let _ = tx.send(packet).await;
        }
    }

    async fn packet_handler_loop(
        mut rx: mpsc::Receiver<NetPacket>,
        uid: u32,
        ctx: Arc<ServiceContext>,
        network_entity: Arc<NetworkEntity>,
    ) {
        const BUFFERED_REQUESTS_LIMIT: usize = 32;

        let mut last_client_packet_id = 0;
        let mut last_server_packet_id = 1;

        let mut buffered_requests = HashMap::with_capacity(BUFFERED_REQUESTS_LIMIT);

        while let Some(packet) = rx.recv().await {
            let packet_id = packet.head.packet_id;

            if packet_id != 0 && packet_id != (last_client_packet_id + 1) {
                if buffered_requests.len() >= BUFFERED_REQUESTS_LIMIT {
                    warn!(
                        "Receive queue full. Packet with cmd_id {} discarded.",
                        packet.cmd_id
                    );
                    continue;
                }

                buffered_requests.insert(packet.head.packet_id, packet);
                continue;
            }

            Self::process_one(
                packet,
                &ctx,
                &network_entity,
                uid,
                &mut last_server_packet_id,
            )
            .await;

            if packet_id != 0 {
                last_client_packet_id = packet_id;
                while let Some(packet) = buffered_requests.remove(&(last_client_packet_id + 1)) {
                    last_client_packet_id = packet.head.packet_id;
                    Self::process_one(
                        packet,
                        &ctx,
                        &network_entity,
                        uid,
                        &mut last_server_packet_id,
                    )
                    .await;
                }
            }
        }
    }

    async fn process_one(
        packet: NetPacket,
        ctx: &Arc<ServiceContext>,
        network_entity: &Arc<NetworkEntity>,
        uid: u32,
        last_server_packet_id: &mut u32,
    ) {
        let rsp = ctx
            .resolve::<NetworkClient>()
            .send_request::<_, ExecuteClientCommandRsp>(
                ServiceType::Game,
                PacketHead {
                    packet_id: 0,
                    player_uid: uid,
                    session_id: packet.head.session_id,
                    ack_packet_id: 0,
                },
                ExecuteClientCommandReq {
                    command: Some(NetCommand {
                        cmd_id: packet.cmd_id as u32,
                        body: packet.body,
                    }),
                },
            )
            .await
            .unwrap();

        if rsp.retcode != 0 {
            error!("ExecuteClientCommandRsp retcode is {}", rsp.retcode);
            return;
        }

        for notify in rsp.notify_list {
            if notify.cmd_id != 0 {
                *last_server_packet_id += 1;

                network_entity.send(NetPacket {
                    cmd_id: notify.cmd_id as u16,
                    body: notify.body,
                    head: PacketHead {
                        packet_id: *last_server_packet_id,
                        player_uid: uid,
                        ..Default::default()
                    },
                });
            }
        }

        if let Some(mut response) = rsp.response {
            if response.cmd_id == 0 {
                response.cmd_id = 5002;
            }

            *last_server_packet_id += 1;
            network_entity.send(NetPacket {
                cmd_id: response.cmd_id as u16,
                body: response.body,
                head: PacketHead {
                    packet_id: *last_server_packet_id,
                    ack_packet_id: packet.head.packet_id,
                    player_uid: uid,
                    ..Default::default()
                },
            });
        }
    }

    pub fn on_player_token_got(&self, rsp: &PlayerGetTokenScRsp) {
        if rsp.retcode == 0 {
            let _ = self.uid.set(rsp.uid);
            debug!("got player token, uid: {}", rsp.uid);
        } else {
            self.login_failed(rsp.retcode);
        }
    }

    pub fn login_failed(&self, retcode: i32) {
        if self.login_failed_status.get().is_none() {
            let _ = self.login_failed_status.set(retcode);
            debug!("[uid: {}] login failed (retcode: {retcode})", self.uid());
        }
    }

    pub fn init_queue_worker(&self, ctx: Arc<ServiceContext>, client_entity: Arc<NetworkEntity>) {
        if self.packet_tx.get().is_none() {
            let (tx, rx) = mpsc::channel(32);
            let _ = self.packet_tx.set(tx);

            tokio::spawn(Self::packet_handler_loop(
                rx,
                self.uid(),
                ctx,
                client_entity,
            ));
        }
    }

    pub fn logged_in(&self) {
        let _ = self.is_logged_in.set(());
    }

    pub fn logged_out(&self) {
        let _ = self.is_logged_out.set(());
    }

    pub fn is_player_token_got(&self) -> bool {
        self.uid.get().is_some() || self.is_login_failed()
    }

    pub fn is_login_failed(&self) -> bool {
        self.login_failed_status.get().is_some()
    }

    pub fn is_logged_in(&self) -> bool {
        self.is_logged_in.get().is_some() && !self.is_login_failed()
    }

    pub fn is_logged_out(&self) -> bool {
        self.is_logged_out.get().is_some()
    }

    pub fn uid(&self) -> u32 {
        self.uid.get().copied().unwrap_or(0)
    }

    pub fn login_failed_status(&self) -> i32 {
        self.login_failed_status.get().copied().unwrap_or(0)
    }
}

impl ServiceModule for PlayerSessionManager {
    fn run(
        self: std::sync::Arc<Self>,
        _service: std::sync::Arc<vivian_service::ServiceContext>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl CreatableServiceModule for PlayerSessionManager {
    fn new(_context: &vivian_service::ServiceContext) -> Self {
        Self::default()
    }
}

use std::{
    collections::HashMap,
    sync::{
        Arc, OnceLock, Weak,
        atomic::{AtomicU32, Ordering},
    },
    time::Duration,
};

use tokio::{net::TcpStream, sync::oneshot};
use tracing::warn;
use vivian_proto::{Message, NetCmd, head::PacketHead};

use crate::{
    ConfigurableServiceModule, ServiceModule,
    config::{ServiceEndPoint, ServiceType},
};

use super::{
    NetworkEventListener,
    entity::NetworkEntity,
    packet::{GetProtoError, NetPacket},
};

struct NetworkClientListener {
    client: Weak<NetworkClient>,
}

impl NetworkEventListener for NetworkClientListener {
    fn on_receive(&self, entity_id: u64, packet: NetPacket) {
        if let Some(client) = self.client.upgrade() {
            client.receive_message(entity_id, packet);
        }
    }

    fn on_disconnect(&self, entity_id: u64) {
        if let Some(client) = self.client.upgrade() {
            client.unregister_entity(entity_id);
        }
    }
}

pub struct NetworkClient {
    endpoints: HashMap<ServiceType, ServiceEndPoint>,
    packet_id_counter: AtomicU32,
    bound_entities: scc::HashMap<u64, Arc<NetworkEntity>>,
    listener: OnceLock<Arc<dyn NetworkEventListener>>,
    request_awaiters: scc::HashMap<(u64, u64, u32), oneshot::Sender<NetPacket>>,
}

#[derive(thiserror::Error, Debug)]
pub enum NetworkRequestError {
    #[error("network request timed out")]
    Timeout,
    #[error("failed to decode response: {0}")]
    Decode(#[from] GetProtoError),
}

impl ConfigurableServiceModule for NetworkClient {
    type Config = HashMap<ServiceType, ServiceEndPoint>;

    fn new(_context: &crate::ServiceContext, config: Self::Config) -> Self {
        NetworkClient::new(config)
    }
}

impl ServiceModule for NetworkClient {
    fn run(
        self: Arc<Self>,
        _service: Arc<crate::ServiceContext>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self.listener.set(Arc::new(NetworkClientListener {
            client: Arc::downgrade(&self),
        }));

        Ok(())
    }
}

impl NetworkClient {
    pub fn new(endpoints: HashMap<ServiceType, ServiceEndPoint>) -> Self {
        Self {
            endpoints,
            packet_id_counter: AtomicU32::new(1),
            bound_entities: scc::HashMap::new(),
            listener: OnceLock::new(),
            request_awaiters: scc::HashMap::new(),
        }
    }

    pub async fn send_request<Req, Rsp>(
        &self,
        destination_service: ServiceType,
        mut head: PacketHead,
        request: Req,
    ) -> Result<Rsp, NetworkRequestError>
    where
        Req: Message + NetCmd,
        Rsp: Message + NetCmd + Default,
    {
        const TIMEOUT: Duration = Duration::from_secs(30);

        let endpoint = self.endpoints.get(&destination_service).unwrap();
        let entity = self
            .acquire_entity(endpoint)
            .await
            .expect("failed to acquire entity");

        head.packet_id = self.packet_id_counter.fetch_add(1, Ordering::SeqCst);
        let (awaiter_tx, awaiter_rx) = oneshot::channel();

        let _ = self
            .request_awaiters
            .insert_async((endpoint.uid, head.session_id, head.packet_id), awaiter_tx)
            .await;

        entity.send(NetPacket::new(head, request));

        let packet = match tokio::time::timeout(TIMEOUT, awaiter_rx).await {
            Ok(result) => result.unwrap(),
            Err(_) => return Err(NetworkRequestError::Timeout),
        };

        Ok(packet.get_proto::<Rsp>()?)
    }

    pub async fn send_notify<Notify>(
        &self,
        destination_service: ServiceType,
        head: PacketHead,
        notify: Notify,
    ) where
        Notify: Message + NetCmd,
    {
        let endpoint = self.endpoints.get(&destination_service).unwrap();
        let entity = self
            .acquire_entity(endpoint)
            .await
            .expect("failed to acquire entity");

        entity.send(NetPacket::new(head, notify));
    }

    async fn acquire_entity(&self, endpoint: &ServiceEndPoint) -> Option<Arc<NetworkEntity>> {
        if let Some(entity) = self
            .bound_entities
            .get(&endpoint.uid)
            .map(|entity| Arc::clone(&entity))
        {
            Some(entity)
        } else {
            let stream = TcpStream::connect(endpoint.addr).await.ok()?;
            let entity = Arc::new(NetworkEntity::start(
                endpoint.uid,
                stream,
                Arc::clone(self.listener.get().unwrap()),
                None,
            ));

            let _ = self
                .bound_entities
                .insert(endpoint.uid, Arc::clone(&entity));

            Some(entity)
        }
    }

    fn receive_message(&self, entity_id: u64, packet: NetPacket) {
        if let Some((_, awaiter_tx)) = self.request_awaiters.remove(&(
            entity_id,
            packet.head.session_id,
            packet.head.ack_packet_id,
        )) {
            let _ = awaiter_tx.send(packet);
        } else {
            warn!(
                "awaiter not found (entity_id: {}, session_id: {}, packet_id: {})",
                entity_id, packet.head.session_id, packet.head.ack_packet_id
            );
        }
    }

    fn unregister_entity(&self, entity_id: u64) {
        let _ = self.bound_entities.remove(&entity_id);
    }
}

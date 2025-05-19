use std::{
    error::Error,
    net::SocketAddr,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use entity::NetworkEntity;
use packet::NetPacket;
use scc::HashMap;
use tokio::net::{TcpListener, TcpStream};
use tracing::debug;

use crate::{ConfigurableServiceModule, ServiceContext, ServiceModule, Startable};

pub mod client;
pub mod entity;
pub mod packet;

pub trait NetworkEventListener: Send + Sync + 'static {
    fn on_receive(&self, entity_id: u64, packet: NetPacket);
    fn on_disconnect(&self, entity_id: u64);
}

pub struct NetworkServer {
    bind_addr: SocketAddr,
}

pub struct NetworkEntityManager {
    entity_id_counter: AtomicU64,
    entity_map: HashMap<u64, Arc<NetworkEntity>>,
    event_listener: Arc<dyn NetworkEventListener>,
    xorpad: Option<&'static [u8; 4096]>,
}

impl NetworkEntityManager {
    pub fn new(
        event_listener: impl NetworkEventListener,
        xorpad: Option<&'static [u8; 4096]>,
    ) -> Self {
        Self {
            entity_id_counter: AtomicU64::new(1),
            entity_map: HashMap::new(),
            event_listener: Arc::new(event_listener),
            xorpad,
        }
    }

    pub(crate) fn on_connect(&self, stream: TcpStream) {
        let id = self.entity_id_counter.fetch_add(1, Ordering::SeqCst);
        let entity =
            NetworkEntity::start(id, stream, Arc::clone(&self.event_listener), self.xorpad);

        let _ = self.entity_map.insert(id, Arc::new(entity));
    }

    pub fn get(&self, id: u64) -> Option<Arc<NetworkEntity>> {
        self.entity_map.get(&id).map(|entity| Arc::clone(&entity))
    }

    pub fn on_disconnect(&self, id: u64) {
        self.entity_map.remove(&id);
    }
}

impl NetworkServer {
    async fn accept_loop(service: Arc<ServiceContext>, listener: TcpListener) {
        loop {
            if let Ok((stream, _addr)) = listener.accept().await {
                service.resolve::<NetworkEntityManager>().on_connect(stream);
            }
        }
    }
}

impl ConfigurableServiceModule for NetworkServer {
    type Config = SocketAddr;

    fn new(_context: &crate::ServiceContext, config: Self::Config) -> Self {
        Self { bind_addr: config }
    }
}

impl Startable for NetworkEntityManager {
    async fn start(&self, _service: Arc<ServiceContext>) {
        // TODO: connection alive check loop
    }
}

impl ServiceModule for NetworkServer {
    fn run(self: Arc<Self>, service: Arc<crate::ServiceContext>) -> Result<(), Box<dyn Error>> {
        let listener = std::net::TcpListener::bind(self.bind_addr)?;
        listener.set_nonblocking(true).unwrap();

        tokio::spawn(Self::accept_loop(
            service,
            TcpListener::from_std(listener).unwrap(),
        ));

        debug!("server is listening at {}", self.bind_addr);

        Ok(())
    }
}

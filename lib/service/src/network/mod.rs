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
pub mod net_util;
pub mod packet;

pub trait NetworkEventListener: Send + Sync + 'static {
    fn on_receive(&self, entity_id: u64, packet: NetPacket);
    fn on_disconnect(&self, entity_id: u64);
}

pub struct NetworkServer {
    bind_addresses: Vec<SocketAddr>,
}

pub struct NetworkEntityManager {
    entity_id_counter: AtomicU64,
    entity_map: HashMap<u64, Arc<NetworkEntity>>,
    event_listener: Arc<dyn NetworkEventListener>,
    xorpad_map: std::collections::HashMap<SocketAddr, &'static [u8; 4096]>,
}

impl NetworkEntityManager {
    pub fn new(
        event_listener: impl NetworkEventListener,
        xorpad_map: std::collections::HashMap<SocketAddr, &'static [u8; 4096]>,
    ) -> Self {
        Self {
            entity_id_counter: AtomicU64::new(1),
            entity_map: HashMap::new(),
            event_listener: Arc::new(event_listener),
            xorpad_map,
        }
    }

    pub(crate) fn on_connect(&self, stream: TcpStream, local_addr: SocketAddr) {
        let id = self.entity_id_counter.fetch_add(1, Ordering::SeqCst);
        let entity = NetworkEntity::start(
            id,
            stream,
            Arc::clone(&self.event_listener),
            Some(local_addr),
            self.xorpad_map.get(&local_addr).copied(),
        );

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
    async fn accept_loop(
        service: Arc<ServiceContext>,
        local_addr: SocketAddr,
        listener: TcpListener,
    ) {
        loop {
            if let Ok((stream, _addr)) = listener.accept().await {
                service
                    .resolve::<NetworkEntityManager>()
                    .on_connect(stream, local_addr);
            }
        }
    }
}

impl ConfigurableServiceModule for NetworkServer {
    type Config = Vec<SocketAddr>;

    fn new(_context: &crate::ServiceContext, config: Self::Config) -> Self {
        Self {
            bind_addresses: config,
        }
    }
}

impl Startable for NetworkEntityManager {
    async fn start(&self, _service: Arc<ServiceContext>) {
        // TODO: connection alive check loop
    }
}

impl ServiceModule for NetworkServer {
    fn run(self: Arc<Self>, service: Arc<crate::ServiceContext>) -> Result<(), Box<dyn Error>> {
        for &bind_addr in self.bind_addresses.iter() {
            tokio::spawn(Self::accept_loop(
                Arc::clone(&service),
                bind_addr,
                net_util::tcp_bind_sync(bind_addr)?,
            ));

            debug!("server is listening at {bind_addr}");
        }

        Ok(())
    }
}

use std::sync::Arc;

use vivian_service::{CreatableServiceModule, ServiceModule};

use crate::logic::cluster::PlayerLogicCluster;

pub struct PlayerSession {
    pub player_uid: u32,
    pub cluster: PlayerLogicCluster,
}

#[derive(Default)]
pub struct PlayerSessionManager {
    pub session_map: scc::HashMap<u32, Arc<PlayerSession>>,
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

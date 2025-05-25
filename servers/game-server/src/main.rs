use std::sync::OnceLock;

use cluster::{LogicClusterConfig, PlayerLogicClusterManager};
use common::logging::init_tracing;
use config::ServerConfig;
use const_format::concatcp;
use resources::{LoadResourcesError, NapResources, ServerGameplayConfig};
use session::PlayerSessionManager;
use vivian_service::{
    ServiceContext, ServiceError,
    config::{ServiceType, load_environment_config},
    network::{NetworkEntityManager, NetworkServer, client::NetworkClient},
};

mod config;
mod handlers;
mod resources;
mod session;
mod util;
mod player;
mod sync;
mod cluster;

const SERVICE_TYPE: ServiceType = ServiceType::Game;
const CONFIG_DIR: &str = "config/40-game-server/";

#[derive(thiserror::Error, Debug)]
pub enum StartupError {
    #[error("{0}")]
    LoadResources(#[from] LoadResourcesError),
    #[error("{0}")]
    Service(#[from] ServiceError),
}

#[tokio::main]
async fn main() -> Result<(), StartupError> {
    static RESOURCES: OnceLock<NapResources> = OnceLock::new();

    init_tracing(tracing::Level::DEBUG);

    let config = common::config_util::load_or_create::<ServerConfig>(
        concatcp!(CONFIG_DIR, "config.toml"),
        include_str!("../config.default.toml"),
    );

    let env_cfg = load_environment_config();
    let gameplay_cfg = ServerGameplayConfig {
        gacha_schedule: common::config_util::load_or_create(
            concatcp!(CONFIG_DIR, "gacha_schedule.toml"),
            include_str!("../gacha_schedule.default.toml"),
        ),
        first_login: common::config_util::load_or_create(
            concatcp!(CONFIG_DIR, "first_login.toml"),
            include_str!("../first_login.default.toml"),
        ),
    };

    let resources = NapResources::load(&config.resources, gameplay_cfg)?;
    let resources = RESOURCES.get_or_init(|| resources);

    let (service_tx, listener) = session::start_handler_task();

    let service = ServiceContext::new()
        .insert_module(NetworkEntityManager::new(listener, None))
        .configure_module::<NetworkServer>(env_cfg.services.get(&SERVICE_TYPE).unwrap().addr)
        .configure_module::<NetworkClient>(env_cfg.services)
        .configure_module::<PlayerLogicClusterManager>(LogicClusterConfig {
            cluster: config.cluster,
            resources,
        })
        .with_module::<PlayerSessionManager>()
        .start()?;

    let _ = service_tx.send(service);

    tokio::signal::ctrl_c().await.unwrap();
    Ok(())
}

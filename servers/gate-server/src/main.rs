use std::{collections::HashMap, sync::LazyLock};

use config::ServerConfig;
use const_format::concatcp;
use encryption::SecurityModule;
use session::PlayerSessionManager;
use yixuan_service::{
    ServiceContext, ServiceError,
    config::{ServiceType, load_environment_config},
    network::{NetworkEntityManager, NetworkServer, client::NetworkClient},
};

mod config;
mod encryption;
mod handlers;
mod session;

const SERVICE_TYPE: ServiceType = ServiceType::Gate;
const CONFIG_DIR: &str = "config/20-gate-server/";

#[tokio::main]
async fn main() -> Result<(), ServiceError> {
    static CONFIG: LazyLock<ServerConfig> = LazyLock::new(|| {
        common::config_util::load_or_create(
            concatcp!(CONFIG_DIR, "config.toml"),
            include_str!("../config.default.toml"),
        )
    });

    let env_cfg = load_environment_config();
    let internal_addr = env_cfg.services.get(&SERVICE_TYPE).unwrap().addr;

    common::logging::init_tracing(tracing::Level::DEBUG);
    let (service_tx, listener) = handlers::start_handler_task(internal_addr);

    let service = ServiceContext::new()
        .insert_module(NetworkEntityManager::new(
            listener,
            HashMap::from([(CONFIG.bind_addr, &CONFIG.client_secret_key.xorpad)]),
        ))
        .with_module::<PlayerSessionManager>()
        .configure_module::<SecurityModule>(&CONFIG.rsa_versions)
        .configure_module::<NetworkServer>(vec![CONFIG.bind_addr, internal_addr])
        .configure_module::<NetworkClient>(env_cfg.services)
        .start()?;

    let _ = service_tx.send(service);

    tokio::signal::ctrl_c().await.unwrap();
    Ok(())
}

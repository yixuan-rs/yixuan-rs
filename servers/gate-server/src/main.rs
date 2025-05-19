use std::sync::LazyLock;

use config::ServerConfig;
use const_format::concatcp;
use encryption::SecurityModule;
use session::PlayerSessionManager;
use vivian_service::{
    ServiceContext, ServiceError,
    config::load_environment_config,
    network::{NetworkEntityManager, NetworkServer, client::NetworkClient},
};

mod config;
mod encryption;
mod handlers;
mod session;

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

    common::logging::init_tracing(tracing::Level::DEBUG);
    let (service_tx, listener) = handlers::start_handler_task();

    let service = ServiceContext::new()
        .insert_module(NetworkEntityManager::new(
            listener,
            Some(&CONFIG.client_secret_key.xorpad),
        ))
        .with_module::<PlayerSessionManager>()
        .configure_module::<SecurityModule>(&CONFIG.rsa_versions)
        .configure_module::<NetworkServer>(CONFIG.bind_addr)
        .configure_module::<NetworkClient>(env_cfg.services)
        .start()?;

    let _ = service_tx.send(service);

    tokio::signal::ctrl_c().await.unwrap();
    Ok(())
}

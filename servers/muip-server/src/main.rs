use config::ServerConfig;
use const_format::concatcp;
use http::HttpServer;
use vivian_service::{
    ServiceContext, ServiceError, config::load_environment_config, network::client::NetworkClient,
};

mod config;
mod http;

const CONFIG_DIR: &str = "config/50-muip-server/";

#[derive(thiserror::Error, Debug)]
pub enum StartupError {
    #[error("{0}")]
    Service(#[from] ServiceError),
}

#[tokio::main]
async fn main() -> Result<(), StartupError> {
    common::logging::init_tracing(tracing::Level::DEBUG);
    let env = load_environment_config();

    let config = common::config_util::load_or_create::<ServerConfig>(
        concatcp!(CONFIG_DIR, "config.toml"),
        include_str!("../config.default.toml"),
    );

    let _service = ServiceContext::new()
        .configure_module::<NetworkClient>(env.services)
        .configure_module::<HttpServer>(config)
        .start()?;

    tokio::signal::ctrl_c().await.unwrap();
    Ok(())
}

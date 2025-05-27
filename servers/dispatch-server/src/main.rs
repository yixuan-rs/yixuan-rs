use std::{collections::HashMap, net::SocketAddr, sync::OnceLock};

use axum::{Router, routing::get};
use common::logging::init_tracing;
use config::{ResVersionConfig, ServerConfig, ServerListConfig};
use const_format::concatcp;
use tokio::net::TcpListener;
use tracing::debug;

mod config;
mod handlers;
mod schema;

#[derive(thiserror::Error, Debug)]
pub enum StartupError {
    #[error("failed to bind at {0}, is another instance of this server already running?")]
    BindFailed(SocketAddr),
}

pub struct SharedState {
    pub config: ServerConfig,
    pub server_list: ServerListConfig,
    pub resource_versions: HashMap<String, ResVersionConfig>,
}

const CONFIG_DIR: &str = "config/10-dispatch-server/";

#[tokio::main]
async fn main() -> Result<(), StartupError> {
    static STATE: OnceLock<SharedState> = OnceLock::new();

    init_tracing(tracing::Level::DEBUG);

    let state = STATE.get_or_init(|| SharedState {
        config: common::config_util::load_or_create(
            concatcp!(CONFIG_DIR, "config.toml"),
            include_str!("../config.default.toml"),
        ),
        server_list: common::config_util::load_or_create(
            concatcp!(CONFIG_DIR, "serverlist.toml"),
            include_str!("../serverlist.default.toml"),
        ),
        resource_versions: common::config_util::load_or_create(
            concatcp!(CONFIG_DIR, "resource_versions.toml"),
            include_str!("../resource_versions.default.toml"),
        ),
    });

    let app = Router::new()
        .route("/query_dispatch", get(handlers::query_dispatch))
        .route("/query_gateway", get(handlers::query_gateway))
        .with_state(state);

    let listener = TcpListener::bind(state.config.bind_addr)
        .await
        .map_err(|_| StartupError::BindFailed(state.config.bind_addr))?;

    debug!("server is listening at {}", state.config.bind_addr);
    debug!("known server list:");
    state.server_list.servers.iter().for_each(|s| {
        debug!(
            "{}: {} ({}) with seed {} @ {}:{}",
            s.sid, s.name, s.bind_version, s.dispatch_seed, s.gateway_ip, s.gateway_port,
        )
    });

    axum::serve(listener, app).await.unwrap();
    Ok(())
}

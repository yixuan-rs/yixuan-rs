use std::{collections::HashSet, net::SocketAddr};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub http_bind_addr: SocketAddr,
    pub tokens: HashSet<String>,
}

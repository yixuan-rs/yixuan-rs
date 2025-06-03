use std::{collections::HashMap, net::SocketAddr};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EnvironmentConfig {
    pub services: HashMap<ServiceType, ServiceEndPoint>,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct ServiceEndPoint {
    pub addr: SocketAddr,
    pub uid: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ServiceType {
    Gate,
    Dbgate,
    Game,
    Muip,
}

pub fn load_environment_config() -> EnvironmentConfig {
    common::config_util::load_or_create(
        "config/00-service/environment.toml",
        include_str!("../environment.default.toml"),
    )
}

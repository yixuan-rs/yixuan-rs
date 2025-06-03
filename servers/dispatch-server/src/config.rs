use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
};

use serde::{Deserialize, Deserializer};
use yixuan_encryption::config::{RsaVersion, ScrambledKey};

#[derive(Deserialize)]
pub struct ServerConfig {
    pub bind_addr: SocketAddr,
    #[serde(deserialize_with = "deserialize_rsa_versions")]
    pub rsa_versions: HashMap<u32, RsaVersion>,
    pub client_secret_key: ScrambledKey,
}

#[derive(Deserialize)]
pub struct ServerListConfig {
    pub bound_sid: Option<i32>,
    pub servers: Vec<ServerListItem>,
}

#[derive(Deserialize)]
pub struct ServerListItem {
    pub sid: i32,
    pub bind_versions: HashSet<String>,
    pub name: String,
    pub title: String,
    pub dispatch_url: String,
    pub gateway_ip: String,
    pub gateway_port: u16,
}

#[derive(Deserialize)]
pub struct ResVersionConfig {
    pub dispatch_seed: String,
    pub design_data_url: String,
    pub design_data_revision: String,
    pub design_data_files: String,
    pub game_res_url: String,
    pub game_res_branch: String,
    pub game_audio_revision: String,
    pub game_res_revision: String,
    pub game_res_files: String,
    pub silence_url: String,
    pub silence_revision: String,
    pub silence_files: String,
}

impl ServerListConfig {
    pub fn bound_server(&self) -> Option<&ServerListItem> {
        self.bound_sid
            .and_then(|bound_sid| self.servers.iter().find(|config| config.sid == bound_sid))
    }
}

fn deserialize_rsa_versions<'de, D>(deserializer: D) -> Result<HashMap<u32, RsaVersion>, D::Error>
where
    D: Deserializer<'de>,
{
    HashMap::<String, RsaVersion>::deserialize(deserializer)?
        .into_iter()
        .map(|(key, value)| match key.parse() {
            Ok(key) => Ok((key, value)),
            Err(_) => Err({
                serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(&key),
                    &"a non-negative integer",
                )
            }),
        })
        .collect()
}

use std::{collections::HashMap, net::SocketAddr};

use serde::{Deserialize, Deserializer};
use vivian_encryption::config::{RsaVersion, ScrambledKey};

#[derive(Deserialize)]
pub struct ServerConfig {
    pub bind_addr: SocketAddr,
    #[serde(deserialize_with = "deserialize_rsa_versions")]
    pub rsa_versions: HashMap<u32, RsaVersion>,
    pub client_secret_key: ScrambledKey,
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

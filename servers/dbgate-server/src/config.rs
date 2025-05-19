use std::fmt;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub auth: AuthConfig,
    pub database: ConnectionString,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum AuthConfig {
    #[serde(rename = "unchecked")]
    Unchecked,
    #[serde(rename = "enforced")]
    Enforced {
        sdk_token_verification_url: String,
        permit_login_on_http_error: bool,
    },
}

#[derive(Deserialize, Debug)]
pub struct ConnectionString {
    pub addr: String,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl fmt::Display for ConnectionString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "postgres://{}:{}@{}/{}",
            self.username, self.password, self.addr, self.database
        )
    }
}

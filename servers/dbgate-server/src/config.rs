use std::fmt; // Keep this import for the new Display impl
use serde::Deserialize; // Keep this import

#[derive(Deserialize)]
pub struct ServerConfig {
    pub auth: AuthConfig,
    pub database: ConnectionString, // This remains
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

// Remove the DbType enum as it's no longer needed

#[derive(Deserialize, Debug)]
pub struct ConnectionString {
    pub database_file_path: String,
}

impl ConnectionString {
    pub fn get_db_path(&self) -> &str {
        &self.database_file_path
    }
}

impl std::fmt::Display for ConnectionString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.database_file_path)
    }
}

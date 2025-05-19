use std::collections::HashMap;

use vivian_encryption::config::RsaVersion;
use vivian_proto::PlayerGetTokenScRsp;
use vivian_service::{ConfigurableServiceModule, ServiceModule};

pub struct SecurityModule {
    versions: &'static HashMap<u32, RsaVersion>,
}

#[derive(thiserror::Error, Debug)]
pub enum GenRandKeyError {
    #[error("missing rsa version: {0}")]
    MissingRsaVersion(u32),
    #[error("failed to decrypt client_rand_key (version: {0})")]
    ClientRandKeyDecryption(u32),
    #[error("unexpected client_rand_key size ({0}/8)")]
    ClientRandKeySize(usize),
}

impl SecurityModule {
    pub fn gen_rand_key(
        &self,
        rsp: &mut PlayerGetTokenScRsp,
        client_rsa_ver: u32,
        client_rand_key: &str,
    ) -> Result<u64, GenRandKeyError> {
        let version = self
            .versions
            .get(&client_rsa_ver)
            .ok_or(GenRandKeyError::MissingRsaVersion(client_rsa_ver))?;

        let client_rand_key =
            vivian_encryption::rsa::decrypt(&version.server_private_key, client_rand_key)
                .ok_or(GenRandKeyError::ClientRandKeyDecryption(client_rsa_ver))?;

        let client_rand_key_size = client_rand_key.len();
        let client_rand_key = u64::from_le_bytes(
            client_rand_key
                .try_into()
                .map_err(|_| GenRandKeyError::ClientRandKeySize(client_rand_key_size))?,
        );

        let server_rand_key = rand::RngCore::next_u64(&mut rand::thread_rng());

        rsp.server_rand_key = vivian_encryption::rsa::encrypt(
            &version.client_public_key,
            &server_rand_key.to_le_bytes(),
        );

        rsp.sign = vivian_encryption::rsa::sign(
            &version.server_private_key,
            &server_rand_key.to_le_bytes(),
        );

        Ok(client_rand_key ^ server_rand_key)
    }
}

impl ConfigurableServiceModule for SecurityModule {
    type Config = &'static HashMap<u32, RsaVersion>;

    fn new(_context: &vivian_service::ServiceContext, config: Self::Config) -> Self {
        Self { versions: config }
    }
}

impl ServiceModule for SecurityModule {
    fn run(
        self: std::sync::Arc<Self>,
        _service: std::sync::Arc<vivian_service::ServiceContext>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

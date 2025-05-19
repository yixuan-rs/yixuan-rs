use serde::{Deserialize, Serialize};
use std::{
    sync::{OnceLock, mpsc},
    thread,
    time::Duration,
};
use tokio::sync::oneshot;
use ureq::Agent;
use vivian_service::{ConfigurableServiceModule, ServiceModule};

use crate::config::AuthConfig;

pub struct TokenVerificationModule {
    config: AuthConfig,
    worker_tx: OnceLock<mpsc::Sender<HttpWorkerTask>>,
}

#[derive(Serialize)]
struct RequestData<'data> {
    pub uid: &'data str,
    pub token: &'data str,
}

#[derive(Serialize)]
struct GranterTokenRequest {
    pub data: String,
}

#[derive(Deserialize)]
struct GranterTokenResponse {
    pub retcode: i32,
}

struct HttpWorkerTask {
    account_uid: String,
    token: String,
    awaiter_tx: oneshot::Sender<bool>,
}

impl TokenVerificationModule {
    pub async fn verify(&self, account_uid: &str, token: &str) -> bool {
        match &self.config {
            AuthConfig::Unchecked => true,
            AuthConfig::Enforced { .. } => {
                self.verify_token_via_granter_api(account_uid, token).await
            }
        }
    }

    async fn verify_token_via_granter_api(&self, account_uid: &str, token: &str) -> bool {
        let tx = self.worker_tx.get().unwrap();

        let (awaiter_tx, awaiter_rx) = oneshot::channel();
        let _ = tx.send(HttpWorkerTask {
            account_uid: account_uid.to_string(),
            token: token.to_string(),
            awaiter_tx,
        });

        awaiter_rx.await.unwrap()
    }

    fn http_worker_loop(
        task_rx: mpsc::Receiver<HttpWorkerTask>,
        sdk_url: String,
        permit_on_http_error: bool,
    ) {
        let agent: Agent = Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(2)))
            .build()
            .into();

        while let Ok(task) = task_rx.recv() {
            if let Ok(Ok(rsp)) = agent
                .post(&sdk_url)
                .send_json(&GranterTokenRequest {
                    data: serde_json5::to_string(&RequestData {
                        uid: &task.account_uid,
                        token: &task.token,
                    })
                    .unwrap(),
                })
                .map(|mut rsp| rsp.body_mut().read_json::<GranterTokenResponse>())
            {
                let _ = task.awaiter_tx.send(rsp.retcode == 0);
            } else {
                let _ = task.awaiter_tx.send(permit_on_http_error);
            }
        }
    }
}

impl ConfigurableServiceModule for TokenVerificationModule {
    type Config = AuthConfig;

    fn new(_context: &vivian_service::ServiceContext, config: Self::Config) -> Self {
        Self {
            config,
            worker_tx: OnceLock::new(),
        }
    }
}

impl ServiceModule for TokenVerificationModule {
    fn run(
        self: std::sync::Arc<Self>,
        _service: std::sync::Arc<vivian_service::ServiceContext>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let AuthConfig::Enforced {
            sdk_token_verification_url,
            permit_login_on_http_error,
        } = &self.config
        {
            let (tx, rx) = mpsc::channel();

            let sdk_token_verification_url = sdk_token_verification_url.clone();
            let permit_login_on_http_error = *permit_login_on_http_error;

            thread::spawn(move || {
                Self::http_worker_loop(rx, sdk_token_verification_url, permit_login_on_http_error)
            });

            let _ = self.worker_tx.set(tx);
        }

        Ok(())
    }
}

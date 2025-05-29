use std::{borrow::Cow, collections::HashSet, net::SocketAddr, sync::Arc};

use axum::{
    Json, Router,
    extract::{Query, State},
    routing,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use vivian_proto::{
    head::PacketHead,
    server_only::{GmTalkByMuipReq, GmTalkByMuipRsp},
};
use vivian_service::{
    ConfigurableServiceModule, ServiceContext, ServiceModule,
    config::ServiceType,
    network::{client::NetworkClient, net_util},
};

use crate::config::ServerConfig;

const GM_API_ROUTE: &str = "/api/gm";

pub struct HttpServer {
    pub allowed_tokens: HashSet<String>,
    http_bind_addr: SocketAddr,
}

#[derive(Deserialize, Debug)]
struct GMQuery {
    player_uid: u32,
    command: String,
    token: String,
}

#[derive(Serialize)]
struct GMResponse {
    pub retcode: i32,
    pub message: Cow<'static, str>,
}

async fn on_api_gm_request(
    Query(query): Query<GMQuery>,
    State(state): State<Arc<ServiceContext>>,
) -> Json<GMResponse> {
    if !state
        .resolve::<HttpServer>()
        .allowed_tokens
        .contains(&query.token)
    {
        return Json(GMResponse {
            retcode: 100,
            message: Cow::Borrowed("GM Token mismatch"),
        });
    }

    match state
        .resolve::<NetworkClient>()
        .send_request::<_, GmTalkByMuipRsp>(
            ServiceType::Game,
            PacketHead {
                player_uid: query.player_uid,
                ..Default::default()
            },
            GmTalkByMuipReq { msg: query.command },
        )
        .await
    {
        Ok(rsp) => Json(GMResponse {
            retcode: rsp.retcode,
            message: Cow::Owned(rsp.retmsg),
        }),
        Err(err) => {
            error!("GM Talk request failed: {err}");

            Json(GMResponse {
                retcode: -1,
                message: Cow::Borrowed("Internal Server Error"),
            })
        }
    }
}

impl ConfigurableServiceModule for HttpServer {
    type Config = ServerConfig;

    fn new(_: &vivian_service::ServiceContext, config: Self::Config) -> Self {
        Self {
            allowed_tokens: config.tokens,
            http_bind_addr: config.http_bind_addr,
        }
    }
}

impl ServiceModule for HttpServer {
    fn run(
        self: Arc<Self>,
        service: Arc<vivian_service::ServiceContext>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let app = Router::new()
            .route(GM_API_ROUTE, routing::get(on_api_gm_request))
            .with_state(service);

        let listener = net_util::tcp_bind_sync(self.http_bind_addr)?;
        tokio::spawn(axum::serve(listener, app.into_make_service()).into_future());

        info!(
            "GMTalk API is available at http://{bind_addr}{GM_API_ROUTE}",
            bind_addr = self.http_bind_addr
        );

        Ok(())
    }
}

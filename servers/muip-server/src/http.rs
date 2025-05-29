use std::{borrow::Cow, collections::HashSet, net::SocketAddr, sync::Arc};

use axum::{
    Json, Router,
    extract::State,
    response::{Html, IntoResponse},
    routing,
};
use axum_extra::extract::OptionalQuery;
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

enum Response {
    HtmlForm,
    GMRsp(Json<GMResponse>),
}

async fn on_api_gm_request(
    mut query: OptionalQuery<GMQuery>,
    State(state): State<Arc<ServiceContext>>,
) -> Response {
    let Some(query) = query.take() else {
        return Response::HtmlForm;
    };

    if !state
        .resolve::<HttpServer>()
        .allowed_tokens
        .contains(&query.token)
    {
        return GMResponse {
            retcode: 100,
            message: Cow::Borrowed("GM Token mismatch"),
        }
        .into();
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
        Ok(rsp) => GMResponse {
            retcode: rsp.retcode,
            message: Cow::Owned(rsp.retmsg),
        }
        .into(),
        Err(err) => {
            error!("GM Talk request failed: {err}");

            GMResponse {
                retcode: -1,
                message: Cow::Borrowed("Internal Server Error"),
            }
            .into()
        }
    }
}

impl From<GMResponse> for Response {
    fn from(value: GMResponse) -> Self {
        Self::GMRsp(Json(value))
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::GMRsp(rsp) => rsp.into_response(),
            Self::HtmlForm => Html(include_str!("../res/gm_form.html")).into_response(),
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

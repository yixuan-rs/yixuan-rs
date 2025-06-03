use std::borrow::Cow;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};
use yixuan_encryption::config::RsaVersion;

use crate::{
    SharedState,
    config::ResVersionConfig,
    schema::{
        CdnConfExt, CdnDesignData, CdnGameRes, CdnSilenceData, QueryDispatchRsp, RegionExtension,
        RegionSwitchFunc, ServerDispatchData, ServerGateway, ServerListInfo,
    },
};

#[derive(Deserialize, Debug)]
pub struct QueryDispatchParam {
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct QueryGatewayParam {
    pub version: String,
    pub rsa_ver: u32,
    pub seed: String,
}

pub async fn query_dispatch(
    State(state): State<&'static SharedState>,
    Query(param): Query<QueryDispatchParam>,
) -> Json<QueryDispatchRsp<'static>> {
    debug!("query_dispatch - version: {}", param.version);

    let server_list: Vec<_> = state
        .server_list
        .servers
        .iter()
        .filter(|item| item.bind_versions.contains(&param.version))
        .collect();

    if server_list.is_empty() {
        warn!("No servers for specified version found");
    }

    Json(QueryDispatchRsp {
        retcode: 0,
        region_list: server_list
            .into_iter()
            .map(|item| ServerListInfo {
                retcode: 0,
                name: Cow::Borrowed(&item.name),
                title: Cow::Borrowed(&item.title),
                dispatch_url: Cow::Borrowed(&item.dispatch_url),
                biz: Cow::Borrowed("nap_global"),
                env: 2,
                area: 2,
                ..Default::default()
            })
            .collect(),
        ..Default::default()
    })
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum QueryGatewayRsp {
    NotSupported,
    ErrorCode { retcode: i32 },
    Data { content: String, sign: String },
}

impl QueryGatewayRsp {
    fn encrypt_data(data: String, rsa_version: &RsaVersion) -> Self {
        let data = data.as_bytes();

        Self::Data {
            content: yixuan_encryption::rsa::encrypt(&rsa_version.client_public_key, data),
            sign: yixuan_encryption::rsa::sign(&rsa_version.server_private_key, data),
        }
    }

    pub fn data(data: ServerDispatchData, rsa_version: &RsaVersion) -> Self {
        Self::encrypt_data(serde_json5::to_string(&data).unwrap(), rsa_version)
    }

    pub fn error(retcode: i32, rsa_version: &RsaVersion) -> Self {
        Self::encrypt_data(format!("{{\"retcode\": {retcode}}}"), rsa_version)
    }

    pub fn error_unencrypted(retcode: i32) -> Self {
        Self::ErrorCode { retcode }
    }
}

impl IntoResponse for QueryGatewayRsp {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::NotSupported => Html("599 Service Unavailable").into_response(),
            _ => Json(self).into_response(),
        }
    }
}

pub async fn query_gateway(
    State(state): State<&'static SharedState>,
    Query(param): Query<QueryGatewayParam>,
) -> (StatusCode, QueryGatewayRsp) {
    debug!(
        "query_dispatch - rsa_ver: {}, seed: {}, version: {}",
        param.rsa_ver, param.seed, param.version,
    );

    let Some(rsa_version) = state.config.rsa_versions.get(&param.rsa_ver) else {
        warn!("Unknown RSA version");
        return (StatusCode::OK, QueryGatewayRsp::error_unencrypted(74));
    };

    let Some(server_config) = state.server_list.bound_server() else {
        return (
            StatusCode::from_u16(599).unwrap(),
            QueryGatewayRsp::NotSupported,
        );
    };

    let Some(resource_version) = state.resource_versions.get(&param.version) else {
        warn!("Unknown version");
        return (StatusCode::OK, QueryGatewayRsp::error(75, rsa_version));
    };

    if resource_version.dispatch_seed != param.seed {
        warn!("Unknown dispatch seed");
        return (StatusCode::OK, QueryGatewayRsp::error(75, rsa_version));
    };

    let data = ServerDispatchData {
        region_name: Cow::Borrowed(&server_config.name),
        title: Cow::Borrowed(&server_config.title),
        cdn_conf_ext: Some(build_cdn_conf_ext(resource_version)),
        client_secret_key: Cow::Borrowed(&state.config.client_secret_key.seed),
        gateway: Some(ServerGateway {
            ip: Cow::Borrowed(&server_config.gateway_ip),
            port: server_config.gateway_port,
        }),
        region_ext: Some(RegionExtension {
            func_switch: RegionSwitchFunc {
                is_kcp: 0,
                enable_performance_log: 1,
                enable_operation_log: 1,
                ..Default::default()
            },
            ..Default::default()
        }),
        ..Default::default()
    };

    (StatusCode::OK, QueryGatewayRsp::data(data, rsa_version))
}

fn build_cdn_conf_ext(config: &ResVersionConfig) -> CdnConfExt {
    use std::borrow::Cow::Borrowed;

    CdnConfExt {
        design_data: CdnDesignData {
            base_url: Borrowed(&config.design_data_url),
            data_revision: Borrowed(&config.design_data_revision),
            md5_files: Borrowed(&config.design_data_files),
        },
        game_res: CdnGameRes {
            audio_revision: Borrowed(&config.game_audio_revision),
            base_url: Borrowed(&config.game_res_url),
            branch: Borrowed(&config.game_res_branch),
            md5_files: Borrowed(&config.game_res_files),
            res_revision: Borrowed(&config.game_res_revision),
        },
        silence_data: CdnSilenceData {
            base_url: Borrowed(&config.silence_url),
            md5_files: Borrowed(&config.silence_files),
            silence_revision: Borrowed(&config.silence_revision),
        },
        pre_download: None,
    }
}

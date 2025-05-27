use std::borrow::Cow;

use axum::{
    Json,
    extract::{Query, State},
};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};
use vivian_encryption::config::RsaVersion;

use crate::{
    SharedState,
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
    debug!("query_dispatch - version: {}", param.version,);
    let server_list: Vec<_> = state
        .server_list
        .servers
        .iter()
        .filter(|item| item.bind_version == param.version)
        .collect();
    if server_list.len() == 0 {
        error!("No servers for specified version found");
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
    ErrorCode(#[serde(rename = "retcode")] i32),
    Data { content: String, sign: String },
}

impl QueryGatewayRsp {
    pub fn new(data: ServerDispatchData, rsa_version: &RsaVersion) -> Self {
        let data = serde_json5::to_string(&data).unwrap();
        let data = data.as_bytes();

        Self::Data {
            content: vivian_encryption::rsa::encrypt(&rsa_version.client_public_key, data),
            sign: vivian_encryption::rsa::sign(&rsa_version.server_private_key, data),
        }
    }
}

pub async fn query_gateway(
    State(state): State<&'static SharedState>,
    Query(param): Query<QueryGatewayParam>,
) -> Json<QueryGatewayRsp> {
    debug!(
        "query_dispatch - rsa_ver: {}, seed: {}, version: {}",
        param.rsa_ver, param.seed, param.version,
    );
    let Some(rsa_version) = state.config.rsa_versions.get(&param.rsa_ver) else {
        error!("Unknown RSA version");
        return Json(QueryGatewayRsp::ErrorCode(74));
    };

    let server_config = state.server_list.bound_server();
    if server_config.dispatch_seed != param.seed {
        error!("Unknown dispatch seed");
        return Json(QueryGatewayRsp::ErrorCode(75));
    }

    let cdn_conf_ext = build_cdn_conf_ext(state, &param.version);
    if cdn_conf_ext.is_none() {
        error!("Unknown version");
        return Json(QueryGatewayRsp::ErrorCode(75));
    }

    let data = ServerDispatchData {
        region_name: Cow::Borrowed(&server_config.name),
        title: Cow::Borrowed(&server_config.title),
        cdn_conf_ext,
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

    Json(QueryGatewayRsp::new(data, rsa_version))
}

fn build_cdn_conf_ext<'state>(
    state: &'state SharedState,
    version: &str,
) -> Option<CdnConfExt<'state>> {
    use std::borrow::Cow::Borrowed;

    state
        .resource_versions
        .get(version)
        .map(|config| CdnConfExt {
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
        })
}

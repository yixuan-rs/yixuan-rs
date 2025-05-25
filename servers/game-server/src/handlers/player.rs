use common::time_util;
use tracing::debug;
use vivian_proto::{
    GetAuthkeyCsReq, GetAuthkeyScRsp, GetPlayerTransactionCsReq, GetPlayerTransactionScRsp,
    GetSelfBasicInfoCsReq, GetSelfBasicInfoScRsp, GetServerTimestampCsReq, GetServerTimestampScRsp,
    ModAvatarCsReq, ModAvatarScRsp, ModNickNameCsReq, ModNickNameScRsp,
};

use crate::sync::SyncType;

use super::NetContext;
use vivian_codegen::{handlers, required_state};

pub struct PlayerHandler;

#[handlers]
impl PlayerHandler {
    #[required_state(BasicDataSync)]
    pub fn on_get_self_basic_info_cs_req(
        context: &mut NetContext<'_>,
        _request: GetSelfBasicInfoCsReq,
    ) -> GetSelfBasicInfoScRsp {
        context
            .player
            .sync_helper
            .remove_sync_response(SyncType::BasicData)
    }

    pub fn on_get_server_timestamp_cs_req(
        _context: &mut NetContext<'_>,
        _request: GetServerTimestampCsReq,
    ) -> GetServerTimestampScRsp {
        GetServerTimestampScRsp {
            retcode: 0,
            timestamp: time_util::unix_timestamp_ms(),
            utc_offset: 3,
        }
    }

    pub fn on_mod_nick_name_cs_req(
        context: &mut NetContext<'_>,
        request: ModNickNameCsReq,
    ) -> ModNickNameScRsp {
        if !(3..=15).contains(&request.nick_name.chars().count()) {
            return ModNickNameScRsp { retcode: 1 };
        }

        let basic_model = &mut context.player.basic_model;
        let name_change_times = basic_model.name_change_times.get() + 1;

        basic_model.nick_name.set(&request.nick_name);
        basic_model.name_change_times.set(name_change_times);

        ModNickNameScRsp { retcode: 0 }
    }

    pub fn on_mod_avatar_cs_req(
        context: &mut NetContext<'_>,
        request: ModAvatarCsReq,
    ) -> ModAvatarScRsp {
        debug!("{request:?}");

        if ![2011, 2021].contains(&request.player_avatar_id) {
            return ModAvatarScRsp { retcode: 1 };
        }

        if !context
            .player
            .avatar_model
            .is_avatar_unlocked(request.control_guise_avatar_id)
            && ![2011, 2021].contains(&request.control_guise_avatar_id)
        {
            return ModAvatarScRsp { retcode: 1 };
        }

        let basic_model = &mut context.player.basic_model;
        basic_model.control_avatar_id.set(request.player_avatar_id);
        basic_model
            .control_guise_avatar_id
            .set(request.control_guise_avatar_id);

        ModAvatarScRsp { retcode: 0 }
    }

    pub fn on_get_authkey_cs_req(
        _context: &mut NetContext<'_>,
        _request: GetAuthkeyCsReq,
    ) -> GetAuthkeyScRsp {
        GetAuthkeyScRsp {
            retcode: 0,
            ..Default::default()
        }
    }

    pub fn on_get_player_transaction_cs_req(
        context: &mut NetContext<'_>,
        _request: GetPlayerTransactionCsReq,
    ) -> GetPlayerTransactionScRsp {
        GetPlayerTransactionScRsp {
            retcode: 0,
            transaction: format!("{}-{}", context.player.uid, 1),
        }
    }
}

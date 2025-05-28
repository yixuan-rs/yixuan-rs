use common::time_util;
use tracing::{debug, warn};
use vivian_codegen::{handlers, required_state};
use vivian_logic::system::{ClientSystemType, EOperator};
use vivian_models::InputSetting;
use vivian_proto::{
    BattleReportCsReq, BattleReportScRsp, EndNewbieCsReq, EndNewbieScRsp, FinishNewbieGroupCsReq, FinishNewbieGroupScRsp, GameLogReportCsReq, GameLogReportScRsp, GetMiscDataCsReq, GetMiscDataScRsp, GetNewsStandDataCsReq, GetNewsStandDataScRsp, GetSwitchDataCsReq, GetSwitchDataScRsp, ItemRewardInfo, NewsStandSignCsReq, NewsStandSignScRsp, PlayerOperationCsReq, PlayerOperationScRsp, ReadNewsCsReq, ReadNewsScRsp, ReportUiLayoutPlatformCsReq, ReportUiLayoutPlatformScRsp, SavePlayerAccessoryCsReq, SavePlayerAccessoryScRsp, SavePlayerSystemSettingCsReq, SavePlayerSystemSettingScRsp, SelectPostGirlCsReq, SelectPostGirlScRsp, SyncGlobalVariablesCsReq, SyncGlobalVariablesScRsp, VideoGetInfoCsReq, VideoGetInfoScRsp
};

use crate::{sync::SyncType, util::item_util};

use super::NetContext;

pub struct MiscHandler;

#[handlers]
impl MiscHandler {
    #[required_state(BasicDataSync)]
    pub fn on_video_get_info_cs_req(
        context: &mut NetContext<'_>,
        _request: VideoGetInfoCsReq,
    ) -> VideoGetInfoScRsp {
        context
            .player
            .sync_helper
            .remove_sync_response(SyncType::BasicData)
    }

    pub fn on_save_player_system_setting_cs_req(
        context: &mut NetContext<'_>,
        request: SavePlayerSystemSettingCsReq,
    ) -> SavePlayerSystemSettingScRsp {
        debug!("{request:?}");

        let switch_data = &mut context.player.misc_model.switch;

        if request.params != 0 {
            switch_data
                .setting_switch_map
                .insert(request.r#type, request.params);
        }

        if let Some(info) = request.system_switch_state {
            if info.r#type != 0 {
                switch_data
                    .system_switch_state_map
                    .insert(info.r#type, info.switch_state);
            }
        }

        if let Some(info) = request.input_setting {
            if !info.input_type_map.is_empty() {
                switch_data.input_setting_map.insert(
                    request.r#type,
                    InputSetting {
                        input_type_map: info.input_type_map,
                    },
                );
            }
        }

        request
            .setting_content_map
            .into_iter()
            .for_each(|(ty, value)| {
                switch_data.setting_switch_map.insert(ty, value);
            });

        SavePlayerSystemSettingScRsp { retcode: 0 }
    }

    #[required_state(ExtendDataSync)]
    pub fn on_get_switch_data_cs_req(
        context: &mut NetContext<'_>,
        _request: GetSwitchDataCsReq,
    ) -> GetSwitchDataScRsp {
        context
            .player
            .sync_helper
            .remove_sync_response(SyncType::ExtendData)
    }

    #[required_state(ExtendDataSync)]
    pub fn on_get_misc_data_cs_req(
        context: &mut NetContext<'_>,
        _request: GetMiscDataCsReq,
    ) -> GetMiscDataScRsp {
        context
            .player
            .sync_helper
            .remove_sync_response(SyncType::ExtendData)
    }

    pub fn on_get_news_stand_data_cs_req(
        context: &mut NetContext<'_>,
        _request: GetNewsStandDataCsReq,
    ) -> GetNewsStandDataScRsp {
        GetNewsStandDataScRsp {
            retcode: 0,
            news_stand_data: Some(context.player.misc_model.news_stand.to_client_proto()),
        }
    }

    pub fn on_read_news_cs_req(
        context: &mut NetContext<'_>,
        _request: ReadNewsCsReq,
    ) -> ReadNewsScRsp {
        context
            .player
            .misc_model
            .news_stand
            .news_read_state
            .set(true);

        ReadNewsScRsp { retcode: 0 }
    }

    pub fn on_news_stand_sign_cs_req(
        context: &mut NetContext<'_>,
        _request: NewsStandSignCsReq,
    ) -> NewsStandSignScRsp {
        if !context.player.misc_model.news_stand.can_sign.get() {
            return NewsStandSignScRsp {
                retcode: 1,
                ..Default::default()
            };
        }

        let sign_count = context.player.misc_model.news_stand.sign_count.get() + 1;

        context.player.misc_model.news_stand.can_sign.set(false);

        context
            .player
            .misc_model
            .news_stand
            .sign_count
            .set(sign_count);

        context
            .player
            .misc_model
            .news_stand
            .last_sign_time
            .set(time_util::unix_timestamp_seconds());

        // TODO: hardcoded reward!
        item_util::add_item(context.player, 10, 8888);

        NewsStandSignScRsp {
            retcode: 0,
            sign_count: sign_count as i32,
            last_sign_time: context.player.misc_model.news_stand.last_sign_time.get(),
            reward_list: vec![ItemRewardInfo {
                item_id: 10,
                amount: 8888,
            }],
        }
    }

    pub fn on_select_post_girl_cs_req(
        context: &mut NetContext<'_>,
        request: SelectPostGirlCsReq,
    ) -> SelectPostGirlScRsp {
        let post_girl = &mut context.player.misc_model.post_girl;

        if request
            .post_girl_id_list
            .iter()
            .try_for_each(|id| post_girl.unlocked_items.contains_key(id).then_some(()))
            .is_none()
        {
            return SelectPostGirlScRsp { retcode: 1 };
        }

        post_girl.selected_id.clear();
        post_girl.random_toggle.set(request.post_girl_random_toggle);

        request.post_girl_id_list.into_iter().for_each(|id| {
            post_girl.selected_id.insert(id);
        });

        SelectPostGirlScRsp { retcode: 0 }
    }

    pub fn on_end_newbie_cs_req(
        context: &mut NetContext<'_>,
        request: EndNewbieCsReq,
    ) -> EndNewbieScRsp {
        context
            .player
            .misc_model
            .newbie
            .finished_groups
            .insert(request.group_id as i32);

        EndNewbieScRsp {
            retcode: 0,
            group_id: 0,
        }
    }

    pub fn on_finish_newbie_group_cs_req(
        context: &mut NetContext<'_>,
        request: FinishNewbieGroupCsReq,
    ) -> FinishNewbieGroupScRsp {
        context
            .player
            .misc_model
            .newbie
            .finished_groups
            .insert(request.group_id as i32);

        FinishNewbieGroupScRsp { retcode: 0 }
    }

    pub fn on_player_operation_cs_req(
        context: &mut NetContext<'_>,
        request: PlayerOperationCsReq,
    ) -> PlayerOperationScRsp {
        let (Ok(system), Ok(operator)) = (
            ClientSystemType::try_from(request.system),
            EOperator::try_from(request.operator),
        ) else {
            warn!(
                "invalid player operation received (sys={}, op={}), uid: {}",
                request.system, request.operator, context.player.uid
            );

            return PlayerOperationScRsp { retcode: 1 };
        };

        debug!(
            "PlayerOperation {system:?}::{operator:?}({param})",
            param = request.param
        );

        if operator == EOperator::Enter {
            context
                .player
                .misc_model
                .switch
                .open_system_id
                .insert(system.into());
        }

        PlayerOperationScRsp { retcode: 0 }
    }

    pub fn on_report_ui_layout_platform(
        _context: &mut NetContext<'_>,
        _request: ReportUiLayoutPlatformCsReq,
    ) -> ReportUiLayoutPlatformScRsp {
        ReportUiLayoutPlatformScRsp { retcode: 0 }
    }

    pub fn on_game_log_report_cs_req(
        _context: &mut NetContext<'_>,
        _request: GameLogReportCsReq,
    ) -> GameLogReportScRsp {
        GameLogReportScRsp { retcode: 0 }
    }

    pub fn on_battle_report_cs_req(
        _context: &mut NetContext<'_>,
        _request: BattleReportCsReq,
    ) -> BattleReportScRsp {
        BattleReportScRsp { retcode: 0 }
    }

    pub fn on_sync_global_variables_cs_req(
        _context: &mut NetContext<'_>,
        _request: SyncGlobalVariablesCsReq,
    ) -> SyncGlobalVariablesScRsp {
        SyncGlobalVariablesScRsp { retcode: 0 }
    }

    pub fn on_save_player_accessory_cs_req(
        context: &mut NetContext<'_>,
        request: SavePlayerAccessoryCsReq,
    ) -> SavePlayerAccessoryScRsp {
        debug!("{request:?}");

        let player_accessory_data = &mut context.player.misc_model.player_accessory;

        if let Some(info) = request.player_accessory {
            let Some(player_accessory) = player_accessory_data
                .player_accessory_map
                .get_mut(&info.avatar_id)
                else {
                    return SavePlayerAccessoryScRsp { retcode: 1 }
                };

            player_accessory.avatar_skin_id = info.avatar_skin_id;

            info
                .player_skin_list
                .into_iter()
                .for_each(|skin_info| {
                    let player_skin = player_accessory
                        .player_skin_map
                        .entry(skin_info.player_skin_id)
                        .or_default();

                    player_skin.equipped_accessory_id_list = skin_info
                        .equipped_accessory_id_list
                        .into_iter()
                        .collect();
                });
        }

        SavePlayerAccessoryScRsp { retcode: 0 }
    }
}

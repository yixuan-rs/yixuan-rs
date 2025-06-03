use crate::sync::SyncType;

use super::NetContext;
use tracing::{debug, error};
use yixuan_codegen::{handlers, required_state};
use yixuan_logic::GameState;
use yixuan_proto::{
    AbyssArpeggioGetDataCsReq, AbyssArpeggioGetDataScRsp, AbyssGetDataCsReq, AbyssGetDataScRsp,
    BeginMonsterCardBattleCsReq, BeginMonsterCardBattleScRsp, GetArchiveDataCsReq,
    GetArchiveDataScRsp, GetHollowDataCsReq, GetHollowDataScRsp, GetQuestDataCsReq,
    GetQuestDataScRsp, HollowQuestProgressCsReq, HollowQuestProgressScRsp,
    RestartBigBossBattleCsReq, RestartBigBossBattleScRsp, StartHollowQuestCsReq,
    StartHollowQuestScRsp, StartTrainingQuestCsReq, StartTrainingQuestScRsp,
};

pub struct QuestHandler;

#[handlers]
impl QuestHandler {
    #[required_state(BasicDataSync)]
    pub fn on_get_quest_data_cs_req(
        context: &mut NetContext<'_>,
        _request: GetQuestDataCsReq,
    ) -> GetQuestDataScRsp {
        context
            .player
            .sync_helper
            .remove_sync_response(SyncType::BasicData)
    }

    #[required_state(BasicDataSync)]
    pub fn on_get_archive_data_cs_req(
        context: &mut NetContext<'_>,
        _request: GetArchiveDataCsReq,
    ) -> GetArchiveDataScRsp {
        context
            .player
            .sync_helper
            .remove_sync_response(SyncType::BasicData)
    }

    #[required_state(BasicDataSync)]
    pub fn on_get_hollow_data_cs_req(
        context: &mut NetContext<'_>,
        _request: GetHollowDataCsReq,
    ) -> GetHollowDataScRsp {
        context
            .player
            .sync_helper
            .remove_sync_response(SyncType::BasicData)
    }

    #[required_state(BasicDataSync)]
    pub fn on_abyss_get_data_cs_req(
        context: &mut NetContext<'_>,
        _request: AbyssGetDataCsReq,
    ) -> AbyssGetDataScRsp {
        context
            .player
            .sync_helper
            .remove_sync_response(SyncType::BasicData)
    }

    #[required_state(BasicDataSync)]
    pub fn on_abyss_arpeggio_get_data_cs_req(
        context: &mut NetContext<'_>,
        _request: AbyssArpeggioGetDataCsReq,
    ) -> AbyssArpeggioGetDataScRsp {
        context
            .player
            .sync_helper
            .remove_sync_response(SyncType::BasicData)
    }

    pub fn on_start_training_quest_cs_req(
        context: &mut NetContext<'_>,
        request: StartTrainingQuestCsReq,
    ) -> StartTrainingQuestScRsp {
        if let Some(state) = context.game_state.as_mut() {
            if let GameState::Hall(hall) = state {
                hall.on_exit(context.player);
            }

            context.player.save_scene_snapshot(state);
        }

        let Some(scene_uid) = context
            .player
            .start_training_quest(request.quest_id, &request.avatar_id_list)
        else {
            error!("failed to start quest {}", request.quest_id);
            return StartTrainingQuestScRsp { retcode: 1 };
        };

        *context.game_state = Some(context.player.load_state_from_snapshot(scene_uid));

        StartTrainingQuestScRsp { retcode: 0 }
    }

    pub fn on_start_hollow_quest_cs_req(
        context: &mut NetContext<'_>,
        request: StartHollowQuestCsReq,
    ) -> StartHollowQuestScRsp {
        if let Some(state) = context.game_state.as_mut() {
            if let GameState::Hall(hall) = state {
                hall.on_exit(context.player);
            }

            context.player.save_scene_snapshot(state);
        }

        let Some(scene_uid) = context
            .player
            .start_hollow_quest(request.quest_id, &request.avatar_id_list)
        else {
            error!("failed to start quest {}", request.quest_id);
            return StartHollowQuestScRsp {
                retcode: 1,
                quest_id: 0,
            };
        };

        *context.game_state = Some(context.player.load_state_from_snapshot(scene_uid));

        StartHollowQuestScRsp {
            retcode: 0,
            quest_id: 0,
        }
    }

    pub fn on_hollow_quest_progress_cs_req(
        _context: &mut NetContext<'_>,
        request: HollowQuestProgressCsReq,
    ) -> HollowQuestProgressScRsp {
        debug!("{request:?}");

        HollowQuestProgressScRsp {
            retcode: 0,
            new_progress: 1,
        }
    }

    pub fn on_begin_monster_card_battle_cs_req(
        context: &mut NetContext<'_>,
        request: BeginMonsterCardBattleCsReq,
    ) -> BeginMonsterCardBattleScRsp {
        if let Some(state) = context.game_state.as_mut() {
            if let GameState::Hall(hall) = state {
                hall.on_exit(context.player);
            }

            context.player.save_scene_snapshot(state);
        }

        let Some(scene_uid) = context.player.start_monster_card_battle(
            request.quest_id,
            &request.avatar_id_list,
            request.level,
        ) else {
            error!("failed to start quest {}", request.quest_id);
            return BeginMonsterCardBattleScRsp { retcode: 1 };
        };

        *context.game_state = Some(context.player.load_state_from_snapshot(scene_uid));

        context
            .player
            .quest_model
            .battle_data
            .activity
            .monster_card
            .selected_level
            .set(request.level);

        BeginMonsterCardBattleScRsp { retcode: 0 }
    }

    pub fn on_restart_big_boss_battle_cs_req(
        context: &mut NetContext<'_>,
        _request: RestartBigBossBattleCsReq,
    ) -> RestartBigBossBattleScRsp {
        let Some(state) = context.game_state.as_mut() else {
            return RestartBigBossBattleScRsp { retcode: 1 };
        };

        context.player.save_scene_snapshot(state);

        let GameState::Fight(fight) = state else {
            return RestartBigBossBattleScRsp { retcode: 1 };
        };

        if let Some(big_boss_info) = &fight.dungeon.big_boss_info {
            let avatar_ids: Vec<u32> = fight
                .dungeon
                .avatar_units
                .iter()
                .map(|avatar_unit| avatar_unit.avatar_id)
                .collect();
            let Some(scene_uid) = context.player.start_monster_card_battle(
                fight.dungeon.quest_id,
                avatar_ids.as_slice(),
                big_boss_info.difficulty,
            ) else {
                error!("failed to restart quest {}", fight.dungeon.quest_id);
                return RestartBigBossBattleScRsp { retcode: 1 };
            };

            *context.game_state = Some(context.player.load_state_from_snapshot(scene_uid));

            RestartBigBossBattleScRsp { retcode: 0 }
        } else {
            RestartBigBossBattleScRsp { retcode: 1 }
        }
    }
}

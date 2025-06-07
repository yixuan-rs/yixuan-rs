use crate::util::big_scene_util;

use super::NetContext;
use tracing::{debug, error};
use yixuan_codegen::handlers;
use yixuan_logic::{GameState, dungeon::AvatarUnit, math::Vector3i};
use yixuan_proto::{
    ActiveRollbackPointCsReq, ActiveRollbackPointScRsp, BigSceneAvatarChangeCsReq,
    BigSceneAvatarChangeScRsp, BigSceneTeamData, BigSceneTeamReplaceCsReq,
    BigSceneTeamReplaceScRsp, EnterBigSceneFloorCsReq, EnterBigSceneFloorScRsp,
    EnterFloorDoneCsReq, EnterFloorDoneScRsp, GetBigSceneDataCsReq, GetBigSceneDataScRsp,
    SaveBigSceneVariablesCsReq, SaveBigSceneVariablesScRsp, SyncSceneEntityPositionCsReq,
    SyncSceneEntityPositionScRsp,
    common::{BigSceneAvatarInfo, SceneAvatarState, TeamMemberOperation, TeamMemberSource},
};

pub struct BigSceneHandler;

#[handlers]
impl BigSceneHandler {
    pub fn on_get_big_scene_data_cs_req(
        context: &mut NetContext,
        _: GetBigSceneDataCsReq,
    ) -> GetBigSceneDataScRsp {
        let big_scene_model = &context.player.big_scene_model;

        GetBigSceneDataScRsp {
            retcode: 0,
            big_scene_team: Some(BigSceneTeamData {
                scene_avatar_list: big_scene_model
                    .team
                    .avatars
                    .iter()
                    .map(|(_, avatar)| BigSceneAvatarInfo {
                        avatar_id: avatar.avatar_id,
                        source: TeamMemberSource::Normal.into(),
                        cur_hp: avatar.cur_hp,
                        avatar_unit: avatar.avatar_unit.as_ref().map(AvatarUnit::as_proto),
                        cur_state: SceneAvatarState::Alive.into(),
                        operation: TeamMemberOperation::None.into(),
                    })
                    .collect(),
                cur_scene_avatar_list: big_scene_model
                    .team
                    .cur_avatars
                    .iter()
                    .map(|(_, avatar)| BigSceneAvatarInfo {
                        avatar_id: avatar.avatar_id,
                        source: TeamMemberSource::Normal.into(),
                        cur_hp: avatar.cur_hp,
                        avatar_unit: avatar.avatar_unit.as_ref().map(AvatarUnit::as_proto),
                        cur_state: SceneAvatarState::Alive.into(),
                        operation: TeamMemberOperation::None.into(),
                    })
                    .collect(),
                cur_avatar_id: big_scene_model.team.cur_avatar_id.get(),
            }),
        }
    }

    pub fn on_big_scene_team_replace_cs_req(
        context: &mut NetContext,
        request: BigSceneTeamReplaceCsReq,
    ) -> BigSceneTeamReplaceScRsp {
        debug!("{request:?}");

        if matches!(request.scene_avatar_id_list.len(), 1..=3)
            && request
                .scene_avatar_id_list
                .iter()
                .all(|&id| context.player.avatar_model.is_avatar_unlocked(id))
        {
            big_scene_util::replace_team(context.player, &request.scene_avatar_id_list);
            BigSceneTeamReplaceScRsp { retcode: 0 }
        } else {
            BigSceneTeamReplaceScRsp { retcode: 1 }
        }
    }

    pub fn on_enter_big_scene_floor_cs_req(
        context: &mut NetContext,
        request: EnterBigSceneFloorCsReq,
    ) -> EnterBigSceneFloorScRsp {
        debug!("{request:?}");

        let Some(floor_config) = context.resources.level_world.floors.get(&request.floor_id) else {
            error!("floor with id {} doesn't exist", request.floor_id);
            return EnterBigSceneFloorScRsp { retcode: 1 };
        };

        if request.floor_version != (floor_config.floor_version as u32) {
            error!(
                "floor version mismatch! Client: {}, server: {}, id: {}",
                request.floor_version, floor_config.floor_version, request.floor_id
            );
            return EnterBigSceneFloorScRsp { retcode: 1 };
        }

        if request.floor_md5 != floor_config.floor_md5 {
            error!(
                "floor MD5 mismatch! Client: {}, server: {}, id: {}",
                request.floor_md5, floor_config.floor_md5, request.floor_id
            );
            return EnterBigSceneFloorScRsp { retcode: 1 };
        }

        if context.player.big_scene_model.team.cur_avatars.is_empty() {
            error!("player team is empty, uid: {}", context.player.uid);
            return EnterBigSceneFloorScRsp { retcode: 1 };
        }

        if let Some(state) = context.game_state.as_mut() {
            if let GameState::Hall(hall) = state {
                hall.on_exit(context.player);
            }

            context.player.save_scene_snapshot(state);
        }

        *context.game_state = Some(big_scene_util::load_big_scene_state(
            context.player,
            request.floor_id,
            request.rollback_point.unwrap(),
            request.floor_position.unwrap(),
        ));

        EnterBigSceneFloorScRsp { retcode: 0 }
    }

    pub fn on_enter_floor_done_cs_req(
        _: &mut NetContext,
        request: EnterFloorDoneCsReq,
    ) -> EnterFloorDoneScRsp {
        debug!("{request:?}");
        EnterFloorDoneScRsp { retcode: 0 }
    }

    pub fn on_save_big_scene_variables_cs_req(
        _: &mut NetContext,
        _request: SaveBigSceneVariablesCsReq,
    ) -> SaveBigSceneVariablesScRsp {
        SaveBigSceneVariablesScRsp { retcode: 0 }
    }

    pub fn on_big_scene_avatar_change_cs_req(
        context: &mut NetContext,
        request: BigSceneAvatarChangeCsReq,
    ) -> BigSceneAvatarChangeScRsp {
        debug!("{request:?}");

        let Some(GameState::BigScene(big_scene)) = context.game_state.as_mut() else {
            error!("BigSceneAvatarChangeCsReq received in wrong state");
            return BigSceneAvatarChangeScRsp { retcode: 1 };
        };

        let team = &mut context.player.big_scene_model.team;
        if team.cur_avatars.contains_key(&request.avatar_id) {
            team.cur_avatar_id.set(request.avatar_id);
            big_scene.process_avatar_change(request.avatar_id);

            BigSceneAvatarChangeScRsp { retcode: 0 }
        } else {
            error!("no avatar with id {} in scene team", request.avatar_id);
            BigSceneAvatarChangeScRsp { retcode: 1 }
        }
    }

    pub fn on_sync_scene_entity_position_cs_req(
        context: &mut NetContext,
        request: SyncSceneEntityPositionCsReq,
    ) -> SyncSceneEntityPositionScRsp {
        debug!("{request:?}");

        let Some(GameState::BigScene(big_scene)) = context.game_state.as_mut() else {
            error!("SyncSceneEntityPositionCsReq received in wrong state");
            return SyncSceneEntityPositionScRsp { retcode: 1 };
        };

        request
            .entity_position_list
            .iter()
            .filter_map(|info| {
                Some((
                    info.entity_id,
                    Vector3i::from_proto(&info.position?),
                    Vector3i::from_proto(&info.rotation?),
                ))
            })
            .for_each(|(entity_id, position, rotation)| {
                big_scene.process_sync_entity_position(entity_id, position, rotation);
            });

        SyncSceneEntityPositionScRsp { retcode: 0 }
    }

    pub fn on_active_rollback_point_cs_req(
        context: &mut NetContext,
        request: ActiveRollbackPointCsReq,
    ) -> ActiveRollbackPointScRsp {
        debug!("{request:?}");

        let Some(GameState::BigScene(big_scene)) = context.game_state.as_mut() else {
            error!("ActiveRollbackPointCsReq received in wrong state");
            return ActiveRollbackPointScRsp { retcode: 1 };
        };

        big_scene.active_rollback_point(request.group_id, request.rollback_point.unwrap());
        ActiveRollbackPointScRsp { retcode: 0 }
    }
}

use tracing::{debug, error};
use vivian_codegen::handlers;
use vivian_logic::{
    hall::HallEventGraphError, listener::NotifyListenerExt, math::Transform, GameState,
};
use vivian_proto::{
    EndBattleCsReq, EndBattleScRsp, EndNpcTalkCsReq, EndNpcTalkScRsp, EnterSectionCompleteCsReq,
    EnterSectionCompleteScRsp, EnterSectionCsReq, EnterSectionScRsp, EnterWorldCsReq,
    EnterWorldScRsp, EventGraphOwnerType, FightSettle, HollowEventReportCsReq,
    HollowEventReportScRsp, HollowMoveCsReq, HollowMoveScRsp, HollowTickCsReq, HollowTickScRsp,
    InteractWithUnitCsReq, InteractWithUnitScRsp, LeaveCurSceneCsReq, LeaveCurSceneScRsp,
    RunEventActionCsReq, RunEventActionScRsp, RunEventGraphCsReq, RunEventGraphScRsp,
    SavePosInMainCityCsReq, SavePosInMainCityScRsp, SceneTransitionCsReq, SceneTransitionScRsp,
    SectionRefreshCsReq, SectionRefreshScRsp, SyncLongFightProgressCsReq,
    SyncLongFightProgressScRsp, TriggerHollowEventCsReq, TriggerHollowEventScRsp,
    TriggerInteractCsReq, TriggerInteractScRsp,
};

use super::NetContext;

pub struct WorldHandler;

#[handlers]
impl WorldHandler {
    pub fn on_enter_world_cs_req(
        context: &mut NetContext,
        _request: EnterWorldCsReq,
    ) -> EnterWorldScRsp {
        if !context.player.prepare_to_enter_world() {
            return EnterWorldScRsp { retcode: 1 };
        }

        context
            .player
            .scene_model
            .cur_scene_uid
            .set(context.player.scene_model.default_scene_uid.get());

        *context.game_state = Some(
            context
                .player
                .load_state_from_snapshot(context.player.scene_model.cur_scene_uid.get()),
        );

        context.player.after_enter_world();

        EnterWorldScRsp { retcode: 0 }
    }

    pub fn on_enter_section_complete_cs_req(
        context: &mut NetContext,
        _request: EnterSectionCompleteCsReq,
    ) -> EnterSectionCompleteScRsp {
        if let Some(GameState::Hall(hall)) = context.game_state.as_mut() {
            hall.enter_section_finished();
        }

        EnterSectionCompleteScRsp { retcode: 0 }
    }

    pub fn on_scene_transition_cs_req(
        _context: &mut NetContext,
        _request: SceneTransitionCsReq,
    ) -> SceneTransitionScRsp {
        SceneTransitionScRsp { retcode: 0 }
    }

    pub fn on_save_pos_in_main_city_cs_req(
        context: &mut NetContext,
        request: SavePosInMainCityCsReq,
    ) -> SavePosInMainCityScRsp {
        let Some(GameState::Hall(hall)) = context.game_state.as_ref() else {
            return SavePosInMainCityScRsp { retcode: 0 };
        };

        if hall.section_id == request.section_id {
            if let Some(new_pos) = request.position {
                let pos = &mut context.player.main_city_model.pos_in_main_city;
                let new_pos = Transform::from_proto(&new_pos);

                pos.position.x.set(new_pos.position.0);
                pos.position.y.set(new_pos.position.1);
                pos.position.z.set(new_pos.position.2);

                pos.rotation.x.set(new_pos.rotation.0);
                pos.rotation.y.set(new_pos.rotation.1);
                pos.rotation.z.set(new_pos.rotation.2);

                context.player.main_city_model.transform_id.set("");
            }
        }

        SavePosInMainCityScRsp { retcode: 0 }
    }

    pub fn on_trigger_interact_cs_req(
        _context: &mut NetContext,
        _request: TriggerInteractCsReq,
    ) -> TriggerInteractScRsp {
        TriggerInteractScRsp { retcode: 0 }
    }

    pub fn on_interact_with_unit_cs_req(
        context: &mut NetContext,
        request: InteractWithUnitCsReq,
    ) -> InteractWithUnitScRsp {
        debug!("{request:?}");

        let Some(GameState::Hall(hall)) = context.game_state.as_mut() else {
            error!("InteractWithUnitCsReq received in incorrect state");
            return InteractWithUnitScRsp { retcode: 1 };
        };

        if !hall.interact_with_unit(
            request.npc_tag_id as u32,
            request.interact_id as u32,
            context.player,
        ) {
            return InteractWithUnitScRsp { retcode: 1 };
        };

        InteractWithUnitScRsp { retcode: 0 }
    }

    pub fn on_run_event_graph_cs_req(
        context: &mut NetContext,
        request: RunEventGraphCsReq,
    ) -> RunEventGraphScRsp {
        debug!("{request:?}");

        let Some(GameState::Hall(hall)) = context.game_state.as_mut() else {
            error!("RunEventGraphCsReq received in incorrect state");
            return RunEventGraphScRsp {
                retcode: 1,
                finish_event: false,
            };
        };

        let owner_type = EventGraphOwnerType::try_from(request.owner_type).unwrap_or_default();

        match hall.run_event_graph(owner_type, request.event_id, context.player) {
            Ok(_) | Err(HallEventGraphError::NotRunning(_, _)) => (),
            Err(err) => {
                error!("RunEventGraphCsReq failed: {err}");
                return RunEventGraphScRsp {
                    retcode: 1,
                    finish_event: false,
                };
            }
        }

        if let Some(notify) = hall.remove_if_finished(owner_type, request.event_id) {
            context.notifies.add(notify);

            RunEventGraphScRsp {
                retcode: 0,
                finish_event: true,
            }
        } else {
            RunEventGraphScRsp {
                retcode: 0,
                finish_event: false,
            }
        }
    }

    pub fn on_run_event_action_cs_req(
        context: &mut NetContext,
        request: RunEventActionCsReq,
    ) -> RunEventActionScRsp {
        debug!("{request:?}");

        if let Some(GameState::Hall(hall)) = context.game_state.as_mut() {
            let owner_type = EventGraphOwnerType::try_from(request.owner_type).unwrap_or_default();

            match hall.run_event_graph(owner_type, request.event_id, context.player) {
                Ok(_) | Err(HallEventGraphError::NotRunning(_, _)) => (),
                Err(err) => {
                    error!("RunEventActionCsReq failed: {err}");
                    return RunEventActionScRsp {
                        retcode: 1,
                        finish_event: false,
                    };
                }
            }

            if let Some(notify) = hall.remove_if_finished(owner_type, request.event_id) {
                context.notifies.add(notify);

                RunEventActionScRsp {
                    retcode: 0,
                    finish_event: true,
                }
            } else {
                RunEventActionScRsp {
                    retcode: 0,
                    finish_event: false,
                }
            }
        } else if let Some(GameState::Hollow(hollow)) = context.game_state.as_mut() {
            let owner_type = EventGraphOwnerType::try_from(request.owner_type).unwrap_or_default();

            if hollow.run_event_graph(
                owner_type,
                request.event_id,
                context.player,
                &request.action_body,
            ) {
                RunEventActionScRsp {
                    retcode: 0,
                    finish_event: false,
                }
            } else {
                RunEventActionScRsp {
                    retcode: 1,
                    finish_event: false,
                }
            }
        } else {
            error!("RunEventActionCsReq received in incorrect state");
            RunEventActionScRsp {
                retcode: 1,
                finish_event: false,
            }
        }
    }

    pub fn on_enter_section_cs_req(
        context: &mut NetContext,
        request: EnterSectionCsReq,
    ) -> EnterSectionScRsp {
        if let Some(state) = context.game_state.as_mut() {
            if let GameState::Hall(hall) = state {
                hall.on_exit(context.player);
            }

            context.player.save_scene_snapshot(state);
        }

        if !context
            .player
            .switch_hall_section(request.section_id, request.transform_id)
        {
            return EnterSectionScRsp { retcode: 1 };
        }

        *context.game_state = Some(
            context
                .player
                .load_state_from_snapshot(context.player.scene_model.cur_scene_uid.get()),
        );

        EnterSectionScRsp { retcode: 0 }
    }

    pub fn on_end_npc_talk_cs_req(
        _context: &mut NetContext,
        _request: EndNpcTalkCsReq,
    ) -> EndNpcTalkScRsp {
        EndNpcTalkScRsp { retcode: 0 }
    }

    pub fn on_end_battle_cs_req(
        context: &mut NetContext,
        request: EndBattleCsReq,
    ) -> EndBattleScRsp {
        debug!("{request:?}");

        if let Some(result) = request.fight_result {
            match context.game_state.as_mut() {
                Some(GameState::LongFight(fight)) => fight.end_battle(
                    result.result,
                    result.battle_statistic.unwrap_or_default(),
                    context.player,
                ),
                Some(GameState::Fight(fight)) => fight.end_battle(
                    result.result,
                    result.battle_statistic.unwrap_or_default(),
                    context.player,
                ),
                _ => error!("received EndBattleCsReq in invalid state"),
            }
        }

        EndBattleScRsp {
            retcode: 0,
            fight_settle: Some(FightSettle::default()),
        }
    }

    pub fn on_sync_long_fight_progress_cs_req(
        _context: &mut NetContext,
        _request: SyncLongFightProgressCsReq,
    ) -> SyncLongFightProgressScRsp {
        SyncLongFightProgressScRsp { retcode: 0 }
    }

    pub fn on_leave_cur_scene_cs_req(
        context: &mut NetContext,
        _request: LeaveCurSceneCsReq,
    ) -> LeaveCurSceneScRsp {
        let cur_scene_uid = context.player.scene_model.cur_scene_uid.get();
        context.player.scene_model.clear_scene_data(cur_scene_uid);

        *context.game_state = Some(
            context
                .player
                .load_state_from_snapshot(context.player.scene_model.default_scene_uid.get()),
        );

        LeaveCurSceneScRsp { retcode: 0 }
    }

    pub fn on_section_refresh_cs_req(
        _context: &mut NetContext,
        _request: SectionRefreshCsReq,
    ) -> SectionRefreshScRsp {
        SectionRefreshScRsp {
            retcode: 0,
            refresh_status: 1,
        }
    }

    pub fn on_hollow_tick_cs_req(
        _context: &mut NetContext,
        request: HollowTickCsReq,
    ) -> HollowTickScRsp {
        debug!("{request:?}");

        HollowTickScRsp { retcode: 0 }
    }

    pub fn on_hollow_move_cs_req(
        context: &mut NetContext,
        request: HollowMoveCsReq,
    ) -> HollowMoveScRsp {
        let Some(GameState::Hollow(hollow)) = context.game_state.as_mut() else {
            error!("HollowMoveCsReq received in wrong state");
            return HollowMoveScRsp {
                retcode: 1,
                ..Default::default()
            };
        };

        debug!("{request:?}");

        let (new_position, section_id) =
            hollow.handle_player_move(request.move_path, context.player);

        HollowMoveScRsp {
            retcode: 0,
            section_id,
            new_position: Some(new_position),
        }
    }

    pub fn on_hollow_event_report_cs_req(
        _context: &mut NetContext,
        request: HollowEventReportCsReq,
    ) -> HollowEventReportScRsp {
        debug!("{request:?}");

        HollowEventReportScRsp { retcode: 0 }
    }

    pub fn on_trigger_hollow_event_cs_req(
        context: &mut NetContext,
        _: TriggerHollowEventCsReq,
    ) -> TriggerHollowEventScRsp {
        let Some(GameState::Hollow(hollow)) = context.game_state.as_mut() else {
            error!("TriggerHollowEventCsReq received in wrong state");
            return TriggerHollowEventScRsp { retcode: 1 };
        };

        hollow.chessboard.retrigger_cur_event(context.player);

        TriggerHollowEventScRsp { retcode: 0 }
    }
}

actions! {
    ActionCreateNpc;
    ActionChangeInteract;
    ActionSetMainCityObjectState;
    ActionForceRefresh;
    ActionResetEvent;
    ActionSetQuestPhase;
    ActionRemoveMainCityQuestInteract;
    ActionRemoveMainCityQuestNpc;
    ActionChangeBackSceneInfo;
    ActionSetBGM;
    ActionSetMainCityTime;
    ConfigUnlockHollowQuest;
}

fn execute_action_create_npc(context: &mut EventGraphContext<'_>, cfg: &ActionCreateNpc) {
    if cfg.tag_id != 0 {
        context.hall.create_npc(cfg.tag_id);
    }

    cfg.tag_ids
        .iter()
        .for_each(|&tag_id| context.hall.create_npc(tag_id));
}

fn execute_action_change_interact(context: &mut EventGraphContext<'_>, cfg: &ActionChangeInteract) {
    context.hall.change_interact(
        cfg.interact_id as i32,
        &cfg.tag_ids,
        &Scale::new_by_config_struct(&cfg.interact_scale),
        cfg.participators.clone(),
        false,
    );
}

fn execute_action_set_main_city_object_state(
    context: &mut EventGraphContext<'_>,
    cfg: &ActionSetMainCityObjectState,
) {
    cfg.object_state.iter().for_each(|(&tag, &state)| {
        context.hall.main_city_objects_state.insert(tag, state);
    });
}

fn execute_action_force_refresh(context: &mut EventGraphContext<'_>, _: &ActionForceRefresh) {
    context.hall.force_refresh();
}

fn execute_action_reset_event(context: &mut EventGraphContext<'_>, _: &ActionResetEvent) {
    context
        .hall
        .already_executed_events
        .remove(&context.event_uid);
}

fn execute_action_change_back_scene_info(
    context: &mut EventGraphContext<'_>,
    cfg: &ActionChangeBackSceneInfo,
) {
    context
        .listener
        .change_back_scene_info(cfg.section_id, cfg.transform.clone());
}

fn execute_action_set_quest_phase(
    context: &mut EventGraphContext<'_>,
    action: &ActionSetQuestPhase,
) {
    if let Some(phase) = context.hall.main_city_quests.get_mut(&action.quest_id) {
        if *phase != action.target_phase {
            *phase = action.target_phase;

            if action.target_phase == EQuestState::Finished {
                context.hall.attach_graph(
                    GraphReference::Quest(action.quest_id * 10 + 1),
                    context.listener,
                );

                for added_quest_id in context.listener.main_city_quest_finished(action.quest_id) {
                    context
                        .hall
                        .main_city_quests
                        .insert(added_quest_id, EQuestState::InProgress);

                    context
                        .hall
                        .attach_graph(GraphReference::Quest(added_quest_id), context.listener);
                }
            }
        }
    }
}

fn execute_config_unlock_hollow_quest(
    context: &mut EventGraphContext<'_>,
    action: &ConfigUnlockHollowQuest,
) {
    context.listener.unlock_hollow_quest(action.quest_id);
}

fn execute_action_remove_main_city_quest_interact(
    context: &mut EventGraphContext<'_>,
    _: &ActionRemoveMainCityQuestInteract,
) {
    context.hall.reset_interacts();
}

fn execute_action_remove_main_city_quest_npc(
    context: &mut EventGraphContext<'_>,
    _: &ActionRemoveMainCityQuestNpc,
) {
    context.hall.remove_quest_npc();
}

fn execute_action_set_b_g_m(context: &mut EventGraphContext<'_>, cfg: &ActionSetBGM) {
    context.hall.bgm_id = cfg.main_city_music_id;
}

fn execute_action_set_main_city_time(
    context: &mut EventGraphContext<'_>,
    cfg: &ActionSetMainCityTime,
) {
    context.hall.set_time_period(cfg.time_period);
}

macro_rules! actions {
    ($($cfg:ident;)*) => {
        pub fn execute(context: &mut EventGraphContext<'_>, cfg: &::config::ConfigEventAction) {
            ::paste::paste!(match cfg {
                $(::config::ConfigEventAction::$cfg(cfg) => [<execute_ $cfg:snake>](context, cfg),)*
                _ => (),
            });
        }
    };
}

use actions;
use config::{
    ActionChangeBackSceneInfo, ActionChangeInteract, ActionCreateNpc, ActionForceRefresh,
    ActionRemoveMainCityQuestInteract, ActionRemoveMainCityQuestNpc, ActionResetEvent,
    ActionSetBGM, ActionSetMainCityObjectState, ActionSetMainCityTime, ActionSetQuestPhase,
    ConfigUnlockHollowQuest, EQuestState, GraphReference,
};

use crate::math::Scale;

use super::EventGraphContext;

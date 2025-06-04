use tracing::info;
use yixuan_proto::{EnterSceneScNotify, common::LogBattleStatistics};

use crate::{
    battle::{BattleLevel, CollectRewardError}, dungeon::Dungeon, listener::{LogicEventListener, NotifyListener, NotifyListenerExt}, scene::SceneType, LogicResources
};

pub struct GameLongFightState {
    pub resources: LogicResources,
    pub battle: BattleLevel,
    pub dungeon: Dungeon,
    pub scene_id: u32,
    has_sent_initial_scene_notify: bool,
}

impl GameLongFightState {
    pub fn new(scene_id: u32, resources: LogicResources, dungeon: Dungeon, listener: &dyn LogicEventListener) -> Self {
        Self {
            scene_id,
            battle: BattleLevel::new(scene_id, &resources, listener),
            dungeon,
            resources,
            has_sent_initial_scene_notify: false,
        }
    }

    pub fn collect_rewards(&mut self, index_list: &[u32], listener: &mut dyn LogicEventListener) -> Result<(), CollectRewardError> {
        for &index in index_list {
            match self.battle.collect_reward(index, listener) {
                Ok(()) | Err(CollectRewardError::AlreadyCollected(_)) => (),
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }

    pub fn end_battle(
        &mut self,
        result: i32,
        statistic: LogBattleStatistics,
        listener: &mut dyn LogicEventListener,
    ) {
        info!(
            "battle at {scene_id} is over, result: {result}",
            scene_id = self.scene_id
        );

        self.dungeon.update_statistic(statistic);
        self.dungeon.finish_main_dungeon_quest(result);

        if result == 1 && self.dungeon.quest_type == 3 {
            listener.hollow_quest_finished(self.dungeon.quest_id);
        }
    }

    // Common

    pub fn scene_type(&self) -> SceneType {
        SceneType::LongFight
    }

    pub fn flush_notifies(&mut self, listener: &mut dyn NotifyListener) {
        if !self.has_sent_initial_scene_notify {
            self.has_sent_initial_scene_notify = true;

            listener.add(EnterSceneScNotify {
                scene: Some(self.client_scene_data_proto()),
                dungeon: Some(self.dungeon.as_client_proto()),
            });
        }

        self.dungeon.flush_dungeon_quest_notifies(listener);
    }

    pub fn client_scene_data_proto(&self) -> yixuan_proto::SceneData {
        yixuan_proto::SceneData {
            scene_id: self.scene_id,
            scene_type: self.scene_type().into(),
            play_type: self.battle.play_type.into(),
            long_fight_scene_data: Some(yixuan_proto::LongFightSceneData {
                scene_reward: Some(self.battle.client_scene_reward_info()),
                scene_perform: Some(self.battle.perform.as_client_proto()),
                scene_progress: Some(yixuan_proto::LongFightProgressInfo {
                    quest_cond_progress: Some(yixuan_proto::QuestCondProgress::default()),
                    quest_variables_info: Some(yixuan_proto::QuestVariablesInfo::default()),
                }),
            }),
            ..Default::default()
        }
    }
}

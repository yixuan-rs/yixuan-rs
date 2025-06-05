use tracing::info;
use yixuan_proto::{EnterSceneScNotify, common::LogBattleStatistics};

use crate::{
    LogicResources,
    battle::{BattleLevel, CollectRewardError},
    dungeon::Dungeon,
    listener::{LogicEventListener, NotifyListener, NotifyListenerExt},
    scene::{ELocalPlayType, SceneType},
};

pub struct GameFightState {
    pub resources: LogicResources,
    pub battle: BattleLevel,
    pub dungeon: Dungeon,
    pub scene_id: u32,
    pub play_type: ELocalPlayType,
    has_sent_initial_scene_notify: bool,
}

impl GameFightState {
    pub fn new(
        scene_id: u32,
        play_type: ELocalPlayType,
        resources: LogicResources,
        dungeon: Dungeon,
        listener: &dyn LogicEventListener,
    ) -> Self {
        Self {
            battle: BattleLevel::new(scene_id, &resources, listener),
            scene_id,
            play_type,
            resources,
            dungeon,
            has_sent_initial_scene_notify: false,
        }
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

        if self.dungeon.quest_type == 10 {
            self.dungeon.finish_main_dungeon_quest(result);

            if result == 1 {
                listener.hollow_quest_finished(self.dungeon.quest_id);
            }
        }
    }

    pub fn collect_rewards(
        &mut self,
        index_list: &[u32],
        listener: &mut dyn LogicEventListener,
    ) -> Result<(), CollectRewardError> {
        for &index in index_list {
            match self.battle.collect_reward(index, listener) {
                Ok(()) | Err(CollectRewardError::AlreadyCollected(_)) => (),
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }

    pub fn scene_type(&self) -> SceneType {
        SceneType::Fight
    }

    pub fn flush_notifies(&mut self, listener: &mut dyn NotifyListener) {
        if !self.has_sent_initial_scene_notify {
            self.has_sent_initial_scene_notify = true;

            listener.add(EnterSceneScNotify {
                scene: Some(self.client_scene_data_proto()),
                dungeon: Some(self.dungeon.as_client_proto()),
            });
        }
    }

    pub fn client_scene_data_proto(&self) -> yixuan_proto::SceneData {
        yixuan_proto::SceneData {
            scene_id: self.scene_id,
            play_type: self.play_type.into(),
            scene_type: self.scene_type().into(),
            fight_scene_data: Some(yixuan_proto::FightSceneData {
                scene_reward: Some(self.battle.client_scene_reward_info()),
                scene_perform: Some(self.battle.perform.as_client_proto()),
            }),
            ..Default::default()
        }
    }
}

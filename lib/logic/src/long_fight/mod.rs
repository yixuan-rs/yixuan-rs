use tracing::info;
use yixuan_proto::{EnterSceneScNotify, common::LogBattleStatistics};

use crate::{
    LogicResources,
    dungeon::Dungeon,
    listener::{LogicEventListener, NotifyListener, NotifyListenerExt},
    scene::{ELocalPlayType, SceneType},
};

pub struct GameLongFightState {
    pub resources: LogicResources,
    pub dungeon: Dungeon,
    pub scene_id: u32,
    pub play_type: ELocalPlayType,
    pub time_period: String,
    pub weather: String,
    has_sent_initial_scene_notify: bool,
}

impl GameLongFightState {
    pub fn new(
        scene_id: u32,
        play_type: ELocalPlayType,
        resources: LogicResources,
        dungeon: Dungeon,
    ) -> Self {
        Self {
            scene_id,
            play_type,
            resources,
            dungeon,
            time_period: String::from("Morning"),
            weather: String::from("SunShine"),
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
            play_type: self.play_type.into(),
            scene_type: self.scene_type().into(),
            long_fight_scene_data: Some(yixuan_proto::LongFightSceneData {
                scene_reward: Some(yixuan_proto::SceneRewardInfo::default()),
                scene_perform: Some(yixuan_proto::ScenePerformInfo {
                    time: self.time_period.clone(),
                    weather: self.weather.clone(),
                }),
                scene_progress: Some(yixuan_proto::LongFightProgressInfo {
                    quest_cond_progress: Some(yixuan_proto::QuestCondProgress::default()),
                    quest_variables_info: Some(yixuan_proto::QuestVariablesInfo::default()),
                }),
            }),
            ..Default::default()
        }
    }
}

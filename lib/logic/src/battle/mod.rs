use std::collections::HashMap;

use config::{ETimePeriodType, EWeatherType};

use crate::{LogicResources, listener::LogicEventListener};

pub struct BattleLevel {
    pub perform: LevelPerform,
    pub special_rewards: Vec<(u32, Vec<(u32, u32)>)>,
}

#[derive(Default)]
pub struct LevelPerform {
    pub time_period: ETimePeriodType,
    pub weather_type: EWeatherType,
}

#[derive(thiserror::Error, Debug)]
pub enum CollectRewardError {
    #[error("no reward at index {0}")]
    InvalidIndex(u32),
    #[error("reward at index {0} is already collected")]
    AlreadyCollected(u32),
}

impl BattleLevel {
    pub fn new(
        battle_event_id: u32,
        res: &LogicResources,
        listener: &dyn LogicEventListener,
    ) -> Self {
        let perform = if let Some(sub_area_data) = res
            .template_collection
            .sub_area_data_template_tb()
            .find(|tmpl| tmpl.battle_event_id() == battle_event_id)
        {
            LevelPerform {
                time_period: sub_area_data.time_period().unwrap_or_default().into(),
                weather_type: sub_area_data.weather().unwrap_or_default().into(),
            }
        } else {
            LevelPerform::default()
        };

        let template = res
            .template_collection
            .battle_event_config_template_tb()
            .find(|tmpl| tmpl.id() == battle_event_id)
            .unwrap();

        let special_rewards = template
            .special_reward()
            .unwrap_or_default()
            .iter()
            .filter(|id| !listener.has_gained_once_reward(*id))
            .filter_map(|id| {
                Some((
                    id,
                    res.template_collection
                        .once_reward_template_tb()
                        .find_map(|tmpl| {
                            (tmpl.reward_id() == id).then(|| {
                                tmpl.reward_list()
                                    .unwrap_or_default()
                                    .iter()
                                    .map(|reward| (reward.item_id(), reward.amount()))
                                    .collect::<Vec<_>>()
                            })
                        })?,
                ))
            })
            .collect();

        Self {
            perform,
            special_rewards,
        }
    }

    pub fn collect_reward(
        &mut self,
        index: u32,
        listener: &mut dyn LogicEventListener,
    ) -> Result<(), CollectRewardError> {
        let (reward_id, _) = self
            .special_rewards
            .get_mut(index as usize)
            .ok_or(CollectRewardError::InvalidIndex(index))?;

        if !listener.has_gained_once_reward(*reward_id) {
            listener.give_once_reward(*reward_id);
            Ok(())
        } else {
            Err(CollectRewardError::AlreadyCollected(index))
        }
    }

    pub fn client_scene_reward_info(&self) -> yixuan_proto::SceneRewardInfo {
        yixuan_proto::SceneRewardInfo {
            special_drop_reward_list: self
                .special_rewards
                .iter()
                .map(|(id, items)| yixuan_proto::BattleRewardInfo {
                    reward_id: *id,
                    battle_reward_map: HashMap::from([(
                        0,
                        yixuan_proto::DropItems {
                            drop_items: items.iter().copied().collect(),
                        },
                    )]),
                })
                .collect(),
            ..Default::default()
        }
    }
}

impl LevelPerform {
    pub fn as_client_proto(&self) -> yixuan_proto::ScenePerformInfo {
        yixuan_proto::ScenePerformInfo {
            time: self.time_period.to_string(),
            weather: self.weather_type.to_string(),
        }
    }
}

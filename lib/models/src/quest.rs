use std::collections::{HashMap, HashSet};

use config::EQuestState;
use yixuan_codegen::Property;
use yixuan_logic::dungeon::EQuestType;
use yixuan_proto::QuestSceneInfo;

use crate::property::{PrimitiveProperty, Property, PropertyHashMap, PropertyHashSet};

use super::*;

#[derive(Default)]
pub struct DoubleEliteProgress {}

#[derive(Property, Default)]
pub struct PropertyDoubleEliteData {
    pub unlocked_levels: PropertyHashSet<u32>,
    pub progress: PropertyHashMap<u32, DoubleEliteProgress>,
    pub selected_difficulty: PrimitiveProperty<u32>,
}

#[derive(Property, Default)]
pub struct PropertyBossBattleData {
    pub selected_level: PrimitiveProperty<u32>,
    pub unlocked_levels: PropertyHashSet<u32>,
}

#[derive(Property, Default)]
pub struct PropertyActivityBattleData {
    pub boss_battle: PropertyBossBattleData,
    pub double_elite: PropertyDoubleEliteData,
}

#[derive(Property, Default)]
pub struct PropertyBattleData {
    pub activity: PropertyActivityBattleData,
}

#[derive(Model)]
pub struct QuestModel {
    pub quest_collections: PropertyHashMap<EQuestType, QuestCollection>,
    pub new_hollow_quests: PropertyHashSet<u32>,
    pub battle_data: PropertyBattleData,
}

pub struct TrackQuest {
    pub cur_main_quest_id: u32,
    pub cur_track_quest_id: u32,
    pub cur_track_special_quest_id: u32,
}

#[derive(Default)]
pub struct QuestCollection {
    pub finished_quests: HashSet<u32>,
    pub quests: HashMap<u32, Quest>,
    pub track_quest: Option<TrackQuest>,
}

pub struct MainCityQuestExt {
    pub track_npc_id: HashSet<u32>,
}

pub struct SpecialQuestExt {
    pub prev_quest_id: u32,
    pub cur_quest_id: u32,
}

pub struct Quest {
    pub id: u32,
    pub state: i32,
    pub unlock_time: i64,
    pub in_progress_time: i64,
    pub progress: u32,
    pub finish_condition_progress: HashMap<u32, u32>,
    pub main_city_ext: Option<MainCityQuestExt>,
    pub special_ext: Option<SpecialQuestExt>,
}

impl QuestModel {
    pub fn is_quest_state(&self, quest_id: u32, state: EQuestState) -> bool {
        if state == EQuestState::Finished {
            self.quest_collections
                .iter()
                .any(|(_, qc)| qc.finished_quests.contains(&quest_id))
        } else {
            self.quest_collections.iter().any(|(_, qc)| {
                qc.quests
                    .get(&quest_id)
                    .map(|q| q.state == i32::from(state))
                    .unwrap_or(false)
            })
        }
    }

    pub fn get_or_insert_collection(&mut self, quest_type: EQuestType) -> &mut QuestCollection {
        if !self.quest_collections.contains_key(&quest_type) {
            self.quest_collections
                .insert(quest_type, QuestCollection::default());
        }

        self.quest_collections.get_mut(&quest_type).unwrap()
    }

    pub fn load_from_pb(pb: QuestData) -> Self {
        Self {
            quest_collections: pb
                .quest_collection_list
                .into_iter()
                .filter_map(|collection| {
                    Some((
                        EQuestType::try_from(collection.quest_type).ok()?,
                        QuestCollection {
                            finished_quests: collection.finish_quest_id_list.into_iter().collect(),
                            quests: collection
                                .quest_list
                                .into_iter()
                                .map(|quest| {
                                    (
                                        quest.id,
                                        Quest {
                                            id: quest.id,
                                            state: quest.state,
                                            unlock_time: quest.unlock_time,
                                            in_progress_time: quest.in_progress_time,
                                            progress: quest.progress,
                                            finish_condition_progress: quest
                                                .finish_condition_progress,
                                            main_city_ext: quest.main_city_quest_info.map(|info| {
                                                MainCityQuestExt {
                                                    track_npc_id: info
                                                        .track_npc_id_list
                                                        .into_iter()
                                                        .collect(),
                                                }
                                            }),
                                            special_ext: quest.special_quest_info.map(|info| {
                                                SpecialQuestExt {
                                                    prev_quest_id: info.prev_quest_id,
                                                    cur_quest_id: info.cur_quest_id,
                                                }
                                            }),
                                        },
                                    )
                                })
                                .collect(),
                            track_quest: collection.track_quest.map(|info| TrackQuest {
                                cur_main_quest_id: info.cur_main_quest_id,
                                cur_track_quest_id: info.cur_track_quest_id,
                                cur_track_special_quest_id: info.cur_track_special_quest_id,
                            }),
                        },
                    ))
                })
                .collect(),
            new_hollow_quests: PropertyHashSet::default(),
            battle_data: pb
                .battle_data
                .map(|data| PropertyBattleData {
                    activity: data
                        .activity
                        .map(|activity_battle_data| PropertyActivityBattleData {
                            boss_battle: activity_battle_data
                                .boss_battle
                                .map(|data| PropertyBossBattleData {
                                    unlocked_levels: data.unlocked_levels.into_iter().collect(),
                                    selected_level: data.selected_level.into(),
                                })
                                .unwrap_or_default(),
                            double_elite: activity_battle_data
                                .double_elite
                                .map(|data| PropertyDoubleEliteData {
                                    unlocked_levels: data.unlocked_levels.into_iter().collect(),
                                    progress: data
                                        .progress_list
                                        .into_iter()
                                        .map(|progress| (progress.quest_id, DoubleEliteProgress {}))
                                        .collect(),
                                    selected_difficulty: data.selected_difficulty.into(),
                                })
                                .unwrap_or_default(),
                        })
                        .unwrap_or_default(),
                })
                .unwrap_or_default(),
        }
    }
}

impl Quest {
    pub fn to_client_proto(&self) -> yixuan_proto::QuestInfo {
        yixuan_proto::QuestInfo {
            id: self.id,
            state: self.state,
            unlock_time: self.unlock_time,
            progress: self.progress,
            in_progress_time: self.in_progress_time,
            finish_condition_progress: self.finish_condition_progress.clone(),
            track_info: self
                .main_city_ext
                .as_ref()
                .map(|ext| yixuan_proto::TrackQuestNpcInfo {
                    track_npc_id_list: ext.track_npc_id.iter().copied().collect(),
                    quest_scene_info: Some(QuestSceneInfo {
                        quest_scene_unit_id_list: ext.track_npc_id.iter().copied().collect(),
                    }),
                }),
            special_quest_info: self.special_ext.as_ref().map(|ext| {
                yixuan_proto::SpecialQuestInfo {
                    previous_main_city_quest_id: ext.prev_quest_id,
                    cur_main_city_quest_id: ext.cur_quest_id,
                }
            }),
        }
    }
}

impl Saveable for QuestModel {
    fn save_to_pb(&self, root: &mut yixuan_proto::server_only::PlayerData) {
        root.quest = Some(QuestData {
            quest_collection_list: self
                .quest_collections
                .iter()
                .map(|(&quest_type, collection)| QuestCollectionInfo {
                    quest_type: quest_type.into(),
                    finish_quest_id_list: collection.finished_quests.iter().copied().collect(),
                    quest_list: collection
                        .quests
                        .values()
                        .map(|quest| QuestInfo {
                            id: quest.id,
                            state: quest.state,
                            unlock_time: quest.unlock_time,
                            in_progress_time: quest.in_progress_time,
                            progress: quest.progress,
                            finish_condition_progress: quest.finish_condition_progress.clone(),
                            main_city_quest_info: quest.main_city_ext.as_ref().map(|ext| {
                                MainCityQuestInfo {
                                    track_npc_id_list: ext.track_npc_id.iter().copied().collect(),
                                }
                            }),
                            special_quest_info: quest.special_ext.as_ref().map(|ext| {
                                SpecialQuestInfo {
                                    cur_quest_id: ext.cur_quest_id,
                                    prev_quest_id: ext.prev_quest_id,
                                }
                            }),
                        })
                        .collect(),
                    track_quest: collection.track_quest.as_ref().map(|track_quest| {
                        TrackQuestInfo {
                            cur_main_quest_id: track_quest.cur_main_quest_id,
                            cur_track_quest_id: track_quest.cur_track_quest_id,
                            cur_track_special_quest_id: track_quest.cur_track_special_quest_id,
                        }
                    }),
                })
                .collect(),
            battle_data: Some(BattleData {
                activity: Some(ActivityBattleData {
                    boss_battle: Some(BossBattleData {
                        unlocked_levels: self
                            .battle_data
                            .activity
                            .boss_battle
                            .unlocked_levels
                            .iter()
                            .copied()
                            .collect(),
                        selected_level: self.battle_data.activity.boss_battle.selected_level.get(),
                    }),
                    double_elite: Some(DoubleEliteData {
                        unlocked_levels: self
                            .battle_data
                            .activity
                            .double_elite
                            .unlocked_levels
                            .iter()
                            .copied()
                            .collect(),
                        progress_list: self
                            .battle_data
                            .activity
                            .double_elite
                            .progress
                            .iter()
                            .map(|(&quest_id, _progress)| {
                                yixuan_proto::server_only::DoubleEliteProgress { quest_id }
                            })
                            .collect(),
                        selected_difficulty: self
                            .battle_data
                            .activity
                            .double_elite
                            .selected_difficulty
                            .get(),
                    }),
                }),
            }),
        });
    }
}

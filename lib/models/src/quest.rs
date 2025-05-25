use std::collections::{HashMap, HashSet};

use config::EQuestState;
use vivian_logic::dungeon::EQuestType;

use crate::property::{Property, PropertyHashMap, PropertyHashSet};

use super::*;

#[derive(Model)]
pub struct QuestModel {
    pub quest_collections: PropertyHashMap<EQuestType, QuestCollection>,
    pub new_hollow_quests: PropertyHashSet<u32>,
}

#[derive(Default)]
pub struct QuestCollection {
    pub finished_quests: HashSet<u32>,
    pub quests: HashMap<u32, Quest>,
}

pub struct MainCityQuestExt {
    pub track_npc_id: HashSet<u32>,
}

pub struct Quest {
    pub id: u32,
    pub state: i32,
    pub unlock_time: i64,
    pub in_progress_time: i64,
    pub progress: u32,
    pub finish_condition_progress: HashMap<u32, u32>,
    pub main_city_ext: Option<MainCityQuestExt>,
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
                                        },
                                    )
                                })
                                .collect(),
                        },
                    ))
                })
                .collect(),
            new_hollow_quests: PropertyHashSet::default(),
        }
    }
}

impl Quest {
    pub fn to_client_proto(&self) -> vivian_proto::QuestInfo {
        vivian_proto::QuestInfo {
            id: self.id,
            state: self.state,
            unlock_time: self.unlock_time,
            progress: self.progress,
            in_progress_time: self.in_progress_time,
            finish_condition_progress: self.finish_condition_progress.clone(),
            main_city_quest_info: self.main_city_ext.as_ref().map(|ext| {
                vivian_proto::MainCityQuestInfo {
                    track_npc_id_list: ext.track_npc_id.iter().copied().collect(),
                }
            }),
        }
    }
}

impl Saveable for QuestModel {
    fn save_to_pb(&self, root: &mut vivian_proto::server_only::PlayerData) {
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
                        })
                        .collect(),
                })
                .collect(),
        });
    }
}



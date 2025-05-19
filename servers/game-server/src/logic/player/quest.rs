use std::collections::{HashMap, HashSet};

use common::time_util;
use config::EQuestState;
use vivian_codegen::Model;
use vivian_logic::dungeon::EQuestType;
use vivian_proto::{
    QuestSync,
    server_only::{MainCityQuestInfo, QuestCollectionInfo, QuestData, QuestInfo},
};

use crate::{
    logic::{
        property::{Property, PropertyHashMap, PropertyHashSet},
        sync::{LoginDataSyncComponent, PlayerSyncComponent, SyncType},
    },
    resources::NapResources,
};

use super::{Model, Saveable};

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
    pub fn add_hollow_quest(&mut self, id: u32, res: &NapResources) {
        let collection = self.get_or_insert_collection(EQuestType::Hollow);

        collection.quests.insert(
            id,
            Quest {
                id,
                state: 0,
                unlock_time: time_util::unix_timestamp_seconds(),
                in_progress_time: 0,
                progress: 0,
                finish_condition_progress: HashMap::new(),
                main_city_ext: None,
            },
        );

        self.new_hollow_quests.insert(id);

        self.add_hollow_challenges(id, res);
    }

    fn add_hollow_challenges(&mut self, hollow_quest_id: u32, res: &NapResources) {
        let collection = self.get_or_insert_collection(EQuestType::HollowChallenge);

        res.templates
            .hollow_challenge_template_tb()
            .filter(|c| c.hollow_quest_id() == hollow_quest_id)
            .for_each(|c| {
                collection.quests.insert(
                    c.id(),
                    Quest {
                        id: c.id(),
                        state: 0,
                        unlock_time: time_util::unix_timestamp_seconds(),
                        in_progress_time: 0,
                        progress: 0,
                        finish_condition_progress: HashMap::new(),
                        main_city_ext: None,
                    },
                );
            })
    }

    pub fn add_main_city_quest(&mut self, id: u32, res: &NapResources) {
        let main_city_quest_template = res
            .templates
            .main_city_quest_template_tb()
            .find(|tmpl| tmpl.id() == id)
            .unwrap();

        let collection = self.get_or_insert_collection(EQuestType::MainCity);

        collection.quests.insert(
            id,
            Quest {
                id,
                state: 1,
                unlock_time: time_util::unix_timestamp_seconds(),
                in_progress_time: 0,
                progress: 0,
                finish_condition_progress: HashMap::new(),
                main_city_ext: Some(MainCityQuestExt {
                    track_npc_id: main_city_quest_template
                        .npcs()
                        .unwrap_or_default()
                        .iter()
                        .map(|id| id as u32)
                        .collect(),
                }),
            },
        );
    }

    pub fn finish_hollow_challenge(&mut self, id: u32) {
        if let Some(collection) = self.quest_collections.get(&EQuestType::HollowChallenge) {
            if collection.finished_quests.contains(&id) {
                return;
            }
        }

        let collection = self.get_or_insert_collection(EQuestType::HollowChallenge);

        collection.quests.remove(&id);
        collection.finished_quests.insert(id);
    }

    pub fn finish_main_city_quest(&mut self, id: u32, res: &NapResources) -> Vec<u32> {
        let collection = self.get_or_insert_collection(EQuestType::MainCity);

        let Some(_quest) = collection.quests.remove(&id) else {
            return Vec::new();
        };

        collection.finished_quests.insert(id);

        let mut newly_added_quests = Vec::new();
        for next_quest_template in res
            .templates
            .quest_config_template_tb()
            .filter(|quest| {
                quest.quest_type() == 5
                    && !collection.finished_quests.contains(&quest.quest_id())
                    && !quest.preorder_quest_ids().unwrap_or_default().is_empty()
                    && quest
                        .preorder_quest_ids()
                        .unwrap_or_default()
                        .iter()
                        .all(|id| collection.finished_quests.contains(&id))
            })
            .collect::<Vec<_>>()
        {
            newly_added_quests.push(next_quest_template.quest_id());
            self.add_main_city_quest(next_quest_template.quest_id(), res);
        }

        let main_city_quest_template = res
            .templates
            .main_city_quest_template_tb()
            .find(|tmpl| tmpl.id() == id)
            .unwrap();

        if let Ok(hollow_quest_id) = main_city_quest_template
            .action_arg_1()
            .unwrap()
            .parse::<u32>()
        {
            self.add_hollow_quest(hollow_quest_id, res);
        }

        newly_added_quests
    }

    pub fn finish_hollow_quest(&mut self, id: u32) {
        let collection = self.get_or_insert_collection(EQuestType::Hollow);

        if collection.quests.remove(&id).is_some() {
            collection.finished_quests.insert(id);
        }
    }

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

    fn get_or_insert_collection(&mut self, quest_type: EQuestType) -> &mut QuestCollection {
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

impl LoginDataSyncComponent for QuestModel {
    fn prepare_responses(
        &self,
        sync_helper: &mut crate::logic::sync::DataSyncHelper,
        _res: &NapResources,
    ) {
        sync_helper.add_response(
            SyncType::BasicData,
            vivian_proto::GetQuestDataScRsp {
                retcode: 0,
                quest_type: 0,
                quest_data: Some(vivian_proto::QuestData {
                    quest_collection_list: (1..10u32)
                        .map(|ty| (ty, self.quest_collections.get(&ty.try_into().unwrap())))
                        .map(|(quest_type, collection)| {
                            collection
                                .map(|collection| vivian_proto::QuestCollection {
                                    quest_type,
                                    finished_quest_id_list: collection
                                        .finished_quests
                                        .iter()
                                        .copied()
                                        .collect(),
                                    quest_list: collection
                                        .quests
                                        .values()
                                        .map(Quest::to_client_proto)
                                        .collect(),
                                })
                                .unwrap_or_else(|| vivian_proto::QuestCollection {
                                    quest_type,
                                    ..Default::default()
                                })
                        })
                        .collect(),
                }),
            },
        );
    }
}

impl PlayerSyncComponent for QuestModel {
    fn supports_player_sync(&self) -> bool {
        true
    }

    fn add_changes_to_player_sync_notify(
        &self,
        player_sync_sc_notify: &mut vivian_proto::PlayerSyncScNotify,
    ) {
        player_sync_sc_notify.quest = Some(QuestSync {
            quest_list: self
                .quest_collections
                .iter_changed_items()
                .flat_map(|(_, qc)| qc.quests.values().map(Quest::to_client_proto))
                .collect(),
            finished_quest_id_list: self
                .quest_collections
                .iter_changed_items()
                .flat_map(|(_, qc)| qc.finished_quests.iter().copied())
                .collect(),
            new_hollow_quest_id_list: self.new_hollow_quests.iter_added_keys().copied().collect(),
        })
    }
}

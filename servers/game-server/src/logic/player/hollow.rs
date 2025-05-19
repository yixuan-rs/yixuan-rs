use vivian_codegen::Model;
use vivian_proto::server_only::{HollowData, HollowInfo};

use crate::{
    logic::{
        property::{Property, PropertyHashMap, PropertyHashSet},
        sync::{LoginDataSyncComponent, PlayerSyncComponent, SyncType},
    },
    resources::NapResources,
};

use super::{Model, Saveable, quest::QuestCollection};

#[derive(Model)]
pub struct HollowModel {
    pub hollow_groups: PropertyHashSet<u32>,
    pub new_hollow_groups: PropertyHashSet<u32>,
    pub unlocked_hollows: PropertyHashSet<u32>,
    pub new_unlocked_hollows: PropertyHashSet<u32>,
    pub hollows: PropertyHashMap<u32, Hollow>,
    pub executed_hollow_events: PropertyHashSet<u32>,
}

pub struct Hollow {
    pub hollow_quest_id: u32,
    pub finished: bool,
    pub acquired_hollow_challenge_reward: u32,
}

impl HollowModel {
    pub fn ensure_hollows(
        &mut self,
        hollow_quest_collection: &QuestCollection,
        res: &NapResources,
    ) {
        hollow_quest_collection.quests.keys().for_each(|&id| {
            if !self.hollows.contains_key(&id) {
                let template = res
                    .templates
                    .hollow_quest_template_tb()
                    .find(|tmpl| tmpl.id() == id)
                    .unwrap();

                let hollow_id = template.hollow_id();

                if !self.unlocked_hollows.contains(&hollow_id) {
                    self.unlocked_hollows.insert(hollow_id);
                    self.new_unlocked_hollows.insert(hollow_id);

                    let template = res
                        .templates
                        .hollow_config_template_tb()
                        .find(|tmpl| tmpl.id() == hollow_id)
                        .unwrap();

                    self.hollow_groups.insert(template.hollow_group());
                    self.new_hollow_groups.insert(template.hollow_group());
                }

                self.hollows.insert(
                    id,
                    Hollow {
                        hollow_quest_id: id,
                        finished: false,
                        acquired_hollow_challenge_reward: 0,
                    },
                );
            }
        });
    }

    pub fn load_from_pb(pb: HollowData) -> Self {
        Self {
            hollow_groups: pb.hollow_group_list.into_iter().collect(),
            new_hollow_groups: pb.unlock_hollow_group_list.into_iter().collect(),
            unlocked_hollows: pb.hollow_id_list.into_iter().collect(),
            new_unlocked_hollows: pb.unlock_hollow_id_list.into_iter().collect(),
            hollows: pb
                .hollow_list
                .into_iter()
                .map(|hollow| {
                    (
                        hollow.hollow_quest_id,
                        Hollow {
                            hollow_quest_id: hollow.hollow_quest_id,
                            finished: hollow.finished,
                            acquired_hollow_challenge_reward: hollow
                                .acquired_hollow_challenge_reward,
                        },
                    )
                })
                .collect(),
            executed_hollow_events: pb.executed_hollow_event_id_list.into_iter().collect(),
        }
    }
}

impl Saveable for HollowModel {
    fn save_to_pb(&self, root: &mut vivian_proto::server_only::PlayerData) {
        root.hollow = Some(HollowData {
            hollow_group_list: self.hollow_groups.iter().copied().collect(),
            unlock_hollow_group_list: self.new_hollow_groups.iter().copied().collect(),
            hollow_id_list: self.unlocked_hollows.iter().copied().collect(),
            unlock_hollow_id_list: self.new_unlocked_hollows.iter().copied().collect(),
            hollow_list: self
                .hollows
                .iter()
                .map(|(_, hollow)| HollowInfo {
                    hollow_quest_id: hollow.hollow_quest_id,
                    finished: hollow.finished,
                    acquired_hollow_challenge_reward: hollow.acquired_hollow_challenge_reward,
                })
                .collect(),
            executed_hollow_event_id_list: self.executed_hollow_events.iter().copied().collect(),
        });
    }
}

impl LoginDataSyncComponent for HollowModel {
    fn prepare_responses(
        &self,
        sync_helper: &mut crate::logic::sync::DataSyncHelper,
        _res: &NapResources,
    ) {
        sync_helper.add_response(
            SyncType::BasicData,
            vivian_proto::GetHollowDataScRsp {
                retcode: 0,
                hollow_data: Some(vivian_proto::HollowData {
                    hollow_group_list: self.hollow_groups.iter().copied().collect(),
                    unlock_hollow_group_list: self.new_hollow_groups.iter().copied().collect(),
                    unlock_hollow_id_list: self.new_unlocked_hollows.iter().copied().collect(),
                    hollow_info_list: self
                        .hollows
                        .iter()
                        .map(|(_, hollow)| vivian_proto::HollowInfo {
                            hollow_quest_id: hollow.hollow_quest_id,
                            unk_hollow_info_100: 0,
                            acquired_hollow_challenge_reward: hollow
                                .acquired_hollow_challenge_reward,
                        })
                        .collect(),
                }),
            },
        );
    }
}

impl PlayerSyncComponent for HollowModel {
    fn supports_player_sync(&self) -> bool {
        true
    }

    fn add_changes_to_player_sync_notify(
        &self,
        player_sync_sc_notify: &mut vivian_proto::PlayerSyncScNotify,
    ) {
        player_sync_sc_notify.hollow = Some(vivian_proto::HollowSync {
            hollow_group_list: self.hollow_groups.iter().copied().collect(),
            unlock_hollow_group_list: self.new_hollow_groups.iter().copied().collect(),
            unlock_hollow_id_list: self.new_unlocked_hollows.iter().copied().collect(),
            hollow_info_list: self
                .hollows
                .iter_changed_items()
                .map(|(_, hollow)| vivian_proto::HollowInfo {
                    hollow_quest_id: hollow.hollow_quest_id,
                    unk_hollow_info_100: 100,
                    acquired_hollow_challenge_reward: hollow.acquired_hollow_challenge_reward,
                })
                .collect(),
        });
    }
}

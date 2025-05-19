use std::collections::HashMap;

use vivian_proto::server_only::{BuddyData, BuddyItemInfo};

use crate::{
    logic::{
        property::{Property, PropertyHashMap},
        sync::{LoginDataSyncComponent, PlayerSyncComponent, SyncType},
    },
    resources::NapResources,
};

use super::{Model, Saveable};

pub struct BuddyModel {
    pub buddy_map: PropertyHashMap<u32, BuddyItem>,
}

pub struct BuddyItem {
    pub id: u32,
    pub level: u32,
    pub exp: u32,
    pub rank: u32,
    pub star: u32,
    pub skill_level_map: HashMap<u32, u32>,
    pub first_get_time: i64,
    pub taken_rank_up_reward_list: Vec<u32>,
}

impl BuddyModel {
    pub fn load_from_pb(pb: BuddyData) -> Self {
        Self {
            buddy_map: pb
                .buddy_list
                .into_iter()
                .map(|buddy| {
                    (
                        buddy.id,
                        BuddyItem {
                            id: buddy.id,
                            level: buddy.level,
                            exp: buddy.exp,
                            rank: buddy.rank,
                            star: buddy.star,
                            skill_level_map: buddy.skill_level_map,
                            first_get_time: buddy.first_get_time,
                            taken_rank_up_reward_list: buddy.taken_rank_up_reward_list,
                        },
                    )
                })
                .collect(),
        }
    }
}

impl Model for BuddyModel {
    fn is_any_field_changed(&self) -> bool {
        self.buddy_map.is_changed()
    }

    fn reset_changed_fields(&mut self) {
        self.buddy_map.reset_changed_state();
    }
}

impl Saveable for BuddyModel {
    fn save_to_pb(&self, root: &mut vivian_proto::server_only::PlayerData) {
        root.buddy = Some(BuddyData {
            buddy_list: self
                .buddy_map
                .iter()
                .map(|(_, buddy)| BuddyItemInfo {
                    id: buddy.id,
                    level: buddy.level,
                    exp: buddy.exp,
                    rank: buddy.rank,
                    star: buddy.star,
                    skill_level_map: buddy.skill_level_map.clone(),
                    first_get_time: buddy.first_get_time,
                    taken_rank_up_reward_list: buddy.taken_rank_up_reward_list.clone(),
                })
                .collect(),
        });
    }
}

impl LoginDataSyncComponent for BuddyModel {
    fn prepare_responses(
        &self,
        sync_helper: &mut crate::logic::sync::DataSyncHelper,
        _res: &NapResources,
    ) {
        sync_helper.add_response(
            SyncType::BasicData,
            vivian_proto::GetBuddyDataScRsp {
                retcode: 0,
                buddy_list: self
                    .buddy_map
                    .iter()
                    .map(|(_, buddy)| buddy.as_client_proto())
                    .collect(),
            },
        );
    }
}

impl PlayerSyncComponent for BuddyModel {
    fn supports_player_sync(&self) -> bool {
        false
    }

    fn add_changes_to_player_sync_notify(
        &self,
        _player_sync_sc_notify: &mut vivian_proto::PlayerSyncScNotify,
    ) {
    }
}

impl BuddyItem {
    pub fn as_client_proto(&self) -> vivian_proto::BuddyInfo {
        vivian_proto::BuddyInfo {
            id: self.id,
            level: self.level,
            exp: self.exp,
            rank: self.rank,
            star: self.star,
            first_get_time: self.first_get_time,
            skill_type_level: self
                .skill_level_map
                .iter()
                .map(|(&skill_type, &level)| vivian_proto::BuddySkillLevel { skill_type, level })
                .collect(),
            taken_rank_up_reward_list: self.taken_rank_up_reward_list.clone(),
        }
    }
}

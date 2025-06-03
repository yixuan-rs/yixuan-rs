use std::collections::HashMap;

use yixuan_models::*;
use yixuan_proto::*;

use crate::resources::NapResources;

use super::{DataSyncHelper, SyncType};

pub trait LoginDataSyncComponent {
    fn prepare_responses(&self, sync_helper: &mut DataSyncHelper, res: &NapResources);
}

impl LoginDataSyncComponent for PlayerBasicModel {
    fn prepare_responses(&self, sync_helper: &mut DataSyncHelper, _: &NapResources) {
        sync_helper.add_response(
            SyncType::BasicData,
            GetSelfBasicInfoScRsp {
                retcode: 0,
                self_basic_info: Some(self.build_self_basic_info()),
            },
        )
    }
}

impl LoginDataSyncComponent for ItemModel {
    fn prepare_responses(&self, sync_helper: &mut DataSyncHelper, _: &NapResources) {
        sync_helper.add_response(
            SyncType::BasicData,
            GetWeaponDataScRsp {
                retcode: 0,
                weapon_list: self
                    .weapon_map
                    .iter()
                    .map(|(&uid, weapon)| weapon.as_client_proto(uid))
                    .collect(),
            },
        );

        sync_helper.add_response(
            SyncType::BasicData,
            yixuan_proto::GetEquipDataScRsp {
                retcode: 0,
                equip_list: self
                    .equip_map
                    .iter()
                    .map(|(&uid, equip)| equip.as_client_proto(uid))
                    .collect(),
            },
        );

        sync_helper.add_response(
            SyncType::BasicData,
            yixuan_proto::GetItemDataScRsp {
                retcode: 0,
                item_list: self
                    .item_count_map
                    .iter()
                    .map(|(&id, &count)| yixuan_proto::ItemInfo { id, count })
                    .collect(),
                auto_recovery_info: HashMap::new(),
            },
        );

        sync_helper.add_response(
            SyncType::BasicData,
            yixuan_proto::GetWishlistDataScRsp { retcode: 0 },
        );
    }
}

impl LoginDataSyncComponent for AvatarModel {
    fn prepare_responses(&self, sync_helper: &mut DataSyncHelper, _: &NapResources) {
        sync_helper.add_response(
            SyncType::BasicData,
            yixuan_proto::GetAvatarDataScRsp {
                retcode: 0,
                avatar_list: self
                    .avatar_map
                    .iter()
                    .map(|(_, avatar)| avatar.as_client_proto())
                    .collect(),
            },
        );
    }
}

impl LoginDataSyncComponent for QuestModel {
    fn prepare_responses(&self, sync_helper: &mut DataSyncHelper, _: &NapResources) {
        sync_helper.add_response(
            SyncType::BasicData,
            yixuan_proto::GetQuestDataScRsp {
                retcode: 0,
                quest_type: 0,
                quest_data: Some(yixuan_proto::QuestData {
                    quest_collection_list: self
                        .quest_collections
                        .iter()
                        .map(|(quest_type, collection)| yixuan_proto::QuestCollection {
                            quest_type: (*quest_type).into(),
                            finished_quest_id_list: collection
                                .finished_quests
                                .iter()
                                .copied()
                                .collect(),
                            quest_list: collection
                                .quests
                                .values()
                                .map(yixuan_models::Quest::to_client_proto)
                                .collect(),
                        })
                        .collect(),
                }),
            },
        );
        sync_helper.add_response(
            SyncType::ExtendData,
            yixuan_proto::GetBattleDataScRsp {
                retcode: 0,
                battle_data: Some(BattleData {
                    battle_data: Some(ActivityBattleData {
                        monster_card: Some(MonsterCardData {
                            unlocked_levels: self
                                .battle_data
                                .activity
                                .monster_card
                                .unlocked_levels
                                .iter()
                                .copied()
                                .collect(),
                            selected_level: self
                                .battle_data
                                .activity
                                .monster_card
                                .selected_level
                                .get(),
                        }),
                    }),
                }),
            },
        );
    }
}

impl LoginDataSyncComponent for ArchiveModel {
    fn prepare_responses(&self, sync_helper: &mut DataSyncHelper, _: &NapResources) {
        sync_helper.add_response(
            SyncType::BasicData,
            yixuan_proto::GetArchiveDataScRsp {
                retcode: 0,
                archive_data: Some(yixuan_proto::ArchiveData {
                    hollow_archive_id_list: self
                        .unlocked_hollow_archive_ids
                        .iter()
                        .copied()
                        .collect(),
                    videotape_list: self
                        .archive_files
                        .iter()
                        .map(|(_, file)| yixuan_proto::VideotapeInfo {
                            archive_file_id: file.id,
                            finished: file.finished,
                        })
                        .collect(),
                }),
            },
        );
    }
}

impl LoginDataSyncComponent for HollowModel {
    fn prepare_responses(&self, sync_helper: &mut DataSyncHelper, _res: &NapResources) {
        sync_helper.add_response(
            SyncType::BasicData,
            yixuan_proto::GetHollowDataScRsp {
                retcode: 0,
                hollow_data: Some(yixuan_proto::HollowData {
                    hollow_group_list: self.hollow_groups.iter().copied().collect(),
                    unlock_hollow_group_list: self.new_hollow_groups.iter().copied().collect(),
                    unlock_hollow_id_list: self.new_unlocked_hollows.iter().copied().collect(),
                    hollow_info_list: self
                        .hollows
                        .iter()
                        .map(|(_, hollow)| yixuan_proto::HollowInfo {
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

impl LoginDataSyncComponent for AbyssModel {
    fn prepare_responses(&self, sync_helper: &mut DataSyncHelper, _: &NapResources) {
        sync_helper.add_response(
            SyncType::BasicData,
            yixuan_proto::AbyssGetDataScRsp {
                retcode: 0,
                abyss_data: Some(yixuan_proto::AbyssData::default()),
                abyss_dungeon_list: Vec::new(),
                abyss_group_list: Vec::new(),
            },
        );

        sync_helper.add_response(
            SyncType::BasicData,
            yixuan_proto::AbyssArpeggioGetDataScRsp { retcode: 0 },
        );
    }
}

impl LoginDataSyncComponent for BuddyModel {
    fn prepare_responses(&self, sync_helper: &mut DataSyncHelper, _res: &NapResources) {
        sync_helper.add_response(
            SyncType::BasicData,
            yixuan_proto::GetBuddyDataScRsp {
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

impl LoginDataSyncComponent for MiscModel {
    fn prepare_responses(&self, sync_helper: &mut DataSyncHelper, res: &NapResources) {
        sync_helper.add_response(
            SyncType::BasicData,
            yixuan_proto::VideoGetInfoScRsp {
                retcode: 0,
                video_key_map: res.video_key_map.clone(),
            },
        );

        sync_helper.add_response(
            SyncType::ExtendData,
            yixuan_proto::GetSwitchDataScRsp {
                retcode: 0,
                r#type: 1,
                setting_switch_map: self.switch.setting_switch_map.base.clone(),
                switch_data: Some(yixuan_proto::SwitchData {
                    open_system_id_list: self.switch.open_system_id.iter().copied().collect(),
                    system_switch_state_list: self
                        .switch
                        .system_switch_state_map
                        .iter()
                        .map(|(&ty, &state)| yixuan_proto::SystemSwitchStateInfo {
                            r#type: ty,
                            switch_state: state,
                        })
                        .collect(),
                    input_setting_map: self
                        .switch
                        .input_setting_map
                        .iter()
                        .map(|(&ty, setting)| {
                            (
                                ty,
                                yixuan_proto::InputSettingInfo {
                                    input_type_map: setting.input_type_map.clone(),
                                },
                            )
                        })
                        .collect(),
                    ..Default::default()
                }),
            },
        );

        sync_helper.add_response(
            SyncType::ExtendData,
            yixuan_proto::GetMiscDataScRsp {
                retcode: 0,
                data: Some(yixuan_proto::MiscData {
                    unlock: Some(yixuan_proto::UnlockInfo {
                        unlocked_list: self.unlock.unlocked_id.iter().copied().collect(),
                        quick_access_list: self
                            .unlock
                            .quick_access
                            .iter()
                            .map(|(&quick_access_id, item)| yixuan_proto::QuickAccessInfo {
                                quick_access_id,
                                quick_access_index: item.index,
                                r#type: item.quick_access_type,
                            })
                            .collect(),
                    }),
                    teleport: Some(yixuan_proto::TeleportUnlockInfo {
                        unlocked_list: self.teleport.unlocked_id.iter().copied().collect(),
                    }),
                    news_stand: Some(self.news_stand.to_client_proto()),
                    post_girl: Some(yixuan_proto::PostGirlInfo {
                        post_girl_item_list: self
                            .post_girl
                            .unlocked_items
                            .iter()
                            .map(|(&id, &unlock_time)| yixuan_proto::PostGirlItem {
                                id,
                                unlock_time,
                            })
                            .collect(),
                        selected_post_girl_id_list: self
                            .post_girl
                            .selected_id
                            .iter()
                            .copied()
                            .collect(),
                        post_girl_random_toggle: self.post_girl.random_toggle.get(),
                    }),
                    business_card: Some(yixuan_proto::BusinessCardData {
                        unlocked_business_card_id_list: self
                            .business_card
                            .unlocked_items
                            .iter()
                            .copied()
                            .collect(),
                        selected_id: self.business_card.selected_id.get(),
                    }),
                    player_accessory: Some(yixuan_proto::PlayerAccessoryData {
                        player_accessory_list: self
                            .player_accessory
                            .player_accessory_map
                            .iter()
                            .map(|(&avatar_id, player_accessory)| {
                                yixuan_proto::PlayerAccessoryInfo {
                                    avatar_id,
                                    avatar_skin_id: player_accessory.avatar_skin_id,
                                    player_skin_list: player_accessory
                                        .player_skin_map
                                        .iter()
                                        .map(|(&player_skin_id, player_skin)| {
                                            yixuan_proto::PlayerSkinInfo {
                                                player_skin_id,
                                                equipped_accessory_id_list: player_skin
                                                    .equipped_accessory_id_list
                                                    .iter()
                                                    .copied()
                                                    .collect(),
                                            }
                                        })
                                        .collect(),
                                }
                            })
                            .collect(),
                    }),
                }),
            },
        );
    }
}

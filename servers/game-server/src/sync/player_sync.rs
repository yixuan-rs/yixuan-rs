use vivian_models::*;
use vivian_proto::*;

use vivian_models::property::Property;

pub trait PlayerSyncComponent {
    fn supports_player_sync(&self) -> bool;
    fn add_changes_to_player_sync_notify(&self, player_sync_sc_notify: &mut PlayerSyncScNotify);
}

impl PlayerSyncComponent for PlayerBasicModel {
    fn add_changes_to_player_sync_notify(&self, player_sync_sc_notify: &mut PlayerSyncScNotify) {
        player_sync_sc_notify.self_basic_info = Some(self.build_self_basic_info());
    }

    fn supports_player_sync(&self) -> bool {
        true
    }
}

impl PlayerSyncComponent for ItemModel {
    fn add_changes_to_player_sync_notify(
        &self,
        player_sync_sc_notify: &mut vivian_proto::PlayerSyncScNotify,
    ) {
        let item_sync = player_sync_sc_notify.item.get_or_insert_default();

        if self.item_count_map.is_changed() {
            item_sync.item_list.extend(
                self.item_count_map
                    .iter_changed_items()
                    .map(|(&id, &count)| vivian_proto::ItemInfo { id, count }),
            )
        }

        if self.weapon_map.is_changed() {
            item_sync.weapon_list.extend(
                self.weapon_map
                    .iter_changed_items()
                    .map(|(&uid, weapon)| weapon.as_client_proto(uid)),
            );
        }

        if self.equip_map.is_changed() {
            item_sync.equip_list.extend(
                self.equip_map
                    .iter_changed_items()
                    .map(|(&uid, equip)| equip.as_client_proto(uid)),
            );
        }

        if self.new_reward_map.is_changed() {
            let item_changed = item_sync.item_changed.get_or_insert_default();
            let reward_changes = item_changed.item_reward_map.entry(1).or_default();

            reward_changes
                .reward_list
                .extend(
                    self.new_reward_map
                        .iter_changed_items()
                        .flat_map(|(_, changes)| {
                            changes
                                .iter()
                                .map(|&(item_id, amount)| vivian_proto::ItemRewardInfo {
                                    item_id,
                                    amount: amount as u32,
                                })
                        }),
                );
        }
    }

    fn supports_player_sync(&self) -> bool {
        true
    }
}

impl PlayerSyncComponent for AvatarModel {
    fn add_changes_to_player_sync_notify(
        &self,
        player_sync_sc_notify: &mut vivian_proto::PlayerSyncScNotify,
    ) {
        player_sync_sc_notify
            .avatar
            .get_or_insert_default()
            .avatar_list
            .extend(
                self.avatar_map
                    .iter_changed_items()
                    .map(|(_, avatar)| avatar.as_client_proto()),
            );
    }

    fn supports_player_sync(&self) -> bool {
        true
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
        });

        if self.battle_data.activity.is_changed() {
            player_sync_sc_notify.activity_battle = Some(ActivityBattleSync {
                monster_card: Some(MonsterCardSync {
                    new_unlocked_levels: self
                        .battle_data
                        .activity
                        .monster_card
                        .unlocked_levels
                        .iter_added_keys()
                        .copied()
                        .collect(),
                    selected_level: self.battle_data.activity.monster_card.selected_level.get(),
                }),
            });
        }
    }
}

impl PlayerSyncComponent for ArchiveModel {
    fn supports_player_sync(&self) -> bool {
        false
    }

    fn add_changes_to_player_sync_notify(
        &self,
        _player_sync_sc_notify: &mut vivian_proto::PlayerSyncScNotify,
    ) {
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

impl PlayerSyncComponent for AbyssModel {
    fn supports_player_sync(&self) -> bool {
        false
    }

    fn add_changes_to_player_sync_notify(
        &self,
        _player_sync_sc_notify: &mut vivian_proto::PlayerSyncScNotify,
    ) {
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

impl PlayerSyncComponent for MiscModel {
    fn supports_player_sync(&self) -> bool {
        true
    }

    fn add_changes_to_player_sync_notify(
        &self,
        player_sync_sc_notify: &mut vivian_proto::PlayerSyncScNotify,
    ) {
        let sync = player_sync_sc_notify.misc.get_or_insert_default();

        sync.trigger_newbie_group_list
            .extend(self.newbie.triggered_groups.iter_added_keys().copied());

        sync.quick_access_list
            .extend(
                self.unlock
                    .quick_access
                    .iter()
                    .map(|(&quick_access_id, item)| vivian_proto::QuickAccessInfo {
                        quick_access_id,
                        quick_access_index: item.index,
                        r#type: item.quick_access_type,
                    }),
            );

        if self.news_stand.current_sign_id.is_changed() || self.news_stand.can_sign.is_changed() {
            sync.news_stand = Some(vivian_proto::NewsStandSync {
                current_sign_id: self.news_stand.current_sign_id.get(),
                can_sign: self.news_stand.can_sign.get(),
            });
        }

        if self.post_girl.is_changed() {
            sync.post_girl = Some(vivian_proto::PostGirlSync {
                new_post_girl_item_list: self
                    .post_girl
                    .unlocked_items
                    .iter_changed_items()
                    .map(|(&id, &unlock_time)| vivian_proto::PostGirlItem { id, unlock_time })
                    .collect(),
                selected_post_girl_id_list: self.post_girl.selected_id.iter().copied().collect(),
                post_girl_random_toggle: self.post_girl.random_toggle.get(),
            });
        }

        if self.business_card.is_changed() {
            sync.business_card = Some(vivian_proto::BusinessCardSync {
                unlocked_business_card_id_list: self
                    .business_card
                    .unlocked_items
                    .iter()
                    .copied()
                    .collect(),
                selected_id: self.business_card.selected_id.get(),
            });
        }

        if self.player_accessory.is_changed() {
            sync.player_accessory = Some(vivian_proto::PlayerAccessorySync {
                player_accessory_list: self
                    .player_accessory
                    .player_accessory_map
                    .iter()
                    .map(
                        |(&avatar_id, player_accessory)| vivian_proto::PlayerAccessoryInfo {
                            avatar_id,
                            avatar_skin_id: player_accessory.avatar_skin_id,
                            player_skin_list: player_accessory
                                .player_skin_map
                                .iter()
                                .map(|(&player_skin_id, player_skin)| {
                                    vivian_proto::PlayerSkinInfo {
                                        player_skin_id,
                                        equipped_accessory_id_list: player_skin
                                            .equipped_accessory_id_list
                                            .iter()
                                            .copied()
                                            .collect(),
                                    }
                                })
                                .collect(),
                        },
                    )
                    .collect(),
            });
        }
    }
}

impl PlayerSyncComponent for GachaModel {
    fn supports_player_sync(&self) -> bool {
        false
    }

    fn add_changes_to_player_sync_notify(&self, _: &mut vivian_proto::PlayerSyncScNotify) {}
}

impl PlayerSyncComponent for MapModel {
    fn supports_player_sync(&self) -> bool {
        false
    }

    fn add_changes_to_player_sync_notify(&self, _: &mut vivian_proto::PlayerSyncScNotify) {}
}

impl PlayerSyncComponent for MainCityModel {
    fn supports_player_sync(&self) -> bool {
        false
    }

    fn add_changes_to_player_sync_notify(
        &self,
        _player_sync_sc_notify: &mut vivian_proto::PlayerSyncScNotify,
    ) {
    }
}

impl PlayerSyncComponent for SceneModel {
    fn supports_player_sync(&self) -> bool {
        false
    }

    fn add_changes_to_player_sync_notify(
        &self,
        _player_sync_sc_notify: &mut vivian_proto::PlayerSyncScNotify,
    ) {
    }
}

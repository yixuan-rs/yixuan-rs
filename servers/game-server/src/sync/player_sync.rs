use itertools::Itertools;
use yixuan_logic::{
    LogicResources,
    dungeon::{AvatarUnit, EQuestType},
};
use yixuan_models::*;
use yixuan_proto::{
    common::{BigSceneAvatarInfo, SceneAvatarState, TeamMemberOperation, TeamMemberSource},
    *,
};

use yixuan_models::property::Property;

pub trait PlayerSyncComponent {
    fn supports_player_sync(&self) -> bool;
    fn add_changes_to_player_sync_notify(
        &self,
        player_sync_sc_notify: &mut PlayerSyncScNotify,
        res: &LogicResources,
    );
}

impl PlayerSyncComponent for PlayerBasicModel {
    fn add_changes_to_player_sync_notify(
        &self,
        player_sync_sc_notify: &mut PlayerSyncScNotify,
        _: &LogicResources,
    ) {
        player_sync_sc_notify.self_basic_info = Some(self.build_self_basic_info());
    }

    fn supports_player_sync(&self) -> bool {
        true
    }
}

impl PlayerSyncComponent for ItemModel {
    fn add_changes_to_player_sync_notify(
        &self,
        player_sync_sc_notify: &mut yixuan_proto::PlayerSyncScNotify,
        res: &LogicResources,
    ) {
        let item_sync = player_sync_sc_notify.item.get_or_insert_default();

        if self.item_count_map.is_changed() {
            item_sync.item_list.extend(
                self.item_count_map
                    .iter_changed_items()
                    .map(|(&id, &count)| yixuan_proto::ItemInfo { id, count }),
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

        if self.gained_once_rewards.is_changed() {
            let item_changed = item_sync.item_changed.get_or_insert_default();
            let reward_changes = item_changed.item_reward_map.entry(1).or_default();

            reward_changes
                .reward_list
                .extend(self.gained_once_rewards.iter_added_keys().flat_map(|&id| {
                    res.template_collection
                        .once_reward_template_tb()
                        .find_map(|tmpl| {
                            (tmpl.reward_id() == id).then(|| {
                                tmpl.reward_list()
                                    .unwrap_or_default()
                                    .iter()
                                    .map(|reward| yixuan_proto::ItemRewardInfo {
                                        item_id: reward.item_id(),
                                        amount: reward.amount(),
                                    })
                                    .collect::<Vec<_>>()
                            })
                        })
                        .unwrap_or_default()
                }));
        }
    }

    fn supports_player_sync(&self) -> bool {
        true
    }
}

impl PlayerSyncComponent for AvatarModel {
    fn add_changes_to_player_sync_notify(
        &self,
        player_sync_sc_notify: &mut yixuan_proto::PlayerSyncScNotify,
        _: &LogicResources,
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
        player_sync_sc_notify: &mut yixuan_proto::PlayerSyncScNotify,
        _: &LogicResources,
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
            track_quest_sync: self.quest_collections.get(&EQuestType::MainCity).and_then(
                |collection| {
                    collection
                        .track_quest
                        .as_ref()
                        .map(|track_quest| TrackQuestSync {
                            cur_main_quest_id: track_quest.cur_main_quest_id,
                            cur_track_quest_id: track_quest.cur_track_quest_id,
                            cur_track_special_quest_id: track_quest.cur_track_special_quest_id,
                        })
                },
            ),
        });

        if self.battle_data.activity.is_changed() {
            player_sync_sc_notify.activity_battle = Some(ActivityBattleSync {
                boss_battle: Some(BossBattleSync {
                    new_unlocked_levels: self
                        .battle_data
                        .activity
                        .boss_battle
                        .unlocked_levels
                        .iter_added_keys()
                        .copied()
                        .collect(),
                    selected_level: self.battle_data.activity.boss_battle.selected_level.get(),
                }),
                double_elite: Some(DoubleEliteSync {
                    new_unlocked_levels: self
                        .battle_data
                        .activity
                        .double_elite
                        .unlocked_levels
                        .iter_added_keys()
                        .copied()
                        .collect(),
                    progress_list: self
                        .battle_data
                        .activity
                        .double_elite
                        .progress
                        .iter_changed_items()
                        .map(|(&quest_id, _progress)| yixuan_proto::DoubleEliteProgress {
                            quest_id,
                            unlocked: true,
                        })
                        .collect(),
                    selected_difficulty: self
                        .battle_data
                        .activity
                        .double_elite
                        .selected_difficulty
                        .get(),
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
        _player_sync_sc_notify: &mut yixuan_proto::PlayerSyncScNotify,
        _: &LogicResources,
    ) {
    }
}

impl PlayerSyncComponent for HollowModel {
    fn supports_player_sync(&self) -> bool {
        true
    }

    fn add_changes_to_player_sync_notify(
        &self,
        player_sync_sc_notify: &mut yixuan_proto::PlayerSyncScNotify,
        _: &LogicResources,
    ) {
        player_sync_sc_notify.hollow = Some(yixuan_proto::HollowSync {
            hollow_group_list: self.hollow_groups.iter().copied().collect(),
            unlock_hollow_group_list: self.new_hollow_groups.iter().copied().collect(),
            unlock_hollow_id_list: self.new_unlocked_hollows.iter().copied().collect(),
            hollow_info_list: self
                .hollows
                .iter_changed_items()
                .map(|(_, hollow)| yixuan_proto::HollowInfo {
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
        _player_sync_sc_notify: &mut yixuan_proto::PlayerSyncScNotify,
        _: &LogicResources,
    ) {
    }
}

impl PlayerSyncComponent for BuddyModel {
    fn supports_player_sync(&self) -> bool {
        false
    }

    fn add_changes_to_player_sync_notify(
        &self,
        _player_sync_sc_notify: &mut yixuan_proto::PlayerSyncScNotify,
        _: &LogicResources,
    ) {
    }
}

impl PlayerSyncComponent for MiscModel {
    fn supports_player_sync(&self) -> bool {
        true
    }

    fn add_changes_to_player_sync_notify(
        &self,
        player_sync_sc_notify: &mut yixuan_proto::PlayerSyncScNotify,
        _: &LogicResources,
    ) {
        let sync = player_sync_sc_notify.misc.get_or_insert_default();

        sync.trigger_newbie_group_list
            .extend(self.newbie.triggered_groups.iter_added_keys().copied());

        sync.quick_access_list
            .extend(
                self.unlock
                    .quick_access
                    .iter()
                    .map(|(&quick_access_id, item)| yixuan_proto::QuickAccessInfo {
                        quick_access_id,
                        quick_access_index: item.index,
                        r#type: item.quick_access_type,
                    }),
            );

        if self.news_stand.current_sign_id.is_changed() || self.news_stand.can_sign.is_changed() {
            sync.news_stand = Some(yixuan_proto::NewsStandSync {
                current_sign_id: self.news_stand.current_sign_id.get(),
                can_sign: self.news_stand.can_sign.get(),
            });
        }

        if self.post_girl.is_changed() {
            sync.post_girl = Some(yixuan_proto::PostGirlSync {
                new_post_girl_item_list: self
                    .post_girl
                    .unlocked_items
                    .iter_changed_items()
                    .map(|(&id, &unlock_time)| yixuan_proto::PostGirlItem { id, unlock_time })
                    .collect(),
                selected_post_girl_id_list: self.post_girl.selected_id.iter().copied().collect(),
                post_girl_random_toggle: self.post_girl.random_toggle.get(),
            });
        }

        if self.business_card.is_changed() {
            sync.business_card = Some(yixuan_proto::BusinessCardSync {
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
            sync.player_accessory = Some(yixuan_proto::PlayerAccessorySync {
                player_accessory_list: self
                    .player_accessory
                    .player_accessory_map
                    .iter()
                    .map(
                        |(&avatar_id, player_accessory)| yixuan_proto::PlayerAccessoryInfo {
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

    fn add_changes_to_player_sync_notify(
        &self,
        _: &mut yixuan_proto::PlayerSyncScNotify,
        _: &LogicResources,
    ) {
    }
}

impl PlayerSyncComponent for MapModel {
    fn supports_player_sync(&self) -> bool {
        false
    }

    fn add_changes_to_player_sync_notify(
        &self,
        _: &mut yixuan_proto::PlayerSyncScNotify,
        _: &LogicResources,
    ) {
    }
}

impl PlayerSyncComponent for MainCityModel {
    fn supports_player_sync(&self) -> bool {
        false
    }

    fn add_changes_to_player_sync_notify(
        &self,
        _player_sync_sc_notify: &mut yixuan_proto::PlayerSyncScNotify,
        _: &LogicResources,
    ) {
    }
}

impl PlayerSyncComponent for SceneModel {
    fn supports_player_sync(&self) -> bool {
        false
    }

    fn add_changes_to_player_sync_notify(
        &self,
        _player_sync_sc_notify: &mut yixuan_proto::PlayerSyncScNotify,
        _: &LogicResources,
    ) {
    }
}

impl PlayerSyncComponent for BigSceneModel {
    fn supports_player_sync(&self) -> bool {
        true
    }

    fn add_changes_to_player_sync_notify(
        &self,
        player_sync_sc_notify: &mut PlayerSyncScNotify,
        _: &LogicResources,
    ) {
        player_sync_sc_notify.big_scene = Some(BigSceneSync {
            scene_avatar_list: self
                .team
                .avatars
                .iter_changed_items()
                .map(|(&id, avatar)| BigSceneAvatarInfo {
                    avatar_id: avatar.avatar_id,
                    source: TeamMemberSource::Normal.into(),
                    cur_hp: avatar.cur_hp,
                    avatar_unit: avatar.avatar_unit.as_ref().map(AvatarUnit::as_proto),
                    cur_state: SceneAvatarState::Alive.into(),
                    operation: if self.team.cur_avatars.contains_key(&id) {
                        TeamMemberOperation::TeamReplace.into()
                    } else {
                        TeamMemberOperation::Unk1.into()
                    },
                })
                .collect(),
            cur_scene_avatar_list: self
                .team
                .cur_avatars
                .iter()
                .sorted_by_key(|(_, avatar)| avatar.team_slot_index)
                .map(|(_, avatar)| BigSceneAvatarInfo {
                    avatar_id: avatar.avatar_id,
                    source: TeamMemberSource::Normal.into(),
                    cur_hp: avatar.cur_hp,
                    avatar_unit: avatar.avatar_unit.as_ref().map(AvatarUnit::as_proto),
                    cur_state: SceneAvatarState::Alive.into(),
                    operation: TeamMemberOperation::None.into(),
                })
                .collect(),
            cur_avatar_id: self.team.cur_avatar_id.get(),
            is_scene_team_replaced: self.team.is_changed(),
        });
    }
}

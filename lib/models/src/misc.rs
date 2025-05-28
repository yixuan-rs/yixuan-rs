use std::collections::{HashMap, HashSet};

use super::*;
use vivian_codegen::Property;

use property::{PrimitiveProperty, Property, PropertyHashMap, PropertyHashSet};

#[derive(Property, Default)]
pub struct PropertyUnlockData {
    pub unlocked_id: PropertyHashSet<i32>,
    pub quick_access: PropertyHashMap<u32, QuickAccess>,
}

#[derive(Default)]
pub struct QuickAccess {
    pub index: u32,
    pub quick_access_type: i32,
}

#[derive(Property, Default)]
pub struct PropertyTeleportUnlockData {
    pub unlocked_id: PropertyHashSet<i32>,
}

#[derive(Property, Default)]
pub struct PropertyPostGirlData {
    pub unlocked_items: PropertyHashMap<u32, i64>,
    pub selected_id: PropertyHashSet<u32>,
    pub random_toggle: PrimitiveProperty<bool>,
}

#[derive(Property, Default)]
pub struct PropertyNewbieData {
    pub finished_groups: PropertyHashSet<i32>,
    pub triggered_groups: PropertyHashSet<i32>,
}

#[derive(Property, Default)]
pub struct PropertyNewsStandData {
    pub cur_style: PrimitiveProperty<i32>,
    pub normal_news_id: PropertyHashSet<u32>,
    pub head_lines_id: PropertyHashSet<u32>,
    pub advertisement_id: PropertyHashSet<u32>,
    pub news_read_state: PrimitiveProperty<bool>,
    pub can_sign: PrimitiveProperty<bool>,
    pub current_sign_id: PrimitiveProperty<u32>,
    pub sign_count: PrimitiveProperty<u32>,
    pub sign_refresh_time: PrimitiveProperty<i64>,
    pub last_sign_time: PrimitiveProperty<i64>,
}

#[derive(Default)]
pub struct InputSetting {
    pub input_type_map: HashMap<u32, i32>,
}

#[derive(Property, Default)]
pub struct PropertySwitchData {
    pub open_system_id: PropertyHashSet<u32>,
    pub setting_switch_map: PropertyHashMap<u32, u32>,
    pub system_switch_state_map: PropertyHashMap<u32, bool>,
    pub input_setting_map: PropertyHashMap<u32, InputSetting>,
}

#[derive(Property, Default)]
pub struct PropertyBusinessCardData {
    pub unlocked_items: PropertyHashSet<u32>,
    pub selected_id: PrimitiveProperty<u32>,
}

#[derive(Default)]
pub struct PlayerSkin {
    pub equipped_accessory_id_list: HashSet<u32>,
}

#[derive(Default)]
pub struct PlayerAccessory {
    pub avatar_skin_id: u32,
    pub player_skin_map: HashMap<u32, PlayerSkin>,
}

#[derive(Property, Default)]
pub struct PropertyPlayerAccessoryData {
    pub player_accessory_map: PropertyHashMap<u32, PlayerAccessory>,
}

#[derive(Model)]
pub struct MiscModel {
    pub switch: PropertySwitchData,
    pub unlock: PropertyUnlockData,
    pub teleport: PropertyTeleportUnlockData,
    pub newbie: PropertyNewbieData,
    pub news_stand: PropertyNewsStandData,
    pub post_girl: PropertyPostGirlData,
    pub business_card: PropertyBusinessCardData,
    pub player_accessory: PropertyPlayerAccessoryData,
}

impl MiscModel {
    pub fn load_from_pb(pb: MiscData) -> Self {
        Self {
            switch: pb
                .switch
                .map(|data| PropertySwitchData {
                    open_system_id: data.open_system_id_list.into_iter().collect(),
                    setting_switch_map: data.setting_switch_map.into_iter().collect(),
                    system_switch_state_map: data.system_switch_state_map.into_iter().collect(),
                    input_setting_map: data
                        .input_setting_map
                        .into_iter()
                        .map(|(ty, setting)| {
                            (
                                ty,
                                InputSetting {
                                    input_type_map: setting.input_type_map,
                                },
                            )
                        })
                        .collect(),
                })
                .unwrap_or_default(),
            unlock: pb
                .unlock
                .map(|data| PropertyUnlockData {
                    unlocked_id: data.unlocked_id_list.into_iter().collect(),
                    quick_access: data
                        .quick_access_list
                        .into_iter()
                        .map(|item| {
                            (
                                item.quick_access_id,
                                QuickAccess {
                                    index: item.quick_access_index,
                                    quick_access_type: item.quick_access_type,
                                },
                            )
                        })
                        .collect(),
                })
                .unwrap_or_default(),
            newbie: pb
                .newbie
                .map(|data| PropertyNewbieData {
                    finished_groups: data.finished_group_id_list.into_iter().collect(),
                    triggered_groups: PropertyHashSet::default(),
                })
                .unwrap_or_default(),
            news_stand: PropertyNewsStandData::load_from_pb(pb.news_stand.unwrap_or_default()),
            post_girl: pb
                .post_girl
                .map(|data| PropertyPostGirlData {
                    unlocked_items: data
                        .post_girl_item_list
                        .into_iter()
                        .map(|item| (item.id, item.unlock_time))
                        .collect(),
                    selected_id: data.selected_id_list.into_iter().collect(),
                    random_toggle: data.post_girl_random_toggle.into(),
                })
                .unwrap_or_default(),
            teleport: pb
                .teleport
                .map(|data| PropertyTeleportUnlockData {
                    unlocked_id: data.unlocked_id_list.into_iter().collect(),
                })
                .unwrap_or_default(),
            business_card: pb
                .business_card
                .map(|data| PropertyBusinessCardData {
                    unlocked_items: data.unlocked_id_list.into_iter().collect(),
                    selected_id: data.selected_id.into(),
                })
                .unwrap_or_default(),
            player_accessory: pb
                .player_accessory
                .map(|data| PropertyPlayerAccessoryData {
                    player_accessory_map: data
                        .player_accessory_list
                        .into_iter()
                        .map(|player_accessory| {
                            (
                                player_accessory.avatar_id,
                                PlayerAccessory {
                                    avatar_skin_id: player_accessory.avatar_skin_id,
                                    player_skin_map: player_accessory
                                        .player_skin_list
                                        .into_iter()
                                        .map(|player_skin| {
                                            (
                                                player_skin.player_skin_id,
                                                PlayerSkin {
                                                    equipped_accessory_id_list: player_skin
                                                        .equipped_accessory_id_list
                                                        .into_iter()
                                                        .collect(),
                                                },
                                            )
                                        })
                                        .collect(),
                                },
                            )
                        })
                        .collect(),
                })
                .unwrap_or_default(),
        }
    }
}

impl Saveable for MiscModel {
    fn save_to_pb(&self, root: &mut vivian_proto::server_only::PlayerData) {
        root.misc = Some(MiscData {
            switch: Some(SwitchData {
                open_system_id_list: self.switch.open_system_id.iter().copied().collect(),
                setting_switch_map: self
                    .switch
                    .setting_switch_map
                    .iter()
                    .map(|(&k, &v)| (k, v))
                    .collect(),
                system_switch_state_map: self
                    .switch
                    .system_switch_state_map
                    .iter()
                    .map(|(&ty, &state)| (ty, state))
                    .collect(),
                input_setting_map: self
                    .switch
                    .input_setting_map
                    .iter()
                    .map(|(&ty, setting)| {
                        (
                            ty,
                            vivian_proto::server_only::InputSettingInfo {
                                input_type_map: setting.input_type_map.clone(),
                            },
                        )
                    })
                    .collect(),
            }),
            unlock: Some(UnlockData {
                unlocked_id_list: self.unlock.unlocked_id.iter().copied().collect(),
                quick_access_list: self
                    .unlock
                    .quick_access
                    .iter()
                    .map(|(&quick_access_id, item)| QuickAccessItem {
                        quick_access_id,
                        quick_access_index: item.index,
                        quick_access_type: item.quick_access_type,
                    })
                    .collect(),
            }),
            newbie: Some(NewbieData {
                finished_group_id_list: self.newbie.finished_groups.iter().copied().collect(),
            }),
            news_stand: Some(self.news_stand.save_to_pb()),
            post_girl: Some(PostGirlData {
                post_girl_item_list: self
                    .post_girl
                    .unlocked_items
                    .iter()
                    .map(|(&id, &unlock_time)| PostGirlItem { id, unlock_time })
                    .collect(),
                selected_id_list: self.post_girl.selected_id.iter().copied().collect(),
                post_girl_random_toggle: self.post_girl.random_toggle.get(),
            }),
            teleport: Some(TeleportUnlockData {
                unlocked_id_list: self.teleport.unlocked_id.iter().copied().collect(),
            }),
            business_card: Some(BusinessCardData {
                unlocked_id_list: self.business_card.unlocked_items.iter().copied().collect(),
                selected_id: self.business_card.selected_id.get(),
            }),
            player_accessory: Some(PlayerAccessoryData {
                player_accessory_list: self
                    .player_accessory
                    .player_accessory_map
                    .iter()
                    .map(|(&avatar_id, player_accessory)| PlayerAccessoryInfo {
                        avatar_id,
                        avatar_skin_id: player_accessory.avatar_skin_id,
                        player_skin_list: player_accessory
                            .player_skin_map
                            .iter()
                            .map(|(&player_skin_id, player_skin)| PlayerSkinInfo {
                                player_skin_id,
                                equipped_accessory_id_list: player_skin
                                    .equipped_accessory_id_list
                                    .iter()
                                    .copied()
                                    .collect(),
                            })
                            .collect(),
                    })
                    .collect(),
            }),
        });
    }
}

impl PropertyNewsStandData {
    pub fn load_from_pb(pb: vivian_proto::server_only::NewsStandData) -> Self {
        Self {
            cur_style: pb.cur_style.into(),
            normal_news_id: pb.normal_news_id_list.iter().copied().collect(),
            head_lines_id: pb.head_lines_id_list.into_iter().collect(),
            advertisement_id: pb.advertisement_id_list.into_iter().collect(),
            news_read_state: pb.news_read_state.into(),
            can_sign: pb.can_sign.into(),
            current_sign_id: pb.current_sign_id.into(),
            sign_count: pb.sign_count.into(),
            sign_refresh_time: pb.sign_refresh_time.into(),
            last_sign_time: pb.last_sign_time.into(),
        }
    }

    pub fn save_to_pb(&self) -> vivian_proto::server_only::NewsStandData {
        vivian_proto::server_only::NewsStandData {
            cur_style: self.cur_style.get(),
            normal_news_id_list: self.normal_news_id.iter().copied().collect(),
            head_lines_id_list: self.head_lines_id.iter().copied().collect(),
            advertisement_id_list: self.advertisement_id.iter().copied().collect(),
            news_read_state: self.news_read_state.get(),
            can_sign: self.can_sign.get(),
            current_sign_id: self.current_sign_id.get(),
            sign_count: self.sign_count.get(),
            sign_refresh_time: self.sign_refresh_time.get(),
            last_sign_time: self.last_sign_time.get(),
        }
    }

    pub fn to_client_proto(&self) -> vivian_proto::NewsStandData {
        vivian_proto::NewsStandData {
            cur_style: self.cur_style.get(),
            normal_news_id_list: self.normal_news_id.iter().copied().collect(),
            head_lines_id_list: self.head_lines_id.iter().copied().collect(),
            advertisement_id_list: self.advertisement_id.iter().copied().collect(),
            news_read_state: self.news_read_state.get(),
            can_sign: self.can_sign.get(),
            current_sign_id: self.current_sign_id.get(),
            sign_count: self.sign_count.get(),
            sign_refresh_time: self.sign_refresh_time.get(),
            last_sign_time: self.last_sign_time.get(),
        }
    }
}

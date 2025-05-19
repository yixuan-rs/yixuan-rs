use common::time_util;
use vivian_codegen::{Model, Property};
use vivian_proto::server_only::{
    MiscData, NewbieData, PostGirlData, PostGirlItem, QuickAccessItem, SwitchData, UnlockData,
};

use crate::{
    logic::{
        property::{PrimitiveProperty, Property, PropertyHashMap, PropertyHashSet},
        sync::{DataSyncHelper, LoginDataSyncComponent, PlayerSyncComponent, SyncType},
    },
    resources::NapResources,
};

use super::{Model, Saveable};

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

#[derive(Property, Default)]
pub struct PropertySwitchData {
    pub open_system_id: PropertyHashSet<u32>,
}

#[derive(Model)]
pub struct MiscModel {
    pub switch: PropertySwitchData,
    pub unlock: PropertyUnlockData,
    pub newbie: PropertyNewbieData,
    pub news_stand: PropertyNewsStandData,
    pub post_girl: PropertyPostGirlData,
}

impl MiscModel {
    pub fn on_first_login(&mut self, res: &NapResources) {
        // Unlock everything by default for now
        let cur_time = time_util::unix_timestamp_seconds();

        res.templates.unlock_config_template_tb().for_each(|tmpl| {
            self.unlock.unlocked_id.insert(tmpl.id());
        });

        res.templates
            .post_girl_config_template_tb()
            .for_each(|tmpl| {
                self.post_girl.unlocked_items.insert(tmpl.id(), cur_time);
            });

        self.post_girl.selected_id = PropertyHashSet::from_iter([3500001]);

        self.news_stand.advertisement_id = PropertyHashSet::from_iter([9, 14]);
        self.news_stand.head_lines_id = PropertyHashSet::from_iter([1000004, 2000001]);
        self.news_stand.normal_news_id = PropertyHashSet::from_iter([37, 12, 7]);
    }

    pub fn load_from_pb(pb: MiscData) -> Self {
        Self {
            switch: pb
                .switch
                .map(|data| PropertySwitchData {
                    open_system_id: data.open_system_id_list.into_iter().collect(),
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
        }
    }
}

impl Saveable for MiscModel {
    fn save_to_pb(&self, root: &mut vivian_proto::server_only::PlayerData) {
        root.misc = Some(MiscData {
            switch: Some(SwitchData {
                open_system_id_list: self.switch.open_system_id.iter().copied().collect(),
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

impl LoginDataSyncComponent for MiscModel {
    fn prepare_responses(&self, sync_helper: &mut DataSyncHelper, res: &NapResources) {
        sync_helper.add_response(
            SyncType::BasicData,
            vivian_proto::VideoGetInfoScRsp {
                retcode: 0,
                video_key_map: res.video_key_map.clone(),
            },
        );

        sync_helper.add_response(
            SyncType::ExtendData,
            vivian_proto::GetMiscDataScRsp {
                retcode: 0,
                data: Some(vivian_proto::MiscData {
                    unlock: Some(vivian_proto::UnlockInfo {
                        unlocked_list: self.unlock.unlocked_id.iter().copied().collect(),
                        quick_access_list: self
                            .unlock
                            .quick_access
                            .iter()
                            .map(|(&quick_access_id, item)| vivian_proto::QuickAccessInfo {
                                quick_access_id,
                                quick_access_index: item.index,
                                r#type: item.quick_access_type,
                            })
                            .collect(),
                    }),
                    news_stand: Some(self.news_stand.to_client_proto()),
                    post_girl: Some(vivian_proto::PostGirlInfo {
                        post_girl_item_list: self
                            .post_girl
                            .unlocked_items
                            .iter()
                            .map(|(&id, &unlock_time)| vivian_proto::PostGirlItem {
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
                }),
            },
        );
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
    }
}

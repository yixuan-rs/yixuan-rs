use std::collections::HashMap;

use config::WeaponTemplate;
use vivian_codegen::Model;
use vivian_logic::item::{EquipItem, WeaponItem};
use vivian_proto::server_only::{
    EquipItemInfo, EquipPropertyInfo, GenericItemInfo, ItemData, WeaponItemInfo,
};

use crate::{
    logic::{
        property::{PrimitiveProperty, Property, PropertyHashMap},
        sync::{LoginDataSyncComponent, PlayerSyncComponent, SyncType},
    },
    resources::NapResources,
};

use super::{Model, Saveable};

#[derive(Model)]
pub struct ItemModel {
    pub item_count_map: PropertyHashMap<u32, i32>,
    pub item_uid_counter: PrimitiveProperty<u32>,
    pub weapon_map: PropertyHashMap<u32, WeaponItem>,
    pub equip_map: PropertyHashMap<u32, EquipItem>,
    pub new_reward_map: PropertyHashMap<u32, Vec<(u32, i32)>>,
}

impl ItemModel {
    pub fn on_first_login(&mut self, res: &NapResources) {
        self.item_count_map.insert(10, 10_000_000);
        self.item_count_map.insert(100, 10_000_000);
        self.item_count_map.insert(501, 240);

        // Unlock all skins by default for now
        res.templates
            .avatar_skin_base_template_tb()
            .for_each(|tmpl| self.item_count_map.insert(tmpl.id(), 1));

        // Unlock all weapons as well
        res.templates.weapon_template_tb().for_each(|tmpl| {
            let uid = self.next_uid();
            self.weapon_map.insert(
                uid,
                WeaponItem {
                    id: tmpl.item_id(),
                    level: 60,
                    exp: 0,
                    star: tmpl.star_limit() + 1,
                    refine_level: tmpl.refine_limit(),
                    lock: false,
                },
            );
        });
    }

    pub fn add_weapon(&mut self, template: &WeaponTemplate) -> u32 {
        let uid = self.next_uid();
        self.weapon_map.insert(
            uid,
            WeaponItem {
                id: template.item_id(),
                level: 0,
                exp: 0,
                star: 1,
                refine_level: 0,
                lock: false,
            },
        );

        uid
    }

    pub fn add_item(&mut self, item_id: u32, amount: u32) {
        let cur = self
            .item_count_map
            .get(&item_id)
            .copied()
            .unwrap_or_default();

        self.item_count_map.insert(item_id, cur + amount as i32);
    }

    pub fn has_enough_items(&self, item_id: u32, amount: u32) -> bool {
        self.item_count_map
            .get(&item_id)
            .copied()
            .unwrap_or_default()
            >= amount as i32
    }

    pub fn use_item(&mut self, item_id: u32, amount: u32) -> bool {
        if self.has_enough_items(item_id, amount) {
            let cur = self
                .item_count_map
                .get(&item_id)
                .copied()
                .unwrap_or_default();

            self.item_count_map.insert(item_id, cur - amount as i32);
            true
        } else {
            false
        }
    }

    pub fn load_from_pb(pb: ItemData) -> Self {
        Self {
            item_count_map: pb
                .item_list
                .into_iter()
                .map(|item| (item.id, item.count))
                .collect(),
            weapon_map: pb
                .weapon_list
                .into_iter()
                .map(|weapon| {
                    (
                        weapon.uid,
                        WeaponItem {
                            id: weapon.id,
                            level: weapon.level,
                            exp: weapon.exp,
                            star: weapon.star,
                            refine_level: weapon.refine_level,
                            lock: weapon.lock,
                        },
                    )
                })
                .collect(),
            equip_map: pb
                .equip_list
                .into_iter()
                .map(|equip| {
                    (
                        equip.uid,
                        EquipItem {
                            id: equip.id,
                            level: equip.level,
                            exp: equip.exp,
                            star: equip.star,
                            lock: equip.lock,
                            properties: equip
                                .properties
                                .into_iter()
                                .map(|prop| (prop.property_type, (prop.value, prop.add_value)))
                                .collect(),
                            sub_properties: equip
                                .sub_properties
                                .into_iter()
                                .map(|prop| (prop.property_type, (prop.value, prop.add_value)))
                                .collect(),
                        },
                    )
                })
                .collect(),
            item_uid_counter: pb.item_uid_counter.into(),
            new_reward_map: PropertyHashMap::default(),
        }
    }

    pub fn next_uid(&mut self) -> u32 {
        let uid = self.item_uid_counter.get() + 1;
        self.item_uid_counter.set(uid);

        uid
    }
}

impl Saveable for ItemModel {
    fn save_to_pb(&self, root: &mut vivian_proto::server_only::PlayerData) {
        root.item = Some(ItemData {
            item_list: self
                .item_count_map
                .iter()
                .map(|(&id, &count)| GenericItemInfo { id, count })
                .collect(),
            weapon_list: self
                .weapon_map
                .iter()
                .map(|(&uid, weapon)| WeaponItemInfo {
                    uid,
                    id: weapon.id,
                    level: weapon.level,
                    exp: weapon.exp,
                    star: weapon.star,
                    refine_level: weapon.refine_level,
                    lock: weapon.lock,
                })
                .collect(),
            equip_list: self
                .equip_map
                .iter()
                .map(|(&uid, equip)| EquipItemInfo {
                    uid,
                    id: equip.id,
                    level: equip.level,
                    exp: equip.exp,
                    star: equip.star,
                    lock: equip.lock,
                    properties: equip
                        .properties
                        .iter()
                        .map(|(&property_type, &(value, add_value))| EquipPropertyInfo {
                            property_type,
                            value,
                            add_value,
                        })
                        .collect(),
                    sub_properties: equip
                        .sub_properties
                        .iter()
                        .map(|(&property_type, &(value, add_value))| EquipPropertyInfo {
                            property_type,
                            value,
                            add_value,
                        })
                        .collect(),
                })
                .collect(),
            auto_recovery_item_map: HashMap::new(), // TODO: AutoRecoveryItem
            item_uid_counter: self.item_uid_counter.get(),
        });
    }
}

impl LoginDataSyncComponent for ItemModel {
    fn prepare_responses(
        &self,
        sync_helper: &mut crate::logic::sync::DataSyncHelper,
        _res: &NapResources,
    ) {
        sync_helper.add_response(
            SyncType::BasicData,
            vivian_proto::GetWeaponDataScRsp {
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
            vivian_proto::GetEquipDataScRsp {
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
            vivian_proto::GetItemDataScRsp {
                retcode: 0,
                item_list: self
                    .item_count_map
                    .iter()
                    .map(|(&id, &count)| vivian_proto::ItemInfo { id, count })
                    .collect(),
                auto_recovery_info: HashMap::new(),
            },
        );

        sync_helper.add_response(
            SyncType::BasicData,
            vivian_proto::GetWishlistDataScRsp { retcode: 0 },
        );
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

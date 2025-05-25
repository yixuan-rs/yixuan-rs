use std::collections::HashMap;

use super::*;
use property::{Property, PropertyHashMap, PrimitiveProperty};
use vivian_logic::item::{EquipItem, WeaponItem};

#[derive(Model)]
pub struct ItemModel {
    pub item_count_map: PropertyHashMap<u32, i32>,
    pub item_uid_counter: PrimitiveProperty<u32>,
    pub weapon_map: PropertyHashMap<u32, WeaponItem>,
    pub equip_map: PropertyHashMap<u32, EquipItem>,
    pub new_reward_map: PropertyHashMap<u32, Vec<(u32, i32)>>,
}

impl ItemModel {
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

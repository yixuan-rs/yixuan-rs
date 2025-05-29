use std::collections::HashMap;

use config::WeaponTemplate;
use rand::{RngCore, seq::IteratorRandom};
use vivian_logic::item::WeaponItem;

use crate::player::Player;

pub fn add_items_on_first_login(player: &mut Player) {
    player.item_model.item_count_map.insert(10, 10_000_000);
    player.item_model.item_count_map.insert(100, 10_000_000);
    player.item_model.item_count_map.insert(501, 240);

    // Unlock all skins by default for now
    player
        .resources
        .templates
        .avatar_skin_base_template_tb()
        .for_each(|tmpl| player.item_model.item_count_map.insert(tmpl.id(), 1));

    // Unlock all player skin accessories by default for now
    player
        .resources
        .templates
        .player_skin_accessories_config_template_tb()
        .for_each(|tmpl| {
            player
                .item_model
                .item_count_map
                .insert(tmpl.accessory_id(), 1)
        });

    player
        .resources
        .templates
        .weapon_template_tb()
        .for_each(|tmpl| {
            let uid = player.item_model.next_uid();
            player.item_model.weapon_map.insert(
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

    // Generate some Drive Discs for now
    let rng = &mut rand::thread_rng();

    type IntVec = Vec<u32>;
    let properties_map: [(u32, IntVec, u32, IntVec, u32); 19] = [
        (11103, vec![1], 550, vec![1, 2, 3, 4, 5, 6], 112),
        (11102, vec![4, 5, 6], 750, vec![1, 2, 3, 4, 5, 6], 300),
        (12103, vec![2], 79, vec![1, 2, 3, 4, 5, 6], 19),
        (12102, vec![4, 5, 6], 750, vec![1, 2, 3, 4, 5, 6], 300),
        (13103, vec![3], 46, vec![1, 2, 3, 4, 5, 6], 15),
        (13102, vec![4, 5, 6], 1200, vec![1, 2, 3, 4, 5, 6], 480),
        (23203, vec![], 0, vec![1, 2, 3, 4, 5, 6], 9),
        (23103, vec![5], 600, vec![], 0),
        (31402, vec![6], 750, vec![], 0),
        (31203, vec![4], 23, vec![1, 2, 3, 4, 5, 6], 9),
        (21103, vec![4], 1200, vec![1, 2, 3, 4, 5, 6], 480),
        (20103, vec![4], 600, vec![1, 2, 3, 4, 5, 6], 240),
        (30502, vec![6], 1500, vec![], 0),
        (12202, vec![6], 450, vec![], 0),
        (31803, vec![5], 750, vec![], 0),
        (31903, vec![5], 750, vec![], 0),
        (31603, vec![5], 750, vec![], 0),
        (31703, vec![5], 750, vec![], 0),
        (31503, vec![5], 750, vec![], 0),
    ];

    for _ in 0..500 {
        let uid = player.item_model.next_uid();

        let id = player
            .resources
            .templates
            .equipment_suit_template_tb()
            .choose(rng)
            .unwrap()
            .id();
        let id = id + 40; // S-rank
        let slot = 1 + rng.next_u32() % 6;
        let id = id + slot;

        let main_property = properties_map
            .iter()
            .filter(|p| p.1.contains(&slot))
            .choose(rng)
            .map(|p| (p.0, (p.2, 1)))
            .unwrap();

        let mut sub_properties = HashMap::new();
        let mut add_value_mod = 6;
        for _ in 0..4 {
            let sub_property = properties_map
                .iter()
                .filter(|p| {
                    p.3.contains(&slot)
                        && p.0 != main_property.0
                        && !sub_properties.contains_key(&p.0)
                })
                .choose(rng)
                .map(|p| {
                    let add_value = rng.next_u32() % add_value_mod;
                    add_value_mod -= add_value;
                    (p.0, (p.4, 1 + add_value))
                })
                .unwrap();
            sub_properties.insert(sub_property.0, sub_property.1);
        }

        player.item_model.equip_map.insert(
            uid,
            vivian_logic::item::EquipItem {
                id,
                level: 15,
                exp: 0,
                star: 1,
                lock: false,
                properties: [main_property].into_iter().collect(),
                sub_properties,
            },
        );
    }
}

pub fn add_weapon(player: &mut Player, template: &WeaponTemplate) -> u32 {
    let uid = player.item_model.next_uid();
    player.item_model.weapon_map.insert(
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

pub fn add_item(player: &mut Player, item_id: u32, amount: u32) {
    let cur = player
        .item_model
        .item_count_map
        .get(&item_id)
        .copied()
        .unwrap_or_default();

    player
        .item_model
        .item_count_map
        .insert(item_id, cur + amount as i32);
}

pub fn has_enough_items(player: &Player, item_id: u32, amount: u32) -> bool {
    player
        .item_model
        .item_count_map
        .get(&item_id)
        .copied()
        .unwrap_or_default()
        >= amount as i32
}

pub fn use_item(player: &mut Player, item_id: u32, amount: u32) -> bool {
    if has_enough_items(player, item_id, amount) {
        let cur = player
            .item_model
            .item_count_map
            .get(&item_id)
            .copied()
            .unwrap_or_default();

        player
            .item_model
            .item_count_map
            .insert(item_id, cur - amount as i32);
        true
    } else {
        false
    }
}

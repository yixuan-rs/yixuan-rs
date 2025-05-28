use config::WeaponTemplate;
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

use yixuan_models::{AreaGroupInfo, AreaStreetInfo};

use crate::player::Player;

pub fn init_map_structs_on_first_login(player: &mut Player) {
    // Unlock everything by default for now
    let model = &mut player.map_model;

    player
        .resources
        .templates
        .urban_area_map_group_template_tb()
        .map(|tmpl| tmpl.area_group_id())
        .chain(
            player
                .resources
                .templates
                .big_scene_map_area_template_tb()
                .map(|tmpl| tmpl.area_id()),
        )
        .for_each(|id| {
            model.area_group_map.insert(
                id,
                AreaGroupInfo {
                    is_unlocked: true,
                    area_progress: (rand::RngCore::next_u32(&mut rand::thread_rng()) % 100) * 100,
                },
            );
        });

    player
        .resources
        .templates
        .urban_area_map_template_tb()
        .map(|tmpl| tmpl.area_id())
        .chain(
            player
                .resources
                .templates
                .big_scene_map_area_template_tb()
                .map(|tmpl| tmpl.area_id()),
        )
        .for_each(|area_id| {
            model.area_street_map.insert(
                area_id,
                AreaStreetInfo {
                    is_unlocked: true,
                    area_progress: (rand::RngCore::next_u32(&mut rand::thread_rng()) % 100) * 100,
                    location_pop_showed: false,
                    new_area_showed: false,
                    new_area_portals_showed: false,
                },
            );
        });
}

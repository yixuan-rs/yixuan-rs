use common::time_util;
use vivian_models::property::PropertyHashSet;

use crate::player::Player;

pub fn init_misc_structs_on_first_login(player: &mut Player) {
    // Unlock everything by default for now
    let cur_time = time_util::unix_timestamp_seconds();
    let model = &mut player.misc_model;

    player
        .resources
        .templates
        .unlock_config_template_tb()
        .for_each(|tmpl| {
            model.unlock.unlocked_id.insert(tmpl.id());
        });

    player
        .resources
        .templates
        .post_girl_config_template_tb()
        .for_each(|tmpl| {
            model.post_girl.unlocked_items.insert(tmpl.id(), cur_time);
        });

    model.post_girl.selected_id = PropertyHashSet::from_iter([3500001]);

    model.news_stand.advertisement_id = PropertyHashSet::from_iter([9, 14]);
    model.news_stand.head_lines_id = PropertyHashSet::from_iter([1000004, 2000001]);
    model.news_stand.normal_news_id = PropertyHashSet::from_iter([37, 12, 7]);

    player
        .resources
        .templates
        .teleport_config_template_tb()
        .for_each(|tmpl| {
            model.teleport.unlocked_id.insert(tmpl.teleport_id());
        });
}

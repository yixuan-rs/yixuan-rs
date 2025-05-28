use common::time_util;
use config::AvatarBaseTemplate;
use std::collections::HashMap;
use vivian_logic::item::{AvatarItem, EAvatarSkillType};

use crate::player::Player;

pub fn unlock_avatars_on_first_login(player: &mut Player) {
    const DEFAULT_AVATARS: &[u32] = &[1011, 1081];

    DEFAULT_AVATARS
        .iter()
        .filter_map(|id| {
            player
                .resources
                .templates
                .avatar_base_template_tb()
                .find(|tmpl| tmpl.id() == *id)
        })
        .for_each(|tmpl| unlock_avatar(player, &tmpl, None));
}

pub fn unlock_avatar(
    player: &mut Player,
    base_template: &AvatarBaseTemplate,
    perform_type: Option<vivian_proto::add_avatar_sc_notify::PerformType>,
) {
    const AVATAR_BLACKLIST: &[u32] = &[];

    let avatar_id = base_template.id();

    if base_template.camp() != 0
        && !player.avatar_model.avatar_map.contains_key(&avatar_id)
        && !AVATAR_BLACKLIST.contains(&avatar_id)
    {
        let skill_level_map: HashMap<EAvatarSkillType, u32> = (0..EAvatarSkillType::EnumCount
            .into())
            .map(|ty| EAvatarSkillType::try_from(ty).unwrap())
            .zip([0].into_iter().cycle())
            .collect();

        player.avatar_model.avatar_map.insert(
            avatar_id,
            AvatarItem {
                id: avatar_id,
                level: 1,
                exp: 0,
                rank: 1,
                unlocked_talent_num: 0,
                talent_switch: [false; 6],
                passive_skill_level: 0,
                skill_level_map,
                weapon_uid: 0,
                show_weapon_type: vivian_proto::AvatarShowWeaponType::ShowWeaponLock.into(),
                dressed_equip_map: HashMap::new(),
                first_get_time: time_util::unix_timestamp_seconds(),
                taken_rank_up_reward_list: Vec::new(),
                avatar_skin_id: 0,
                is_favorite: false,
            },
        );

        if let Some(perform_type) = perform_type {
            player
                .avatar_model
                .add_avatar_perform_map
                .insert(avatar_id, perform_type);
        }
    }
}

pub fn dress_equip(player: &mut Player, avatar_id: u32, (equip_uid, dress_index): (u32, u32)) {
    player
        .avatar_model
        .avatar_map
        .iter()
        .filter_map(|(&id, avatar)| {
            avatar
                .dressed_equip_map
                .contains_key(&equip_uid)
                .then_some((id, equip_uid))
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|(id, equip_uid)| {
            player
                .avatar_model
                .avatar_map
                .get_mut(&id)
                .unwrap()
                .dressed_equip_map
                .remove(&equip_uid);
        });

    let avatar = player.avatar_model.avatar_map.get_mut(&avatar_id).unwrap();

    avatar
        .dressed_equip_map
        .retain(|_, dressed_equip_index| *dressed_equip_index != dress_index);

    avatar.dressed_equip_map.insert(equip_uid, dress_index);
}

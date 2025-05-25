use common::time_util;
use config::AvatarBaseTemplate;
use std::collections::HashMap;
use vivian_logic::item::{AvatarItem, EAvatarSkillType};

use crate::{config::FirstLoginAvatarConfig, logic::player::Player};

pub fn unlock_avatars_on_first_login(player: &mut Player) {
    let cfg = &player.resources.gameplay.first_login.avatar;

    if cfg.unlock_all {
        player
            .resources
            .templates
            .avatar_base_template_tb()
            .for_each(|tmpl| unlock_avatar(player, &tmpl, None, Some(cfg)));
    } else {
        cfg.unlock_id_list
            .iter()
            .filter_map(|id| {
                player
                    .resources
                    .templates
                    .avatar_base_template_tb()
                    .find(|tmpl| tmpl.id() == *id)
            })
            .for_each(|tmpl| unlock_avatar(player, &tmpl, None, Some(cfg)));
    }
}

pub fn unlock_avatar(
    player: &mut Player,
    base_template: &AvatarBaseTemplate,
    perform_type: Option<vivian_proto::add_avatar_sc_notify::PerformType>,
    first_login_cfg: Option<&FirstLoginAvatarConfig>,
) {
    const AVATAR_BLACKLIST: &[u32] = &[];

    let avatar_id = base_template.id();

    if base_template.camp() != 0
        && !player.avatar_model.avatar_map.contains_key(&avatar_id)
        && !AVATAR_BLACKLIST.contains(&avatar_id)
    {
        let mut skill_level_map: HashMap<EAvatarSkillType, u32> = (0..EAvatarSkillType::EnumCount
            .into())
            .map(|ty| EAvatarSkillType::try_from(ty).unwrap())
            .zip([0].into_iter().cycle())
            .collect();
        if let Some(cfg) = first_login_cfg {
            skill_level_map = (0..EAvatarSkillType::EnumCount.into())
                .map(|ty| EAvatarSkillType::try_from(ty).unwrap())
                .zip(cfg.skill_level_map.clone())
                .collect();
        }

        player.avatar_model.avatar_map.insert(
            avatar_id,
            AvatarItem {
                id: avatar_id,
                level: first_login_cfg.map_or(1, |cfg| cfg.level),
                exp: 0,
                rank: first_login_cfg.map_or(1, |cfg| cfg.rank),
                unlocked_talent_num: first_login_cfg.map_or(0, |cfg| cfg.unlocked_talent_num),
                talent_switch: first_login_cfg.map_or([false; 6], |cfg| {
                    cfg.talent_switch.as_slice().try_into().unwrap()
                }),
                passive_skill_level: first_login_cfg.map_or(0, |cfg| cfg.passive_skill_level),
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

use std::cmp;

use tracing::{error, instrument};
use vivian_logic::{debug::GMCmd, item::EAvatarSkillType};
use vivian_models::SceneSnapshotExt;

use crate::{
    player::{AddItemSource, Player},
    util::{avatar_util, item_util},
};

#[instrument(skip(player))]
pub fn execute_gm_cmd(player: &mut Player, cmd: GMCmd) {
    use GMCmd::*;

    match cmd {
        AddItem { item_id } => {
            player.add_item(item_id, 1, AddItemSource::Custom);
        }
        SetYorozuyaLv { level } => {
            player.basic_model.level.set(level);
        }
        AddAllAvatar => player
            .resources
            .templates
            .avatar_base_template_tb()
            .for_each(|tmpl| avatar_util::unlock_avatar(player, &tmpl, None)),
        AvatarLvUp { avatar_id, level } => {
            let Some(target_rank) = player
                .resources
                .templates
                .avatar_level_advance_template_tb()
                .find(|tmpl| tmpl.min_level() <= level && tmpl.max_level() >= level)
            else {
                error!("no rank for level {level}");
                return;
            };

            let target_avatar_ids = player
                .avatar_model
                .avatar_map
                .keys()
                .filter(|&&id| avatar_id == 0 || id == avatar_id)
                .copied()
                .collect::<Vec<_>>();

            target_avatar_ids.into_iter().for_each(|avatar_id| {
                let avatar = player.avatar_model.avatar_map.get_mut(&avatar_id).unwrap();
                avatar.level = level;
                avatar.rank = cmp::max(target_rank.id(), avatar.rank);
            });
        }
        AvatarSkillUp {
            avatar_id,
            skill_type,
            level,
        } => {
            let Ok(skill_type) = EAvatarSkillType::try_from(skill_type) else {
                error!("skill type {skill_type} is invalid");
                return;
            };

            let target_avatar_ids = player
                .avatar_model
                .avatar_map
                .keys()
                .filter(|&&id| avatar_id == 0 || id == avatar_id)
                .copied()
                .collect::<Vec<_>>();

            target_avatar_ids.into_iter().for_each(|avatar_id| {
                let avatar = player.avatar_model.avatar_map.get_mut(&avatar_id).unwrap();
                avatar.skill_level_map.insert(skill_type, level);

                if skill_type == EAvatarSkillType::CoreSkill {
                    avatar.passive_skill_level = level;
                }
            });
        }
        AvatarTalentUp {
            avatar_id,
            talent_num,
        } => {
            let target_avatar_ids = player
                .avatar_model
                .avatar_map
                .keys()
                .filter(|&&id| avatar_id == 0 || id == avatar_id)
                .copied()
                .collect::<Vec<_>>();

            target_avatar_ids.into_iter().for_each(|avatar_id| {
                let avatar = player.avatar_model.avatar_map.get_mut(&avatar_id).unwrap();
                avatar.unlocked_talent_num = talent_num;
            });
        }
        AddWeapon {
            weapon_id,
            level,
            star,
            refine_level,
        } => {
            let Some(tmpl) = player
                .resources
                .templates
                .weapon_template_tb()
                .find(|tmpl| tmpl.item_id() == weapon_id)
            else {
                error!("weapon with id {weapon_id} doesn't exist");
                return;
            };

            let uid = item_util::add_weapon(player, &tmpl);
            let weapon = player.item_model.weapon_map.get_mut(&uid).unwrap();

            weapon.level = level;
            weapon.star = star;
            weapon.refine_level = refine_level;
        }
        SetAvatarSkin {
            avatar_id,
            avatar_skin_id,
        } => {
            if let Some(avatar) = player.avatar_model.avatar_map.get_mut(&avatar_id) {
                avatar.avatar_skin_id = avatar_skin_id;
            }
        }
        SetControlGuiseAvatar { avatar_id } => {
            player.basic_model.control_guise_avatar_id.set(avatar_id);
        }
        Jump {
            section_id,
            transform_id,
        } => {
            if let Some(default_scene) = player
                .scene_model
                .scene_snapshots
                .get_mut(&player.scene_model.default_scene_uid.get())
            {
                if let SceneSnapshotExt::Hall(hall) = &mut default_scene.ext {
                    hall.cur_section_id = section_id;
                    player.main_city_model.transform_id.set(&transform_id);
                }
            }
        }
    }
}

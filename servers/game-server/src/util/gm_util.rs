use std::cmp;

use config::{ActionSwitchSection, ConfigEvent, ConfigEventAction};
use itertools::Itertools;
use tracing::{error, instrument};
use vivian_logic::{
    GameState,
    debug::GMCmd,
    dungeon::EQuestType,
    item::{EAvatarSkillType, EquipItem},
};
use vivian_models::SceneSnapshotExt;

use crate::{
    player::{AddItemSource, Player},
    util::{avatar_util, item_util, quest_util},
};

#[instrument(skip(player, state))]
pub fn execute_gm_cmd(player: &mut Player, state: Option<&mut GameState>, cmd: GMCmd) {
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
                avatar
                    .skill_level_map
                    .insert(skill_type, level.clamp(1, 12));

                if skill_type == EAvatarSkillType::CoreSkill {
                    let passive_skill_level = cmp::min(level, 6);
                    avatar.passive_skill_level = passive_skill_level;
                    avatar
                        .skill_level_map
                        .insert(skill_type, passive_skill_level + 1);
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
        AddEquip {
            equip_id,
            level,
            star,
            property_params,
        } => {
            let mut property_params = property_params.into_iter().tuples();

            let properties = property_params
                .next()
                .into_iter()
                .map(|(key, base_value, add_value)| (key, (base_value, add_value)))
                .collect();

            let sub_properties = property_params
                .map(|(key, base_value, add_value)| (key, (base_value, add_value)))
                .collect();

            let uid = player.item_model.next_uid();

            player.item_model.equip_map.insert(
                uid,
                EquipItem {
                    id: equip_id,
                    level,
                    star,
                    exp: 0,
                    lock: false,
                    properties,
                    sub_properties,
                },
            );
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

            if let Some(GameState::Hall(hall)) = state {
                hall.control_guise_avatar_id = avatar_id;
                hall.force_refresh();
            }
        }
        UnlockHollowQuest { quest_id } => player
            .resources
            .templates
            .hollow_quest_template_tb()
            .filter_map(|q| (quest_id == 0 || q.id() == quest_id).then_some(q.id()))
            .for_each(|quest_id| {
                quest_util::add_hollow_quest(player, quest_id);
            }),
        ClearMainCityQuestCollection => {
            player
                .quest_model
                .get_or_insert_collection(EQuestType::MainCity)
                .quests
                .clear();

            if let Some(GameState::Hall(hall)) = state {
                hall.clear_main_city_quests();
                hall.force_refresh();
            }
        }
        UnlockBigBossQuest { quest_id } => {
            player
                .resources
                .templates
                .quest_config_template_tb()
                .filter(|q| q.quest_type() == EQuestType::BigBoss.into())
                .filter_map(|q| (quest_id == 0 || q.quest_id() == quest_id).then_some(q.quest_id()))
                .for_each(|quest_id| {
                    quest_util::add_big_boss_quest(player, quest_id);
                });
        }
        UnlockMonsterCardLevel { level } => {
            player
                .resources
                .templates
                .monster_card_difficulty_template_tb()
                .filter(|tmpl| tmpl.card_type() == 1)
                .filter_map(|tmpl| {
                    (level == 0 || tmpl.difficulty() == level).then_some(tmpl.difficulty())
                })
                .for_each(|level| {
                    player
                        .quest_model
                        .battle_data
                        .activity
                        .monster_card
                        .unlocked_levels
                        .insert(level);
                });
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

                    if let Some(GameState::Hall(hall)) = state {
                        hall.execute_gm_event(
                            ConfigEvent {
                                id: 1337,
                                actions: vec![ConfigEventAction::ActionSwitchSection(
                                    ActionSwitchSection {
                                        id: 100,
                                        section_id,
                                        transform: transform_id,
                                        camera_x: 6000,
                                        camera_y: 0,
                                        predicates: Vec::new(),
                                    },
                                )],
                            },
                            player,
                        );
                    }
                }
            }
        }
    }
}

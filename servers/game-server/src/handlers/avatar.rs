use yixuan_codegen::{handlers, required_state};
use yixuan_logic::item::{AvatarItem, EItemType};
use yixuan_proto::{
    AvatarFavoriteCsReq, AvatarFavoriteScRsp, AvatarLevelUpCsReq, AvatarLevelUpScRsp,
    AvatarSetAwakeCsReq, AvatarSetAwakeScRsp, AvatarShowWeaponCsReq, AvatarShowWeaponScRsp,
    AvatarSkinDressCsReq, AvatarSkinDressScRsp, AvatarSkinUnDressCsReq, AvatarSkinUnDressScRsp,
    EquipmentDressCsReq, EquipmentDressScRsp, EquipmentSuitDressCsReq, EquipmentSuitDressScRsp,
    EquipmentUnDressCsReq, EquipmentUnDressScRsp, GetAvatarDataCsReq, GetAvatarDataScRsp,
    GetAvatarRecommendEquipCsReq, GetAvatarRecommendEquipScRsp, ItemRewardInfo, TalentSwitchCsReq,
    TalentSwitchScRsp, WeaponDressCsReq, WeaponDressScRsp, WeaponUnDressCsReq, WeaponUnDressScRsp,
};

use crate::{
    sync::SyncType,
    util::{avatar_util, item_util},
};

use super::NetContext;

pub struct AvatarHandler;

#[handlers]
impl AvatarHandler {
    #[required_state(BasicDataSync)]
    pub fn on_get_avatar_data_cs_req(
        context: &mut NetContext<'_>,
        _request: GetAvatarDataCsReq,
    ) -> GetAvatarDataScRsp {
        context
            .player
            .sync_helper
            .remove_sync_response(SyncType::BasicData)
    }

    pub fn on_avatar_level_up_cs_req(
        context: &mut NetContext<'_>,
        request: AvatarLevelUpCsReq,
    ) -> AvatarLevelUpScRsp {
        if !context
            .player
            .avatar_model
            .is_avatar_unlocked(request.avatar_id)
        {
            return AvatarLevelUpScRsp {
                retcode: 1,
                ..Default::default()
            };
        }

        if !request
            .exp_materials
            .iter()
            .all(|(&id, &count)| item_util::has_enough_items(context.player, id, count))
        {
            return AvatarLevelUpScRsp {
                retcode: 1,
                ..Default::default()
            };
        }

        request.exp_materials.iter().for_each(|(&id, &count)| {
            item_util::use_item(context.player, id, count);
        });

        let added_exp = request
            .exp_materials
            .iter()
            .filter_map(|(&id, &count)| {
                let template = context
                    .resources
                    .templates
                    .item_template_tb()
                    .find(|tmpl| tmpl.id() == id)?;

                (template.class() == EItemType::AvatarLevelUpMaterial.into())
                    .then(|| template.parameters().unwrap().get(0) * count)
            })
            .sum::<u32>();

        let avatar = context
            .player
            .avatar_model
            .avatar_map
            .get_mut(&request.avatar_id)
            .unwrap();

        avatar.exp += added_exp;

        let max_level = context
            .resources
            .templates
            .avatar_level_advance_template_tb()
            .find(|tmpl| tmpl.avatar_id() == avatar.id && tmpl.id() == avatar.rank)
            .unwrap()
            .max_level();

        while avatar.level < max_level {
            let required_exp = context
                .resources
                .templates
                .avatar_level_template_tb()
                .find(|tmpl| tmpl.level() == avatar.level)
                .unwrap()
                .exp();

            if avatar.exp < required_exp {
                break;
            }

            avatar.exp -= required_exp;
            avatar.level += 1;
        }

        let mut rsp = AvatarLevelUpScRsp::default();

        if avatar.level == max_level {
            while avatar.exp > 0 {
                let Some(return_material) = context
                    .resources
                    .templates
                    .item_template_tb()
                    .filter(|tmpl| {
                        tmpl.class() == EItemType::AvatarLevelUpMaterial.into()
                            && tmpl.parameters().unwrap().get(0) <= avatar.exp
                    })
                    .max_by_key(|tmpl| tmpl.parameters().unwrap().get(0))
                else {
                    avatar.exp = 0;
                    break;
                };

                let exp_amount = return_material.parameters().unwrap().get(0);

                rsp.return_item_list.push(ItemRewardInfo {
                    item_id: return_material.id(),
                    amount: avatar.exp / exp_amount,
                });

                let cur = context
                    .player
                    .item_model
                    .item_count_map
                    .get(&return_material.id())
                    .copied()
                    .unwrap_or_default();

                context
                    .player
                    .item_model
                    .item_count_map
                    .insert(return_material.id(), cur + (avatar.exp / exp_amount) as i32);

                avatar.exp %= exp_amount;
            }
        }

        rsp
    }

    pub fn on_weapon_dress_cs_req(
        context: &mut NetContext<'_>,
        request: WeaponDressCsReq,
    ) -> WeaponDressScRsp {
        if !context
            .player
            .avatar_model
            .is_avatar_unlocked(request.avatar_id)
        {
            return WeaponDressScRsp { retcode: 1 };
        }

        let Some(weapon) = context
            .player
            .item_model
            .weapon_map
            .get(&request.weapon_uid)
        else {
            return WeaponDressScRsp { retcode: 1 };
        };

        context
            .player
            .avatar_model
            .avatar_map
            .iter()
            .filter_map(|(&id, avatar)| (avatar.weapon_uid == request.weapon_uid).then_some(id))
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|id| {
                context
                    .player
                    .avatar_model
                    .avatar_map
                    .get_mut(&id)
                    .unwrap()
                    .weapon_uid = 0
            });

        let avatar = context
            .player
            .avatar_model
            .avatar_map
            .get_mut(&request.avatar_id)
            .unwrap();

        avatar.weapon_uid = request.weapon_uid;

        if avatar.show_weapon_type == yixuan_proto::AvatarShowWeaponType::ShowWeaponLock.into()
            && context
                .resources
                .templates
                .weapon_template_tb()
                .any(|tmpl| tmpl.item_id() == weapon.id && tmpl.avatar_id() == request.avatar_id)
        {
            avatar.show_weapon_type = yixuan_proto::AvatarShowWeaponType::ShowWeaponActive.into();
        }

        WeaponDressScRsp { retcode: 0 }
    }

    pub fn on_weapon_un_dress_cs_req(
        context: &mut NetContext<'_>,
        request: WeaponUnDressCsReq,
    ) -> WeaponUnDressScRsp {
        if let Some(avatar) = context
            .player
            .avatar_model
            .avatar_map
            .get_mut(&request.avatar_id)
        {
            avatar.weapon_uid = 0;
            WeaponUnDressScRsp { retcode: 0 }
        } else {
            WeaponUnDressScRsp { retcode: 1 }
        }
    }

    pub fn on_avatar_show_weapon_cs_req(
        context: &mut NetContext<'_>,
        request: AvatarShowWeaponCsReq,
    ) -> AvatarShowWeaponScRsp {
        let Some(avatar) = context
            .player
            .avatar_model
            .avatar_map
            .get_mut(&request.avatar_id)
        else {
            return AvatarShowWeaponScRsp { retcode: 1 };
        };

        if avatar.show_weapon_type == yixuan_proto::AvatarShowWeaponType::ShowWeaponLock.into() {
            return AvatarShowWeaponScRsp { retcode: 1 };
        }

        avatar.show_weapon_type = request.show_weapon_type;

        AvatarShowWeaponScRsp { retcode: 0 }
    }

    pub fn on_avatar_favorite_cs_req(
        context: &mut NetContext<'_>,
        request: AvatarFavoriteCsReq,
    ) -> AvatarFavoriteScRsp {
        if let Some(avatar) = context
            .player
            .avatar_model
            .avatar_map
            .get_mut(&request.avatar_id)
        {
            avatar.is_favorite = request.is_favorite;

            AvatarFavoriteScRsp { retcode: 0 }
        } else {
            AvatarFavoriteScRsp { retcode: 1 }
        }
    }

    pub fn on_avatar_set_awake_cs_req(
        context: &mut NetContext<'_>,
        request: AvatarSetAwakeCsReq,
    ) -> AvatarSetAwakeScRsp {
        if let Some(avatar) = context
            .player
            .avatar_model
            .avatar_map
            .get_mut(&request.avatar_id)
        {
            match avatar.awake_id {
                0 => {
                    avatar.awake_id = match context
                        .resources
                        .templates
                        .avatar_battle_template_tb()
                        .find(|tmpl| tmpl.id() == avatar.id)
                        .and_then(|tmpl| tmpl.awake_ids())
                    {
                        Some(awake_ids) => awake_ids.iter().next().unwrap(),
                        None => 0,
                    };
                }
                _ => {
                    avatar.awake_id = 0;
                }
            }

            AvatarSetAwakeScRsp { retcode: 0 }
        } else {
            AvatarSetAwakeScRsp { retcode: 1 }
        }
    }

    pub fn on_avatar_skin_dress_cs_req(
        context: &mut NetContext<'_>,
        request: AvatarSkinDressCsReq,
    ) -> AvatarSkinDressScRsp {
        let Some(avatar) = context
            .player
            .avatar_model
            .avatar_map
            .get_mut(&request.avatar_id)
        else {
            return AvatarSkinDressScRsp { retcode: 1 };
        };

        if context
            .player
            .item_model
            .item_count_map
            .get(&request.avatar_skin_id)
            .copied()
            .unwrap_or_default()
            > 0
            && context
                .resources
                .templates
                .avatar_skin_base_template_tb()
                .find(|tmpl| tmpl.id() == request.avatar_skin_id)
                .map(|tmpl| tmpl.avatar_id() == request.avatar_id)
                .unwrap_or(false)
        {
            avatar.avatar_skin_id = request.avatar_skin_id;

            AvatarSkinDressScRsp { retcode: 0 }
        } else {
            AvatarSkinDressScRsp { retcode: 1 }
        }
    }

    pub fn on_avatar_skin_un_dress_cs_req(
        context: &mut NetContext<'_>,
        request: AvatarSkinUnDressCsReq,
    ) -> AvatarSkinUnDressScRsp {
        if let Some(avatar) = context
            .player
            .avatar_model
            .avatar_map
            .get_mut(&request.avatar_id)
        {
            avatar.avatar_skin_id = 0;
            AvatarSkinUnDressScRsp { retcode: 0 }
        } else {
            AvatarSkinUnDressScRsp { retcode: 1 }
        }
    }

    pub fn on_get_avatar_recommend_equip_cs_req(
        _context: &mut NetContext<'_>,
        _request: GetAvatarRecommendEquipCsReq,
    ) -> GetAvatarRecommendEquipScRsp {
        GetAvatarRecommendEquipScRsp { retcode: 0 }
    }

    pub fn on_equipment_dress_cs_req(
        context: &mut NetContext<'_>,
        request: EquipmentDressCsReq,
    ) -> EquipmentDressScRsp {
        if !context
            .player
            .avatar_model
            .is_avatar_unlocked(request.avatar_id)
        {
            return EquipmentDressScRsp { retcode: 1 };
        }

        if !context
            .player
            .item_model
            .equip_map
            .contains_key(&request.equip_uid)
        {
            return EquipmentDressScRsp { retcode: 1 };
        };

        avatar_util::dress_equip(
            context.player,
            request.avatar_id,
            (request.equip_uid, request.dress_index),
        );

        EquipmentDressScRsp { retcode: 0 }
    }

    pub fn on_equipment_un_dress_cs_req(
        context: &mut NetContext<'_>,
        request: EquipmentUnDressCsReq,
    ) -> EquipmentUnDressScRsp {
        if let Some(avatar) = context
            .player
            .avatar_model
            .avatar_map
            .get_mut(&request.avatar_id)
        {
            request.undress_index_list.iter().for_each(|undress_index| {
                avatar
                    .dressed_equip_map
                    .retain(|_, index| index != undress_index);
            });

            EquipmentUnDressScRsp { retcode: 0 }
        } else {
            EquipmentUnDressScRsp { retcode: 1 }
        }
    }

    pub fn on_equipment_suit_dress_cs_req(
        context: &mut NetContext<'_>,
        request: EquipmentSuitDressCsReq,
    ) -> EquipmentSuitDressScRsp {
        if !context
            .player
            .avatar_model
            .is_avatar_unlocked(request.avatar_id)
        {
            return EquipmentSuitDressScRsp { retcode: 1 };
        }

        if !request.param_list.iter().all(|param| {
            context
                .player
                .item_model
                .equip_map
                .contains_key(&param.equip_uid)
        }) {
            return EquipmentSuitDressScRsp { retcode: 1 };
        }

        request.param_list.iter().for_each(|param| {
            avatar_util::dress_equip(
                context.player,
                request.avatar_id,
                (param.equip_uid, param.dress_index),
            );
        });

        EquipmentSuitDressScRsp { retcode: 0 }
    }

    pub fn on_talent_switch_cs_req(
        context: &mut NetContext<'_>,
        request: TalentSwitchCsReq,
    ) -> TalentSwitchScRsp {
        let Some(avatar) = context
            .player
            .avatar_model
            .avatar_map
            .get_mut(&request.avatar_id)
        else {
            return TalentSwitchScRsp { retcode: 1 };
        };

        if request.talent_switch_list.len() != AvatarItem::MAX_TALENT_NUM
            || (request
                .talent_switch_list
                .iter()
                .enumerate()
                .rev()
                .find_map(|(i, switch)| switch.then_some(i + 1))
                .unwrap_or(0))
                > avatar.unlocked_talent_num as usize
        {
            return TalentSwitchScRsp { retcode: 1 };
        }

        if (0..AvatarItem::MAX_TALENT_NUM / 2).any(|i| {
            request.talent_switch_list[i]
                && request.talent_switch_list[i + AvatarItem::MAX_TALENT_NUM / 2]
        }) {
            return TalentSwitchScRsp { retcode: 1 };
        }

        avatar.talent_switch = request.talent_switch_list.try_into().unwrap();

        TalentSwitchScRsp { retcode: 0 }
    }
}

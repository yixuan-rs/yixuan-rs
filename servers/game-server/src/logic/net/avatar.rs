use vivian_codegen::{handlers, required_state};
use vivian_logic::item::EItemType;
use vivian_proto::{
    AvatarFavoriteCsReq, AvatarFavoriteScRsp, AvatarLevelUpCsReq, AvatarLevelUpScRsp,
    AvatarSkinDressCsReq, AvatarSkinDressScRsp, AvatarSkinUnDressCsReq, AvatarSkinUnDressScRsp,
    GetAvatarDataCsReq, GetAvatarDataScRsp, GetAvatarRecommendEquipCsReq,
    GetAvatarRecommendEquipScRsp, ItemRewardInfo, WeaponDressCsReq, WeaponDressScRsp,
    WeaponUnDressCsReq, WeaponUnDressScRsp,
};

use crate::logic::sync::SyncType;

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

        let item_model = &mut context.player.item_model;

        if !request
            .exp_materials
            .iter()
            .all(|(&id, &count)| item_model.has_enough_items(id, count))
        {
            return AvatarLevelUpScRsp {
                retcode: 1,
                ..Default::default()
            };
        }

        request.exp_materials.iter().for_each(|(&id, &count)| {
            item_model.use_item(id, count);
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

                item_model.add_item(return_material.id(), avatar.exp / exp_amount);
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

        if !context
            .player
            .item_model
            .weapon_map
            .contains_key(&request.weapon_uid)
        {
            return WeaponDressScRsp { retcode: 1 };
        }

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

        context
            .player
            .avatar_model
            .avatar_map
            .get_mut(&request.avatar_id)
            .unwrap()
            .weapon_uid = request.weapon_uid;

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
}

use yixuan_codegen::{handlers, required_state};
use yixuan_logic::item::EItemType;
use yixuan_proto::{
    GetEquipDataCsReq, GetEquipDataScRsp, GetItemDataCsReq, GetItemDataScRsp, GetWeaponDataCsReq,
    GetWeaponDataScRsp, GetWishlistDataCsReq, GetWishlistDataScRsp, ItemRewardInfo,
    WeaponLevelUpCsReq, WeaponLevelUpScRsp,
};

use crate::{sync::SyncType, util::item_util};

use super::NetContext;

pub struct ItemHandler;

#[handlers]
impl ItemHandler {
    #[required_state(BasicDataSync)]
    pub fn on_get_weapon_data_cs_req(
        context: &mut NetContext<'_>,
        _request: GetWeaponDataCsReq,
    ) -> GetWeaponDataScRsp {
        context
            .player
            .sync_helper
            .remove_sync_response(SyncType::BasicData)
    }

    #[required_state(BasicDataSync)]
    pub fn on_get_equip_data_cs_req(
        context: &mut NetContext<'_>,
        _request: GetEquipDataCsReq,
    ) -> GetEquipDataScRsp {
        context
            .player
            .sync_helper
            .remove_sync_response(SyncType::BasicData)
    }

    #[required_state(BasicDataSync)]
    pub fn on_get_resource_data_cs_req(
        context: &mut NetContext<'_>,
        _request: GetItemDataCsReq,
    ) -> GetItemDataScRsp {
        context
            .player
            .sync_helper
            .remove_sync_response(SyncType::BasicData)
    }

    #[required_state(BasicDataSync)]
    pub fn on_get_wishlist_data_cs_req(
        context: &mut NetContext<'_>,
        _request: GetWishlistDataCsReq,
    ) -> GetWishlistDataScRsp {
        context
            .player
            .sync_helper
            .remove_sync_response(SyncType::BasicData)
    }

    pub fn on_weapon_level_up_cs_req(
        context: &mut NetContext,
        request: WeaponLevelUpCsReq,
    ) -> WeaponLevelUpScRsp {
        if !context
            .player
            .item_model
            .weapon_map
            .contains_key(&request.weapon_uid)
        {
            return WeaponLevelUpScRsp {
                retcode: 1,
                ..Default::default()
            };
        }

        if !request
            .exp_materials
            .iter()
            .all(|(&id, &count)| item_util::has_enough_items(context.player, id, count))
        {
            return WeaponLevelUpScRsp {
                retcode: 1,
                ..Default::default()
            };
        }

        request.exp_materials.iter().for_each(|(&id, &count)| {
            item_util::use_item(context.player, id, count);
        });

        let added_exp = item_util::materials_to_exp(
            &request.exp_materials,
            EItemType::WeaponLevelUpMaterial,
            context.resources,
        );

        let weapon = context
            .player
            .item_model
            .weapon_map
            .get_mut(&request.weapon_uid)
            .unwrap();

        weapon.exp += added_exp;

        let weapon_rarity = context
            .resources
            .templates
            .item_template_tb()
            .find(|tmpl| tmpl.id() == weapon.id)
            .map(|tmpl| tmpl.rarity())
            .unwrap();

        let max_level = context
            .resources
            .templates
            .weapon_star_template_tb()
            .find(|tmpl| tmpl.rarity() == weapon_rarity && tmpl.star() == weapon.star)
            .unwrap()
            .max_level();

        while weapon.level < max_level {
            let required_exp = context
                .resources
                .templates
                .weapon_level_template_tb()
                .find(|tmpl| tmpl.rarity() == weapon_rarity && tmpl.level() == weapon.level)
                .unwrap()
                .exp();

            if weapon.exp < required_exp {
                break;
            }

            weapon.exp -= required_exp;
            weapon.level += 1;
        }

        WeaponLevelUpScRsp {
            retcode: 0,
            return_item_list: if weapon.level == max_level {
                item_util::exp_to_materials(
                    &mut weapon.exp,
                    EItemType::WeaponLevelUpMaterial,
                    context.resources,
                )
                .into_iter()
                .map(|(item_id, amount)| {
                    item_util::add_item(context.player, item_id, amount);
                    ItemRewardInfo { item_id, amount }
                })
                .collect()
            } else {
                Vec::new()
            },
        }
    }
}

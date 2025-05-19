use vivian_codegen::handlers;
use vivian_proto::{
    DoGachaCsReq, DoGachaScRsp, GachaBuyMaterialCsReq, GachaBuyMaterialScRsp,
    GachaSetNewbieAvatarCsReq, GachaSetNewbieAvatarScRsp, GetGachaDataCsReq, GetGachaDataScRsp,
};

use crate::logic::player::AddItemSource;

use super::NetContext;

pub struct GachaHandler;

#[handlers]
impl GachaHandler {
    pub fn on_get_gacha_data_cs_req(
        context: &mut NetContext<'_>,
        request: GetGachaDataCsReq,
    ) -> GetGachaDataScRsp {
        if request.gacha_type == 3 {
            let schedule = &context.resources.gameplay.gacha_schedule;

            schedule.gacha_schedule_list.iter().for_each(|schedule| {
                context
                    .player
                    .gacha_model
                    .ensure_statistics(schedule.gacha_id);
            });

            GetGachaDataScRsp {
                retcode: 0,
                gacha_type: request.gacha_type,
                display: Some(context.player.gacha_model.display_data(schedule)),
            }
        } else {
            GetGachaDataScRsp {
                retcode: 0,
                gacha_type: request.gacha_type,
                display: None,
            }
        }
    }

    fn on_gacha_set_newbie_avatar_cs_req(
        _context: &mut NetContext<'_>,
        _request: GachaSetNewbieAvatarCsReq,
    ) -> GachaSetNewbieAvatarScRsp {
        GachaSetNewbieAvatarScRsp { retcode: 0 }
    }

    fn on_do_gacha_cs_req(context: &mut NetContext<'_>, request: DoGachaCsReq) -> DoGachaScRsp {
        let Some(schedule) = context
            .resources
            .gameplay
            .gacha_schedule
            .gacha_schedule_list
            .iter()
            .find(|schedule| schedule.gacha_id == request.gacha_id)
        else {
            return DoGachaScRsp {
                retcode: 1,
                ..Default::default()
            };
        };

        if !schedule.gacha_materials.iter().all(|material| {
            context
                .player
                .item_model
                .has_enough_items(material.id, material.count * request.times)
        }) {
            return DoGachaScRsp {
                retcode: 1,
                ..Default::default()
            };
        }

        schedule.gacha_materials.iter().for_each(|material| {
            context
                .player
                .item_model
                .use_item(material.id, material.count * request.times);
        });

        let rewards = (0..request.times)
            .map(|_| {
                context
                    .player
                    .gacha_model
                    .do_gacha(schedule, context.resources)
            })
            .collect::<Vec<_>>();

        let mut drop_item_list = Vec::with_capacity(request.times as usize);

        for reward_item_id in rewards {
            let item_uid = context
                .player
                .add_item(reward_item_id, 1, AddItemSource::Gacha);

            drop_item_list.push(vivian_proto::DropItem {
                item_id: reward_item_id,
                uid: item_uid.unwrap_or(0),
                count: 1,
                point_item_id: 0,
                point_item_count: 0,
                lock: false,
            });
        }

        DoGachaScRsp {
            retcode: 0,
            times: request.times,
            drop_item_list,
            display: Some(
                context
                    .player
                    .gacha_model
                    .display_data(&context.resources.gameplay.gacha_schedule),
            ),
        }
    }

    fn on_gacha_buy_material_cs_req(
        context: &mut NetContext<'_>,
        request: GachaBuyMaterialCsReq,
    ) -> GachaBuyMaterialScRsp {
        const GACHA_MATERIAL_ID: &[u32] = &[110, 111];
        const PURCHASE_CURRENCY_ID: u32 = 100;
        const MATERIAL_PRICE: u32 = 160;

        if !GACHA_MATERIAL_ID.contains(&request.buy_material_id) {
            return GachaBuyMaterialScRsp { retcode: 1 };
        }

        let item_model = &mut context.player.item_model;

        if item_model.use_item(PURCHASE_CURRENCY_ID, MATERIAL_PRICE * request.count) {
            item_model.add_item(request.buy_material_id, request.count);
            GachaBuyMaterialScRsp { retcode: 0 }
        } else {
            GachaBuyMaterialScRsp { retcode: 1 }
        }
    }
}

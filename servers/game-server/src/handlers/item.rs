use yixuan_codegen::{handlers, required_state};
use yixuan_proto::{
    GetEquipDataCsReq, GetEquipDataScRsp, GetItemDataCsReq, GetItemDataScRsp, GetWeaponDataCsReq,
    GetWeaponDataScRsp, GetWishlistDataCsReq, GetWishlistDataScRsp,
};

use crate::sync::SyncType;

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
}

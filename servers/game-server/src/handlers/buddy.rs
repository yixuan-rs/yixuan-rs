use yixuan_codegen::{handlers, required_state};
use yixuan_proto::{GetBuddyDataCsReq, GetBuddyDataScRsp};

use crate::sync::SyncType;

use super::NetContext;

pub struct BuddyHandler;

#[handlers]
impl BuddyHandler {
    #[required_state(BasicDataSync)]
    pub fn on_get_buddy_data_cs_req(
        context: &mut NetContext<'_>,
        _request: GetBuddyDataCsReq,
    ) -> GetBuddyDataScRsp {
        context
            .player
            .sync_helper
            .remove_sync_response(SyncType::BasicData)
    }
}

use vivian_codegen::handlers;
use vivian_proto::{
    GetDisplayCaseDataCsReq, GetDisplayCaseDataScRsp, GetFriendListCsReq, GetFriendListScRsp,
};

use super::NetContext;

pub struct SocialHandler;

#[handlers]
impl SocialHandler {
    pub fn on_get_display_case_data_cs_req(
        _context: &mut NetContext<'_>,
        _request: GetDisplayCaseDataCsReq,
    ) -> GetDisplayCaseDataScRsp {
        GetDisplayCaseDataScRsp { retcode: 0 }
    }

    pub fn on_get_friend_list_cs_req(
        _context: &mut NetContext<'_>,
        _request: GetFriendListCsReq,
    ) -> GetFriendListScRsp {
        GetFriendListScRsp { retcode: 0 }
    }
}

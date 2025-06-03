use yixuan_codegen::handlers;
use yixuan_proto::{GetMonthDailyRewardListCsReq, GetMonthDailyRewardListScRsp};

use super::NetContext;

pub struct MonthRewardHandler;

#[handlers]
impl MonthRewardHandler {
    pub fn on_get_month_reward_list_cs_req(
        _context: &mut NetContext<'_>,
        _request: GetMonthDailyRewardListCsReq,
    ) -> GetMonthDailyRewardListScRsp {
        GetMonthDailyRewardListScRsp { retcode: 0 }
    }
}

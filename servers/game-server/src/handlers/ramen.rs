use yixuan_codegen::handlers;
use yixuan_proto::{GetRamenDataCsReq, GetRamenDataScRsp};

use super::NetContext;

pub struct RamenHandler;

#[handlers]
impl RamenHandler {
    pub fn on_get_ramen_data_cs_req(
        _context: &mut NetContext<'_>,
        _request: GetRamenDataCsReq,
    ) -> GetRamenDataScRsp {
        GetRamenDataScRsp {
            retcode: 0,
            ramen_data: Some(yixuan_proto::RamenData::default()),
        }
    }
}

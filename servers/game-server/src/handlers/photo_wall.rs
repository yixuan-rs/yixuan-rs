use yixuan_codegen::handlers;
use yixuan_proto::{GetPhotoWallDataCsReq, GetPhotoWallDataScRsp};

use super::NetContext;

pub struct PhotoWallHandler;

#[handlers]
impl PhotoWallHandler {
    pub fn on_get_photo_wall_data_cs_req(
        _context: &mut NetContext<'_>,
        _request: GetPhotoWallDataCsReq,
    ) -> GetPhotoWallDataScRsp {
        GetPhotoWallDataScRsp { retcode: 0 }
    }
}

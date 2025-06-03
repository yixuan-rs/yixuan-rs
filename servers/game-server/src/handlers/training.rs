use tracing::debug;
use yixuan_codegen::handlers;
use yixuan_proto::{
    PerformEndCsReq, PerformEndScRsp, PerformJumpCsReq, PerformJumpScRsp, PerformTriggerCsReq,
    PerformTriggerScRsp, StartTrainingCsReq, StartTrainingScRsp,
};

use super::NetContext;

pub struct TrainingHandler;

#[handlers]
impl TrainingHandler {
    // Apparently, Perform and Training are in the same cmdid group..
    // the similar thing about them is that they both 'allocate' an uid on server
    // and then sending results (perform skip/end), (training end if it was a special training)
    // bound to this UID

    pub fn on_perform_trigger_cs_req(
        _context: &mut NetContext<'_>,
        request: PerformTriggerCsReq,
    ) -> PerformTriggerScRsp {
        // TODO: actual incremental UID, same for training
        PerformTriggerScRsp {
            retcode: 0,
            perform_uid: ((request.perform_type as i64) << 32) | (request.perform_id as i64),
        }
    }

    pub fn on_perform_end_cs_req(
        _context: &mut NetContext<'_>,
        _request: PerformEndCsReq,
    ) -> PerformEndScRsp {
        PerformEndScRsp { retcode: 0 }
    }

    pub fn on_perform_jump_cs_req(
        _context: &mut NetContext<'_>,
        _request: PerformJumpCsReq,
    ) -> PerformJumpScRsp {
        PerformJumpScRsp { retcode: 0 }
    }

    pub fn on_start_training_cs_req(
        _context: &mut NetContext<'_>,
        request: StartTrainingCsReq,
    ) -> StartTrainingScRsp {
        debug!("{request:?}");

        // TODO: TrainingModel
        StartTrainingScRsp {
            retcode: 0,
            training_uid: 100,
        }
    }
}

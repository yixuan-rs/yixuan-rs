use std::{
    any::{Any, TypeId, type_name},
    collections::HashMap,
};

use tracing::{debug, error};
use vivian_proto::{Message, NetCmd, NetResponse};

mod login_sync;
mod player_sync;

pub use login_sync::LoginDataSyncComponent;
pub use player_sync::PlayerSyncComponent;

#[derive(Default)]
pub struct DataSyncHelper {
    sync_responses: HashMap<(SyncType, TypeId), Box<dyn Any>>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum SyncType {
    BasicData,
    ExtendData,
}

impl DataSyncHelper {
    pub fn add_response<Rsp>(&mut self, ty: SyncType, rsp: Rsp)
    where
        Rsp: NetCmd + NetResponse + Message + Default + 'static,
    {
        self.sync_responses
            .insert((ty, TypeId::of::<Rsp>()), Box::new(rsp));
    }

    pub fn remove_sync_response<Rsp>(&mut self, ty: SyncType) -> Rsp
    where
        Rsp: NetCmd + NetResponse + Message + Default + 'static,
    {
        if let Some(rsp) = self.sync_responses.remove(&(ty, TypeId::of::<Rsp>())) {
            debug!(
                "[{ty:?}Sync] successfully fetched response {}",
                type_name::<Rsp>()
            );

            *rsp.downcast().unwrap()
        } else {
            error!(
                "[{ty:?}Sync] tried to fetch {}, even response should have been sent already!",
                type_name::<Rsp>()
            );

            let mut rsp = Rsp::default();
            rsp.set_retcode(-1);

            rsp
        }
    }

    pub fn is_basic_data_synced(&self) -> bool {
        !self
            .sync_responses
            .keys()
            .any(|&(ty, _)| ty == SyncType::BasicData)
    }
}

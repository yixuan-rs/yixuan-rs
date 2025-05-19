use std::{
    any::{Any, TypeId, type_name},
    collections::HashMap,
};

use tracing::{debug, error};
use vivian_proto::{Message, NetCmd, NetResponse, PlayerSyncScNotify};

use crate::resources::NapResources;

pub trait PlayerSyncComponent {
    fn supports_player_sync(&self) -> bool;
    fn add_changes_to_player_sync_notify(&self, player_sync_sc_notify: &mut PlayerSyncScNotify);
}

pub trait LoginDataSyncComponent {
    fn prepare_responses(&self, sync_helper: &mut DataSyncHelper, res: &NapResources);
}

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

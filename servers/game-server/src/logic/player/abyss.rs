use vivian_proto::server_only::AbyssData;

use crate::{
    logic::sync::{LoginDataSyncComponent, PlayerSyncComponent, SyncType},
    resources::NapResources,
};

use super::{Model, Saveable};

pub struct AbyssModel {}

impl AbyssModel {
    pub fn load_from_pb(_pb: AbyssData) -> Self {
        Self {}
    }
}

impl Model for AbyssModel {
    fn is_any_field_changed(&self) -> bool {
        false
    }

    fn reset_changed_fields(&mut self) {}
}

impl Saveable for AbyssModel {
    fn save_to_pb(&self, root: &mut vivian_proto::server_only::PlayerData) {
        root.abyss = Some(AbyssData {});
    }
}

impl LoginDataSyncComponent for AbyssModel {
    fn prepare_responses(
        &self,
        sync_helper: &mut crate::logic::sync::DataSyncHelper,
        _res: &NapResources,
    ) {
        sync_helper.add_response(
            SyncType::BasicData,
            vivian_proto::AbyssGetDataScRsp {
                retcode: 0,
                abyss_data: Some(vivian_proto::AbyssData::default()),
                abyss_dungeon_list: Vec::new(),
                abyss_group_list: Vec::new(),
            },
        );

        sync_helper.add_response(
            SyncType::BasicData,
            vivian_proto::AbyssArpeggioGetDataScRsp { retcode: 0 },
        );
    }
}

impl PlayerSyncComponent for AbyssModel {
    fn supports_player_sync(&self) -> bool {
        false
    }

    fn add_changes_to_player_sync_notify(
        &self,
        _player_sync_sc_notify: &mut vivian_proto::PlayerSyncScNotify,
    ) {
    }
}

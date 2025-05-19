use vivian_codegen::Model;
use vivian_proto::server_only::{ArchiveData, VideotapeInfo};

use crate::{
    logic::{
        property::{Property, PropertyHashMap, PropertyHashSet},
        sync::{LoginDataSyncComponent, PlayerSyncComponent, SyncType},
    },
    resources::NapResources,
};

use super::{Model, Saveable};

#[derive(Model)]
pub struct ArchiveModel {
    pub unlocked_hollow_archive_ids: PropertyHashSet<u32>,
    pub archive_files: PropertyHashMap<u32, ArchiveFile>,
}

pub struct ArchiveFile {
    pub id: u32,
    pub finished: bool,
}

impl ArchiveModel {
    pub fn load_from_pb(pb: ArchiveData) -> Self {
        Self {
            unlocked_hollow_archive_ids: pb.hollow_archive_id_list.into_iter().collect(),
            archive_files: pb
                .videotape_list
                .into_iter()
                .map(|tape| {
                    (
                        tape.archive_file_id,
                        ArchiveFile {
                            id: tape.archive_file_id,
                            finished: tape.finished,
                        },
                    )
                })
                .collect(),
        }
    }
}

impl Saveable for ArchiveModel {
    fn save_to_pb(&self, root: &mut vivian_proto::server_only::PlayerData) {
        root.archive = Some(ArchiveData {
            hollow_archive_id_list: self.unlocked_hollow_archive_ids.iter().copied().collect(),
            videotape_list: self
                .archive_files
                .iter()
                .map(|(_, file)| VideotapeInfo {
                    archive_file_id: file.id,
                    finished: file.finished,
                })
                .collect(),
        });
    }
}

impl LoginDataSyncComponent for ArchiveModel {
    fn prepare_responses(
        &self,
        sync_helper: &mut crate::logic::sync::DataSyncHelper,
        _res: &NapResources,
    ) {
        sync_helper.add_response(
            SyncType::BasicData,
            vivian_proto::GetArchiveDataScRsp {
                retcode: 0,
                archive_data: Some(vivian_proto::ArchiveData {
                    hollow_archive_id_list: self
                        .unlocked_hollow_archive_ids
                        .iter()
                        .copied()
                        .collect(),
                    videotape_list: self
                        .archive_files
                        .iter()
                        .map(|(_, file)| vivian_proto::VideotapeInfo {
                            archive_file_id: file.id,
                            finished: file.finished,
                        })
                        .collect(),
                }),
            },
        );
    }
}

impl PlayerSyncComponent for ArchiveModel {
    fn supports_player_sync(&self) -> bool {
        false
    }

    fn add_changes_to_player_sync_notify(
        &self,
        _player_sync_sc_notify: &mut vivian_proto::PlayerSyncScNotify,
    ) {
    }
}

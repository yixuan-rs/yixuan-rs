use vivian_codegen::Model;
use vivian_proto::{GetSelfBasicInfoScRsp, PlayerSyncScNotify, server_only::BasicData};

use crate::{
    logic::{
        property::{PrimitiveProperty, Property},
        sync::{LoginDataSyncComponent, PlayerSyncComponent, SyncType},
    },
    resources::NapResources,
};

use super::{Model, Saveable};

#[derive(Model)]
pub struct PlayerBasicModel {
    pub nick_name: PrimitiveProperty<String>,
    pub create_time: PrimitiveProperty<i64>,
    pub name_change_times: PrimitiveProperty<u32>,
    pub level: PrimitiveProperty<u32>,
    pub exp: PrimitiveProperty<u32>,
    pub avatar_id: PrimitiveProperty<u32>,
    pub control_avatar_id: PrimitiveProperty<u32>,
    pub control_guise_avatar_id: PrimitiveProperty<u32>,
    pub portrait_id: PrimitiveProperty<u32>,
}

impl PlayerBasicModel {
    const MAX_YOROZUYA_LEVEL: u32 = 60;

    pub fn add_experience(&mut self, amount: u32, res: &NapResources) {
        if self.level.get() == Self::MAX_YOROZUYA_LEVEL {
            return;
        }

        self.exp.set(self.exp.get() + amount);

        while let Some(exp_cost) = self.should_increase_level(res) {
            self.exp.set(self.exp.get() - exp_cost);
            self.level.set(self.level.get() + 1);

            if self.level.get() == Self::MAX_YOROZUYA_LEVEL {
                self.exp.set(0);
            }
        }
    }

    fn should_increase_level(&self, res: &NapResources) -> Option<u32> {
        if self.level.get() < Self::MAX_YOROZUYA_LEVEL {
            let yorozuya_level_template = res
                .templates
                .yorozuya_level_template_tb()
                .find(|tmpl| tmpl.level() == self.level.get())
                .unwrap();

            (yorozuya_level_template.experience() <= self.exp.get())
                .then_some(yorozuya_level_template.experience())
        } else {
            None
        }
    }

    pub fn load_from_pb(pb: BasicData) -> Self {
        Self {
            nick_name: pb.nick_name.into(),
            create_time: pb.create_time.into(),
            name_change_times: pb.name_change_times.into(),
            level: pb.level.into(),
            exp: pb.exp.into(),
            avatar_id: pb.avatar_id.into(),
            control_avatar_id: pb.control_avatar_id.into(),
            control_guise_avatar_id: pb.control_guise_avatar_id.into(),
            portrait_id: pb.portrait_id.into(),
        }
    }

    fn build_self_basic_info(&self) -> vivian_proto::SelfBasicInfo {
        vivian_proto::SelfBasicInfo {
            nick_name: self.nick_name.get().to_string(),
            role_create_time: self.create_time.get(),
            name_change_times: self.name_change_times.get(),
            level: self.level.get(),
            exp: self.exp.get(),
            avatar_id: self.avatar_id.get(),
            player_avatar_id: self.control_avatar_id.get(),
            control_guise_avatar_id: self.control_guise_avatar_id.get(),
            portrait_id: self.portrait_id.get(),
        }
    }
}

impl Saveable for PlayerBasicModel {
    fn save_to_pb(&self, root: &mut vivian_proto::server_only::PlayerData) {
        root.basic = Some(BasicData {
            nick_name: self.nick_name.get().to_string(),
            create_time: self.create_time.get(),
            name_change_times: self.name_change_times.get(),
            level: self.level.get(),
            exp: self.exp.get(),
            avatar_id: self.avatar_id.get(),
            control_avatar_id: self.control_avatar_id.get(),
            control_guise_avatar_id: self.control_guise_avatar_id.get(),
            portrait_id: self.portrait_id.get(),
        });
    }
}

impl LoginDataSyncComponent for PlayerBasicModel {
    fn prepare_responses(
        &self,
        sync_helper: &mut crate::logic::sync::DataSyncHelper,
        _res: &NapResources,
    ) {
        sync_helper.add_response(
            SyncType::BasicData,
            GetSelfBasicInfoScRsp {
                retcode: 0,
                self_basic_info: Some(self.build_self_basic_info()),
            },
        );
    }
}

impl PlayerSyncComponent for PlayerBasicModel {
    fn add_changes_to_player_sync_notify(&self, player_sync_sc_notify: &mut PlayerSyncScNotify) {
        player_sync_sc_notify.self_basic_info = Some(self.build_self_basic_info());
    }

    fn supports_player_sync(&self) -> bool {
        true
    }
}

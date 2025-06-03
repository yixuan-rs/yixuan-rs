use super::*;
use property::{PrimitiveProperty, Property};

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

    pub fn build_self_basic_info(&self) -> yixuan_proto::SelfBasicInfo {
        yixuan_proto::SelfBasicInfo {
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
    fn save_to_pb(&self, root: &mut yixuan_proto::server_only::PlayerData) {
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

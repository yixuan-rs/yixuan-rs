use yixuan_codegen::{Model, Property};
use yixuan_logic::dungeon::AvatarUnit;

use crate::property::{PrimitiveProperty, Property, PropertyHashMap};

use super::*;

#[derive(Model)]
pub struct BigSceneModel {
    pub team: PropertyBigSceneTeam,
}

#[derive(Property, Default)]
pub struct PropertyBigSceneTeam {
    pub avatars: PropertyHashMap<u32, BigSceneAvatar>,
    pub cur_avatars: PropertyHashMap<u32, BigSceneAvatar>,
    pub cur_avatar_id: PrimitiveProperty<u32>,
}

#[derive(Debug, Default)]
pub struct BigSceneAvatar {
    pub avatar_id: u32,
    pub team_slot_index: u32,
    pub cur_hp: u32,
    pub avatar_unit: Option<AvatarUnit>,
}

impl BigSceneModel {
    pub fn load_from_pb(pb: BigSceneData) -> Self {
        Self {
            team: pb
                .scene_team_data
                .map(|data| PropertyBigSceneTeam {
                    avatars: data
                        .scene_avatar_list
                        .into_iter()
                        .map(|avatar| {
                            (
                                avatar.avatar_id,
                                BigSceneAvatar {
                                    avatar_id: avatar.avatar_id,
                                    cur_hp: avatar.cur_hp,
                                    avatar_unit: avatar.avatar_unit.map(AvatarUnit::from),
                                    team_slot_index: avatar.team_slot_index,
                                },
                            )
                        })
                        .collect(),
                    cur_avatars: data
                        .cur_scene_avatar_list
                        .into_iter()
                        .map(|avatar| {
                            (
                                avatar.avatar_id,
                                BigSceneAvatar {
                                    avatar_id: avatar.avatar_id,
                                    cur_hp: avatar.cur_hp,
                                    avatar_unit: avatar.avatar_unit.map(AvatarUnit::from),
                                    team_slot_index: avatar.team_slot_index,
                                },
                            )
                        })
                        .collect(),
                    cur_avatar_id: data.cur_avatar_id.into(),
                })
                .unwrap_or_default(),
        }
    }
}

impl Saveable for BigSceneModel {
    fn save_to_pb(&self, root: &mut PlayerData) {
        root.big_scene = Some(BigSceneData {
            scene_team_data: Some(BigSceneTeamData {
                scene_avatar_list: self
                    .team
                    .avatars
                    .iter()
                    .map(|(_, avatar)| BigSceneAvatarData {
                        avatar_id: avatar.avatar_id,
                        cur_hp: avatar.cur_hp,
                        avatar_unit: avatar.avatar_unit.as_ref().map(AvatarUnit::as_proto),
                        team_slot_index: avatar.team_slot_index,
                    })
                    .collect(),
                cur_scene_avatar_list: self
                    .team
                    .cur_avatars
                    .iter()
                    .map(|(_, avatar)| BigSceneAvatarData {
                        avatar_id: avatar.avatar_id,
                        cur_hp: avatar.cur_hp,
                        avatar_unit: avatar.avatar_unit.as_ref().map(AvatarUnit::as_proto),
                        team_slot_index: avatar.team_slot_index,
                    })
                    .collect(),
                cur_avatar_id: self.team.cur_avatar_id.get(),
            }),
        });
    }
}

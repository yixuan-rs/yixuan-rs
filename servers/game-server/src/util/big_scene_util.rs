use itertools::Itertools;
use yixuan_logic::{
    GameState,
    big_scene::{GameBigSceneState, TeamInitData, TeamMemberInitData},
    dungeon::AvatarUnit,
};
use yixuan_models::BigSceneAvatar;
use yixuan_proto::{FloorPositionInfo, common::RollbackPointInfo};

use crate::player::{EquipmentDataProxy, Player};

pub fn add_avatar_to_big_scene_team_list(player: &mut Player, id: u32) {
    if !player.big_scene_model.team.avatars.contains_key(&id) {
        player.big_scene_model.team.avatars.insert(
            id,
            BigSceneAvatar {
                avatar_id: id,
                cur_hp: 100,
                avatar_unit: None,
                team_slot_index: 0,
            },
        )
    }
}

pub fn replace_team(player: &mut Player, ids: &[u32]) {
    let big_scene_model = &mut player.big_scene_model;
    big_scene_model.team.cur_avatars.clear();

    let equipment_data_proxy = EquipmentDataProxy {
        avatar_model: &player.avatar_model,
        item_model: &player.item_model,
    };

    if let Some(&avatar_id) = ids.first() {
        big_scene_model.team.cur_avatar_id.set(avatar_id);
    }

    ids.iter().enumerate().for_each(|(index, &id)| {
        if let Some(cur_hp) = big_scene_model
            .team
            .avatars
            .get_mut(&id) // get_mut for it to be in PlayerSync
            .map(|avatar| avatar.cur_hp)
        {
            let unit = AvatarUnit::new(id, &player.resources.templates, &equipment_data_proxy);

            // TODO: scale HP to the cur_hp percentage!

            big_scene_model.team.cur_avatars.insert(
                id,
                BigSceneAvatar {
                    avatar_id: id,
                    cur_hp,
                    avatar_unit: Some(unit),
                    team_slot_index: index as u32,
                },
            );
        }
    });
}

pub fn load_big_scene_state(
    player: &mut Player,
    floor_id: u32,
    rbp: RollbackPointInfo,
    pos: FloorPositionInfo,
) -> GameState {
    let day_of_week = player.main_city_model.day_of_week.get();

    let team = &player.big_scene_model.team;
    let team = TeamInitData {
        members: team
            .cur_avatars
            .iter()
            .sorted_by_key(|(_, avatar)| avatar.team_slot_index)
            .map(|(_, avatar)| TeamMemberInitData {
                avatar_id: avatar.avatar_id,
            })
            .collect(),
        cur_member_index: team
            .cur_avatars
            .iter()
            .find(|(avatar_id, _)| **avatar_id == team.cur_avatar_id.get())
            .map(|(_, avatar)| avatar.team_slot_index as usize)
            .unwrap_or_default(),
    };

    GameState::BigScene(Box::new(GameBigSceneState::new(
        floor_id,
        rbp,
        pos,
        team,
        day_of_week,
        player.resources.logic_resources(),
    )))
}

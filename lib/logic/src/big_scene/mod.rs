use event::{AvatarChangeEvt, EntityMoveEvt};
use view_object::{ViewObjectHandle, ViewObjectManager};
use yixuan_proto::{FloorPositionInfo, common::RollbackPointInfo};

use crate::{
    LogicResources,
    listener::{NotifyListener, NotifyListenerExt},
    math::Vector3i,
};

mod component;
mod event;
mod view_object;

pub struct GameBigSceneState {
    pub scene_id: u32,
    pub floor_id: u32,
    pub cur_player_pos: Vector3i,
    pub day_of_week: u32,
    rollback_point: RollbackPointInfo,
    enter_pos: FloorPositionInfo,
    view_object_manager: ViewObjectManager,
    #[expect(dead_code)]
    resources: LogicResources,
    has_sent_initial_scene_notify: bool,
}

pub struct TeamInitData {
    pub members: Vec<TeamMemberInitData>,
    pub cur_member_index: usize,
}

pub struct TeamMemberInitData {
    pub avatar_id: u32,
}

impl GameBigSceneState {
    pub fn new(
        floor_id: u32,
        rbp: RollbackPointInfo,
        enter_pos: FloorPositionInfo,
        team: TeamInitData,
        day_of_week: u32,
        res: LogicResources,
    ) -> Self {
        let floor_config = res.level_world.floors.get(&floor_id).unwrap();

        let cur_player_pos = Vector3i::from_proto(enter_pos.player_pos.as_ref().unwrap());
        let cur_avatar_id = team.members.get(team.cur_member_index).unwrap().avatar_id;

        let mut view_object_manager = ViewObjectManager::new(floor_id, res);

        view_object_manager.team_avatar_handle = Some(view_object_manager.create_team_avatar(
            cur_avatar_id,
            cur_player_pos.clone(),
            Vector3i::default(),
        ));

        Self {
            floor_id,
            scene_id: floor_config.scene_config_id,
            rollback_point: rbp,
            cur_player_pos,
            enter_pos,
            view_object_manager,
            day_of_week,
            resources: res,
            has_sent_initial_scene_notify: false,
        }
    }

    pub fn on_enter_done(&mut self) {
        self.view_object_manager.load_all_groups();
    }

    pub fn process_avatar_change(&mut self, avatar_id: u32) {
        if let Some(team_avatar_handle) = self.view_object_manager.team_avatar_handle {
            self.view_object_manager.send_event(
                team_avatar_handle,
                AvatarChangeEvt {
                    current_avatar_id: avatar_id,
                },
            );
        }
    }

    pub fn process_sync_entity_position(
        &mut self,
        net_id: u32,
        position: Vector3i,
        rotation: Vector3i,
    ) {
        if self.view_object_manager.team_avatar_handle == Some(ViewObjectHandle(net_id)) {
            self.enter_pos.player_pos = Some(position.to_proto());
            self.enter_pos.player_rot = Some(position.to_proto());
        }

        self.view_object_manager.send_event(
            ViewObjectHandle(net_id),
            EntityMoveEvt { position, rotation },
        );
    }

    pub fn active_rollback_point(&mut self, _group_id: u32, point: RollbackPointInfo) {
        self.rollback_point = point;
    }

    pub fn cur_rollback_point(&self) -> RollbackPointInfo {
        self.rollback_point.clone()
    }

    pub fn cur_floor_position(&self) -> FloorPositionInfo {
        self.enter_pos.clone()
    }

    pub fn flush_notifies(&mut self, listener: &mut dyn NotifyListener) {
        if !self.has_sent_initial_scene_notify {
            self.has_sent_initial_scene_notify = true;

            listener.add(yixuan_proto::EnterBigSceneScNotify {
                scene_id: self.scene_id,
                floor_id: self.floor_id,
                floor_position: Some(self.enter_pos.clone()),
                rollback_point: Some(self.rollback_point.clone()),
                pos: self
                    .view_object_manager
                    .get_team_avatar_position()
                    .map(|pos| pos.to_proto()),
                cur_entity_info: self
                    .view_object_manager
                    .serialize_object(self.view_object_manager.team_avatar_handle.unwrap()),
                day_of_week: self.day_of_week,
            });
        }

        self.view_object_manager.notify_group_orders(listener);

        if let Some(scene_entity_appear) = self.view_object_manager.flush_scene_entity_appear() {
            listener.add(scene_entity_appear);
        }
    }
}

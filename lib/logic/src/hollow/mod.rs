use chessboard::Chessboard;
use common::time_util;
use config::HollowChessboardConfig;
use yixuan_proto::{
    EnterSceneScNotify, EventGraphOwnerType, Message, action::MakeChoiceOfEvent, common::Vector2Int,
};

mod action_executor;
mod chessboard;
mod component;
mod entity;
mod entity_util;
mod expression_util;
mod net_helper;
mod predicate_executor;

use crate::{
    LogicResources,
    dungeon::Dungeon,
    listener::{LogicEventListener, NotifyListener},
    scene::SceneType,
};

pub struct GameHollowState {
    pub dungeon: Dungeon,
    pub scene_id: u32,
    pub chessboard: Chessboard,
    #[expect(dead_code)]
    resources: LogicResources,
    chessboard_finish_acknowledged: bool,
}

impl GameHollowState {
    pub fn new(
        chessboard: &'static HollowChessboardConfig,
        resources: LogicResources,
        scene_id: u32,
        dungeon: Dungeon,
        player_avatar_id: u32,
    ) -> Self {
        Self {
            chessboard: Chessboard::new(dungeon.quest_id, player_avatar_id, chessboard, resources),
            dungeon,
            scene_id,
            resources,
            chessboard_finish_acknowledged: false,
        }
    }

    pub fn on_enter(&mut self, listener: &mut dyn LogicEventListener) {
        self.dungeon.begin_time = time_util::unix_timestamp_seconds();
        self.chessboard.execute_first_turn(listener);

        self.chessboard
            .global_event_helper
            .set_enter_notify(EnterSceneScNotify {
                scene: Some(self.client_scene_data_proto()),
                dungeon: Some(self.dungeon.as_client_proto()),
            });
    }

    pub fn run_event_graph(
        &mut self,
        owner_type: EventGraphOwnerType,
        event_id: u32,
        listener: &mut dyn LogicEventListener,
        client_input: &[u8],
    ) -> bool {
        if !client_input.is_empty() {
            if let Ok(choice) = MakeChoiceOfEvent::decode(client_input) {
                self.chessboard.event_choice_uid = choice.uid;
            }
        }

        let resume_result = self
            .chessboard
            .resume_event_logic(owner_type, event_id, listener);

        self.update_hollow_quest_state(listener);
        resume_result
    }

    pub fn handle_player_move(
        &mut self,
        move_path: Vec<Vector2Int>,
        logic_listener: &mut dyn LogicEventListener,
    ) -> (Vector2Int, u32) {
        for point in move_path {
            self.chessboard.player_move(point, logic_listener, false);
        }

        self.update_hollow_quest_state(logic_listener);

        (
            self.chessboard.cur_player_pos(),
            self.chessboard.cur_section_id(),
        )
    }

    pub fn update_hollow_quest_state(&mut self, listener: &mut dyn LogicEventListener) {
        if self.chessboard.finished && !self.chessboard_finish_acknowledged {
            self.chessboard_finish_acknowledged = true;
            self.dungeon.finish_main_dungeon_quest(1);
            listener.hollow_quest_finished(self.dungeon.quest_id);
        }
    }

    // Common

    pub fn scene_type(&self) -> SceneType {
        SceneType::Hollow
    }

    pub fn flush_notifies(&mut self, listener: &mut dyn NotifyListener) {
        self.chessboard.global_event_helper.flush(listener);
        self.dungeon.flush_dungeon_quest_notifies(listener);
    }

    pub fn client_scene_data_proto(&self) -> yixuan_proto::SceneData {
        yixuan_proto::SceneData {
            scene_id: self.scene_id,
            play_type: 0,
            scene_type: self.scene_type().into(),
            hollow_scene_data: Some(yixuan_proto::HollowSceneData {
                hollow_scene: Some(self.chessboard.as_client_proto()),
            }),
            ..Default::default()
        }
    }
}

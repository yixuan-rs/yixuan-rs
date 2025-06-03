use std::fmt;

use yixuan_proto::common::{HollowGridFlag, HollowGridState, NodeState, NodeSubState, NodeVisible};

use super::*;

#[derive(PartialEq, Eq, Clone)]
pub struct GridStateComponent {
    pub node_state: NodeState,
    pub node_visible: NodeVisible,
    pub node_sub_state: NodeSubState,
    pub flag: i32,
}

impl GridStateComponent {
    pub fn new(state: NodeState, visible: NodeVisible) -> Self {
        Self {
            node_state: state,
            node_visible: visible,
            node_sub_state: NodeSubState::EmptySub,
            flag: 0,
        }
    }

    pub fn enable(self, flag: HollowGridFlag) -> Self {
        Self {
            flag: self.flag | i32::from(flag),
            ..self
        }
    }

    pub fn disable(self, flag: HollowGridFlag) -> Self {
        Self {
            flag: self.flag & !i32::from(flag),
            ..self
        }
    }

    pub fn is_enabled(&self, flag: HollowGridFlag) -> bool {
        (self.flag & i32::from(flag)) != 0
    }
}

impl SerializableComponent<yixuan_proto::GridStateComponent> for GridStateComponent {
    fn component_type(&self) -> HollowComponentType {
        HollowComponentType::GridStateComponent
    }

    fn component_info(&self) -> yixuan_proto::GridStateComponent {
        yixuan_proto::GridStateComponent {
            cur_grid_state: Some(HollowGridState {
                node_state: self.node_state.into(),
                node_visible: self.node_visible.into(),
                sub_state: self.node_sub_state.into(),
                flag: self.flag,
            }),
            prev_grid_state: None,
            grid_state_component_unknown: 3, // ?
        }
    }
}

impl fmt::Debug for GridStateComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const GRID_FLAGS: [HollowGridFlag; 26] = [
            HollowGridFlag::Core,
            HollowGridFlag::CanMove,
            HollowGridFlag::Travelled,
            HollowGridFlag::ShowEventType,
            HollowGridFlag::ShowEventId,
            HollowGridFlag::CanTriggerEvent,
            HollowGridFlag::Visible,
            HollowGridFlag::VisibleAtGridAround,
            HollowGridFlag::VisibleByTriggerEvent,
            HollowGridFlag::SyncToClient,
            HollowGridFlag::Door,
            HollowGridFlag::CanTriggerMultiTimes,
            HollowGridFlag::TemporaryVisibleAtAround,
            HollowGridFlag::Unlocked,
            HollowGridFlag::Brighten,
            HollowGridFlag::Guide,
            HollowGridFlag::Target,
            HollowGridFlag::BrightenOnlyVisible,
            HollowGridFlag::Unstable,
            HollowGridFlag::Empty,
            HollowGridFlag::Blocked,
            HollowGridFlag::Gdhpcijjoah,
            HollowGridFlag::Blblfbdlbbo,
            HollowGridFlag::Nihgbijfiae,
            HollowGridFlag::Ebjcidkjnki,
            HollowGridFlag::Jgjdbhllmai,
        ];

        f.debug_struct("GridStateComponent")
            .field("node_state", &self.node_state)
            .field("node_visible", &self.node_visible)
            .field("node_sub_state", &self.node_sub_state)
            .field(
                "flag",
                &GRID_FLAGS.iter().filter(|&&flag| self.is_enabled(flag)),
            )
            .finish()
    }
}

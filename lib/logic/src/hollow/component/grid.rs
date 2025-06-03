use std::fmt;

use yixuan_proto::{GridLink, GridType, GridUnkEnum};

use super::*;

#[derive(PartialEq, Eq, Clone)]
pub struct HollowGridComponent {
    pub grid_type: GridType,
    grid_link: i32,
}

impl HollowGridComponent {
    pub fn new(grid_type: GridType) -> Self {
        Self {
            grid_type,
            grid_link: GridLink::None.into(),
        }
    }

    pub fn link(self, against: GridLink) -> Self {
        Self {
            grid_link: self.grid_link | i32::from(against),
            ..self
        }
    }

    pub fn is_linked(&self, against: GridLink) -> bool {
        (self.grid_link & i32::from(against)) != 0
    }
}

impl SerializableComponent<yixuan_proto::HollowGridComponent> for HollowGridComponent {
    fn component_type(&self) -> HollowComponentType {
        HollowComponentType::HollowGridComponent
    }

    fn component_info(&self) -> yixuan_proto::HollowGridComponent {
        yixuan_proto::HollowGridComponent {
            grid_type: self.grid_type.into(),
            grid_link: self.grid_link,
            grid_unk_enum: GridUnkEnum::Unk1.into(),

            // these fields are abandoned and moved to GridState
            cur_grid_state: None,
            prev_grid_state: None,
        }
    }
}

impl fmt::Debug for HollowGridComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const GRID_LINKS: [GridLink; 4] = [
            GridLink::Up,
            GridLink::Down,
            GridLink::Right,
            GridLink::Left,
        ];

        f.debug_struct("HollowGridComponent")
            .field("grid_type", &self.grid_type)
            .field(
                "grid_link",
                &GRID_LINKS.iter().filter(|&&link| self.is_linked(link)),
            )
            .finish()
    }
}

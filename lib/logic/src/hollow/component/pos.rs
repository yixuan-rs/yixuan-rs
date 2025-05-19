use std::fmt;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PosComponent {
    pub pos: (i32, i32),
    pub section: HollowEntity,
}

impl SerializableComponent<HollowPosComponent> for PosComponent {
    fn component_type(&self) -> HollowComponentType {
        HollowComponentType::PosComponent
    }

    fn component_info(&self) -> HollowPosComponent {
        HollowPosComponent {
            pos: Some(Vector2Int {
                x: self.pos.0,
                y: self.pos.1,
            }),
            section_id: self.section.as_raw_u32(),
        }
    }
}

impl fmt::Display for PosComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[x:{}, y:{}, section:{}]",
            self.pos.0,
            self.pos.1,
            self.section.as_raw_u32()
        )
    }
}

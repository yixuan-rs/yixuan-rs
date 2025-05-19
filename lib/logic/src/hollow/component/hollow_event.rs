use super::SerializableComponent;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HollowEventComponent {
    pub icon_texture_sheet_id: i32,
    pub interact_texture_sheet_id: i32,
}

impl SerializableComponent<vivian_proto::HollowEventComponent> for HollowEventComponent {
    fn component_type(&self) -> vivian_proto::HollowComponentType {
        vivian_proto::HollowComponentType::HollowEventComponent
    }

    fn component_info(&self) -> vivian_proto::HollowEventComponent {
        vivian_proto::HollowEventComponent {
            icon_texture_sheet_id: self.icon_texture_sheet_id,
            interact_texture_sheet_id: self.interact_texture_sheet_id,
        }
    }
}

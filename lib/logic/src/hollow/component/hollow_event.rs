use super::SerializableComponent;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HollowEventComponent {
    pub icon_texture_sheet_id: i32,
    pub interact_texture_sheet_id: i32,
}

impl SerializableComponent<yixuan_proto::HollowEventComponent> for HollowEventComponent {
    fn component_type(&self) -> yixuan_proto::HollowComponentType {
        yixuan_proto::HollowComponentType::HollowEventComponent
    }

    fn component_info(&self) -> yixuan_proto::HollowEventComponent {
        yixuan_proto::HollowEventComponent {
            icon_texture_sheet_id: self.icon_texture_sheet_id,
            interact_texture_sheet_id: self.interact_texture_sheet_id,
        }
    }
}

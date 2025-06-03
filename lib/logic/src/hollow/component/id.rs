use super::SerializableComponent;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IdComponent(pub u32);

impl SerializableComponent<yixuan_proto::IdComponent> for IdComponent {
    fn component_type(&self) -> yixuan_proto::HollowComponentType {
        yixuan_proto::HollowComponentType::IdComponent
    }

    fn component_info(&self) -> yixuan_proto::IdComponent {
        yixuan_proto::IdComponent { id: self.0 }
    }
}

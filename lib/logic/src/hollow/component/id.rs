use super::SerializableComponent;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IdComponent(pub u32);

impl SerializableComponent<vivian_proto::IdComponent> for IdComponent {
    fn component_type(&self) -> vivian_proto::HollowComponentType {
        vivian_proto::HollowComponentType::IdComponent
    }

    fn component_info(&self) -> vivian_proto::IdComponent {
        vivian_proto::IdComponent { id: self.0 }
    }
}

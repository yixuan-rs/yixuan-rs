use crate::hollow::entity::HollowEntity;

use super::SerializableComponent;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OwnerComponent(pub HollowEntity);

impl SerializableComponent<vivian_proto::OwnerComponent> for OwnerComponent {
    fn component_type(&self) -> vivian_proto::HollowComponentType {
        vivian_proto::HollowComponentType::OwnerComponent
    }

    fn component_info(&self) -> vivian_proto::OwnerComponent {
        vivian_proto::OwnerComponent {
            owner_entity_uid: self.0.as_raw_u32(),
        }
    }
}

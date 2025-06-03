use crate::hollow::entity::HollowEntity;

use super::SerializableComponent;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OwnerComponent(pub HollowEntity);

impl SerializableComponent<yixuan_proto::OwnerComponent> for OwnerComponent {
    fn component_type(&self) -> yixuan_proto::HollowComponentType {
        yixuan_proto::HollowComponentType::OwnerComponent
    }

    fn component_info(&self) -> yixuan_proto::OwnerComponent {
        yixuan_proto::OwnerComponent {
            owner_entity_uid: self.0.as_raw_u32(),
        }
    }
}

use super::SerializableComponent;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntityPriorityComponent {
    pub priority: i32,
    pub secondary_priority: i32,
}

impl SerializableComponent<yixuan_proto::EntityPriorityComponent> for EntityPriorityComponent {
    fn component_type(&self) -> yixuan_proto::HollowComponentType {
        yixuan_proto::HollowComponentType::EntityPriorityComponent
    }

    fn component_info(&self) -> yixuan_proto::EntityPriorityComponent {
        yixuan_proto::EntityPriorityComponent {
            priority: self.priority,
            secondary_priority: self.secondary_priority,
        }
    }
}

use super::SerializableComponent;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntityPriorityComponent {
    pub priority: i32,
    pub secondary_priority: i32,
}

impl SerializableComponent<vivian_proto::EntityPriorityComponent> for EntityPriorityComponent {
    fn component_type(&self) -> vivian_proto::HollowComponentType {
        vivian_proto::HollowComponentType::EntityPriorityComponent
    }

    fn component_info(&self) -> vivian_proto::EntityPriorityComponent {
        vivian_proto::EntityPriorityComponent {
            priority: self.priority,
            secondary_priority: self.secondary_priority,
        }
    }
}

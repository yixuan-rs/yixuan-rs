use std::collections::HashMap;

use super::SerializableComponent;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct BehaviorComponent {
    pub int_specials: HashMap<String, i32>,
    pub str_specials: HashMap<String, String>,
}

impl SerializableComponent<vivian_proto::BehaviorComponent> for BehaviorComponent {
    fn component_type(&self) -> vivian_proto::HollowComponentType {
        vivian_proto::HollowComponentType::BehaviorComponent
    }

    fn component_info(&self) -> vivian_proto::BehaviorComponent {
        vivian_proto::BehaviorComponent {
            unk_behavior_state: false,
            int_specials: self.int_specials.clone(),
            str_specials: self.str_specials.clone(),
        }
    }
}

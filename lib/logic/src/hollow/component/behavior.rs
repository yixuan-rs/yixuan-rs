use std::collections::HashMap;

use super::SerializableComponent;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct BehaviorComponent {
    pub int_specials: HashMap<String, i32>,
    pub str_specials: HashMap<String, String>,
}

impl SerializableComponent<yixuan_proto::BehaviorComponent> for BehaviorComponent {
    fn component_type(&self) -> yixuan_proto::HollowComponentType {
        yixuan_proto::HollowComponentType::BehaviorComponent
    }

    fn component_info(&self) -> yixuan_proto::BehaviorComponent {
        yixuan_proto::BehaviorComponent {
            unk_behavior_state: false,
            int_specials: self.int_specials.clone(),
            str_specials: self.str_specials.clone(),
        }
    }
}

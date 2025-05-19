use std::collections::HashSet;

use super::SerializableComponent;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct CategoryComponent {
    pub category_list: HashSet<String>,
}

impl SerializableComponent<vivian_proto::CategoryComponent> for CategoryComponent {
    fn component_type(&self) -> vivian_proto::HollowComponentType {
        vivian_proto::HollowComponentType::CategoryComponent
    }

    fn component_info(&self) -> vivian_proto::CategoryComponent {
        vivian_proto::CategoryComponent {
            category_list: self.category_list.iter().cloned().collect(),
        }
    }
}

use std::collections::HashSet;

use super::SerializableComponent;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct CategoryComponent {
    pub category_list: HashSet<String>,
}

impl SerializableComponent<yixuan_proto::CategoryComponent> for CategoryComponent {
    fn component_type(&self) -> yixuan_proto::HollowComponentType {
        yixuan_proto::HollowComponentType::CategoryComponent
    }

    fn component_info(&self) -> yixuan_proto::CategoryComponent {
        yixuan_proto::CategoryComponent {
            category_list: self.category_list.iter().cloned().collect(),
        }
    }
}

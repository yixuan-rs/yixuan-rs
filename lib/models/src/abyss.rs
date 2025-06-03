use super::*;

pub struct AbyssModel {}

impl AbyssModel {
    pub fn load_from_pb(_pb: AbyssData) -> Self {
        Self {}
    }
}

impl Model for AbyssModel {
    fn is_any_field_changed(&self) -> bool {
        false
    }

    fn reset_changed_fields(&mut self) {}
}

impl Saveable for AbyssModel {
    fn save_to_pb(&self, root: &mut PlayerData) {
        root.abyss = Some(AbyssData {});
    }
}

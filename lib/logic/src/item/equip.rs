use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EquipItem {
    pub id: u32,
    pub level: u32,
    pub exp: u32,
    pub star: u32,
    pub lock: bool,
    pub properties: HashMap<u32, (u32, u32)>,
    pub sub_properties: HashMap<u32, (u32, u32)>,
}

impl EquipItem {
    pub fn as_client_proto(&self, uid: u32) -> yixuan_proto::EquipInfo {
        yixuan_proto::EquipInfo {
            uid,
            id: self.id,
            level: self.level,
            exp: self.exp,
            star: self.star,
            lock: self.lock,
            propertys: self
                .properties
                .iter()
                .map(
                    |(&key, &(base_value, add_value))| yixuan_proto::EquipProperty {
                        key,
                        base_value,
                        add_value,
                    },
                )
                .collect(),
            sub_propertys: self
                .sub_properties
                .iter()
                .map(
                    |(&key, &(base_value, add_value))| yixuan_proto::EquipProperty {
                        key,
                        base_value,
                        add_value,
                    },
                )
                .collect(),
        }
    }
}

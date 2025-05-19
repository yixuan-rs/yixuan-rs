#[derive(Debug, Clone)]
pub struct WeaponItem {
    pub id: u32,
    pub level: u32,
    pub exp: u32,
    pub star: u32,
    pub refine_level: u32,
    pub lock: bool,
}

impl WeaponItem {
    pub fn as_client_proto(&self, uid: u32) -> vivian_proto::WeaponInfo {
        vivian_proto::WeaponInfo {
            uid,
            id: self.id,
            level: self.level,
            exp: self.exp,
            star: self.star,
            refine_level: self.refine_level,
            lock: self.lock,
        }
    }
}

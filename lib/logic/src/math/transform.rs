use config::world::ConfigVector3;

#[derive(Debug, Default, Clone)]
pub struct Transform {
    pub position: Vector3,
    pub rotation: Vector3,
}

#[derive(Debug, Default, Clone)]
pub struct Vector3(pub f64, pub f64, pub f64);

#[derive(Debug, Default, Clone)]
pub struct Vector3i(pub i32, pub i32, pub i32);

impl Vector3i {
    pub fn from_proto(pb: &yixuan_proto::common::Vector3) -> Self {
        Self(pb.x, pb.y, pb.z)
    }

    pub fn to_proto(&self) -> yixuan_proto::common::Vector3 {
        yixuan_proto::common::Vector3 {
            x: self.0,
            y: self.1,
            z: self.2,
        }
    }

    pub fn from_config(config: &ConfigVector3) -> Self {
        Self(
            (config.x * 100.0) as i32,
            (config.y * 100.0) as i32,
            (config.z * 100.0) as i32,
        )
    }
}

impl Transform {
    pub fn from_proto(pb: &yixuan_proto::common::Transform) -> Self {
        #[allow(clippy::get_first)]
        Self {
            position: Vector3(
                pb.position.get(0).copied().unwrap_or_default(),
                pb.position.get(1).copied().unwrap_or_default(),
                pb.position.get(2).copied().unwrap_or_default(),
            ),
            rotation: Vector3(
                pb.rotation.get(0).copied().unwrap_or_default(),
                pb.rotation.get(1).copied().unwrap_or_default(),
                pb.rotation.get(2).copied().unwrap_or_default(),
            ),
        }
    }

    pub fn to_proto(&self) -> yixuan_proto::common::Transform {
        yixuan_proto::common::Transform {
            position: vec![self.position.0, self.position.1, self.position.2],
            rotation: vec![self.rotation.0, self.rotation.1, self.rotation.2],
        }
    }
}

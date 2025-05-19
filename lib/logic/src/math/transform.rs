#[derive(Debug, Default, Clone)]
pub struct Transform {
    pub position: Vector3,
    pub rotation: Vector3,
}

#[derive(Debug, Default, Clone)]
pub struct Vector3(pub f64, pub f64, pub f64);

impl Transform {
    pub fn from_proto(pb: &vivian_proto::common::Transform) -> Self {
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

    pub fn to_proto(&self) -> vivian_proto::common::Transform {
        vivian_proto::common::Transform {
            position: vec![self.position.0, self.position.1, self.position.2],
            rotation: vec![self.rotation.0, self.rotation.1, self.rotation.2],
        }
    }
}

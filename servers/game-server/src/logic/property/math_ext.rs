use vivian_logic::math::{Transform, Vector3};

use super::{PrimitiveProperty, Property};

#[derive(Debug, Default, Clone)]
pub struct PropertyVector3 {
    pub x: PrimitiveProperty<f64>,
    pub y: PrimitiveProperty<f64>,
    pub z: PrimitiveProperty<f64>,
}

#[derive(Debug, Default, Clone)]
pub struct PropertyTransform {
    pub position: PropertyVector3,
    pub rotation: PropertyVector3,
}

impl PropertyTransform {
    pub fn is_zero(&self) -> bool {
        self.position.x.get() == 0.0
            && self.position.y.get() == 0.0
            && self.position.z.get() == 0.0
            && self.rotation.y.get() == 0.0
    }
}

impl Property for PropertyVector3 {
    fn is_changed(&self) -> bool {
        self.x.is_changed() || self.y.is_changed() || self.z.is_changed()
    }

    fn reset_changed_state(&mut self) {
        self.x.reset_changed_state();
        self.y.reset_changed_state();
        self.z.reset_changed_state();
    }
}

impl Property for PropertyTransform {
    fn is_changed(&self) -> bool {
        self.position.is_changed() || self.rotation.is_changed()
    }

    fn reset_changed_state(&mut self) {
        self.position.reset_changed_state();
        self.rotation.reset_changed_state();
    }
}

impl From<PropertyVector3> for Vector3 {
    fn from(value: PropertyVector3) -> Self {
        Self(value.x.get(), value.y.get(), value.z.get())
    }
}

impl From<PropertyTransform> for Transform {
    fn from(value: PropertyTransform) -> Self {
        Self {
            position: value.position.into(),
            rotation: value.rotation.into(),
        }
    }
}

impl From<Transform> for PropertyTransform {
    fn from(value: Transform) -> Self {
        Self {
            position: value.position.into(),
            rotation: value.rotation.into(),
        }
    }
}

impl From<Vector3> for PropertyVector3 {
    fn from(Vector3(x, y, z): Vector3) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
}

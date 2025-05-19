mod transform;

use config::ConfigInteractScale;
pub use transform::{Transform, Vector3};

#[derive(Debug, Default, Clone)]
pub struct Scale(pub f64, pub f64, pub f64, pub f64, pub f64);

impl Scale {
    pub fn new_by_config_str(scale_str: &str) -> Self {
        let mut s = scale_str.split(',');
        Self(
            s.next().and_then(|s| s.parse().ok()).unwrap_or_default(),
            s.next().and_then(|s| s.parse().ok()).unwrap_or_default(),
            s.next().and_then(|s| s.parse().ok()).unwrap_or_default(),
            s.next().and_then(|s| s.parse().ok()).unwrap_or_default(),
            s.next().and_then(|s| s.parse().ok()).unwrap_or_default(),
        )
    }

    pub fn new_by_config_struct(scale: &ConfigInteractScale) -> Self {
        Self(scale.x, scale.y, scale.z, scale.w, scale.r)
    }
}

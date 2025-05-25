mod map;
mod math_ext;
mod primitive;
mod set;
mod random;

pub use map::PropertyHashMap;
pub use math_ext::PropertyTransform;
pub use primitive::PrimitiveProperty;
pub use set::PropertyHashSet;
pub use random::GachaRandom;

pub trait Property {
    fn is_changed(&self) -> bool;
    fn reset_changed_state(&mut self);
}

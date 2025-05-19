use crate::logic::property::{PrimitiveProperty, Property};

pub struct GachaRandom(PrimitiveProperty<u32>);

impl GachaRandom {
    pub fn rand(&mut self, max: u32) -> u32 {
        let seed = self.0.get();
        let tmp = seed ^ (seed << 13);
        let tmp = tmp ^ (tmp >> 17);
        let result = tmp ^ (tmp.wrapping_mul(32));

        self.0.set(result);
        result % max
    }

    pub fn seed(&self) -> u32 {
        self.0.get()
    }
}

impl Property for GachaRandom {
    fn is_changed(&self) -> bool {
        self.0.is_changed()
    }

    fn reset_changed_state(&mut self) {
        self.0.reset_changed_state();
    }
}

impl From<u32> for GachaRandom {
    fn from(value: u32) -> Self {
        GachaRandom(value.into())
    }
}

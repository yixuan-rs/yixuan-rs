use super::Property;

#[derive(Debug, Default, Clone)]
pub struct PrimitiveProperty<T> {
    value: T,
    is_changed: bool,
}

macro_rules! impl_primitives {
    ($($ty:ty),*) => {
        $(
            #[allow(dead_code)]
            impl PrimitiveProperty<$ty> {
                pub fn new(value: $ty) -> Self {
                    Self {
                        value,
                        is_changed: false,
                    }
                }

                pub fn get(&self) -> $ty {
                    self.value
                }

                pub fn set(&mut self, value: $ty) {
                    if self.value != value {
                        self.value = value;
                        self.is_changed = true;
                    }
                }
            }

            #[allow(dead_code)]
            impl From<$ty> for PrimitiveProperty<$ty> {
                fn from(value: $ty) -> Self {
                    Self::new(value)
                }
            }
        )*
    };
}

impl_primitives!(bool, u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);

impl PrimitiveProperty<String> {
    pub fn new(value: String) -> Self {
        Self {
            value,
            is_changed: false,
        }
    }

    pub fn get(&self) -> &str {
        &self.value
    }

    pub fn set(&mut self, value: &str) {
        if self.value != value {
            self.value = value.to_string();
            self.is_changed = true;
        }
    }
}

impl From<String> for PrimitiveProperty<String> {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl<T> Property for PrimitiveProperty<T> {
    fn is_changed(&self) -> bool {
        self.is_changed
    }

    fn reset_changed_state(&mut self) {
        self.is_changed = false;
    }
}

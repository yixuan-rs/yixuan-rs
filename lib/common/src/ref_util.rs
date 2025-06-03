use std::ops::Deref;

pub enum Ref<T: 'static> {
    Static(&'static T),
    Owned(T),
}

impl<T> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            Self::Static(s) => s,
            Self::Owned(s) => s,
        }
    }
}

impl<T> From<&'static T> for Ref<T> {
    fn from(value: &'static T) -> Self {
        Self::Static(value)
    }
}

impl<T> From<T> for Ref<T> {
    fn from(value: T) -> Self {
        Self::Owned(value)
    }
}

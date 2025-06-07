use crate::math::Vector3i;

#[derive(Debug)]
pub struct EntityMoveEvt {
    pub position: Vector3i,
    pub rotation: Vector3i,
}

#[derive(Debug)]
pub struct AvatarChangeEvt {
    pub current_avatar_id: u32,
}

pub trait ViewObjectEventBase: std::fmt::Debug {
    fn pack(self) -> ViewObjectEvent;
}

macro_rules! events {
    ($($name:ident;)*) => {
        pub enum ViewObjectEvent {
            $($name($name),)*
        }

        $(impl ViewObjectEventBase for $name {
            fn pack(self) -> ViewObjectEvent {
                ViewObjectEvent::$name(self)
            }
        })*
    };
}

events! {
    EntityMoveEvt;
    AvatarChangeEvt;
}

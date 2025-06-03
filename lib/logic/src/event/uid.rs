use std::ops::Deref;

use yixuan_proto::EventGraphOwnerType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventUID(u64);

impl EventUID {
    pub fn new(owner_type: EventGraphOwnerType, event_id: u32) -> Self {
        let owner_type = i32::from(owner_type) as u64;
        Self((owner_type << 32) | event_id as u64)
    }

    pub fn owner_type(&self) -> EventGraphOwnerType {
        EventGraphOwnerType::try_from((self.0 >> 32) as i32).unwrap()
    }

    pub fn event_id(&self) -> u32 {
        (self.0 & 0xFFFFFFFF) as u32
    }
}

impl Deref for EventUID {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u64> for EventUID {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

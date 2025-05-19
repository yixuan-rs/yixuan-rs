use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::Deserialize;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, TryFromPrimitive, IntoPrimitive,
)]
#[repr(i32)]
pub enum EQuestState {
    Unlocked = 0,
    InProgress = 1,
    ToFinish = 2,
    Finished = 3,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, TryFromPrimitive, IntoPrimitive,
)]
#[repr(u32)]
pub enum ETimePeriodType {
    Morning = 1,
    Afternoon = 2,
    Evening = 3,
    Night = 4,
    Now = 99,
}

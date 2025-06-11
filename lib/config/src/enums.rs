use std::{fmt, str::FromStr};

use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::Deserialize;

macro_rules! enumeration {
    ($name:ident($repr:ident), $($variant_name:ident = $variant_value:literal),*) => {
        #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, TryFromPrimitive, IntoPrimitive)]
        #[repr($repr)]
        pub enum $name {
            #[default]
            $($variant_name = $variant_value),*
        }

        impl FromStr for $name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $(stringify!($variant_name) => Self::$variant_name,)*
                    _ => Self::default()
                })
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::from_str(value).unwrap()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{self:?}")
            }
        }
    }
}

enumeration!(
    EQuestState(i32),
    Unlocked = 0,
    InProgress = 1,
    ToFinish = 2,
    Finished = 3
);

enumeration!(
    EWeatherType(u32),
    Random = 0,
    SunShine = 1,
    Fog = 2,
    Cloudy = 3,
    Rain = 4,
    Thunder = 5
);

enumeration!(
    ETimePeriodType(u32),
    Morning = 1,
    Afternoon = 2,
    Evening = 3,
    Night = 4,
    Now = 99
);

enumeration!(
    EGadgetType(u32),
    Default = 0,
    Portal = 1,
    ViewPoint = 2,
    Treasure = 3,
    Collection = 4,
    Unknown7 = 7
);

enumeration!(EntityType(u32), Avatar = 1, Monster = 2);

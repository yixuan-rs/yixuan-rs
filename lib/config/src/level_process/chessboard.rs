use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::Deserialize;

use super::ConfigVector2Int;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HollowChessboardConfig {
    #[serde(rename = "ID")]
    pub id: u32,
    pub default_player_pos: ConfigVector2Int,
    pub default_section_index: usize,
    pub sections: Vec<HollowSectionConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HollowSectionConfig {
    pub grids: Vec<HollowGridConfig>,
    pub events: Vec<HollowEventConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HollowGridConfig {
    pub position: ConfigVector2Int,
    pub node_state: ConfigNodeState,
    pub node_visible: ConfigNodeVisible,
    pub grid_links: Vec<ConfigGridLink>,
    pub grid_flags: Vec<ConfigGridFlag>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HollowEventConfig {
    pub owned_by: ConfigEventOwner,
    #[serde(rename = "EventID")]
    pub event_id: u32,
    pub priority: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(tag = "$type")]
pub enum ConfigEventOwner {
    #[serde(rename = "Share.CEventOwnerGlobal")]
    Global,
    #[serde(rename = "Share.CEventOwnerGrid")]
    #[serde(rename_all = "PascalCase")]
    Grid { position: ConfigVector2Int },
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize, IntoPrimitive)]
#[repr(i32)]
pub enum ConfigNodeState {
    All = 0,
    Locked = 1,
    Unlocked = 2,
    Finished = 3,
    ShowEvent = 4,
    Door = 5,
    Brighten = 6,
    Guide = 7,
    Target = 8,
    BrightenOnlyVisible = 9,
    Unstable = 10,
    Empty = 11,
    LockedWithStamina = 14,
    UnEmpty = 15,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize, IntoPrimitive)]
#[repr(i32)]
pub enum ConfigNodeVisible {
    All = 0,
    Visible = 1,
    VisibleAtGridAround = 2,
    VisibleByTriggerEvent = 3,
    TemporaryVisibleAtAround = 4,
    Blocked = 5,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, IntoPrimitive)]
#[repr(i32)]
pub enum ConfigGridLink {
    Up = 1,
    Down = 2,
    Right = 4,
    Left = 8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum ConfigGridDir {
    Down = 1,
    Left = 2,
    Right = 3,
    Up = 4,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, IntoPrimitive)]
#[repr(i32)]
pub enum ConfigGridFlag {
    Core = 1,
    CanMove = 2,
    Travelled = 4,
    ShowEventType = 8,
    ShowEventId = 16,
    CanTriggerEvent = 32,
    Visible = 64,
    VisibleAtGridAround = 128,
    VisibleByTriggerEvent = 256,
    SyncToClient = 512,
    Door = 1024,
    CanTriggerMultiTimes = 2048,
    TemporaryVisibleAtAround = 4096,
    Unlocked = 8192,
    Brighten = 16384,
    Guide = 32768,
    Target = 65536,
    BrightenOnlyVisible = 131072,
    Unstable = 262144,
    Empty = 524288,
    Blocked = 1048576,
    Gdhpcijjoah = 2097152,
    Blblfbdlbbo = 4194304,
    Nihgbijfiae = 8388608,
    Ebjcidkjnki = 16777216,
    Jgjdbhllmai = 33554432,
}

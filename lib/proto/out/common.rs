// This file is @generated by prost-build.
#[derive(::proto_derive::NetCmd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transform {
    #[prost(double, repeated, tag = "1")]
    pub position: ::prost::alloc::vec::Vec<f64>,
    #[prost(double, repeated, tag = "2")]
    pub rotation: ::prost::alloc::vec::Vec<f64>,
}
#[derive(::proto_derive::NetCmd)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Vector2Int {
    #[prost(int32, tag = "1")]
    pub x: i32,
    #[prost(int32, tag = "2")]
    pub y: i32,
}
#[derive(::proto_derive::NetCmd)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct HollowGridState {
    #[prost(int32, tag = "1")]
    pub node_state: i32,
    #[prost(int32, tag = "2")]
    pub node_visible: i32,
    #[prost(enumeration = "HollowGridFlag", tag = "3")]
    pub flag: i32,
    #[prost(int32, tag = "4")]
    pub sub_state: i32,
}
#[derive(::proto_derive::NetCmd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvatarUnitInfo {
    #[prost(uint32, tag = "1")]
    pub avatar_id: u32,
    #[prost(map = "uint32, int32", tag = "2")]
    pub properties: ::std::collections::HashMap<u32, i32>,
}
#[derive(::proto_derive::NetCmd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogSkillUseInfo {
    #[prost(int32, tag = "1")]
    pub skill_id: i32,
    #[prost(int32, tag = "2")]
    pub damage: i32,
    #[prost(int32, tag = "3")]
    pub level: i32,
    #[prost(int32, tag = "4")]
    pub use_times: i32,
    #[prost(int32, tag = "5")]
    pub hit_times: i32,
    #[prost(string, tag = "6")]
    pub skill_name: ::prost::alloc::string::String,
}
#[derive(::proto_derive::NetCmd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogBattleAvatarInfo {
    #[prost(int32, tag = "1")]
    pub avatar_id: i32,
    #[prost(int64, tag = "2")]
    pub avatar_uid: i64,
    #[prost(int32, tag = "3")]
    pub power: i32,
    #[prost(int32, tag = "4")]
    pub is_live: i32,
    #[prost(int32, tag = "5")]
    pub max_hp: i32,
    #[prost(int32, tag = "6")]
    pub hp: i32,
    #[prost(int32, tag = "7")]
    pub damage: i32,
    #[prost(int32, tag = "8")]
    pub be_damage: i32,
    #[prost(int32, tag = "9")]
    pub be_hit: i32,
    #[prost(int32, tag = "10")]
    pub dodge: i32,
    #[prost(int32, tag = "11")]
    pub succ_dodge: i32,
    #[prost(int32, tag = "12")]
    pub resident: i32,
    #[prost(message, repeated, tag = "14")]
    pub skill_use: ::prost::alloc::vec::Vec<LogSkillUseInfo>,
}
#[derive(::proto_derive::NetCmd)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct LogMonsterInfo {
    #[prost(int32, tag = "1")]
    pub monster_id: i32,
}
#[derive(::proto_derive::NetCmd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogBattleStatistics {
    #[prost(int64, tag = "1")]
    pub battle_uid: i64,
    #[prost(int32, tag = "2")]
    pub battle_id: i32,
    #[prost(int64, tag = "3")]
    pub pass_time: i64,
    #[prost(int32, tag = "4")]
    pub result: i32,
    #[prost(int32, tag = "5")]
    pub switch_num: i32,
    #[prost(int32, tag = "6")]
    pub score: i32,
    #[prost(message, repeated, tag = "7")]
    pub avatar_list: ::prost::alloc::vec::Vec<LogBattleAvatarInfo>,
    #[prost(message, repeated, tag = "8")]
    pub monster_list: ::prost::alloc::vec::Vec<LogMonsterInfo>,
    #[prost(int32, tag = "11")]
    pub star: i32,
}
#[derive(::proto_derive::NetCmd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightResult {
    #[prost(int32, tag = "1")]
    pub result: i32,
    #[prost(int32, tag = "2")]
    pub star: i32,
    #[prost(message, optional, tag = "6")]
    pub battle_statistic: ::core::option::Option<LogBattleStatistics>,
}
#[derive(::proto_derive::NetCmd)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Vector3 {
    #[prost(int32, tag = "1")]
    pub x: i32,
    #[prost(int32, tag = "2")]
    pub y: i32,
    #[prost(int32, tag = "3")]
    pub z: i32,
}
#[derive(::proto_derive::NetCmd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigSceneAvatarInfo {
    #[prost(uint32, tag = "1")]
    pub avatar_id: u32,
    #[prost(enumeration = "TeamMemberSource", tag = "2")]
    pub source: i32,
    #[prost(enumeration = "SceneAvatarState", tag = "3")]
    pub cur_state: i32,
    #[prost(uint32, tag = "4")]
    pub cur_hp: u32,
    #[prost(enumeration = "TeamMemberOperation", tag = "5")]
    pub operation: i32,
    #[prost(message, optional, tag = "6")]
    pub avatar_unit: ::core::option::Option<AvatarUnitInfo>,
}
#[derive(::proto_derive::NetCmd)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SceneAvatarInfo {
    #[prost(int32, tag = "1")]
    pub avatar_id: i32,
}
#[derive(::proto_derive::NetCmd)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SceneMonsterInfo {
    #[prost(int32, tag = "1")]
    pub monster_id: i32,
    #[prost(int32, tag = "2")]
    pub level: i32,
}
#[derive(::proto_derive::NetCmd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SceneEntityInfo {
    #[prost(uint32, tag = "1")]
    pub entity_id: u32,
    #[prost(uint32, tag = "2")]
    pub group_id: u32,
    #[prost(uint32, tag = "3")]
    pub config_id: u32,
    #[prost(message, optional, tag = "4")]
    pub position: ::core::option::Option<Vector3>,
    #[prost(message, optional, tag = "5")]
    pub rotation: ::core::option::Option<Vector3>,
    #[prost(map = "int32, int32", tag = "6")]
    pub unk_big_scene_entity_map_1: ::std::collections::HashMap<i32, i32>,
    #[prost(map = "string, int32", tag = "7")]
    pub unk_big_scene_entity_map_2: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        i32,
    >,
    #[prost(oneof = "scene_entity_info::Entity", tags = "8, 9")]
    pub entity: ::core::option::Option<scene_entity_info::Entity>,
}
/// Nested message and enum types in `SceneEntityInfo`.
pub mod scene_entity_info {
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum Entity {
        #[prost(message, tag = "8")]
        Avatar(super::SceneAvatarInfo),
        #[prost(message, tag = "9")]
        Monster(super::SceneMonsterInfo),
    }
}
#[derive(::proto_derive::NetCmd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackPointInfo {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub position: ::core::option::Option<Vector3>,
    #[prost(message, optional, tag = "3")]
    pub rotation: ::core::option::Option<Vector3>,
    #[prost(uint32, tag = "4")]
    pub group_id: u32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TimePeriodType {
    None = 0,
    Morning = 1,
    Afternoon = 2,
    Evening = 3,
    Night = 4,
    Now = 99,
}
impl TimePeriodType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "TIME_PERIOD_TYPE_NONE",
            Self::Morning => "TIME_PERIOD_TYPE_MORNING",
            Self::Afternoon => "TIME_PERIOD_TYPE_AFTERNOON",
            Self::Evening => "TIME_PERIOD_TYPE_EVENING",
            Self::Night => "TIME_PERIOD_TYPE_NIGHT",
            Self::Now => "TIME_PERIOD_TYPE_NOW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TIME_PERIOD_TYPE_NONE" => Some(Self::None),
            "TIME_PERIOD_TYPE_MORNING" => Some(Self::Morning),
            "TIME_PERIOD_TYPE_AFTERNOON" => Some(Self::Afternoon),
            "TIME_PERIOD_TYPE_EVENING" => Some(Self::Evening),
            "TIME_PERIOD_TYPE_NIGHT" => Some(Self::Night),
            "TIME_PERIOD_TYPE_NOW" => Some(Self::Now),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HollowGridFlag {
    None = 0,
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
impl HollowGridFlag {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "HOLLOW_GRID_FLAG_NONE",
            Self::Core => "HOLLOW_GRID_FLAG_CORE",
            Self::CanMove => "HOLLOW_GRID_FLAG_CAN_MOVE",
            Self::Travelled => "HOLLOW_GRID_FLAG_TRAVELLED",
            Self::ShowEventType => "HOLLOW_GRID_FLAG_SHOW_EVENT_TYPE",
            Self::ShowEventId => "HOLLOW_GRID_FLAG_SHOW_EVENT_ID",
            Self::CanTriggerEvent => "HOLLOW_GRID_FLAG_CAN_TRIGGER_EVENT",
            Self::Visible => "HOLLOW_GRID_FLAG_VISIBLE",
            Self::VisibleAtGridAround => "HOLLOW_GRID_FLAG_VISIBLE_AT_GRID_AROUND",
            Self::VisibleByTriggerEvent => "HOLLOW_GRID_FLAG_VISIBLE_BY_TRIGGER_EVENT",
            Self::SyncToClient => "HOLLOW_GRID_FLAG_SYNC_TO_CLIENT",
            Self::Door => "HOLLOW_GRID_FLAG_DOOR",
            Self::CanTriggerMultiTimes => "HOLLOW_GRID_FLAG_CAN_TRIGGER_MULTI_TIMES",
            Self::TemporaryVisibleAtAround => {
                "HOLLOW_GRID_FLAG_TEMPORARY_VISIBLE_AT_AROUND"
            }
            Self::Unlocked => "HOLLOW_GRID_FLAG_UNLOCKED",
            Self::Brighten => "HOLLOW_GRID_FLAG_BRIGHTEN",
            Self::Guide => "HOLLOW_GRID_FLAG_GUIDE",
            Self::Target => "HOLLOW_GRID_FLAG_TARGET",
            Self::BrightenOnlyVisible => "HOLLOW_GRID_FLAG_BRIGHTEN_ONLY_VISIBLE",
            Self::Unstable => "HOLLOW_GRID_FLAG_UNSTABLE",
            Self::Empty => "HOLLOW_GRID_FLAG_EMPTY",
            Self::Blocked => "HOLLOW_GRID_FLAG_BLOCKED",
            Self::Gdhpcijjoah => "HOLLOW_GRID_FLAG_GDHPCIJJOAH",
            Self::Blblfbdlbbo => "HOLLOW_GRID_FLAG_BLBLFBDLBBO",
            Self::Nihgbijfiae => "HOLLOW_GRID_FLAG_NIHGBIJFIAE",
            Self::Ebjcidkjnki => "HOLLOW_GRID_FLAG_EBJCIDKJNKI",
            Self::Jgjdbhllmai => "HOLLOW_GRID_FLAG_JGJDBHLLMAI",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HOLLOW_GRID_FLAG_NONE" => Some(Self::None),
            "HOLLOW_GRID_FLAG_CORE" => Some(Self::Core),
            "HOLLOW_GRID_FLAG_CAN_MOVE" => Some(Self::CanMove),
            "HOLLOW_GRID_FLAG_TRAVELLED" => Some(Self::Travelled),
            "HOLLOW_GRID_FLAG_SHOW_EVENT_TYPE" => Some(Self::ShowEventType),
            "HOLLOW_GRID_FLAG_SHOW_EVENT_ID" => Some(Self::ShowEventId),
            "HOLLOW_GRID_FLAG_CAN_TRIGGER_EVENT" => Some(Self::CanTriggerEvent),
            "HOLLOW_GRID_FLAG_VISIBLE" => Some(Self::Visible),
            "HOLLOW_GRID_FLAG_VISIBLE_AT_GRID_AROUND" => Some(Self::VisibleAtGridAround),
            "HOLLOW_GRID_FLAG_VISIBLE_BY_TRIGGER_EVENT" => {
                Some(Self::VisibleByTriggerEvent)
            }
            "HOLLOW_GRID_FLAG_SYNC_TO_CLIENT" => Some(Self::SyncToClient),
            "HOLLOW_GRID_FLAG_DOOR" => Some(Self::Door),
            "HOLLOW_GRID_FLAG_CAN_TRIGGER_MULTI_TIMES" => {
                Some(Self::CanTriggerMultiTimes)
            }
            "HOLLOW_GRID_FLAG_TEMPORARY_VISIBLE_AT_AROUND" => {
                Some(Self::TemporaryVisibleAtAround)
            }
            "HOLLOW_GRID_FLAG_UNLOCKED" => Some(Self::Unlocked),
            "HOLLOW_GRID_FLAG_BRIGHTEN" => Some(Self::Brighten),
            "HOLLOW_GRID_FLAG_GUIDE" => Some(Self::Guide),
            "HOLLOW_GRID_FLAG_TARGET" => Some(Self::Target),
            "HOLLOW_GRID_FLAG_BRIGHTEN_ONLY_VISIBLE" => Some(Self::BrightenOnlyVisible),
            "HOLLOW_GRID_FLAG_UNSTABLE" => Some(Self::Unstable),
            "HOLLOW_GRID_FLAG_EMPTY" => Some(Self::Empty),
            "HOLLOW_GRID_FLAG_BLOCKED" => Some(Self::Blocked),
            "HOLLOW_GRID_FLAG_GDHPCIJJOAH" => Some(Self::Gdhpcijjoah),
            "HOLLOW_GRID_FLAG_BLBLFBDLBBO" => Some(Self::Blblfbdlbbo),
            "HOLLOW_GRID_FLAG_NIHGBIJFIAE" => Some(Self::Nihgbijfiae),
            "HOLLOW_GRID_FLAG_EBJCIDKJNKI" => Some(Self::Ebjcidkjnki),
            "HOLLOW_GRID_FLAG_JGJDBHLLMAI" => Some(Self::Jgjdbhllmai),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NodeState {
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
impl NodeState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::All => "NODE_STATE_ALL",
            Self::Locked => "NODE_STATE_LOCKED",
            Self::Unlocked => "NODE_STATE_UNLOCKED",
            Self::Finished => "NODE_STATE_FINISHED",
            Self::ShowEvent => "NODE_STATE_SHOW_EVENT",
            Self::Door => "NODE_STATE_DOOR",
            Self::Brighten => "NODE_STATE_BRIGHTEN",
            Self::Guide => "NODE_STATE_GUIDE",
            Self::Target => "NODE_STATE_TARGET",
            Self::BrightenOnlyVisible => "NODE_STATE_BRIGHTEN_ONLY_VISIBLE",
            Self::Unstable => "NODE_STATE_UNSTABLE",
            Self::Empty => "NODE_STATE_EMPTY",
            Self::LockedWithStamina => "NODE_STATE_LOCKED_WITH_STAMINA",
            Self::UnEmpty => "NODE_STATE_UN_EMPTY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NODE_STATE_ALL" => Some(Self::All),
            "NODE_STATE_LOCKED" => Some(Self::Locked),
            "NODE_STATE_UNLOCKED" => Some(Self::Unlocked),
            "NODE_STATE_FINISHED" => Some(Self::Finished),
            "NODE_STATE_SHOW_EVENT" => Some(Self::ShowEvent),
            "NODE_STATE_DOOR" => Some(Self::Door),
            "NODE_STATE_BRIGHTEN" => Some(Self::Brighten),
            "NODE_STATE_GUIDE" => Some(Self::Guide),
            "NODE_STATE_TARGET" => Some(Self::Target),
            "NODE_STATE_BRIGHTEN_ONLY_VISIBLE" => Some(Self::BrightenOnlyVisible),
            "NODE_STATE_UNSTABLE" => Some(Self::Unstable),
            "NODE_STATE_EMPTY" => Some(Self::Empty),
            "NODE_STATE_LOCKED_WITH_STAMINA" => Some(Self::LockedWithStamina),
            "NODE_STATE_UN_EMPTY" => Some(Self::UnEmpty),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NodeVisible {
    All = 0,
    Visible = 1,
    VisibleAtGridAround = 2,
    VisibleByTriggerEvent = 3,
    TemporaryVisibleAtAround = 4,
    Blocked = 5,
}
impl NodeVisible {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::All => "NODE_VISIBLE_ALL",
            Self::Visible => "NODE_VISIBLE_VISIBLE",
            Self::VisibleAtGridAround => "NODE_VISIBLE_VISIBLE_AT_GRID_AROUND",
            Self::VisibleByTriggerEvent => "NODE_VISIBLE_VISIBLE_BY_TRIGGER_EVENT",
            Self::TemporaryVisibleAtAround => "NODE_VISIBLE_TEMPORARY_VISIBLE_AT_AROUND",
            Self::Blocked => "NODE_VISIBLE_BLOCKED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NODE_VISIBLE_ALL" => Some(Self::All),
            "NODE_VISIBLE_VISIBLE" => Some(Self::Visible),
            "NODE_VISIBLE_VISIBLE_AT_GRID_AROUND" => Some(Self::VisibleAtGridAround),
            "NODE_VISIBLE_VISIBLE_BY_TRIGGER_EVENT" => Some(Self::VisibleByTriggerEvent),
            "NODE_VISIBLE_TEMPORARY_VISIBLE_AT_AROUND" => {
                Some(Self::TemporaryVisibleAtAround)
            }
            "NODE_VISIBLE_BLOCKED" => Some(Self::Blocked),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NodeSubState {
    None = 0,
    EmptySub = 1,
    BlockedDown = 2,
    BlockedLeft = 3,
    BlockedRight = 4,
    BlockedUp = 5,
    Anchor = 6,
    Aim = 7,
    Block = 8,
    AnchorUnlock = 9,
    AimOverrideAll = 10,
    AimOverridePlayer = 11,
    AimOverrideNpc = 12,
    AffectedEmp = 28,
    Unavailable = 29,
    AnchorLockerShow = 30,
}
impl NodeSubState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "NODE_SUB_STATE_NONE",
            Self::EmptySub => "NODE_SUB_STATE_EMPTY_SUB",
            Self::BlockedDown => "NODE_SUB_STATE_BLOCKED_DOWN",
            Self::BlockedLeft => "NODE_SUB_STATE_BLOCKED_LEFT",
            Self::BlockedRight => "NODE_SUB_STATE_BLOCKED_RIGHT",
            Self::BlockedUp => "NODE_SUB_STATE_BLOCKED_UP",
            Self::Anchor => "NODE_SUB_STATE_ANCHOR",
            Self::Aim => "NODE_SUB_STATE_AIM",
            Self::Block => "NODE_SUB_STATE_BLOCK",
            Self::AnchorUnlock => "NODE_SUB_STATE_ANCHOR_UNLOCK",
            Self::AimOverrideAll => "NODE_SUB_STATE_AIM_OVERRIDE_ALL",
            Self::AimOverridePlayer => "NODE_SUB_STATE_AIM_OVERRIDE_PLAYER",
            Self::AimOverrideNpc => "NODE_SUB_STATE_AIM_OVERRIDE_NPC",
            Self::AffectedEmp => "NODE_SUB_STATE_AFFECTED_EMP",
            Self::Unavailable => "NODE_SUB_STATE_UNAVAILABLE",
            Self::AnchorLockerShow => "NODE_SUB_STATE_ANCHOR_LOCKER_SHOW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NODE_SUB_STATE_NONE" => Some(Self::None),
            "NODE_SUB_STATE_EMPTY_SUB" => Some(Self::EmptySub),
            "NODE_SUB_STATE_BLOCKED_DOWN" => Some(Self::BlockedDown),
            "NODE_SUB_STATE_BLOCKED_LEFT" => Some(Self::BlockedLeft),
            "NODE_SUB_STATE_BLOCKED_RIGHT" => Some(Self::BlockedRight),
            "NODE_SUB_STATE_BLOCKED_UP" => Some(Self::BlockedUp),
            "NODE_SUB_STATE_ANCHOR" => Some(Self::Anchor),
            "NODE_SUB_STATE_AIM" => Some(Self::Aim),
            "NODE_SUB_STATE_BLOCK" => Some(Self::Block),
            "NODE_SUB_STATE_ANCHOR_UNLOCK" => Some(Self::AnchorUnlock),
            "NODE_SUB_STATE_AIM_OVERRIDE_ALL" => Some(Self::AimOverrideAll),
            "NODE_SUB_STATE_AIM_OVERRIDE_PLAYER" => Some(Self::AimOverridePlayer),
            "NODE_SUB_STATE_AIM_OVERRIDE_NPC" => Some(Self::AimOverrideNpc),
            "NODE_SUB_STATE_AFFECTED_EMP" => Some(Self::AffectedEmp),
            "NODE_SUB_STATE_UNAVAILABLE" => Some(Self::Unavailable),
            "NODE_SUB_STATE_ANCHOR_LOCKER_SHOW" => Some(Self::AnchorLockerShow),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TeamMemberSource {
    None = 0,
    Normal = 1,
    Unk2 = 2,
    Unk3 = 3,
}
impl TeamMemberSource {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "TEAM_MEMBER_SOURCE_NONE",
            Self::Normal => "TEAM_MEMBER_SOURCE_NORMAL",
            Self::Unk2 => "TEAM_MEMBER_SOURCE_UNK_2",
            Self::Unk3 => "TEAM_MEMBER_SOURCE_UNK_3",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TEAM_MEMBER_SOURCE_NONE" => Some(Self::None),
            "TEAM_MEMBER_SOURCE_NORMAL" => Some(Self::Normal),
            "TEAM_MEMBER_SOURCE_UNK_2" => Some(Self::Unk2),
            "TEAM_MEMBER_SOURCE_UNK_3" => Some(Self::Unk3),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SceneAvatarState {
    None = 0,
    Alive = 1,
    Dead = 3,
}
impl SceneAvatarState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "SCENE_AVATAR_STATE_NONE",
            Self::Alive => "SCENE_AVATAR_STATE_ALIVE",
            Self::Dead => "SCENE_AVATAR_STATE_DEAD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SCENE_AVATAR_STATE_NONE" => Some(Self::None),
            "SCENE_AVATAR_STATE_ALIVE" => Some(Self::Alive),
            "SCENE_AVATAR_STATE_DEAD" => Some(Self::Dead),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TeamMemberOperation {
    None = 0,
    Unk1 = 1,
    Unk2 = 2,
    TeamReplace = 3,
}
impl TeamMemberOperation {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "TEAM_MEMBER_OPERATION_NONE",
            Self::Unk1 => "TEAM_MEMBER_OPERATION_UNK_1",
            Self::Unk2 => "TEAM_MEMBER_OPERATION_UNK_2",
            Self::TeamReplace => "TEAM_MEMBER_OPERATION_TEAM_REPLACE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TEAM_MEMBER_OPERATION_NONE" => Some(Self::None),
            "TEAM_MEMBER_OPERATION_UNK_1" => Some(Self::Unk1),
            "TEAM_MEMBER_OPERATION_UNK_2" => Some(Self::Unk2),
            "TEAM_MEMBER_OPERATION_TEAM_REPLACE" => Some(Self::TeamReplace),
            _ => None,
        }
    }
}

use std::collections::{HashMap, HashSet};

use num_enum::IntoPrimitive;
use serde::Deserialize;
use yixuan_codegen::action;

use crate::{EQuestState, ETimePeriodType, util::nap_expr_parser::NapExpr};

use super::{ConfigNodeState, ConfigNodeVisible, ConfigPredicate};

#[action]
pub struct ActionOpenUI {
    #[serde(rename = "UI")]
    pub ui: String,
    #[serde(default)]
    pub args: i32,
    #[serde(default)]
    #[serde(rename = "StoreTemplateID")]
    pub store_template_id: i32,
}

#[action]
pub struct ActionSwitchSection {
    #[serde(rename = "SectionID")]
    pub section_id: u32,
    pub transform: String,
    pub camera_y: u32,
    pub camera_x: u32,
}

#[action]
pub struct ActionResetEvent {}

#[action]
pub struct ActionCreateNpc {
    #[serde(rename = "TagID")]
    #[serde(default)]
    pub tag_id: u32,
    #[serde(rename = "TagIDs")]
    #[serde(default)]
    pub tag_ids: Vec<u32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigInteractScale {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    #[serde(default)]
    pub w: f64,
    #[serde(default)]
    pub r: f64,
}

#[action]
pub struct ActionChangeInteract {
    #[serde(rename = "InteractID")]
    pub interact_id: u32,
    #[serde(rename = "TagIDs")]
    pub tag_ids: Vec<u32>,
    #[serde(deserialize_with = "deserialize_participators")]
    pub participators: HashMap<u32, String>,
    pub interact_shape: String,
    pub interact_scale: ConfigInteractScale,
}

#[action]
pub struct ActionSetMainCityObjectState {
    #[serde(deserialize_with = "deserialize_map_int_int")]
    pub object_state: HashMap<i32, i32>,
}

#[action]
pub struct ActionForceRefresh {}

#[action]
pub struct ActionShowTeleportUi {
    pub black_mask: bool,
}

#[action]
pub struct ActionPerform {
    #[serde(rename = "PerformID")]
    #[serde(default)]
    pub perform_id: u32,
    #[serde(rename = "PerformID2")]
    #[serde(default)]
    pub perform_id_2: u32,
    #[serde(rename = "PerformID3")]
    #[serde(default)]
    pub perform_id_3: u32,
    #[serde(default)]
    pub black_mask: bool,
    #[serde(default)]
    pub black_mask_fade_out: bool,
    #[serde(default)]
    pub black_mask_fade_out_2: bool,
    #[serde(default)]
    #[serde(rename = "AvatarID")]
    pub avatar_id: u32,
    #[serde(default)]
    #[serde(rename = "NpcID")]
    pub npc_id: u32,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_participators")]
    pub participators: HashMap<u32, String>,
}

#[action]
pub struct ActionShowTip {
    #[serde(rename = "TipID")]
    pub tip_id: u32,
}

#[action]
pub struct ConfigShowTip {
    #[serde(rename = "TipID")]
    pub tip_id: u32,
}

#[action]
pub struct ActionSetQuestPhase {
    pub target_phase: EQuestState,
    #[serde(rename = "QuestID")]
    pub quest_id: u32,
}

#[action]
pub struct ActionChangeBackSceneInfo {
    #[serde(rename = "SectionID")]
    pub section_id: u32,
    pub transform: String,
}

#[action]
pub struct ActionTriggerInteract {
    #[serde(rename = "TagID")]
    pub tag_id: u32,
    #[serde(rename = "InteractID")]
    pub interact_id: u32,
}

#[action]
pub struct ActionDownloadFullResource {}

#[action]
pub struct ActionRemoveMainCityQuestInteract {}

#[action]
pub struct ActionRemoveMainCityQuestNpc {}

#[action]
pub struct ConfigUnlockHollowQuest {
    #[serde(rename = "QuestID")]
    pub quest_id: u32,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OpenDialogParam {
    pub unk_open_dialog_flag_1: bool,
    pub start_texture_sheet: String,
    pub unk_open_dialog_flag_2: bool,
    pub loop_texture_sheet: String,
    pub end_texture_sheet: String,
    pub unk_open_dialog_flag_3: bool,
}

#[action]
pub struct ConfigOpenDialog {
    pub open_event: bool,
    pub camera_move: bool,
    #[serde(default)]
    pub unk_open_dialog: bool,
    #[serde(default)]
    pub open_param: OpenDialogParam,
}

#[action]
pub struct ConfigLogText {
    pub messages: Vec<String>,
    pub log_title: String,
    pub voicelines: Vec<String>,
}

#[action]
pub struct ConfigCloseDialog {
    pub camera_move: bool,
    pub need_reset_center: bool,
}

#[derive(Debug, Clone, Copy, Deserialize, IntoPrimitive)]
#[repr(i32)]
pub enum CameraMove {
    TriggerPosition = 0,
    Player = 1,
    Center = 2,
    CustomBound = 3,
    Back = 4,
}

#[derive(Debug, Clone, Copy, Deserialize, IntoPrimitive)]
#[repr(i32)]
pub enum HollowPositionOffsetType {
    Relative = 0,
    EventPos = 1,
    Absolute = 2,
    HollowNpcPos = 3,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigVector2Int {
    pub x: i32,
    pub y: i32,
}

#[action]
pub struct ConfigCameraMoveV2 {
    pub stretch_key: String,
    pub r#type: CameraMove,
    pub position_offset_type: HollowPositionOffsetType,
    #[serde(default)]
    pub position_offset_x: String,
    #[serde(default)]
    pub position_offset_y: String,
    #[serde(default)]
    pub radius_x: String,
    #[serde(default)]
    pub radius_y: String,
    #[serde(default)]
    pub bound_index_x: Option<ConfigVector2Int>,
    #[serde(default)]
    pub bound_index_y: Option<ConfigVector2Int>,
    #[serde(default)]
    pub freeze_z: bool,
    #[serde(default)]
    pub parallel: bool,
}

#[action]
pub struct ConfigWaitSeconds {
    pub wait_time: f32,
}

#[action]
pub struct ConfigShowPopWindow {
    #[serde(rename = "PopID")]
    pub pop_id: i32,
    pub show_directly: bool,
}

#[action]
pub struct ConfigCameraStretch {
    #[serde(default)]
    pub stretch_key: String,
    #[serde(default)]
    pub shake_key: String,
    pub r#type: CameraMove,
    #[serde(default)]
    pub parallel: bool,
}

#[action]
pub struct ConfigPlayAnim {
    #[serde(rename = "AnimID")]
    pub anim_id: u32,
    pub indexes: Vec<ConfigVector2Int>,
    #[serde(default)]
    pub looping: bool,
}

#[action]
pub struct ConfigStopAnim {
    pub indexes: Vec<ConfigVector2Int>,
}

#[action]
pub struct ConfigEventModification {
    pub x: i32,
    pub y: i32,
    pub position: HollowPositionOffsetType,
    pub radius: i32,
    pub modification_num: i32,
    pub modification_type: i32,
    #[serde(rename = "TargetEventID")]
    #[serde(default)]
    pub target_event_id: Vec<u32>,
    #[serde(default)]
    pub target_event_type: u32,
    #[serde(rename = "EventID")]
    pub event_id: Vec<u32>,
    pub event_state: ConfigNodeState,
    pub visible_state: ConfigNodeVisible,
}

#[action]
pub struct ConfigPushWithDirection {
    pub direction: DynamicInteger,
}

#[action]
pub struct ConfigWaitTipsEnd {
    #[serde(rename = "TipsID")]
    pub tips_id: Vec<u32>,
}

#[action]
pub struct ConfigSetMapState {
    pub x: i32,
    pub y: i32,
    pub position: HollowPositionOffsetType,
    pub radius: i32,
    pub count: i32,
    #[serde(default)]
    pub from_visible_state: HashSet<ConfigNodeVisible>,
    #[serde(default)]
    pub to_visible_state: Vec<ConfigNodeVisible>,
    #[serde(default)]
    pub from_state: HashSet<ConfigNodeState>,
    #[serde(default)]
    pub to_state: Vec<ConfigNodeState>,
}

#[action]
pub struct ConfigReward {
    #[serde(rename = "OnceRewardID")]
    pub once_reward_id: DynamicInteger,
}

#[action]
pub struct ConfigAddItem {
    #[serde(rename = "ItemID")]
    pub item_id: u32,
    pub count: DynamicInteger,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum DynamicInteger {
    Static(i32),
    Dynamic(#[serde(deserialize_with = "deserialize_expr")] NapExpr),
}

#[action]
pub struct ConfigBreakDialogAnim {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigDialogChoiceDetail {
    pub option_text: String,
    #[serde(rename = "OptionID")]
    pub option_id: String,
    pub option_text_2: String,
    #[serde(rename = "ChoiceID")]
    pub choice_id: u32,
    #[serde(rename = "UID")]
    pub uid: u32,
}

#[action]
pub struct ConfigMakeDialogChoice {
    pub title: String,
    pub description: String,
    pub question_description: String,
    pub choice_details: Vec<ConfigDialogChoiceDetail>,
}

#[action]
pub struct ConfigSetHollowVariable {
    pub key: String,
    pub value: i32,
}

#[action]
pub struct ConfigFinishHollow {}

#[action]
pub struct ActionSetBGM {
    #[serde(rename = "MainCityMusicID")]
    pub main_city_music_id: u32,
}

#[action]
pub struct ActionSetMainCityTime {
    pub time_period: ETimePeriodType,
}

#[action]
pub struct ActionEnterHollowQuest {
    #[serde(rename = "HollowID")]
    pub hollow_id: u32,
}

fn deserialize_participators<'de, D>(deserializer: D) -> Result<HashMap<u32, String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use serde::de;
    let str_map = HashMap::<String, String>::deserialize(deserializer)?;

    str_map
        .into_iter()
        .map(|(str_key, value)| match str_key.parse() {
            Ok(int_key) => Ok((int_key, value)),
            Err(_) => Err(de::Error::invalid_value(
                de::Unexpected::Str(&str_key),
                &"u32",
            )),
        })
        .collect::<Result<HashMap<_, _>, _>>()
}

fn deserialize_map_int_int<'de, D>(deserializer: D) -> Result<HashMap<i32, i32>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use serde::de;
    let str_map = HashMap::<String, i32>::deserialize(deserializer)?;

    str_map
        .into_iter()
        .map(|(str_key, value)| match str_key.parse() {
            Ok(int_key) => Ok((int_key, value)),
            Err(_) => Err(de::Error::invalid_value(
                de::Unexpected::Str(&str_key),
                &"i32",
            )),
        })
        .collect::<Result<HashMap<_, _>, _>>()
}

fn deserialize_expr<'de, D>(deserializer: D) -> Result<NapExpr, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let str_map = String::deserialize(deserializer)?;

    NapExpr::parse(&str_map).map_err(serde::de::Error::custom)
}

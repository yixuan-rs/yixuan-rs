use std::{collections::HashMap, fmt, fs::File, str::FromStr};

use serde::{Deserialize, Deserializer, de::DeserializeOwned};

mod action;
mod chessboard;
mod predicate;

pub use action::*;
pub use chessboard::*;
pub use predicate::*;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SectionEvent {
    OnAdd,
    OnBeforeEnter,
    OnEnter,
    OnInteract,
    OnStart,
    GM,
    #[serde(untagged)]
    Custom(String),
}

impl fmt::Display for SectionEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(name) => write!(f, "{name}"),
            other => fmt::Debug::fmt(other, f),
        }
    }
}

impl FromStr for SectionEvent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "OnAdd" => Self::OnAdd,
            "OnBeforeEnter" => Self::OnBeforeEnter,
            "OnEnter" => Self::OnEnter,
            "OnInteract" => Self::OnInteract,
            "OnStart" => Self::OnStart,
            custom => Self::Custom(custom.to_string()),
        })
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MainCitySectionConfig {
    #[serde(default)]
    pub unity_scene_path: String,
    #[serde(default)]
    pub born_transform: String,
    pub section_progress: SectionEventGraphConfig,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MainCityConfig {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "DefaultSectionID")]
    #[serde(default)]
    pub default_section_id: u32,
    #[serde(deserialize_with = "deserialize_section_map")]
    pub sections: HashMap<u32, MainCitySectionConfig>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SectionEventGraphConfig {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(default)]
    pub specials: HashMap<String, i32>,
    #[serde(default)]
    pub on_add: Vec<String>,
    #[serde(default)]
    pub on_before_enter: Vec<String>,
    #[serde(default)]
    pub on_enter: Vec<String>,
    #[serde(default)]
    pub on_exit: Vec<String>,
    #[serde(default)]
    pub events: HashMap<SectionEvent, ConfigEvent>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigEvent {
    #[serde(rename = "ID")]
    #[serde(default)]
    pub id: u32,
    pub actions: Vec<ConfigEventAction>,
}

trait ConfigID {
    fn config_id(&self) -> u32;
}

impl ConfigID for SectionEventGraphConfig {
    fn config_id(&self) -> u32 {
        self.id
    }
}

impl ConfigID for MainCityConfig {
    fn config_id(&self) -> u32 {
        self.id
    }
}

macro_rules! actions {
    ($($name:ident: $tag:literal),*) => {
        #[derive(Deserialize)]
        #[serde(tag = "$type")]
        pub enum ConfigEventAction {
            $(#[serde(rename = $tag)]
            $name($name),)*
        }

        impl ConfigEventAction {
            pub fn id(&self) -> u32 {
                match self {
                    $(Self::$name(cfg) => cfg.id),*
                }
            }

            pub fn predicates(&self) -> &[ConfigPredicate] {
                match self {
                    $(Self::$name(cfg) => &cfg.predicates),*
                }
            }
        }
    };
}

actions! {
    ActionResetEvent: "Share.CActionResetEventCfg",
    ActionOpenUI: "Share.CActionOpenUI",
    ActionSwitchSection: "Share.CActionSwitchSection",
    ActionCreateNpc: "Share.CActionCreateNPCCfg",
    ActionChangeInteract: "Share.CActionChangeInteractCfg",
    ActionSetMainCityObjectState: "Share.CActionSetMainCityObjectState",
    ActionForceRefresh: "Share.CActionForceRefreshCfg",
    ActionPerform: "Share.CActionPerformCfg",
    ActionShowTip: "Share.CActionShowTip",
    ActionSetQuestPhase: "Share.CActionSetQuestPhaseCfg",
    ActionRemoveMainCityQuestInteract: "Share.CActionRemoveMainCityQuestInteractCfg",
    ActionRemoveMainCityQuestNpc: "Share.CActionRemoveMainCityQuestNpcCfg",
    ActionChangeBackSceneInfo: "Share.CActionChangeBackSceneInfoCfg",
    ActionTriggerInteract: "Share.CActionTriggerInteractCfg",
    ActionDownloadFullResource: "Share.CActionDownloadFullResourceCfg",
    ActionShowTeleportUi: "Share.CActionShowTeleportUI",
    ActionSetBGM: "Share.CActionSetBGM",
    ActionSetMainCityTime: "Share.CActionSetMainCityTime",
    ActionEnterHollowQuest: "Share.CActionEnterHollowQuest",
    ConfigUnlockHollowQuest: "Share.CConfigUnlockHollowQuest",
    // Hollow
    ConfigWaitSeconds: "Share.CConfigWaitSeconds",
    ConfigOpenDialog: "Share.CConfigOpenDialog",
    ConfigLogText: "Share.CConfigLogText",
    ConfigCloseDialog: "Share.CConfigCloseDialog",
    ConfigCameraMoveV2: "Share.CConfigCameraMoveV2",
    ConfigShowTip: "Share.CConfigShowTip",
    ConfigShowPopWindow: "Share.CConfigShowPopWindow",
    ConfigCameraStretch: "Share.CConfigCameraStretch",
    ConfigPlayAnim: "Share.CConfigPlayAnim",
    ConfigStopAnim: "Share.CConfigStopAnim",
    ConfigEventModification: "Share.CConfigEventModification",
    ConfigPushWithDirection: "Share.CConfigPushWithDirection",
    ConfigWaitTipsEnd: "Share.CConfigWaitTipsEnd",
    ConfigSetMapState: "Share.CConfigSetMapState",
    ConfigReward: "Share.CConfigReward",
    ConfigAddItem: "Share.CConfigAddItem",
    ConfigBreakDialogAnim: "Share.CConfigBreakDialogAnim",
    ConfigMakeDialogChoice: "Share.CConfigMakeDialogChoice",
    ConfigSetHollowVariable: "Share.CConfigSetHollowVariable",
    ConfigFinishHollow: "Share.CConfigFinishHollow"
}

pub struct EventGraphCollection {
    pub main_city_config: MainCityConfig,
    pub main_city_interacts: HashMap<u32, SectionEventGraphConfig>,
    pub quest_events: HashMap<u32, MainCityConfig>,
    pub hollow_events: HashMap<u32, SectionEventGraphConfig>,
}

#[derive(thiserror::Error, Debug)]
pub enum EventGraphCollectionLoadError {
    #[error("failed to open directory {0}, cause: {1}")]
    DirOpen(String, std::io::Error),
    #[error("failed to read asset file at {0}, cause: {1}")]
    Read(String, std::io::Error),
    #[error("JSON config at {0} is invalid, cause: {1}")]
    InvalidConfig(String, serde_json5::Error),
    #[error("duplicate event graph config id: {0}")]
    DuplicateGraphID(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphReference {
    MainCitySection(u32),
    Interact(u32),
    Quest(u32),
    HollowEvent(u32),
}

impl GraphReference {
    pub fn id(&self) -> u32 {
        match self {
            GraphReference::MainCitySection(id) => *id,
            GraphReference::Interact(id) => *id,
            GraphReference::Quest(id) => *id,
            GraphReference::HollowEvent(id) => *id,
        }
    }
}

impl EventGraphCollection {
    pub fn load(level_process_path: &str) -> Result<Self, EventGraphCollectionLoadError> {
        type Error = EventGraphCollectionLoadError;

        let main_city_interacts =
            Self::load_events_from_dir(&format!("{level_process_path}/MainCity/Interact"))?;

        let quest_events =
            Self::load_events_from_dir(&format!("{level_process_path}/MainCity/Quest"))?;

        let hollow_events =
            Self::load_events_from_dir(&format!("{level_process_path}/Hollow/Event"))?;

        let main_city_config_path = format!("{level_process_path}/MainCity/MainCity_1.json");
        let mut main_city_config = match File::open(&main_city_config_path) {
            Ok(file) => file,
            Err(err) => return Err(Error::Read(main_city_config_path, err)),
        };

        let main_city_config = serde_json5::from_reader(&mut main_city_config)
            .map_err(|err| Error::InvalidConfig(main_city_config_path, err))?;

        Ok(Self {
            main_city_config,
            main_city_interacts,
            quest_events,
            hollow_events,
        })
    }

    fn load_events_from_dir<T: DeserializeOwned + ConfigID>(
        path: &str,
    ) -> Result<HashMap<u32, T>, EventGraphCollectionLoadError> {
        type Error = EventGraphCollectionLoadError;

        let dir = std::fs::read_dir(path).map_err(|err| Error::DirOpen(path.to_string(), err))?;
        let mut map = HashMap::new();

        for entry in dir.flatten() {
            let mut file = File::open(entry.path())
                .map_err(|err| Error::Read(entry.path().to_string_lossy().to_string(), err))?;

            let config = serde_json5::from_reader::<_, T>(&mut file).map_err(|err| {
                Error::InvalidConfig(entry.path().to_string_lossy().to_string(), err)
            })?;

            let config_id = config.config_id();
            if map.insert(config_id, config).is_some() {
                return Err(Error::DuplicateGraphID(config_id));
            }
        }

        Ok(map)
    }

    pub fn get(
        &self,
        graph_ref: GraphReference,
        cur_section_id: u32,
    ) -> Option<&SectionEventGraphConfig> {
        use GraphReference::*;

        match graph_ref {
            MainCitySection(id) => self
                .main_city_config
                .sections
                .get(&id)
                .map(|section| &section.section_progress),
            Interact(id) => self.main_city_interacts.get(&id),
            Quest(id) => self.quest_events.get(&id).and_then(|cfg| {
                cfg.sections
                    .get(&cur_section_id)
                    .map(|cfg| &cfg.section_progress)
            }),
            HollowEvent(id) => self.hollow_events.get(&id),
        }
    }
}

fn deserialize_section_map<'de, D>(d: D) -> Result<HashMap<u32, MainCitySectionConfig>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de;
    let str_map = HashMap::<String, MainCitySectionConfig>::deserialize(d)?;

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

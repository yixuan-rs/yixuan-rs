use std::{collections::HashMap, fs::File};

use config::{
    Condition, EventGraphCollection, EventGraphCollectionLoadError, HollowChessboardConfig,
    LoadConditionsError, TemplateCollection, TemplateCollectionError,
};

use crate::config::{GMGroupConfig, GachaScheduleConfig, ResourceConfig};

#[derive(thiserror::Error, Debug)]
pub enum LoadResourcesError {
    #[error("failed to load TemplateTb assets: {0}")]
    TemplateCollection(#[from] TemplateCollectionError),
    #[error("failed to load EventGraph assets: {0}")]
    EventGraphCollection(#[from] EventGraphCollectionLoadError),
    #[error("failed to parse condition configs: {0}")]
    Conditions(#[from] LoadConditionsError),
    #[error("failed to load Hollow Chessboard configs, cause: {0}")]
    HollowChessboard(DataLoadError),
    #[error("failed to load USM encryption keys, cause: {0}")]
    VideoKeyMap(DataLoadError),
    #[error("failed to load GMGroup, cause: {0}")]
    GMGroup(DataLoadError),
}

#[derive(thiserror::Error, Debug)]
pub enum DataLoadError {
    #[error("I/O error occurred: {0}")]
    ReadFail(#[from] std::io::Error),
    #[error("JSON deserialization failed: {0}")]
    JsonDeserialization(#[from] serde_json5::Error),
}

pub struct NapResources {
    pub templates: TemplateCollection,
    pub event_graphs: EventGraphCollection,
    pub hollow_chessboard: HashMap<u32, HollowChessboardConfig>,
    pub conditions: HashMap<i32, Condition>,
    pub video_key_map: HashMap<u32, u64>,
    pub gameplay: ServerGameplayConfig,
    pub first_login_gm_groups: Vec<GMGroupConfig>,
}

pub struct ServerGameplayConfig {
    pub gacha_schedule: GachaScheduleConfig,
}

impl NapResources {
    pub fn load(
        config: &ResourceConfig,
        gameplay: ServerGameplayConfig,
    ) -> Result<Self, LoadResourcesError> {
        let templates = TemplateCollection::load(&config.fileconfig_directory)?;

        Ok(Self {
            conditions: config::load_all_conditions(templates.condition_config_template_tb())?,
            templates,
            event_graphs: EventGraphCollection::load(&config.level_process_directory)?,
            hollow_chessboard: Self::load_hollow_chessboard_map(&format!(
                "{}/Hollow/Chessboard",
                config.level_process_directory
            ))
            .map_err(LoadResourcesError::HollowChessboard)?,
            video_key_map: Self::load_video_key_map(&config.usm_keys_path)
                .map_err(LoadResourcesError::VideoKeyMap)?,
            gameplay,
            first_login_gm_groups: config
                .first_login_gm_group_list
                .iter()
                .map(|path| Self::load_gm_group(path))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    fn load_hollow_chessboard_map(
        dir: &str,
    ) -> Result<HashMap<u32, HollowChessboardConfig>, DataLoadError> {
        let dir = std::fs::read_dir(dir)?;
        let mut map = HashMap::new();

        for entry in dir {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                let mut file = File::open(entry.path())?;
                let config = serde_json5::from_reader::<_, HollowChessboardConfig>(&mut file)?;
                map.insert(config.id, config);
            }
        }

        Ok(map)
    }

    fn load_gm_group(path: &str) -> Result<GMGroupConfig, LoadResourcesError> {
        Self::load_json_data(path).map_err(LoadResourcesError::GMGroup)
    }

    fn load_video_key_map(path: &str) -> Result<HashMap<u32, u64>, DataLoadError> {
        let map = Self::load_json_data::<HashMap<String, String>>(path)?;
        Ok(map
            .into_iter()
            .map(|(k, v)| (k.parse::<u32>().unwrap(), v.parse::<u64>().unwrap()))
            .collect())
    }

    fn load_json_data<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, DataLoadError> {
        Ok(serde_json5::from_reader(&std::fs::File::open(path)?)?)
    }
}

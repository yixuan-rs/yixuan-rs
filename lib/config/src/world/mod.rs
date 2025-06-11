use serde::de::DeserializeOwned;
use std::{collections::HashMap, fs::File, num::ParseIntError};

mod floor;
mod group;
mod object;

pub use floor::*;
pub use group::*;
pub use object::*;

pub struct LevelWorldConfig {
    pub floors: HashMap<u32, ConfigFloor>,
    pub groups: HashMap<u32, ConfigGroup>,
    pub view_objects: HashMap<u32, ConfigViewObject>,
}

#[derive(thiserror::Error, Debug)]
pub enum LevelWorldConfigLoadError {
    #[error("failed to read asset file at {0}, cause: {1}")]
    Read(String, std::io::Error),
    #[error("failed to parse integer key in file: {0}, cause: {1}")]
    ParseInt(String, ParseIntError),
    #[error("JSON config at {0} is invalid, cause: {1}")]
    InvalidConfig(String, serde_json5::Error),
}

impl LevelWorldConfig {
    pub fn load(level_world_path: &str) -> Result<Self, LevelWorldConfigLoadError> {
        Ok(Self {
            floors: Self::load_map(&format!("{level_world_path}/ConfigFloor.json"))?,
            groups: Self::load_map(&format!("{level_world_path}/ConfigGroup.json"))?,
            view_objects: Self::load_map(&format!("{level_world_path}/ConfigViewObject.json"))?,
        })
    }

    fn load_map<T: DeserializeOwned>(
        path: &str,
    ) -> Result<HashMap<u32, T>, LevelWorldConfigLoadError> {
        let mut file = File::open(path)
            .map_err(|err| LevelWorldConfigLoadError::Read(path.to_string(), err))?;

        serde_json5::from_reader::<_, HashMap<String, T>>(&mut file)
            .map_err(|err| LevelWorldConfigLoadError::InvalidConfig(path.to_string(), err))?
            .into_iter()
            .map(|(key, value)| Ok((key.parse::<u32>()?, value)))
            .collect::<Result<HashMap<u32, T>, ParseIntError>>()
            .map_err(|err| LevelWorldConfigLoadError::ParseInt(path.to_string(), err))
    }
}

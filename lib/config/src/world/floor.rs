use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigFloor {
    #[serde(rename = "FloorID")]
    pub floor_id: u32,
    pub floor_version: u64,
    #[serde(rename = "FloorMD5")]
    pub floor_md5: String,
    #[serde(rename = "SceneConfigID")]
    pub scene_config_id: u32,
    #[serde(rename = "GroupIDList")]
    pub group_id_list: Vec<u32>,
}

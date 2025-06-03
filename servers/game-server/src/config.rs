use serde::Deserialize;
use yixuan_logic::debug::GMCmd;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub cluster: ClusterConfig,
    pub resources: ResourceConfig,
}

#[derive(Deserialize)]
pub struct ClusterConfig {
    pub num_clusters: u32,
}

#[derive(Deserialize)]
pub struct ResourceConfig {
    pub fileconfig_directory: String,
    pub level_process_directory: String,
    pub usm_keys_path: String,
    pub first_login_gm_group_list: Vec<String>,
}

#[derive(Deserialize)]
pub struct GachaScheduleConfig {
    #[serde(rename = "schedule")]
    pub gacha_schedule_list: Vec<GachaSchedule>,
}

#[derive(Debug, Deserialize)]
pub struct GachaSchedule {
    pub gacha_id: u32,
    pub gacha_type: u32,
    pub gacha_schedule_id: u32,
    #[serde(default)]
    pub up_item_id_list: Vec<u32>,
    #[serde(default)]
    pub optional_up_item_id_list: Vec<u32>,
    pub begin_time: i64,
    pub end_time: i64,
    pub gacha_materials: Vec<GachaMaterialConfig>,
}

#[derive(Debug, Deserialize)]
pub struct GachaMaterialConfig {
    pub id: u32,
    pub count: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GMGroupConfig {
    pub commands: Vec<GMCmd>,
}

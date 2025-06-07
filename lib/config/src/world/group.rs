use serde::Deserialize;

use crate::EGadgetType;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigGroup {
    #[serde(rename = "GroupID")]
    pub group_id: u32,
    pub meta_data: ConfigGroupMetaData,
    pub members: Vec<ConfigGroupMember>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigGroupMetaData {
    #[serde(rename = "DifficultyTagID")]
    pub difficulty_tag_id: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigGroupMember {
    #[serde(rename = "ConfigID")]
    pub config_id: u32,
    pub gadget_server_metadata: ConfigGadgetServerMetadata,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigGadgetServerMetadata {
    pub gadget_id: u32,
    #[serde(deserialize_with = "deserialize_gadget_type_from_int")]
    pub gadget_type: EGadgetType,
}

fn deserialize_gadget_type_from_int<'de, D>(deserializer: D) -> Result<EGadgetType, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    EGadgetType::try_from(u32::deserialize(deserializer)?).map_err(serde::de::Error::custom)
}

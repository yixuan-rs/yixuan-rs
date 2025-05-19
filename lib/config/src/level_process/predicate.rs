use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "$type")]
pub enum ConfigPredicate {
    #[serde(rename = "Share.CConfigEventByMainCharacter")]
    ConfigEventByMainCharacter(ConfigEventByMainCharacter),
    #[serde(rename = "Share.CConfigEventByHollowVariable")]
    ConfigEventByHollowVariable(ConfigEventByHollowVariable),
    #[serde(rename = "Share.CConfigEventByChoiceServer")]
    ConfigEventByChoiceServer(ConfigEventByChoiceServer),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigEventByMainCharacter {
    pub compare_type: ECompareFunction,
    #[serde(rename = "AvatarID")]
    pub avatar_id: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigEventByHollowVariable {
    pub key: String,
    pub compare_type: ECompareFunction,
    pub count: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigEventByChoiceServer {
    #[serde(rename = "UID")]
    pub uid: u32,
    pub compare_type: ECompareFunction,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum ECompareFunction {
    Greater,
    GreaterEqual,
    Equal,
    Less,
    LessEqual,
    NotEqual,
}

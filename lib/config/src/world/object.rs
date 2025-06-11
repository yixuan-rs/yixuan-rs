use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigViewObject {
    #[serde(rename = "ViewObjectID")]
    pub view_object_id: u32,
    pub traits: Vec<ViewObjectTrait>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ViewObjectTrait {
    DefaultMonsterTrait {
        #[serde(rename = "AIConfigId")]
        ai_config_id: u32,
        #[serde(rename = "MuteAI")]
        mute_ai: bool,
    },
    MonsterAlertTrait {
        #[serde(rename = "UseNewMode")]
        use_new_mode: bool,
    },
    GamePlayTagTrait {
        #[serde(rename = "EntityTagList")]
        entity_tag_list: Vec<GameplayTag>,
    },
    #[serde(untagged)]
    Unknown(HashMap<String, serde::de::IgnoredAny>),
}

#[derive(Debug, Deserialize)]
pub struct GameplayTag {
    #[serde(rename = "tagName")]
    pub tag_name: String,
}

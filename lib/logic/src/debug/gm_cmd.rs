use serde::{Deserialize, Deserializer};
use yixuan_codegen::GMInput;

#[derive(Debug, Clone, GMInput)]
pub enum GMCmd {
    AddItem {
        item_id: u32,
    },
    SetYorozuyaLv {
        level: u32,
    },
    AddAllAvatar,
    AvatarLvUp {
        avatar_id: u32,
        level: u32,
    },
    AvatarSkillUp {
        avatar_id: u32,
        skill_type: u32,
        level: u32,
    },
    AvatarTalentUp {
        avatar_id: u32,
        talent_num: u32,
    },
    AddWeapon {
        weapon_id: u32,
        level: u32,
        star: u32,
        refine_level: u32,
    },
    AddEquip {
        equip_id: u32,
        level: u32,
        star: u32,
        property_params: Vec<u32>,
    },
    SetAvatarSkin {
        avatar_id: u32,
        avatar_skin_id: u32,
    },
    SetControlGuiseAvatar {
        avatar_id: u32,
    },
    UnlockHollowQuest {
        quest_id: u32,
    },
    ClearMainCityQuestCollection,
    UnlockBossBattleQuest {
        quest_id: u32,
    },
    UnlockDoubleEliteQuest {
        quest_id: u32,
    },
    UnlockMonsterCardLevel {
        level: u32,
    },
    Jump {
        section_id: u32,
        transform_id: String,
    },
    ResetSectionState,
}

#[derive(thiserror::Error, Debug)]
pub enum GMInputParseError {
    #[error("GM command {0} doesn't exist")]
    UnknownCommand(String),
    #[error("missing non-optional argument: {0}")]
    MissingArgument(&'static str),
    #[error("failed to parse argument of type '{1}' (name: {0})")]
    InvalidArgumentFormat(&'static str, &'static str),
}

pub trait GMInput: Sized {
    fn from_str(input: &str) -> Result<Self, GMInputParseError>;
}

impl<'de> Deserialize<'de> for GMCmd {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <String>::deserialize(deserializer)?;
        GMCmd::from_str(&s).map_err(serde::de::Error::custom)
    }
}

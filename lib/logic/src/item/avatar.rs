use std::collections::HashMap;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use yixuan_proto::AvatarShowWeaponType;

#[derive(Debug, Clone)]
pub struct AvatarItem {
    pub id: u32,
    pub level: u32,
    pub exp: u32,
    pub rank: u32,
    pub unlocked_talent_num: u32,
    pub talent_switch: [bool; 6],
    pub passive_skill_level: u32,
    pub skill_level_map: HashMap<EAvatarSkillType, u32>,
    pub weapon_uid: u32,
    pub show_weapon_type: i32,
    pub dressed_equip_map: HashMap<u32, u32>,
    pub first_get_time: i64,
    pub taken_rank_up_reward_list: Vec<u32>,
    pub avatar_skin_id: u32,
    pub is_favorite: bool,
    pub is_awake_available: bool,
    pub awake_id: u32,
}

#[derive(
    Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, IntoPrimitive, TryFromPrimitive,
)]
#[repr(u32)]
pub enum EAvatarSkillType {
    CommonAttack = 0,
    SpecialAttack = 1,
    Evade = 2,
    CooperateSkill = 3,
    UniqueSkill = 4,
    CoreSkill = 5,
    AssistSkill = 6,
    EnumCount = 7,
}

impl AvatarItem {
    pub const MAX_TALENT_NUM: usize = 6;

    pub fn get_skill_level(&self, ty: EAvatarSkillType) -> u32 {
        self.skill_level_map.get(&ty).copied().unwrap_or(0)
    }

    pub fn as_client_proto(&self) -> yixuan_proto::AvatarInfo {
        yixuan_proto::AvatarInfo {
            id: self.id,
            level: self.level,
            exp: self.exp,
            rank: self.rank,
            unlocked_talent_num: self.unlocked_talent_num,
            cur_weapon_uid: self.weapon_uid,
            show_weapon_type: self.show_weapon_type,
            passive_skill_level: self.passive_skill_level,
            skill_type_level: self
                .skill_level_map
                .iter()
                .map(|(&skill_type, &level)| yixuan_proto::AvatarSkillLevel {
                    skill_type: skill_type.into(),
                    level,
                })
                .collect(),
            dressed_equip_list: self
                .dressed_equip_map
                .iter()
                .map(|(&equip_uid, &index)| yixuan_proto::DressedEquip { equip_uid, index })
                .collect(),
            avatar_skin_id: self.avatar_skin_id,
            first_get_time: self.first_get_time,
            talent_switch_list: self.talent_switch.to_vec(),
            taken_rank_up_reward_list: self.taken_rank_up_reward_list.clone(),
            is_favorite: self.is_favorite,
            is_awake_available: self.is_awake_available,
            awake_id: self.awake_id,
        }
    }
}

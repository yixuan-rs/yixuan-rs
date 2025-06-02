use std::collections::HashMap;

use crate::{
    item::{AvatarItem, EquipItem, WeaponItem},
    listener::{NotifyListener, NotifyListenerExt},
};

mod avatar_unit;
mod enums;
mod property;

pub use avatar_unit::AvatarUnit;
use common::time_util;
use config::TemplateCollection;
pub use enums::*;
use vivian_proto::{BigBossInfo, DungeonQuestFinishedScNotify, common::LogBattleStatistics};

#[derive(Default, Clone)]
pub struct Dungeon {
    pub quest_id: u32,
    pub quest_type: u32,
    pub begin_time: i64,
    pub equipment: DungeonEquipment,
    pub avatar_units: Vec<AvatarUnit>,
    pub inner_quest_id_list: Vec<u32>,
    pub avatar_contribution: HashMap<u32, u32>,
    pub battle_statistic: LogBattleStatistics,
    pub pending_quest_finish_notifies: HashMap<u32, DungeonQuestFinishedScNotify>,
    pub big_boss_info: Option<BigBossInfo>,
}

#[derive(Default, Clone)]
pub struct DungeonEquipment {
    pub avatars: HashMap<u32, AvatarItem>,
    pub weapons: HashMap<u32, WeaponItem>,
    pub equips: HashMap<u32, EquipItem>,
}

impl Dungeon {
    pub fn new(quest_id: u32, quest_type: u32) -> Self {
        Self {
            quest_id,
            quest_type,
            begin_time: time_util::unix_timestamp_seconds(),
            equipment: DungeonEquipment::default(),
            avatar_units: Vec::new(),
            inner_quest_id_list: Vec::new(),
            avatar_contribution: HashMap::new(),
            battle_statistic: LogBattleStatistics::default(),
            pending_quest_finish_notifies: HashMap::new(),
            big_boss_info: None,
        }
    }

    pub fn finish_main_dungeon_quest(&mut self, result: i32) {
        self.pending_quest_finish_notifies.insert(
            self.quest_id,
            DungeonQuestFinishedScNotify {
                quest_id: self.quest_id,
                result: if result == 1 { 1 } else { 0 },
                rank: if result == 1 { 4 } else { 0 },
                statistics: HashMap::from([(
                    EQuestStatisticsType::CostTime.into(),
                    (time_util::unix_timestamp_seconds() - self.begin_time) as u64,
                )]),
                avatar_contribution_list: self
                    .avatar_contribution
                    .iter()
                    .map(
                        |(&avatar_id, &contribution)| vivian_proto::AvatarContributionInfo {
                            avatar_id,
                            contribution,
                        },
                    )
                    .collect(),
                ..Default::default()
            },
        );
    }

    pub fn flush_dungeon_quest_notifies(&mut self, listener: &mut dyn NotifyListener) {
        std::mem::take(&mut self.pending_quest_finish_notifies)
            .into_iter()
            .for_each(|(_, notify)| listener.add(notify));
    }

    pub fn update_statistic(&mut self, statistic: LogBattleStatistics) {
        statistic.avatar_list.iter().for_each(|avatar| {
            *self
                .avatar_contribution
                .entry(avatar.avatar_id as u32)
                .or_default() = avatar.damage as u32; // TODO: calculate contribution points properly
        });

        self.battle_statistic = statistic;
    }

    pub fn add_avatar(&mut self, avatar_id: u32, templates: &TemplateCollection) {
        self.avatar_units
            .push(AvatarUnit::new(avatar_id, templates, &self.equipment));
    }

    pub fn as_client_proto(&self) -> vivian_proto::DungeonInfo {
        vivian_proto::DungeonInfo {
            quest_id: self.quest_id,
            quest_type: self.quest_type,
            dungeon_equip_info: Some(vivian_proto::DungeonEquipInfo {
                avatar_list: self
                    .equipment
                    .avatars
                    .values()
                    .map(AvatarItem::as_client_proto)
                    .collect(),
                weapon_list: self
                    .equipment
                    .weapons
                    .iter()
                    .map(|(&uid, weapon)| weapon.as_client_proto(uid))
                    .collect(),
                equip_list: self
                    .equipment
                    .equips
                    .iter()
                    .map(|(&uid, equip)| equip.as_client_proto(uid))
                    .collect(),
                buddy: Some(vivian_proto::BuddyInfo::default()),
                ..Default::default()
            }),
            avatar_list: self
                .avatar_units
                .iter()
                .map(|unit| vivian_proto::AvatarUnitInfo {
                    avatar_id: unit.avatar_id,
                    properties: unit
                        .properties
                        .iter()
                        .map(|(&ty, &value)| (ty.into(), value))
                        .collect(),
                })
                .collect(),
            dungeon_quest_info: Some(vivian_proto::DungeonQuestInfo {
                inner_quest_id_list: self.inner_quest_id_list.clone(),
            }),
            dungeon_statistics: Some(vivian_proto::DungeonStatistics::default()),
            begin_time: self.begin_time,
            big_boss_info: self.big_boss_info.clone(),
        }
    }
}

impl DungeonEquipment {
    pub fn get_avatar_weapon(&self, avatar_id: u32) -> Option<&WeaponItem> {
        self.avatars
            .get(&avatar_id)
            .and_then(|avatar| {
                (avatar.weapon_uid != 0).then(|| self.weapons.get(&avatar.weapon_uid))
            })
            .flatten()
    }

    pub fn get_avatar_equips(&self, avatar_id: u32) -> Option<Vec<&EquipItem>> {
        self.avatars.get(&avatar_id).map(|avatar| {
            self.equips
                .iter()
                .filter(|(uid, _)| avatar.dressed_equip_map.contains_key(uid))
                .map(|(_, equip)| equip)
                .collect()
        })
    }
}

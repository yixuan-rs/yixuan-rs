use rand::RngCore;
use vivian_codegen::Model;
use vivian_logic::item::{EItemRarity, EItemType};
use vivian_proto::{
    GachaDisplayData,
    server_only::{GachaData, GachaStatisticsInfo},
};

use crate::{
    config::{GachaSchedule, GachaScheduleConfig},
    logic::{
        player::Model,
        property::{Property, PropertyHashMap},
        sync::PlayerSyncComponent,
    },
    resources::NapResources,
    util::GachaRandom,
};

use super::Saveable;

pub struct GachaStatistics {
    pub remain_up_item_times: u32,
    pub remain_optional_up_item_times: u32,
    pub newbie_remain_up_item_times: u32,
    pub up_item_state: bool,
    pub optional_up_item_state: bool,
}

#[derive(Model)]
pub struct GachaModel {
    pub gacha_stats: PropertyHashMap<u32, GachaStatistics>,
    pub gacha_random: GachaRandom,
}

impl GachaModel {
    const COMMON_GACHA_ID: u32 = 1001;
    const COMMON_AVATAR_ID: &[u32] = &[1181, 1211, 1101, 1021, 1041, 1141];
    const UP_ITEM_TIMES: u32 = 90;
    const OPTIONAL_UP_ITEM_TIMES: u32 = 10;
    const NEWBIE_UP_ITEM_TIMES: u32 = 50;

    pub fn on_first_login(&mut self) {
        self.gacha_random = GachaRandom::from(rand::thread_rng().next_u32());
    }

    pub fn do_gacha(&mut self, schedule: &GachaSchedule, res: &NapResources) -> u32 {
        if self.next_is_special_rare_item(schedule.gacha_id) {
            self.next_special_rare_item(schedule, res)
        } else if self.next_is_rare_item(schedule.gacha_id) {
            self.next_rare_item(schedule, res)
        } else {
            let candidates = res
                .templates
                .item_template_tb()
                .filter(|tmpl| {
                    tmpl.class() == EItemType::Weapon.into()
                        && tmpl.rarity() == EItemRarity::R.into()
                        && !schedule.optional_up_item_id_list.contains(&tmpl.id())
                })
                .map(|tmpl| tmpl.id())
                .collect::<Vec<_>>();

            candidates[self.gacha_random.rand(candidates.len() as u32) as usize]
        }
    }

    fn next_is_special_rare_item(&mut self, gacha_id: u32) -> bool {
        const DEFAULT_RATE: u32 = 100;
        const RATE_INCREMENT_THRESHOLD: u32 = 15;
        const RATE_INCREMENT: u32 = 750;

        let statistics = self.get_or_init_statistics(gacha_id);

        let rate = DEFAULT_RATE
            + RATE_INCREMENT_THRESHOLD.saturating_sub(statistics.remain_up_item_times)
                * RATE_INCREMENT;

        statistics.remain_up_item_times -= 1;

        let drop_by_newbie = if statistics.newbie_remain_up_item_times > 0 {
            statistics.newbie_remain_up_item_times -= 1;
            statistics.newbie_remain_up_item_times == 0
        } else {
            false
        };

        if drop_by_newbie || rate > self.gacha_random.rand(11350) {
            self.get_or_init_statistics(gacha_id).remain_up_item_times = Self::UP_ITEM_TIMES;
            true
        } else {
            false
        }
    }

    fn next_is_rare_item(&mut self, gacha_id: u32) -> bool {
        const DEFAULT_RATE: u32 = 200;
        const RATE_INCREMENT_THRESHOLD: u32 = 5;
        const RATE_INCREMENT: u32 = 450;

        let statistics = self.get_or_init_statistics(gacha_id);

        let rate = DEFAULT_RATE
            + RATE_INCREMENT_THRESHOLD.saturating_sub(statistics.remain_optional_up_item_times)
                * RATE_INCREMENT;

        statistics.remain_optional_up_item_times -= 1;

        if rate > self.gacha_random.rand(2000) {
            self.get_or_init_statistics(gacha_id)
                .remain_optional_up_item_times = Self::OPTIONAL_UP_ITEM_TIMES;

            true
        } else {
            false
        }
    }

    fn next_rare_item(&mut self, schedule: &GachaSchedule, res: &NapResources) -> u32 {
        if !schedule.optional_up_item_id_list.is_empty()
            && (self
                .get_or_init_statistics(schedule.gacha_id)
                .optional_up_item_state
                || self.gacha_random.rand(10_000) > 5_000)
        {
            self.get_or_init_statistics(schedule.gacha_id)
                .optional_up_item_state = false;

            let cnt = schedule.optional_up_item_id_list.len() as u32;
            schedule.optional_up_item_id_list[self.gacha_random.rand(cnt) as usize]
        } else {
            let candidates = res
                .templates
                .item_template_tb()
                .filter(|tmpl| {
                    (tmpl.class() == 5 || tmpl.class() == 3)
                        && tmpl.rarity() == 3
                        && !schedule.optional_up_item_id_list.contains(&tmpl.id())
                })
                .map(|tmpl| tmpl.id())
                .collect::<Vec<_>>();

            candidates[self.gacha_random.rand(candidates.len() as u32) as usize]
        }
    }

    fn next_special_rare_item(&mut self, schedule: &GachaSchedule, res: &NapResources) -> u32 {
        if !schedule.up_item_id_list.is_empty()
            && (self.get_or_init_statistics(schedule.gacha_id).up_item_state
                || self.gacha_random.rand(100_000) > 50_000)
        {
            self.get_or_init_statistics(schedule.gacha_id).up_item_state = false;

            let cnt = schedule.up_item_id_list.len() as u32;
            schedule.up_item_id_list[self.gacha_random.rand(cnt) as usize]
        } else {
            self.get_or_init_statistics(schedule.gacha_id).up_item_state = true;

            if self.gacha_random.rand(100_000) > 25_000 {
                Self::COMMON_AVATAR_ID
                    [self.gacha_random.rand(Self::COMMON_AVATAR_ID.len() as u32) as usize]
            } else {
                let candidates = res
                    .templates
                    .item_template_tb()
                    .filter(|tmpl| {
                        tmpl.class() == 5
                            && tmpl.rarity() == 4
                            && !schedule.up_item_id_list.contains(&tmpl.id())
                    })
                    .map(|tmpl| tmpl.id())
                    .collect::<Vec<_>>();

                candidates[self.gacha_random.rand(candidates.len() as u32) as usize]
            }
        }
    }

    pub fn display_data(&self, schedule_config: &GachaScheduleConfig) -> GachaDisplayData {
        let gacha_list = schedule_config
            .gacha_schedule_list
            .iter()
            .filter_map(|schedule| Some((schedule, self.gacha_stats.get(&schedule.gacha_id)?)))
            .map(|(schedule, statistics)| vivian_proto::Gacha {
                gacha_type: schedule.gacha_type,
                gacha_id: schedule.gacha_id,
                gacha_schedule_id: schedule.gacha_schedule_id,
                begin_time: schedule.begin_time,
                end_time: schedule.end_time,
                up_item_id_list: schedule.up_item_id_list.clone(),
                optional_up_item_id_list: schedule.optional_up_item_id_list.clone(),
                remain_up_item_times: statistics.remain_up_item_times,
                remain_optional_up_item_times: statistics.remain_optional_up_item_times,
                newbie_remain_up_item_times: statistics.newbie_remain_up_item_times,
                gacha_info_webview: String::new(),
                gacha_log_webview: String::new(),
                newbie_avatar_id_list: (schedule.gacha_id == Self::COMMON_GACHA_ID)
                    .then(|| Self::COMMON_AVATAR_ID.to_vec())
                    .unwrap_or_default(),
                gacha_material_list: schedule
                    .gacha_materials
                    .iter()
                    .map(|material| vivian_proto::GachaMaterial {
                        material_id: material.id,
                        num: material.count,
                    })
                    .collect(),
            })
            .collect();

        GachaDisplayData {
            gacha_random: self.gacha_random.seed(),
            gacha_info: Some(vivian_proto::GachaInfo { gacha_list }),
        }
    }

    pub fn get_or_init_statistics(&mut self, gacha_id: u32) -> &mut GachaStatistics {
        self.ensure_statistics(gacha_id);
        self.gacha_stats.get_mut(&gacha_id).unwrap()
    }

    pub fn ensure_statistics(&mut self, gacha_id: u32) {
        if !self.gacha_stats.contains_key(&gacha_id) {
            self.gacha_stats.insert(
                gacha_id,
                GachaStatistics {
                    remain_up_item_times: Self::UP_ITEM_TIMES,
                    remain_optional_up_item_times: Self::OPTIONAL_UP_ITEM_TIMES,
                    newbie_remain_up_item_times: if gacha_id == Self::COMMON_GACHA_ID {
                        Self::NEWBIE_UP_ITEM_TIMES
                    } else {
                        0
                    },
                    up_item_state: false,
                    optional_up_item_state: false,
                },
            );
        }
    }

    pub fn load_from_pb(pb: GachaData) -> Self {
        Self {
            gacha_stats: pb
                .gacha_statistics_list
                .into_iter()
                .map(|info| {
                    (
                        info.gacha_id,
                        GachaStatistics {
                            remain_up_item_times: info.remain_up_item_times,
                            remain_optional_up_item_times: info.remain_optional_up_item_times,
                            newbie_remain_up_item_times: info.newbie_remain_up_item_times,
                            up_item_state: info.up_item_state,
                            optional_up_item_state: info.optional_up_item_state,
                        },
                    )
                })
                .collect(),
            gacha_random: pb.gacha_random.into(),
        }
    }
}

impl Saveable for GachaModel {
    fn save_to_pb(&self, root: &mut vivian_proto::server_only::PlayerData) {
        root.gacha = Some(GachaData {
            gacha_statistics_list: self
                .gacha_stats
                .iter()
                .map(|(&gacha_id, statistics)| GachaStatisticsInfo {
                    gacha_id,
                    remain_up_item_times: statistics.remain_up_item_times,
                    remain_optional_up_item_times: statistics.remain_optional_up_item_times,
                    newbie_remain_up_item_times: statistics.newbie_remain_up_item_times,
                    up_item_state: statistics.up_item_state,
                    optional_up_item_state: statistics.optional_up_item_state,
                })
                .collect(),
            gacha_random: self.gacha_random.seed(),
        });
    }
}

impl PlayerSyncComponent for GachaModel {
    fn supports_player_sync(&self) -> bool {
        false
    }

    fn add_changes_to_player_sync_notify(&self, _: &mut vivian_proto::PlayerSyncScNotify) {}
}

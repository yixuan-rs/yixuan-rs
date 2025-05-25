use crate::property::GachaRandom;

use super::*;
use property::{Property, PropertyHashMap};

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
    pub const UP_ITEM_TIMES: u32 = 90;
    pub const OPTIONAL_UP_ITEM_TIMES: u32 = 10;
    pub const NEWBIE_UP_ITEM_TIMES: u32 = 50;
    pub const COMMON_GACHA_ID: u32 = 1001;
    pub const COMMON_AVATAR_ID: &[u32] = &[1181, 1211, 1101, 1021, 1041, 1141];

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

use yixuan_logic::item::{EItemRarity, EItemType};
use yixuan_models::GachaModel;
use yixuan_proto::GachaDisplayData;

use crate::{
    config::{GachaSchedule, GachaScheduleConfig},
    player::Player,
    resources::NapResources,
};

pub fn do_gacha(player: &mut Player, schedule: &GachaSchedule, res: &NapResources) -> u32 {
    if next_is_special_rare_item(&mut player.gacha_model, schedule.gacha_id) {
        next_special_rare_item(&mut player.gacha_model, schedule, res)
    } else if next_is_rare_item(&mut player.gacha_model, schedule.gacha_id) {
        next_rare_item(&mut player.gacha_model, schedule, res)
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

        candidates[player
            .gacha_model
            .gacha_random
            .rand(candidates.len() as u32) as usize]
    }
}

fn next_is_special_rare_item(model: &mut GachaModel, gacha_id: u32) -> bool {
    const DEFAULT_RATE: u32 = 100;
    const RATE_INCREMENT_THRESHOLD: u32 = 15;
    const RATE_INCREMENT: u32 = 750;

    let statistics = model.get_or_init_statistics(gacha_id);

    let rate = DEFAULT_RATE
        + RATE_INCREMENT_THRESHOLD.saturating_sub(statistics.remain_up_item_times) * RATE_INCREMENT;

    statistics.remain_up_item_times -= 1;

    let drop_by_newbie = if statistics.newbie_remain_up_item_times > 0 {
        statistics.newbie_remain_up_item_times -= 1;
        statistics.newbie_remain_up_item_times == 0
    } else {
        false
    };

    if drop_by_newbie || rate > model.gacha_random.rand(11350) {
        model.get_or_init_statistics(gacha_id).remain_up_item_times = GachaModel::UP_ITEM_TIMES;
        true
    } else {
        false
    }
}

fn next_is_rare_item(model: &mut GachaModel, gacha_id: u32) -> bool {
    const DEFAULT_RATE: u32 = 200;
    const RATE_INCREMENT_THRESHOLD: u32 = 5;
    const RATE_INCREMENT: u32 = 450;

    let statistics = model.get_or_init_statistics(gacha_id);

    let rate = DEFAULT_RATE
        + RATE_INCREMENT_THRESHOLD.saturating_sub(statistics.remain_optional_up_item_times)
            * RATE_INCREMENT;

    statistics.remain_optional_up_item_times -= 1;

    if rate > model.gacha_random.rand(2000) {
        model
            .get_or_init_statistics(gacha_id)
            .remain_optional_up_item_times = GachaModel::OPTIONAL_UP_ITEM_TIMES;

        true
    } else {
        false
    }
}

fn next_rare_item(model: &mut GachaModel, schedule: &GachaSchedule, res: &NapResources) -> u32 {
    if !schedule.optional_up_item_id_list.is_empty()
        && (model
            .get_or_init_statistics(schedule.gacha_id)
            .optional_up_item_state
            || model.gacha_random.rand(10_000) > 5_000)
    {
        model
            .get_or_init_statistics(schedule.gacha_id)
            .optional_up_item_state = false;

        let cnt = schedule.optional_up_item_id_list.len() as u32;
        schedule.optional_up_item_id_list[model.gacha_random.rand(cnt) as usize]
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

        candidates[model.gacha_random.rand(candidates.len() as u32) as usize]
    }
}

fn next_special_rare_item(
    model: &mut GachaModel,
    schedule: &GachaSchedule,
    res: &NapResources,
) -> u32 {
    if !schedule.up_item_id_list.is_empty()
        && (model
            .get_or_init_statistics(schedule.gacha_id)
            .up_item_state
            || model.gacha_random.rand(100_000) > 50_000)
    {
        model
            .get_or_init_statistics(schedule.gacha_id)
            .up_item_state = false;

        let cnt = schedule.up_item_id_list.len() as u32;
        schedule.up_item_id_list[model.gacha_random.rand(cnt) as usize]
    } else {
        model
            .get_or_init_statistics(schedule.gacha_id)
            .up_item_state = true;

        if model.gacha_random.rand(100_000) > 25_000 {
            GachaModel::COMMON_AVATAR_ID[model
                .gacha_random
                .rand(GachaModel::COMMON_AVATAR_ID.len() as u32)
                as usize]
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

            candidates[model.gacha_random.rand(candidates.len() as u32) as usize]
        }
    }
}

pub fn display_data(player: &Player, schedule_config: &GachaScheduleConfig) -> GachaDisplayData {
    let gacha_list = schedule_config
        .gacha_schedule_list
        .iter()
        .filter_map(|schedule| {
            Some((
                schedule,
                player.gacha_model.gacha_stats.get(&schedule.gacha_id)?,
            ))
        })
        .map(|(schedule, statistics)| yixuan_proto::Gacha {
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
            newbie_avatar_id_list: (schedule.gacha_id == GachaModel::COMMON_GACHA_ID)
                .then(|| GachaModel::COMMON_AVATAR_ID.to_vec())
                .unwrap_or_default(),
            gacha_material_list: schedule
                .gacha_materials
                .iter()
                .map(|material| yixuan_proto::GachaMaterial {
                    material_id: material.id,
                    num: material.count,
                })
                .collect(),
        })
        .collect();

    GachaDisplayData {
        gacha_random: player.gacha_model.gacha_random.seed(),
        gacha_info: Some(yixuan_proto::GachaInfo { gacha_list }),
    }
}

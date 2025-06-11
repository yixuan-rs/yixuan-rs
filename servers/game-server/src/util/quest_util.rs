use std::collections::HashMap;

use common::time_util;
use tracing::warn;
use yixuan_logic::dungeon::EQuestType;
use yixuan_models::{
    DoubleEliteProgress, Hollow, HollowModel, MainCityQuestExt, Quest, SpecialQuestExt, TrackQuest,
};

use crate::{player::Player, resources::NapResources};

pub fn add_hollow_quest(player: &mut Player, id: u32) {
    let collection = player
        .quest_model
        .get_or_insert_collection(EQuestType::Hollow);

    collection.quests.insert(
        id,
        Quest {
            id,
            state: 0,
            unlock_time: time_util::unix_timestamp_seconds(),
            progress: 0,
            in_progress_time: 0,
            finish_condition_progress: HashMap::new(),
            main_city_ext: None,
            special_ext: None,
        },
    );

    player.quest_model.new_hollow_quests.insert(id);
    add_hollow_to_model(&mut player.hollow_model, id, player.resources);
    add_hollow_challenges(player, id);
}

fn add_hollow_to_model(model: &mut HollowModel, id: u32, res: &NapResources) {
    if !model.hollows.contains_key(&id) {
        let Some(template) = res
            .templates
            .hollow_quest_template_tb()
            .find(|tmpl| tmpl.id() == id)
        else {
            warn!("add_hollow_to_model: quest with id {id} doesn't exist!");
            return;
        };

        let hollow_id = template.hollow_id();

        if !model.unlocked_hollows.contains(&hollow_id) {
            model.unlocked_hollows.insert(hollow_id);
            model.new_unlocked_hollows.insert(hollow_id);

            if let Some(template) = res
                .templates
                .hollow_config_template_tb()
                .find(|tmpl| tmpl.id() == hollow_id)
            {
                model.hollow_groups.insert(template.hollow_group());
                model.new_hollow_groups.insert(template.hollow_group());
            }
        }

        model.hollows.insert(
            id,
            Hollow {
                hollow_quest_id: id,
                finished: false,
                acquired_hollow_challenge_reward: 0,
            },
        );
    }
}

fn add_hollow_challenges(player: &mut Player, hollow_quest_id: u32) {
    let collection = player
        .quest_model
        .get_or_insert_collection(EQuestType::HollowChallenge);

    player
        .resources
        .templates
        .hollow_challenge_template_tb()
        .filter(|c| c.hollow_quest_id() == hollow_quest_id)
        .for_each(|c| {
            collection.quests.insert(
                c.id(),
                Quest {
                    id: c.id(),
                    state: 0,
                    unlock_time: time_util::unix_timestamp_seconds(),
                    in_progress_time: 0,
                    progress: 0,
                    finish_condition_progress: HashMap::new(),
                    main_city_ext: None,
                    special_ext: None,
                },
            );
        })
}

pub fn add_special_quest(player: &mut Player, id: u32, main: bool) {
    let special_quest_template = player
        .resources
        .templates
        .special_quest_template_tb()
        .find(|tmpl| tmpl.id() == id)
        .unwrap();

    let collection = player
        .quest_model
        .get_or_insert_collection(EQuestType::MainCity);

    if let Some(first_quest_id) = special_quest_template
        .quest_lists()
        .and_then(|list| list.iter().next())
    {
        if main {
            collection.track_quest = Some(TrackQuest {
                cur_main_quest_id: id,
                cur_track_special_quest_id: id,
                cur_track_quest_id: first_quest_id,
            });

            collection.quests.insert(
                id,
                Quest {
                    id,
                    state: 1,
                    unlock_time: time_util::unix_timestamp_seconds(),
                    in_progress_time: 0,
                    progress: 0,
                    finish_condition_progress: HashMap::new(),
                    main_city_ext: None,
                    special_ext: Some(SpecialQuestExt {
                        prev_quest_id: first_quest_id,
                        cur_quest_id: first_quest_id,
                    }),
                },
            );
        }

        add_main_city_quest(player, first_quest_id);
    }
}

pub fn add_main_city_quest(player: &mut Player, id: u32) {
    let main_city_quest_template = player
        .resources
        .templates
        .main_city_quest_template_tb()
        .find(|tmpl| tmpl.id() == id)
        .unwrap();

    let collection = player
        .quest_model
        .get_or_insert_collection(EQuestType::MainCity);

    collection.quests.insert(
        id,
        Quest {
            id,
            state: 1,
            unlock_time: time_util::unix_timestamp_seconds(),
            in_progress_time: 0,
            progress: 0,
            finish_condition_progress: HashMap::new(),
            main_city_ext: Some(MainCityQuestExt {
                track_npc_id: main_city_quest_template
                    .npcs()
                    .unwrap_or_default()
                    .iter()
                    .collect(),
            }),
            special_ext: None,
        },
    );
}

pub fn add_boss_battle_quest(player: &mut Player, id: u32) {
    let collection = player
        .quest_model
        .get_or_insert_collection(EQuestType::BossBattle);

    collection.quests.insert(
        id,
        Quest {
            id,
            state: 0,
            unlock_time: time_util::unix_timestamp_seconds(),
            progress: 0,
            in_progress_time: 0,
            finish_condition_progress: HashMap::new(),
            main_city_ext: None,
            special_ext: None,
        },
    );
}

pub fn add_double_elite_quest(player: &mut Player, id: u32) {
    let collection = player
        .quest_model
        .get_or_insert_collection(EQuestType::DoubleElite);

    collection.quests.insert(
        id,
        Quest {
            id,
            state: 0,
            unlock_time: time_util::unix_timestamp_seconds(),
            progress: 0,
            in_progress_time: 0,
            finish_condition_progress: HashMap::new(),
            main_city_ext: None,
            special_ext: None,
        },
    );

    player
        .quest_model
        .battle_data
        .activity
        .double_elite
        .progress
        .insert(id, DoubleEliteProgress::default());
}

pub fn finish_hollow_challenge(player: &mut Player, id: u32) {
    if let Some(collection) = player
        .quest_model
        .quest_collections
        .get(&EQuestType::HollowChallenge)
    {
        if collection.finished_quests.contains(&id) {
            return;
        }
    }

    let collection = player
        .quest_model
        .get_or_insert_collection(EQuestType::HollowChallenge);

    collection.quests.remove(&id);
    collection.finished_quests.insert(id);
}

pub fn finish_main_city_quest(player: &mut Player, id: u32) -> Vec<u32> {
    let collection = player
        .quest_model
        .get_or_insert_collection(EQuestType::MainCity);

    let Some(_quest) = collection.quests.remove(&id) else {
        return Vec::new();
    };

    collection.finished_quests.insert(id);

    let mut newly_added_quests = Vec::new();
    for next_quest_template in player
        .resources
        .templates
        .quest_config_template_tb()
        .filter(|quest| {
            quest.quest_type() == EQuestType::MainCity.into()
                && !collection.finished_quests.contains(&quest.quest_id())
                && !quest.preorder_quest_ids().unwrap_or_default().is_empty()
                && quest
                    .preorder_quest_ids()
                    .unwrap_or_default()
                    .iter()
                    .all(|id| collection.finished_quests.contains(&id))
        })
        .collect::<Vec<_>>()
    {
        newly_added_quests.push(next_quest_template.quest_id());
        add_main_city_quest(player, next_quest_template.quest_id());

        if let Some(track_quest) = player
            .quest_model
            .get_or_insert_collection(EQuestType::MainCity)
            .track_quest
            .as_mut()
        {
            if track_quest.cur_track_quest_id == id {
                track_quest.cur_track_quest_id = next_quest_template.quest_id();
            }
        }

        if let Some((_, special_quest)) = player
            .quest_model
            .get_or_insert_collection(EQuestType::MainCity)
            .quests
            .iter_mut()
            .find(|(_, quest)| {
                quest
                    .special_ext
                    .as_ref()
                    .map(|ext| ext.cur_quest_id == id)
                    .unwrap_or(false)
            })
        {
            let ext = special_quest.special_ext.as_mut().unwrap();

            ext.prev_quest_id = ext.cur_quest_id;
            ext.cur_quest_id = next_quest_template.quest_id();
        }
    }

    let main_city_quest_template = player
        .resources
        .templates
        .main_city_quest_template_tb()
        .find(|tmpl| tmpl.id() == id)
        .unwrap();

    if let Ok(hollow_quest_id) = main_city_quest_template
        .action_arg_1()
        .unwrap()
        .parse::<u32>()
    {
        add_hollow_quest(player, hollow_quest_id);
    }

    newly_added_quests
}

pub fn finish_hollow_quest(player: &mut Player, id: u32) {
    let collection = player
        .quest_model
        .get_or_insert_collection(EQuestType::Hollow);

    if collection.quests.remove(&id).is_some() {
        collection.finished_quests.insert(id);
    }
}

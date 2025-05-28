use std::{
    collections::HashMap,
    time::{Duration, UNIX_EPOCH},
};

use common::time_util;
use config::{BoundConditions, Condition, EQuestState, GraphReference, HollowQuestType};
use rand::RngCore;
use vivian_codegen::ModelManager;
use vivian_logic::{
    GameState, LogicResources,
    dungeon::{DungeonEquipment, EQuestType},
    event::Event,
    fight::GameFightState,
    hall::{GameHallState, HallGameParameters},
    hollow::GameHollowState,
    listener::LogicEventListener,
    long_fight::GameLongFightState,
    scene::ELocalPlayType,
};
use vivian_proto::{PlayerSyncScNotify, server_only::PlayerData};

use tracing::{error, info, warn};
use vivian_models::{property::GachaRandom, *};

use crate::{
    resources::NapResources,
    util::{avatar_util, basic_util, gm_util, item_util, map_util, misc_util, quest_util},
};

use super::sync::{DataSyncHelper, LoginDataSyncComponent, PlayerSyncComponent};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LoadingState {
    Login,
    BasicDataSync,
    ExtendDataSync,
    EnterWorldCsReq,
    EnterWorldScRsp,
}

#[derive(ModelManager)]
pub struct Player {
    pub uid: u32,
    pub loading_state: LoadingState,
    pub sync_helper: DataSyncHelper,
    pub resources: &'static NapResources,
    is_back_scene_changed: bool,
    #[model]
    pub basic_model: PlayerBasicModel,
    #[model]
    pub avatar_model: AvatarModel,
    #[model]
    pub item_model: ItemModel,
    #[model]
    pub quest_model: QuestModel,
    #[model]
    pub archive_model: ArchiveModel,
    #[model]
    pub hollow_model: HollowModel,
    #[model]
    pub abyss_model: AbyssModel,
    #[model]
    pub buddy_model: BuddyModel,
    #[model]
    pub misc_model: MiscModel,
    #[model]
    pub main_city_model: MainCityModel,
    #[model]
    pub scene_model: SceneModel,
    #[model]
    pub gacha_model: GachaModel,
    #[model]
    pub map_model: MapModel,
}

#[derive(Debug, Clone, Copy)]
pub enum AddItemSource {
    Custom,
    OnceReward,
    Gacha,
    #[expect(dead_code)]
    Mail,
}

impl Player {
    pub fn load_from_pb(uid: u32, pb: PlayerData, res: &'static NapResources) -> Self {
        Self {
            uid,
            loading_state: LoadingState::Login,
            resources: res,
            is_back_scene_changed: false,
            sync_helper: DataSyncHelper::default(),
            basic_model: PlayerBasicModel::load_from_pb(pb.basic.unwrap()),
            avatar_model: AvatarModel::load_from_pb(pb.avatar.unwrap()),
            item_model: ItemModel::load_from_pb(pb.item.unwrap()),
            quest_model: QuestModel::load_from_pb(pb.quest.unwrap()),
            archive_model: ArchiveModel::load_from_pb(pb.archive.unwrap()),
            hollow_model: HollowModel::load_from_pb(pb.hollow.unwrap()),
            abyss_model: AbyssModel::load_from_pb(pb.abyss.unwrap()),
            buddy_model: BuddyModel::load_from_pb(pb.buddy.unwrap()),
            misc_model: MiscModel::load_from_pb(pb.misc.unwrap()),
            main_city_model: MainCityModel::load_from_pb(pb.main_city.unwrap()),
            scene_model: SceneModel::load_from_pb(pb.scene.unwrap()),
            gacha_model: GachaModel::load_from_pb(pb.gacha.unwrap()),
            map_model: MapModel::load_from_pb(pb.map.unwrap()),
        }
    }

    pub fn on_login(&mut self) {
        if self.avatar_model.avatar_map.is_empty() {
            self.on_first_login();
        }

        self.begin_sync_basic_data();
    }

    pub fn on_first_login(&mut self) {
        // TODO: later we'll have to initiate a beginner procedure scene,
        // and it won't be needed to set avatar ids in basic module
        // (player will select them in beginner procedure)

        self.basic_model.level.set(1);
        self.basic_model.avatar_id.set(2011);
        self.basic_model.control_avatar_id.set(2011);

        self.main_city_model.day_of_week.set(5);
        self.gacha_model.gacha_random = GachaRandom::new(rand::thread_rng().next_u32());

        avatar_util::unlock_avatars_on_first_login(self);
        item_util::add_items_on_first_login(self);
        misc_util::init_misc_structs_on_first_login(self);
        map_util::init_map_structs_on_first_login(self);
        quest_util::add_main_city_quest(self, 10020001);

        // Initialize hall scene with WorkShop section
        let scene_uid = self.scene_model.next_scene_uid();
        self.scene_model.scene_snapshots.insert(
            scene_uid,
            SceneSnapshot {
                scene_id: 1,
                ext: SceneSnapshotExt::Hall(HallSceneSnapshot {
                    cur_section_id: 2,
                    sections: HashMap::new(),
                    main_city_objects_state: HashMap::new(),
                }),
                play_type: ELocalPlayType::Unknown,
                dungeon_uid: 0,
                to_be_destroyed: false,
                back_scene_uid: 0,
            },
        );

        self.scene_model.cur_scene_uid.set(scene_uid);
        self.scene_model.default_scene_uid.set(scene_uid);

        self.resources
            .first_login_gm_groups
            .iter()
            .for_each(|gm_group| {
                gm_group
                    .commands
                    .iter()
                    .for_each(|cmd| gm_util::execute_gm_cmd(self, cmd.clone()));
            });
    }

    pub fn claim_reward(&mut self, once_reward_id: u32) {
        let Some(once_reward_template) = self
            .resources
            .templates
            .once_reward_template_tb()
            .find(|tmpl| tmpl.reward_id() == once_reward_id)
        else {
            warn!("OnceRewardTemplate with id {once_reward_id} not found!");
            return;
        };

        let reward_item_list = once_reward_template
            .reward_list()
            .unwrap()
            .iter()
            .map(|reward| {
                self.add_item(reward.item_id(), reward.amount(), AddItemSource::OnceReward);
                (reward.item_id(), reward.amount() as i32)
            })
            .collect();

        self.item_model
            .new_reward_map
            .insert(once_reward_id, reward_item_list);
    }

    pub fn add_item(&mut self, id: u32, count: u32, source: AddItemSource) -> Option<u32> {
        const EXP_ITEM_ID: u32 = 201;

        let item_template = self
            .resources
            .templates
            .item_template_tb()
            .find(|tmpl| tmpl.id() == id)?;

        match item_template.class() {
            _ if id == EXP_ITEM_ID => {
                basic_util::add_experience(self, count);
                None
            }
            3 if !self.avatar_model.is_avatar_unlocked(id) => {
                if let Some(template) = self
                    .resources
                    .templates
                    .avatar_base_template_tb()
                    .find(|tmpl| tmpl.id() == id)
                {
                    use vivian_proto::add_avatar_sc_notify::PerformType;

                    let perform_type = match source {
                        AddItemSource::Custom => None,
                        AddItemSource::OnceReward => Some(PerformType::PerformPopUp),
                        AddItemSource::Gacha => Some(PerformType::PerformNone),
                        AddItemSource::Mail => Some(PerformType::PerformAnimation),
                    };

                    avatar_util::unlock_avatar(self, &template, perform_type);
                }
                None
            }
            5 => {
                if let Some(template) = self
                    .resources
                    .templates
                    .weapon_template_tb()
                    .find(|tmpl| tmpl.item_id() == id)
                {
                    Some(item_util::add_weapon(self, &template))
                } else {
                    Some(0)
                }
            }
            // TODO: equip (disc)
            _ => {
                item_util::add_item(self, id, count);
                None
            }
        }
    }

    pub fn start_hollow_quest(&mut self, quest_id: u32, avatars: &[u32]) -> Option<u64> {
        let hollow_quest_template = self
            .resources
            .templates
            .hollow_quest_template_tb()
            .find(|tmpl| tmpl.id() == quest_id)?;

        let (dungeon_uid, scene_uid) = match hollow_quest_template.hollow_quest_type() {
            HollowQuestType::MainQuest => {
                let battle_group_config = self
                    .resources
                    .templates
                    .battle_group_config_template_tb()
                    .find(|tmpl| tmpl.quest_id() == quest_id)
                    .unwrap();

                self.scene_model.create_long_fight_dungeon(
                    quest_id,
                    battle_group_config.battle_event_id(),
                    ELocalPlayType::PureHollowBattleLonghfight,
                    self.build_dungeon_equipment(avatars),
                )
            }
            HollowQuestType::SideQuest => self.scene_model.create_hollow_dungeon(
                quest_id,
                hollow_quest_template.chessboard_id(),
                ELocalPlayType::Unknown,
                self.build_dungeon_equipment(avatars),
            ),
            HollowQuestType::NormalBattle => {
                let battle_group_config = self
                    .resources
                    .templates
                    .battle_group_config_template_tb()
                    .find(|tmpl| tmpl.quest_id() == quest_id)
                    .unwrap();

                self.scene_model.create_pure_fight_dungeon(
                    quest_id,
                    10,
                    battle_group_config.battle_event_id(),
                    ELocalPlayType::PureHollowBattle,
                    self.build_dungeon_equipment(avatars),
                )
            }
            other => {
                error!("start_hollow_quest: {other:?} is not implemented");
                return None;
            }
        };

        let dungeon = self.scene_model.dungeons.get_mut(&dungeon_uid).unwrap();

        for &id in avatars {
            dungeon.add_avatar(id, &self.resources.templates);
        }

        Some(scene_uid)
    }

    pub fn start_training_quest(&mut self, quest_id: u32, avatars: &[u32]) -> Option<u64> {
        let quest_template = self
            .resources
            .templates
            .traning_quest_template_tb()
            .find(|tmpl| tmpl.id() == quest_id)?;

        let (dungeon_uid, scene_uid) = self.scene_model.create_pure_fight_dungeon(
            quest_id,
            0,
            quest_template.battle_event_id(),
            ELocalPlayType::TrainingRoom,
            self.build_dungeon_equipment(avatars),
        );

        let dungeon = self.scene_model.dungeons.get_mut(&dungeon_uid).unwrap();

        for &id in avatars {
            dungeon.add_avatar(id, &self.resources.templates);
        }

        Some(scene_uid)
    }

    pub fn build_dungeon_equipment(&self, avatar_id_list: &[u32]) -> DungeonEquipment {
        let avatars = self
            .avatar_model
            .avatar_map
            .iter()
            .filter(|(id, _)| avatar_id_list.contains(id))
            .map(|(_, avatar)| avatar)
            .collect::<Vec<_>>();

        DungeonEquipment {
            avatars: avatars
                .iter()
                .map(|avatar| (avatar.id, (*avatar).clone()))
                .collect(),
            weapons: avatars
                .iter()
                .filter_map(|avatar| {
                    Some((
                        avatar.weapon_uid,
                        self.item_model.weapon_map.get(&avatar.weapon_uid)?.clone(),
                    ))
                })
                .collect(),
            equips: avatars
                .iter()
                .flat_map(|avatar| {
                    avatar
                        .dressed_equip_map
                        .keys()
                        .filter_map(|uid| Some((*uid, self.item_model.equip_map.get(uid)?.clone())))
                })
                .collect(),
        }
    }

    pub fn load_state_from_snapshot(&mut self, scene_snapshot_uid: u64) -> GameState {
        let resources = LogicResources {
            template_collection: &self.resources.templates,
            event_graphs: &self.resources.event_graphs,
        };

        self.scene_model.cur_scene_uid.set(scene_snapshot_uid);

        let snapshot = self
            .scene_model
            .scene_snapshots
            .get(&scene_snapshot_uid)
            .unwrap();

        match &snapshot.ext {
            SceneSnapshotExt::Hall(hall) => {
                let mut state = GameHallState::new(
                    resources,
                    hall.cur_section_id,
                    self.main_city_model.get_hall_position(),
                    HallGameParameters {
                        player_avatar_id: self.basic_model.control_avatar_id.get(),
                        control_guise_avatar_id: self.basic_model.control_guise_avatar_id.get(),
                        bgm_id: self.main_city_model.bgm_id.get(),
                        day_of_week: self.main_city_model.day_of_week.get(),
                        time_of_day: self.main_city_model.time_of_day.get(),
                        time_period: self.main_city_model.time_period.get(),
                    },
                );

                state.main_city_objects_state = hall.main_city_objects_state.clone();

                if let Some(section_snapshot) = hall.sections.get(&hall.cur_section_id) {
                    state.attached_graphs = section_snapshot.attached_graphs.clone();
                    state.already_executed_events =
                        section_snapshot.already_executed_events.clone();

                    for snapshot in section_snapshot.event_snapshots.iter() {
                        if let Some(config) = self
                            .resources
                            .event_graphs
                            .get(snapshot.graph, section_snapshot.section_id)
                        {
                            if let Some(event) = config.events.get(&snapshot.ty) {
                                let mut event = Event::new(
                                    snapshot.ty.clone(),
                                    snapshot.tag,
                                    snapshot.graph,
                                    event,
                                );

                                event.state = snapshot.state;
                                event.cur_action_index = snapshot.cur_action_idx;

                                state.running_events.insert(snapshot.uid, event);
                            }
                        }
                    }

                    for unit in section_snapshot.scene_units.iter() {
                        state.create_npc(unit.npc_tag_id);

                        for (i, (&id, interact)) in unit.interacts.iter().enumerate() {
                            state.change_interact(
                                id as i32,
                                &[unit.npc_tag_id],
                                &interact.scale,
                                interact.participators.clone(),
                                i != 0,
                            );
                        }
                    }
                } else {
                    state.attach_graph(GraphReference::MainCitySection(hall.cur_section_id), self);
                }

                state.main_city_quests = self
                    .quest_model
                    .quest_collections
                    .get(&EQuestType::MainCity)
                    .map(|qc| {
                        qc.quests
                            .iter()
                            .map(|(&id, quest)| (id, EQuestState::try_from(quest.state).unwrap()))
                            .collect()
                    })
                    .unwrap_or_default();

                let active_main_city_quests = state
                    .main_city_quests
                    .iter()
                    .filter(|(_, state)| **state != EQuestState::Finished)
                    .map(|(&id, _)| id)
                    .collect::<Vec<_>>();

                state.clear_attached_graphs(&active_main_city_quests);
                active_main_city_quests.into_iter().for_each(|id| {
                    state.attach_graph(GraphReference::Quest(id), self);
                });

                state.on_before_enter(self);

                if self.is_back_scene_changed {
                    self.is_back_scene_changed = false;

                    state.on_exit(self);
                    self.save_scene_snapshot(&mut GameState::from(state));

                    return self.load_state_from_snapshot(scene_snapshot_uid);
                }

                state.on_enter(self);
                state.reset_refresh_state();

                state.into()
            }
            SceneSnapshotExt::Hollow(_hollow) => {
                let dungeon = self
                    .scene_model
                    .dungeons
                    .remove(&snapshot.dungeon_uid)
                    .unwrap()
                    .clone();

                let mut state = GameHollowState::new(
                    self.resources
                        .hollow_chessboard
                        .get(&snapshot.scene_id)
                        .unwrap(),
                    resources,
                    10002,
                    dungeon,
                    self.basic_model.avatar_id.get(),
                );
                state.on_enter(self);

                state.into()
            }
            SceneSnapshotExt::Fight(_fight) => {
                let dungeon = self
                    .scene_model
                    .dungeons
                    .get(&snapshot.dungeon_uid)
                    .unwrap()
                    .clone();

                GameFightState::new(snapshot.scene_id, snapshot.play_type, resources, dungeon)
                    .into()
            }
            SceneSnapshotExt::LongFight(_fight) => {
                let dungeon = self
                    .scene_model
                    .dungeons
                    .get(&snapshot.dungeon_uid)
                    .unwrap()
                    .clone();

                GameLongFightState::new(snapshot.scene_id, snapshot.play_type, resources, dungeon)
                    .into()
            }
        }
    }

    pub fn switch_hall_section(&mut self, section_id: u32, transform: String) -> bool {
        let scene = self
            .scene_model
            .scene_snapshots
            .get_mut(&self.scene_model.cur_scene_uid.get())
            .unwrap();

        let SceneSnapshotExt::Hall(ext) = &mut scene.ext else {
            error!("switch_hall_section: currently not in hall");
            return false;
        };

        ext.cur_section_id = section_id;
        self.main_city_model.transform_id.set(&transform);
        true
    }

    pub fn save_scene_snapshot(&mut self, state: &mut GameState) {
        match state {
            GameState::Hall(hall) => {
                self.update_main_city_model_by_hall(hall);

                let scene = self
                    .scene_model
                    .scene_snapshots
                    .get_mut(&self.scene_model.cur_scene_uid.get())
                    .unwrap();
                let SceneSnapshotExt::Hall(ext) = &mut scene.ext else {
                    panic!("scene snapshot is corrupted (scene type and scene ext type mismatch)");
                };

                ext.main_city_objects_state = hall.main_city_objects_state.clone();

                ext.sections.insert(
                    hall.section_id,
                    HallSectionSnapshot {
                        section_id: hall.section_id,
                        scene_units: hall
                            .scene_units
                            .values()
                            .map(|unit| HallSceneUnitSnapshot {
                                npc_tag_id: unit.npc_id,
                                is_interactable: true,
                                interacts: unit
                                    .interacts
                                    .iter()
                                    .map(|(&id, interact)| {
                                        (
                                            id,
                                            InteractSnapshot {
                                                tag_id: unit.npc_id as i32,
                                                name: interact.name.clone(),
                                                scale: interact.scale.clone(),
                                                participators: interact.participators.clone(),
                                                targets: interact.targets.clone(),
                                            },
                                        )
                                    })
                                    .collect(),
                            })
                            .collect(),
                        attached_graphs: hall.attached_graphs.clone(),
                        event_snapshots: hall
                            .running_events
                            .iter()
                            .filter(|(_, event)| event.is_persistent())
                            .map(|(&uid, event)| EventSnapshot {
                                graph: event.graph,
                                ty: event.ty.clone(),
                                uid,
                                state: event.state,
                                cur_action_idx: event.cur_action_index,
                                tag: event.tag,
                            })
                            .collect(),
                        already_executed_events: hall.already_executed_events.clone(),
                    },
                );
            }
            // Not persistent yet
            GameState::Hollow(_) => (),
            GameState::Fight(_) => (),
            GameState::LongFight(_) => (),
        }
    }

    pub fn update_main_city_model_by_hall(&mut self, hall: &GameHallState) {
        self.main_city_model.time_of_day.set(hall.time_of_day);
        self.main_city_model.day_of_week.set(hall.day_of_week);
        self.main_city_model.bgm_id.set(hall.bgm_id);
        self.main_city_model
            .time_period
            .set(hall.time_period.into());
    }

    fn begin_sync_basic_data(&mut self) {
        self.loading_state = LoadingState::BasicDataSync;
        let sync_helper = &mut self.sync_helper;

        self.basic_model
            .prepare_responses(sync_helper, self.resources);
        self.avatar_model
            .prepare_responses(sync_helper, self.resources);
        self.item_model
            .prepare_responses(sync_helper, self.resources);
        self.quest_model
            .prepare_responses(sync_helper, self.resources);
        self.archive_model
            .prepare_responses(sync_helper, self.resources);
        self.hollow_model
            .prepare_responses(sync_helper, self.resources);
        self.abyss_model
            .prepare_responses(sync_helper, self.resources);
        self.buddy_model
            .prepare_responses(sync_helper, self.resources);
        self.misc_model
            .prepare_responses(sync_helper, self.resources);
    }

    pub fn update(&mut self, state: &mut Option<GameState>) {
        if !self.loading_finished() {
            self.update_loading();
        } else {
            self.update_quest_progress(state);
            self.update_news_stand();
            self.update_newbie();
        }
    }

    pub fn update_quest_progress(&mut self, state: &mut Option<GameState>) {
        self.update_main_city_quest_progress(state);
        self.update_hollow_challenge_progress();
    }

    fn update_main_city_quest_progress(&mut self, state: &mut Option<GameState>) {
        let Some(collection) = self
            .quest_model
            .quest_collections
            .get(&EQuestType::MainCity)
        else {
            return;
        };

        for quest_id in collection.quests.keys().copied().collect::<Vec<_>>() {
            let quest_config_template = self
                .resources
                .templates
                .quest_config_template_tb()
                .find(|tmpl| tmpl.quest_id() == quest_id)
                .unwrap();

            let finish_conditions = BoundConditions::parse(
                quest_config_template.finish_condition().unwrap_or_default(),
            );

            if self.check_conditions(finish_conditions) {
                for quest_id in quest_util::finish_main_city_quest(self, quest_id) {
                    if let Some(GameState::Hall(hall)) = state.as_mut() {
                        hall.main_city_quests
                            .insert(quest_id, EQuestState::InProgress);

                        hall.attach_graph(GraphReference::Quest(quest_id), self);
                    }
                }
            }
        }
    }

    fn update_hollow_challenge_progress(&mut self) {
        self.resources
            .templates
            .quest_config_template_tb()
            .filter(|tmpl| tmpl.quest_type() == 6)
            .for_each(|tmpl| {
                let finish_conditions =
                    BoundConditions::parse(tmpl.finish_condition().unwrap_or_default());

                if self.check_conditions(finish_conditions) {
                    quest_util::finish_hollow_challenge(self, tmpl.quest_id());

                    let challenge_tmpl = self
                        .resources
                        .templates
                        .hollow_challenge_template_tb()
                        .find(|t| t.id() == tmpl.quest_id())
                        .unwrap();

                    let hollow_quest_id = challenge_tmpl.hollow_quest_id();

                    let num_finished_challenges = self
                        .resources
                        .templates
                        .hollow_challenge_template_tb()
                        .filter(|tmpl| tmpl.hollow_quest_id() == hollow_quest_id)
                        .filter(|tmpl| {
                            self.quest_model
                                .is_quest_state(tmpl.id(), EQuestState::Finished)
                        })
                        .count();

                    if let Some(hollow) = self.hollow_model.hollows.get_mut(&hollow_quest_id) {
                        hollow.acquired_hollow_challenge_reward = num_finished_challenges as u32;
                    }
                }
            });
    }

    pub fn update_newbie(&mut self) {
        for newbie_group_template in self
            .resources
            .templates
            .newbie_group_template_tb()
            .filter(|tmpl| !self.misc_model.newbie.finished_groups.contains(&tmpl.id()))
        {
            let bound_conditions = BoundConditions::parse(
                newbie_group_template
                    .trigger_condition()
                    .unwrap_or_default(),
            );

            if self.check_conditions(bound_conditions) {
                self.misc_model
                    .newbie
                    .triggered_groups
                    .insert(newbie_group_template.id());

                // TODO: ????
                #[allow(clippy::single_match)]
                match newbie_group_template.id() {
                    11 => {
                        self.misc_model.unlock.quick_access.insert(
                            1005,
                            QuickAccess {
                                index: 1,
                                quick_access_type: 1,
                            },
                        );
                    }
                    _ => (),
                }
            }
        }
    }

    fn update_news_stand(&mut self) {
        let data = &mut self.misc_model.news_stand;
        if !data.can_sign.get() {
            let cur_time = time_util::unix_timestamp();
            let sign_refresh_time = UNIX_EPOCH
                .checked_add(Duration::from_secs(data.sign_refresh_time.get() as u64))
                .unwrap();

            if (cur_time - sign_refresh_time.duration_since(UNIX_EPOCH).unwrap()).as_secs()
                >= 24 * 3600
            {
                data.sign_refresh_time.set(cur_time.as_secs() as i64);
                data.can_sign.set(true);
                data.current_sign_id.set(22); // TODO!
            }
        }
    }

    fn check_conditions(&self, conditions: BoundConditions) -> bool {
        match conditions {
            BoundConditions::None => false,
            BoundConditions::One(id) => self.check_condition(id),
            BoundConditions::Oneof(ids) => {
                for id in ids {
                    if self.check_condition(id) {
                        return true;
                    }
                }

                false
            }
            BoundConditions::All(ids) => {
                for id in ids {
                    if !self.check_condition(id) {
                        return false;
                    }
                }

                true
            }
        }
    }

    fn check_condition(&self, condition_id: i32) -> bool {
        let Some(condition) = self.resources.conditions.get(&condition_id) else {
            return false;
        };

        match condition {
            Condition::HasExecutedHollowEvent {
                event_ids,
                target_num,
            } => {
                event_ids
                    .iter()
                    .filter(|&id| self.hollow_model.executed_hollow_events.contains(id))
                    .count() as u32
                    >= *target_num
            }
            Condition::QuestState { id, state } => self.quest_model.is_quest_state(*id, *state),
            Condition::ClientSystemOpen { id } => {
                self.misc_model.switch.open_system_id.contains(id)
            }
            Condition::FinishNewbie { group_id } => self
                .misc_model
                .newbie
                .finished_groups
                .contains(&(*group_id as i32)),
            Condition::SignedNewsStandToday => !self.misc_model.news_stand.can_sign.get(),
        }
    }

    pub fn update_loading(&mut self) {
        if self.loading_state == LoadingState::BasicDataSync
            && self.sync_helper.is_basic_data_synced()
        {
            info!(
                "successfully synced basic data for player with uid {}",
                self.uid
            );

            self.loading_state = LoadingState::ExtendDataSync;
        }
    }

    pub fn loading_finished(&self) -> bool {
        self.loading_state == LoadingState::EnterWorldScRsp
    }

    pub fn prepare_to_enter_world(&mut self) -> bool {
        if self.loading_state == LoadingState::ExtendDataSync {
            self.loading_state = LoadingState::EnterWorldCsReq;
            true
        } else {
            false
        }
    }

    pub fn after_enter_world(&mut self) {
        self.loading_state = LoadingState::EnterWorldScRsp;
    }

    pub fn build_partial_update(&self) -> PlayerData {
        let mut data = PlayerData::default();

        self.for_each_model(|model| {
            if model.is_any_field_changed() {
                model.save_to_pb(&mut data)
            }
        });

        data
    }

    pub fn build_full_update(&self) -> PlayerData {
        let mut data = PlayerData::default();
        self.for_each_model(|model| model.save_to_pb(&mut data));

        data
    }

    pub fn ensure_state(&mut self, state: LoadingState) -> bool {
        self.loading_state == state
    }
}

impl LogicEventListener for Player {
    fn main_city_quest_finished(&mut self, quest_id: u32) -> Vec<u32> {
        let newly_added_quests = quest_util::finish_main_city_quest(self, quest_id);

        if let Some(quest_config_template) = self
            .resources
            .templates
            .quest_config_template_tb()
            .find(|tmpl| tmpl.quest_id() == quest_id)
        {
            if quest_config_template.reward_id() != 0 {
                self.claim_reward(quest_config_template.reward_id());
            }
        }

        newly_added_quests
    }

    fn hollow_quest_finished(&mut self, quest_id: u32) {
        quest_util::finish_hollow_quest(self, quest_id);
    }

    fn change_back_scene_info(&mut self, section_id: u32, transform: String) {
        self.is_back_scene_changed = true;
        self.switch_hall_section(section_id, transform);
    }

    fn unlock_hollow_quest(&mut self, quest_id: u32) {
        quest_util::add_hollow_quest(self, quest_id);
    }

    fn give_once_reward(&mut self, once_reward_id: u32) {
        self.claim_reward(once_reward_id);
    }

    fn hollow_event_executed(&mut self, hollow_event_id: u32) {
        self.hollow_model
            .executed_hollow_events
            .insert(hollow_event_id);
    }

    fn add_item(&mut self, id: u32, count: u32) {
        self.add_item(id, count, AddItemSource::Custom);
    }
}

pub trait ModelManager {
    fn is_any_model_modified(&self) -> bool;
    fn changes_acknowledged(&mut self);
    fn has_models_to_synchronize(&self) -> bool;
    fn build_player_sync_notify(&self) -> PlayerSyncScNotify;
    fn for_each_model(&self, f: impl FnMut(&dyn Model));
    #[expect(dead_code)]
    fn for_each_model_mut(&mut self, f: impl FnMut(&mut dyn Model));
}

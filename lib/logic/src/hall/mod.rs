use std::collections::{HashMap, HashSet};

use config::{
    ConfigEvent, ConfigEventAction, EQuestState, ETimePeriodType, GraphReference, SectionEvent,
};
use npc::{Interact, InteractTarget, SceneUnit};
use tracing::{error, warn};
use vivian_proto::{
    common::TimePeriodType, EnterSceneScNotify, EventGraphOwnerType, FinishEventGraphScNotify,
    SectionEventScNotify,
};

use crate::{
    event::{event_util, ActionListener, Event, EventState, EventUID},
    listener::{LogicEventListener, NotifyListener},
    math::{Scale, Transform},
    scene::SceneType,
    LogicResources,
};

pub mod npc;

mod action_executor;
mod predicate_executor;
mod time_period_util;

pub struct GameHallState {
    pub resources: LogicResources,
    pub section_id: u32,
    pub position: HallPosition,
    pub bgm_id: u32,
    pub day_of_week: u32,
    pub time_of_day: u32,
    pub time_period: TimePeriodType,
    pub player_avatar_id: u32,
    pub control_guise_avatar_id: u32,
    pub scene_units: HashMap<u32, SceneUnit>,
    pub running_events: HashMap<EventUID, Event>,
    pub already_executed_events: HashSet<u64>,
    pub attached_graphs: HashSet<GraphReference>,
    pub main_city_quests: HashMap<u32, EQuestState>,
    pub main_city_objects_state: HashMap<i32, i32>,
    pending_event_notifies: Vec<SectionEventScNotify>,
    has_sent_initial_scene_notify: bool,
    refresh_required: bool,
    enter_finished: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum HallEventGraphError {
    #[error("event graph is not running (owner: {0:?}, id: {1})")]
    NotRunning(EventGraphOwnerType, u32),
    #[error("event graph cannot be resumed by client (owner: {0:?}, id: {1}, state: {2:?})")]
    NotWaitingClient(EventGraphOwnerType, u32, EventState),
}

pub enum HallPosition {
    Id(String),
    Transform(Transform),
}

pub struct HallGameParameters {
    pub player_avatar_id: u32,
    pub control_guise_avatar_id: u32,
    pub bgm_id: u32,
    pub day_of_week: u32,
    pub time_of_day: u32,
    pub time_period: i32,
}

pub struct EventGraphContext<'logic> {
    pub hall: &'logic mut GameHallState,
    pub listener: &'logic mut dyn LogicEventListener,
    pub event_uid: u64,
}

impl GameHallState {
    pub fn new(
        resources: LogicResources,
        section_id: u32,
        position: Option<HallPosition>,
        parameters: HallGameParameters,
    ) -> Self {
        let section = resources
            .template_collection
            .section_config_template_tb()
            .find(|tmpl| tmpl.section_id() == section_id)
            .unwrap();

        let position = position
            .unwrap_or_else(|| HallPosition::Id(section.born_transform().unwrap().to_string()));

        Self {
            resources,
            position,
            section_id,
            bgm_id: parameters.bgm_id,
            day_of_week: parameters.day_of_week,
            time_of_day: parameters.time_of_day,
            time_period: TimePeriodType::try_from(parameters.time_period).unwrap_or_default(),
            player_avatar_id: parameters.player_avatar_id,
            control_guise_avatar_id: parameters.control_guise_avatar_id,
            scene_units: HashMap::new(),
            running_events: HashMap::new(),
            already_executed_events: HashSet::new(),
            attached_graphs: HashSet::new(),
            main_city_quests: HashMap::new(),
            main_city_objects_state: HashMap::new(),
            pending_event_notifies: Vec::new(),
            has_sent_initial_scene_notify: false,
            refresh_required: false,
            enter_finished: false,
        }
    }

    pub fn enter_section_finished(&mut self) {
        self.enter_finished = true;
    }

    pub fn attach_graph(
        &mut self,
        graph_ref: GraphReference,
        listener: &mut dyn LogicEventListener,
    ) {
        let Some(event_graph) = self.resources.event_graphs.get(graph_ref, self.section_id) else {
            error!("tried to attach graph, config is missing {graph_ref:?}");
            return;
        };

        if self.attached_graphs.insert(graph_ref) {
            // Execute all on_add events
            for event_name in event_graph.on_add.iter() {
                let event_type = SectionEvent::Custom(event_name.clone());
                if let Some(event_config) = event_graph.events.get(&event_type) {
                    self.initiate_event(
                        graph_ref,
                        event_type,
                        event_util::graph_reference_to_owner_type(graph_ref),
                        0,
                        event_config,
                        listener,
                    );
                }
            }
        }
    }

    pub fn clear_attached_graphs(&mut self, active_main_city_quests: &[u32]) {
        self.attached_graphs.retain(|graph| {
            if let GraphReference::Quest(id) = graph {
                active_main_city_quests.contains(id)
            } else {
                true
            }
        });
    }

    pub fn remove_finished_quests(&mut self) -> Vec<u32> {
        let finished_quest_id_list = self
            .main_city_quests
            .iter()
            .filter(|(_, state)| **state == EQuestState::Finished)
            .map(|(&id, _)| id)
            .collect::<Vec<_>>();

        self.main_city_quests
            .retain(|_, state| *state != EQuestState::Finished);

        finished_quest_id_list
    }

    pub fn create_npc(&mut self, tag_id: u32) {
        let Some(main_city_object) = self
            .resources
            .template_collection
            .main_city_object_template_tb()
            .find(|tmpl| tmpl.tag_id() == tag_id as i32)
        else {
            error!("MainCityObjectTemplate with tag_id {tag_id} doesn't exist",);
            return;
        };

        self.refresh_required = true;
        self.scene_units.insert(
            tag_id,
            SceneUnit {
                npc_id: tag_id,
                interacts: main_city_object
                    .default_interact_ids()
                    .unwrap_or_default()
                    .iter()
                    .map(|interact_id| {
                        (
                            interact_id as u32,
                            Interact {
                                id: interact_id as u32,
                                tag_id: tag_id as i32,
                                name: main_city_object
                                    .interact_name()
                                    .map(|name| name.to_string())
                                    .unwrap_or_default(),
                                participators: HashMap::from([(tag_id, String::from("A"))]),
                                targets: HashSet::from([InteractTarget::Npc]),
                                scale: Scale::new_by_config_str(
                                    main_city_object.interact_scale().unwrap(),
                                ),
                            },
                        )
                    })
                    .collect(),
            },
        );
    }

    pub fn change_interact(
        &mut self,
        interact_id: i32,
        tag_ids: &[u32],
        scale: &Scale,
        participators: HashMap<u32, String>,
        is_additional: bool,
    ) {
        for (_, unit) in self
            .scene_units
            .iter_mut()
            .filter(|(_, unit)| tag_ids.contains(&unit.npc_id))
        {
            self.refresh_required = true;

            if !is_additional {
                unit.interacts.clear();
            }

            let main_city_object = self
                .resources
                .template_collection
                .main_city_object_template_tb()
                .find(|tmpl| tmpl.tag_id() == unit.npc_id as i32)
                .unwrap();

            unit.interacts.insert(
                interact_id as u32,
                Interact {
                    id: interact_id as u32,
                    tag_id: unit.npc_id as i32,
                    name: main_city_object
                        .interact_name()
                        .map(|name| name.to_string())
                        .unwrap_or_default(),
                    participators: participators.clone(),
                    targets: HashSet::from([InteractTarget::Npc]),
                    scale: scale.clone(),
                },
            );
        }
    }

    pub fn remove_quest_npc(&mut self) {
        self.refresh_required = true;

        self.scene_units.retain(|_, unit| {
            self.resources
                .template_collection
                .main_city_object_template_tb()
                .find(|tmpl| tmpl.tag_id() == unit.npc_id as i32)
                .map(|tmpl| tmpl.create_type() != 1)
                .unwrap_or(false)
        });
    }

    pub fn reset_interacts(&mut self) {
        self.refresh_required = true;

        for (_, unit) in self.scene_units.iter_mut() {
            let Some(main_city_object) = self
                .resources
                .template_collection
                .main_city_object_template_tb()
                .find(|tmpl| tmpl.tag_id() == unit.npc_id as i32)
            else {
                error!(
                    "MainCityObjectTemplate with tag_id {} doesn't exist",
                    unit.npc_id
                );
                return;
            };

            unit.interacts = main_city_object
                .default_interact_ids()
                .unwrap_or_default()
                .iter()
                .map(|interact_id| {
                    (
                        interact_id as u32,
                        Interact {
                            id: interact_id as u32,
                            tag_id: unit.npc_id as i32,
                            name: main_city_object
                                .interact_name()
                                .map(|name| name.to_string())
                                .unwrap_or_default(),
                            participators: HashMap::from([(unit.npc_id, String::from("A"))]),
                            targets: HashSet::from([InteractTarget::Npc]),
                            scale: Scale::new_by_config_str(
                                main_city_object.interact_scale().unwrap(),
                            ),
                        },
                    )
                })
                .collect();
        }
    }

    pub fn set_time_period(&mut self, time_period: ETimePeriodType) {
        self.time_period = time_period_util::logic_to_proto(time_period);

        if let Some(time_of_day) = time_period_util::get_time_of_day(time_period) {
            if time_of_day <= self.time_of_day {
                self.day_of_week = (self.day_of_week + 1) % 7;
            }

            self.time_of_day = time_of_day;
        }
    }

    fn remove_pending_refresh(&mut self) -> Option<vivian_proto::HallRefreshScNotify> {
        self.refresh_required.then(|| {
            self.refresh_required = false;

            vivian_proto::HallRefreshScNotify {
                force_refresh: true,
                section_id: self.section_id,
                bgm_id: self.bgm_id,
                day_of_week: self.day_of_week,
                time_of_day: self.time_of_day,
                player_avatar_id: self.player_avatar_id,
                control_guise_avatar_id: self.control_guise_avatar_id,
                transform_id: match &self.position {
                    HallPosition::Id(id) => id.clone(),
                    HallPosition::Transform(_) => String::new(),
                },
                position: match &self.position {
                    HallPosition::Transform(transform) => Some(transform.to_proto()),
                    HallPosition::Id(_) => None,
                },
                scene_unit_list: self
                    .scene_units
                    .values()
                    .map(|unit| unit.to_client_proto())
                    .collect(),
                main_city_objects_state: self.main_city_objects_state.clone(),
                hall_unknown_map_string_int: HashMap::new(),
                hall_unknown_map_uint_uint: HashMap::new(),
                main_city_quest_id_list: self.main_city_quests.keys().copied().collect(),
                time_period: i32::from(self.time_period) as u32,
            }
        })
    }

    pub fn force_refresh(&mut self) {
        self.refresh_required = true;
    }

    pub fn reset_refresh_state(&mut self) {
        self.refresh_required = false;
    }

    fn initiate_event(
        &mut self,
        graph: GraphReference,
        event_type: SectionEvent,
        owner_type: EventGraphOwnerType,
        tag: u32,
        config: &'static ConfigEvent,
        listener: &mut dyn LogicEventListener,
    ) {
        let event_uid = EventUID::new(owner_type, config.id);

        if event_type == SectionEvent::OnInteract
            || self
                .already_executed_events
                .insert(((graph.id() as u64) << 32) | config.id as u64)
        {
            let mut event = Event::new(event_type, tag, graph, config);
            event.wakeup(event_uid, self, listener);

            if !event.is_finished() {
                self.running_events.insert(event_uid, event);
            }
        }
    }

    pub fn on_before_enter(&mut self, listener: &mut dyn LogicEventListener) {
        for graph_ref in self.attached_graphs.clone() {
            if let Some(graph) = self.resources.event_graphs.get(graph_ref, self.section_id) {
                for event_name in graph.on_before_enter.iter() {
                    let event_type = SectionEvent::Custom(event_name.clone());
                    if let Some(event_config) = graph.events.get(&event_type) {
                        self.initiate_event(
                            graph_ref,
                            event_type,
                            event_util::graph_reference_to_owner_type(graph_ref),
                            0,
                            event_config,
                            listener,
                        );
                    }
                }
            }
        }

        for (uid, mut event) in std::mem::take(&mut self.running_events).into_iter() {
            // Re-execute last action to rebuild pending client action on re-enter.
            if event.state == EventState::WaitingClient {
                event.state = EventState::Running;
                event.cur_action_index -= 1;
                event.wakeup(uid, self, listener);
            }

            self.running_events.insert(uid, event);
        }
    }

    pub fn on_enter(&mut self, listener: &mut dyn LogicEventListener) {
        for graph_ref in self.attached_graphs.clone() {
            if let Some(graph) = self.resources.event_graphs.get(graph_ref, self.section_id) {
                for event_name in graph.on_enter.iter() {
                    let event_type = SectionEvent::Custom(event_name.clone());
                    if let Some(event_config) = graph.events.get(&event_type) {
                        self.initiate_event(
                            graph_ref,
                            event_type,
                            event_util::graph_reference_to_owner_type(graph_ref),
                            0,
                            event_config,
                            listener,
                        );
                    }
                }
            }
        }
    }

    pub fn on_exit(&mut self, listener: &mut dyn LogicEventListener) {
        for graph_ref in self.attached_graphs.clone() {
            if let Some(graph) = self.resources.event_graphs.get(graph_ref, self.section_id) {
                for event_name in graph.on_exit.iter() {
                    let event_type = SectionEvent::Custom(event_name.clone());
                    if let Some(event_config) = graph.events.get(&event_type) {
                        self.initiate_event(
                            graph_ref,
                            event_type,
                            event_util::graph_reference_to_owner_type(graph_ref),
                            0,
                            event_config,
                            listener,
                        );
                    }
                }
            }
        }
    }

    pub fn interact_with_unit(
        &mut self,
        npc_tag_id: u32,
        interact_id: u32,
        listener: &mut dyn LogicEventListener,
    ) -> bool {
        let event_type = SectionEvent::OnInteract;
        let owner_type = EventGraphOwnerType::SceneUnit;
        let graph = GraphReference::Interact(interact_id);

        let event_graph_uid = EventUID::new(owner_type, interact_id);

        if self.running_events.contains_key(&event_graph_uid) {
            warn!("tried to interact, but event is already running (interact_id: {interact_id})");
            return false;
        }

        // Check if npc_tag_id/interact_id pair is valid
        if !self
            .scene_units
            .get(&npc_tag_id)
            .map(|unit| unit.interacts.contains_key(&interact_id))
            .unwrap_or(false)
        {
            error!("[tag:{npc_tag_id}|interact:{interact_id}] pair is invalid");
            return false;
        }

        let Some(config) = self.resources.event_graphs.get(graph, self.section_id) else {
            error!("event config for interact id {interact_id} not found!",);
            return false;
        };

        let Some(event) = config.events.get(&event_type) else {
            error!("interact graph {interact_id} doesn't have event {event_type}");
            return false;
        };

        self.initiate_event(graph, event_type, owner_type, npc_tag_id, event, listener);
        true
    }

    pub fn run_event_graph(
        &mut self,
        owner_type: EventGraphOwnerType,
        event_id: u32,
        listener: &mut dyn LogicEventListener,
    ) -> Result<(), HallEventGraphError> {
        let event_graph_uid = EventUID::new(owner_type, event_id);

        let mut event = self
            .running_events
            .remove(&event_graph_uid)
            .ok_or(HallEventGraphError::NotRunning(owner_type, event_id))?;

        (event.state == EventState::WaitingClient)
            .then_some(())
            .ok_or(HallEventGraphError::NotWaitingClient(
                owner_type,
                event_id,
                event.state,
            ))?;

        event.wakeup(event_graph_uid, self, listener);
        self.running_events.insert(event_graph_uid, event);

        Ok(())
    }

    pub fn remove_if_finished(
        &mut self,
        owner_type: EventGraphOwnerType,
        event_id: u32,
    ) -> Option<FinishEventGraphScNotify> {
        let event_graph_uid = EventUID::new(owner_type, event_id);

        if let Some(event) = self.running_events.remove(&event_graph_uid) {
            if event.state == EventState::Finished {
                return Some(FinishEventGraphScNotify {
                    tag: event.tag,
                    owner_type: owner_type.into(),
                    event_name: event.ty.to_string(),
                    section_event_unk_2: 0,
                    ..Default::default()
                });
            }

            if !event.is_finished() {
                self.running_events.insert(event_graph_uid, event);
            }
        }

        None
    }

    // Common

    pub fn scene_type(&self) -> SceneType {
        SceneType::Hall
    }

    pub fn flush_notifies(&mut self, listener: &mut dyn NotifyListener) {
        use crate::listener::NotifyListenerExt;

        if !self.has_sent_initial_scene_notify {
            self.has_sent_initial_scene_notify = true;

            listener.add(EnterSceneScNotify {
                scene: Some(self.client_scene_data_proto()),
                dungeon: None,
            });
        }

        if self.enter_finished {
            if let Some(hall_refresh_notify) = self.remove_pending_refresh() {
                listener.add(hall_refresh_notify);
            }

            std::mem::take(&mut self.pending_event_notifies)
                .into_iter()
                .for_each(|section_event_notify| listener.add(section_event_notify));
        }
    }

    pub fn client_scene_data_proto(&self) -> vivian_proto::SceneData {
        vivian_proto::SceneData {
            scene_type: self.scene_type().into(),
            hall_scene_data: Some(vivian_proto::HallSceneData {
                section_id: self.section_id,
                bgm_id: self.bgm_id,
                day_of_week: self.day_of_week,
                time_of_day: self.time_of_day,
                player_avatar_id: self.player_avatar_id,
                control_guise_avatar_id: self.control_guise_avatar_id,
                transform_id: match &self.position {
                    HallPosition::Id(id) => id.clone(),
                    HallPosition::Transform(_) => String::new(),
                },
                position: match &self.position {
                    HallPosition::Transform(transform) => Some(transform.to_proto()),
                    HallPosition::Id(_) => None,
                },
                scene_unit_list: self
                    .scene_units
                    .values()
                    .map(|unit| unit.to_client_proto())
                    .collect(),
                main_city_objects_state: self.main_city_objects_state.clone(),
                hall_unknown_map_string_int: HashMap::new(),
                hall_unknown_map_uint_uint: HashMap::new(),
                main_city_quest_id_list: self.main_city_quests.keys().copied().collect(),
                time_period: i32::from(self.time_period) as u32,
            }),
            ..Default::default()
        }
    }
}

impl ActionListener for GameHallState {
    fn should_execute_action(
        &mut self,
        event_uid: u64,
        action: &ConfigEventAction,
        listener: &mut dyn LogicEventListener,
        _specials: &HashMap<String, i32>,
    ) -> bool {
        let mut context = EventGraphContext {
            hall: self,
            event_uid,
            listener,
        };

        for predicate in action.predicates() {
            if !predicate_executor::execute(&mut context, predicate) {
                return false;
            }
        }

        true
    }

    fn execute_action(
        &mut self,
        event_uid: u64,
        action: &ConfigEventAction,
        listener: &mut dyn LogicEventListener,
        _specials: &HashMap<String, i32>,
    ) {
        action_executor::execute(
            &mut EventGraphContext {
                hall: self,
                event_uid,
                listener,
            },
            action,
        );
    }

    fn enqueue_client_action(
        &mut self,
        (event_uid, event): (EventUID, &Event),
        info: vivian_proto::ActionInfo,
    ) {
        self.pending_event_notifies.push(SectionEventScNotify {
            event_id: event_uid.event_id(),
            tag: event.tag,
            section_id: self.section_id,
            owner_type: event_uid.owner_type().into(),
            event_name: event.ty.to_string(),
            action_list: vec![info],
            ..Default::default()
        });
    }

    fn is_immediate_mode(&self) -> bool {
        true
    }
}

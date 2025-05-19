use std::collections::HashMap;

use config::{GraphReference, HollowChessboardConfig, HollowSectionConfig, SectionEvent};
use tracing::{error, warn};
use vivian_proto::{
    EventGraphOwnerType, FinishEventGraphScNotify, GridLink, GridType, HenshinType,
    HollowEntityType, HollowGridMapType, HollowPushReason, HollowPushScNotify,
    SectionEventScNotify,
    common::{HollowGridFlag, NodeState, NodeVisible, Vector2Int},
};

use crate::{
    LogicResources,
    event::{ActionListener, Event, EventState, EventUID},
    listener::LogicEventListener,
};

use super::{
    action_executor,
    component::{
        BehaviorComponent, CategoryComponent, EntityPriorityComponent, GridStateComponent,
        HollowComponentManager, HollowEventComponent, HollowGridComponent, IdComponent,
        OwnerComponent, PosComponent,
    },
    entity::{HollowEntity, HollowEntityManager},
    entity_util,
    net_helper::HollowGlobalEventHelper,
    predicate_executor,
};

pub struct Chessboard {
    pub running_events: HashMap<EventUID, Event>,
    pub global_event_helper: HollowGlobalEventHelper,
    pub hollow_quest_id: u32,
    pub hollow_variables: HashMap<String, i32>,
    pub player_avatar_id: u32,
    pub event_choice_uid: u32,
    pub finished: bool,
    resources: LogicResources,
    henshin: HenshinType,
    entity_manager: HollowEntityManager,
    component_manager: HollowComponentManager,
    cur_section: HollowEntity,
    player: HollowEntity,
    sections: Vec<HollowEntity>,
}

#[derive(Debug, Clone, Copy)]
pub enum GridReference {
    Absolute(i32, i32),
    Relative(i32, i32),
}

impl Chessboard {
    pub fn new(
        quest_id: u32,
        player_avatar_id: u32,
        config: &HollowChessboardConfig,
        res: LogicResources,
    ) -> Self {
        let mut entity_manager = HollowEntityManager::default();
        let mut component_manager = HollowComponentManager::default();

        entity_manager.create(HollowEntityType::Global);

        let sections = config
            .sections
            .iter()
            .map(|config| {
                Self::create_hollow_section(
                    config,
                    &res,
                    &mut entity_manager,
                    &mut component_manager,
                )
            })
            .collect::<Vec<_>>();

        let cur_section = *sections.get(config.default_section_index).unwrap();
        let player = entity_manager.create(HollowEntityType::Player);

        component_manager.insert_pos_component(
            player,
            PosComponent {
                pos: (config.default_player_pos.x, config.default_player_pos.y),
                section: cur_section,
            },
        );

        Self {
            hollow_quest_id: quest_id,
            player_avatar_id,
            event_choice_uid: 0,
            finished: false,
            resources: res,
            henshin: HenshinType::DefaultPlayer,
            entity_manager,
            component_manager,
            cur_section,
            player,
            sections,
            running_events: HashMap::new(),
            hollow_variables: HashMap::new(),
            global_event_helper: HollowGlobalEventHelper::default(),
        }
    }

    pub fn player_move(
        &mut self,
        pos: Vector2Int,
        listener: &mut dyn LogicEventListener,
        by_server: bool,
    ) {
        let player_pos = self
            .component_manager
            .get_pos_component(self.player)
            .unwrap();

        if player_pos.pos.0 == pos.x && player_pos.pos.1 == pos.y {
            return;
        }

        if ((pos.x - player_pos.pos.0).abs() + (pos.y - player_pos.pos.1).abs()) > 1 {
            error!(
                "Chessboard::player_move: destination is too far! player_pos: {:?}, destination: {:?}",
                player_pos.pos,
                (pos.x, pos.y)
            );
            return;
        }

        if entity_util::get_grid_at(
            (pos.x, pos.y),
            self.cur_section,
            &self.entity_manager,
            &self.component_manager,
        )
        .is_none()
        {
            error!(
                "Chessboard::player_move: target_grid is None! Requested position: ({}, {})",
                pos.x, pos.y
            );
            return;
        };

        // TODO: check GridLinks

        self.component_manager.insert_pos_component(
            self.player,
            PosComponent {
                pos: (pos.x, pos.y),
                ..player_pos
            },
        );

        if by_server {
            self.global_event_helper.add(HollowPushScNotify {
                cur_section_id: self.cur_section.as_raw_u32(),
                prev_position: Some(Vector2Int {
                    x: player_pos.pos.0,
                    y: player_pos.pos.1,
                }),
                cur_position: Some(Vector2Int { x: pos.x, y: pos.y }),
                reason: HollowPushReason::ConveyorBelt.into(), // TODO?
                entity_type: HollowEntityType::Player.into(),
            });
        }

        self.on_player_pos_update(listener);
    }

    pub fn retrigger_cur_event(&mut self, logic_listener: &mut dyn LogicEventListener) {
        self.on_player_pos_update(logic_listener);
    }

    pub fn execute_first_turn(&mut self, logic_listener: &mut dyn LogicEventListener) {
        self.on_player_pos_update(logic_listener);

        // EnterSceneScNotify will contain latest data anyway
        self.component_manager.reset_change_state();
    }

    pub fn resume_event_logic(
        &mut self,
        owner: EventGraphOwnerType,
        id: u32,
        logic_listener: &mut dyn LogicEventListener,
    ) -> bool {
        let event_graph_uid = EventUID::new(owner, id);

        let Some(mut event) = self.running_events.remove(&event_graph_uid) else {
            error!("event {owner:?}:{id} is not running");
            return false;
        };

        if event.state != EventState::WaitingClient {
            error!(
                "event {owner:?}:{id} can't be resumed by client, current state: {:?}",
                event.state
            );
            return false;
        }

        event.wakeup(event_graph_uid, self, logic_listener);

        if event.is_finished() {
            self.notify_finish_event_graph(event_graph_uid, &event);
            self.on_event_finish(event.tag);
            self.sync_hollow_grid();
        } else {
            self.running_events.insert(event_graph_uid, event);
        }

        true
    }

    fn on_event_finish(&mut self, tag: u32) {
        let Some(event_entity) = self.entity_manager.uid_exists(tag) else {
            error!(
                "Chessboard::on_event_finish: entity with uid of event tag ({tag}) doesn't exist"
            );
            return;
        };

        let Some(IdComponent(event_id)) = self.component_manager.get_id_component(event_entity)
        else {
            error!("Chessboard::on_event_finish: event doesn't have an ID (tag: {tag})");
            return;
        };

        let Some(OwnerComponent(owner_entity)) =
            self.component_manager.get_owner_component(event_entity)
        else {
            error!("Chessboard::on_event_finish: event doesn't have an owner (tag: {tag})");
            return;
        };

        if owner_entity.entity_type() == HollowEntityType::Grid {
            let Some(grid_state) = self
                .component_manager
                .get_grid_state_component(owner_entity)
            else {
                error!(
                    "Chessboard::on_event_finish: owner grid doesn't have a GridStateComponent!"
                );
                return;
            };

            let template = self
                .resources
                .template_collection
                .hollow_event_template_tb()
                .find(|tmpl| tmpl.id() == event_id)
                .unwrap();

            if template.can_trigger_times() == -1 {
                self.component_manager.insert_grid_state_component(
                    owner_entity,
                    GridStateComponent {
                        node_state: NodeState::ShowEvent,
                        ..grid_state.enable(HollowGridFlag::CanTriggerEvent)
                    },
                );
            } else {
                self.component_manager.insert_grid_state_component(
                    owner_entity,
                    GridStateComponent {
                        node_state: NodeState::Finished,
                        ..grid_state.disable(HollowGridFlag::CanTriggerEvent)
                    },
                );
            }
        }
    }

    fn notify_finish_event_graph(&mut self, uid: EventUID, event: &Event) {
        self.global_event_helper.add(FinishEventGraphScNotify {
            tag: event.tag,
            owner_type: EventGraphOwnerType::Hollow.into(),
            event_name: event.ty.to_string(),
            section_event_unk_2: 1,
            finish_event_unk_1: 1,
            event_uid: uid.event_id(),
        });
    }

    fn on_player_pos_update(&mut self, logic_listener: &mut dyn LogicEventListener) {
        let player_pos = self
            .component_manager
            .get_pos_component(self.player)
            .unwrap();

        let Some(cur_grid) = entity_util::get_grid_at(
            player_pos.pos,
            player_pos.section,
            &self.entity_manager,
            &self.component_manager,
        ) else {
            error!("Chessboard::on_player_pos_update: cur_grid is None, position: {player_pos}");
            return;
        };

        let Some(state_comp) = self.component_manager.get_grid_state_component(cur_grid) else {
            error!(
                "Chessboard::on_player_pos_update: cur_grid is missing a GridStateComponent, position: {player_pos}"
            );
            return;
        };

        if state_comp.node_state != NodeState::Finished
            && state_comp.is_enabled(HollowGridFlag::CanTriggerEvent)
        {
            let state_comp = state_comp
                .enable(HollowGridFlag::Travelled)
                .enable(HollowGridFlag::ShowEventId)
                .enable(HollowGridFlag::ShowEventType)
                .disable(HollowGridFlag::Guide)
                .disable(HollowGridFlag::CanTriggerEvent);

            if let Some(cur_event) =
                entity_util::get_grid_event(cur_grid, &self.entity_manager, &self.component_manager)
            {
                self.component_manager
                    .insert_grid_state_component(cur_grid, state_comp);

                self.trigger_event(cur_event, SectionEvent::OnStart, logic_listener);
            } else {
                self.component_manager.insert_grid_state_component(
                    cur_grid,
                    GridStateComponent {
                        node_state: NodeState::Finished,
                        ..state_comp
                    },
                );
            }
        }

        self.sync_hollow_grid();
    }

    fn trigger_event(
        &mut self,
        event_entity: HollowEntity,
        ty: SectionEvent,
        logic_listener: &mut dyn LogicEventListener,
    ) {
        let Some(IdComponent(event_id)) = self.component_manager.get_id_component(event_entity)
        else {
            error!("Chessboard::trigger_event: event entity doesn't have IdComponent!");
            return;
        };

        let template = self
            .resources
            .template_collection
            .hollow_event_template_tb()
            .find(|tmpl| tmpl.id() == event_id)
            .unwrap();

        let target_template = match template.target_id() {
            0 => template,
            other if template.id() == other => template,
            other => self
                .resources
                .template_collection
                .hollow_event_template_tb()
                .find(|tmpl| tmpl.id() == other)
                .unwrap(),
        };

        let owner_type = EventGraphOwnerType::Hollow;
        let event_uid = EventUID::new(owner_type, event_id);
        let graph = GraphReference::HollowEvent(target_template.id());

        if let Some(config) = self
            .resources
            .event_graphs
            .get(graph, 0)
            .and_then(|graph| graph.events.get(&ty))
        {
            logic_listener.hollow_event_executed(event_id);

            let mut event = Event::new(
                SectionEvent::OnStart,
                event_entity.as_raw_u32(),
                GraphReference::HollowEvent(target_template.id()),
                config,
            );

            event.add_parameter(template.param_1());
            event.add_parameter(template.param_2());
            event.add_parameter(template.param_3());
            event.add_parameter(template.param_4());
            event.add_parameter(template.param_5());
            event.add_parameter(template.param_6());

            event.wakeup(event_uid, self, logic_listener);

            if event.is_finished() {
                self.notify_finish_event_graph(event_uid, &event);
                self.on_event_finish(event.tag);
                self.sync_hollow_grid();
            } else {
                self.running_events.insert(event_uid, event);
            }
        } else {
            error!(
                "No config for hollow event with id {}",
                target_template.id()
            );
        }
    }

    pub fn set_map_state(
        &mut self,
        grid_pos: GridReference,
        visible_state_change: Option<(NodeVisible, NodeVisible)>,
        event_state_change: Option<(NodeState, NodeState)>,
    ) -> bool {
        let grid = match grid_pos {
            GridReference::Absolute(x, y) => entity_util::get_grid_at(
                (x, y),
                self.cur_section,
                &self.entity_manager,
                &self.component_manager,
            ),
            GridReference::Relative(x, y) => {
                let pos = self.cur_player_pos();
                entity_util::get_grid_at(
                    (pos.x + x, pos.y + y),
                    self.cur_section,
                    &self.entity_manager,
                    &self.component_manager,
                )
            }
        };

        let Some(grid) = grid else {
            warn!("Chessboard::set_map_state: no grid at {grid_pos:?}");
            return true;
        };

        let Some(mut grid_state) = self.component_manager.get_grid_state_component(grid) else {
            error!("Chessboard::set_map_state: missing GridStateComponent at {grid_pos:?}");
            return true;
        };

        let mut did_change = false;

        if let Some((from_visible, to_visible)) = visible_state_change {
            if grid_state.node_visible == from_visible {
                grid_state.node_visible = to_visible;

                grid_state = match to_visible {
                    NodeVisible::Visible => grid_state
                        .enable(HollowGridFlag::CanMove)
                        .enable(HollowGridFlag::Visible)
                        .disable(HollowGridFlag::VisibleByTriggerEvent)
                        .disable(HollowGridFlag::VisibleAtGridAround),
                    NodeVisible::VisibleByTriggerEvent => grid_state
                        .disable(HollowGridFlag::CanMove)
                        .disable(HollowGridFlag::Visible)
                        .enable(HollowGridFlag::VisibleByTriggerEvent)
                        .disable(HollowGridFlag::VisibleAtGridAround),
                    NodeVisible::VisibleAtGridAround => grid_state
                        .enable(HollowGridFlag::CanMove)
                        .disable(HollowGridFlag::Visible)
                        .disable(HollowGridFlag::VisibleByTriggerEvent)
                        .enable(HollowGridFlag::VisibleAtGridAround),
                    _ => grid_state,
                };

                did_change = true;
            }
        }

        if let Some((from_state, to_state)) = event_state_change {
            if grid_state.node_state == from_state {
                grid_state.node_state = to_state;
                did_change |= true;
            }
        }

        if did_change {
            self.component_manager
                .insert_grid_state_component(grid, grid_state);
        }

        did_change
    }

    pub fn modify_grid_event(
        &mut self,
        grid_pos: GridReference,
        target_event_id: u32,
        set_event_id: u32,
        set_state: NodeState,
        set_visible: NodeVisible,
    ) -> bool {
        let Some(template) = self
            .resources
            .template_collection
            .hollow_event_template_tb()
            .find(|tmpl| tmpl.id() == set_event_id)
        else {
            error!("Chessboard::modify_grid_event: event with id {set_event_id} doesn't exist!");
            return true;
        };

        let grid = match grid_pos {
            GridReference::Absolute(x, y) => entity_util::get_grid_at(
                (x, y),
                self.cur_section,
                &self.entity_manager,
                &self.component_manager,
            ),
            GridReference::Relative(x, y) => {
                let pos = self.cur_player_pos();
                entity_util::get_grid_at(
                    (pos.x + x, pos.y + y),
                    self.cur_section,
                    &self.entity_manager,
                    &self.component_manager,
                )
            }
        };

        let Some(grid) = grid else {
            warn!("Chessboard::modify_grid_event: no grid at {grid_pos:?}");
            return true;
        };

        let Some(grid_state) = self.component_manager.get_grid_state_component(grid) else {
            error!("Chessboard::modify_grid_event: missing GridStateComponent at {grid_pos:?}");
            return true;
        };

        self.component_manager.insert_grid_state_component(
            grid,
            GridStateComponent {
                node_state: set_state,
                node_visible: set_visible,
                ..grid_state.enable(HollowGridFlag::CanTriggerEvent)
            },
        );

        let Some(event) =
            entity_util::get_grid_event(grid, &self.entity_manager, &self.component_manager)
        else {
            error!("Chessboard::modify_grid_event: no event entity for grid at {grid_pos:?}");
            return true;
        };

        let Some(IdComponent(cur_event_id)) = self.component_manager.get_id_component(event) else {
            error!("Chessboard::modify_grid_event: no ID on event entity for grid at {grid_pos:?}");
            return true;
        };

        if target_event_id == 0 || cur_event_id == target_event_id {
            self.component_manager
                .insert_id_component(event, IdComponent(set_event_id));

            self.component_manager.insert_hollow_event_component(
                event,
                HollowEventComponent {
                    icon_texture_sheet_id: template.icon_texture_sheet_id(),
                    interact_texture_sheet_id: template.interact_icon_texture_sheet_id(),
                },
            );

            true
        } else {
            false
        }
    }

    fn create_hollow_section(
        config: &HollowSectionConfig,
        res: &LogicResources,
        em: &mut HollowEntityManager,
        cm: &mut HollowComponentManager,
    ) -> HollowEntity {
        let section = em.create(HollowEntityType::Section);

        config.grids.iter().for_each(|config| {
            let grid = em.create(HollowEntityType::Grid);

            cm.insert_pos_component(
                grid,
                PosComponent {
                    pos: (config.position.x, config.position.y),
                    section,
                },
            );

            cm.insert_hollow_grid_component(
                grid,
                config
                    .grid_links
                    .iter()
                    .map(|&config_link| GridLink::try_from(i32::from(config_link)).unwrap())
                    .fold(
                        HollowGridComponent::new(GridType::CommonGrid),
                        |comp, link| comp.link(link),
                    ),
            );

            cm.insert_grid_state_component(
                grid,
                config
                    .grid_flags
                    .iter()
                    .map(|&config_flag| HollowGridFlag::try_from(i32::from(config_flag)).unwrap())
                    .fold(
                        GridStateComponent::new(
                            NodeState::try_from(i32::from(config.node_state)).unwrap(),
                            NodeVisible::try_from(i32::from(config.node_visible)).unwrap(),
                        ),
                        |comp, flag| comp.enable(flag),
                    ),
            );
        });

        config.events.iter().for_each(|config| {
            let event = em.create(HollowEntityType::Event);

            let Some(owner) = entity_util::get_owner_by_config(&config.owned_by, section, em, cm)
            else {
                error!("failed to find the configured owner for hollow event: {config:?}");
                return;
            };

            cm.insert_id_component(event, IdComponent(config.event_id));
            cm.insert_owner_component(event, OwnerComponent(owner));
            cm.insert_behavior_component(event, BehaviorComponent::default());
            cm.insert_category_component(event, CategoryComponent::default());

            let template = res
                .template_collection
                .hollow_event_template_tb()
                .find(|tmpl| tmpl.id() == config.event_id)
                .unwrap_or_else(|| {
                    panic!("HollowEventTemplate with id {} not found", config.event_id)
                });

            cm.insert_hollow_event_component(
                event,
                HollowEventComponent {
                    icon_texture_sheet_id: template.icon_texture_sheet_id(),
                    interact_texture_sheet_id: template.interact_icon_texture_sheet_id(),
                },
            );

            if config.priority != 0 {
                cm.insert_entity_priority_component(
                    event,
                    EntityPriorityComponent {
                        priority: config.priority,
                        secondary_priority: em.next_secondary_priority(config.priority),
                    },
                );
            }
        });

        section
    }

    fn global_event(&self) -> Option<HollowEntity> {
        self.entity_manager
            .iter()
            .filter(|e| e.entity_type() == HollowEntityType::Event)
            .find(|&e| {
                self.component_manager
                    .get_owner_component(e)
                    .map(|owner| owner.0.entity_type() == HollowEntityType::Global)
                    .unwrap_or(false)
            })
    }

    pub fn cur_section_id(&self) -> u32 {
        self.cur_section.as_raw_u32()
    }

    pub fn cur_player_pos(&self) -> Vector2Int {
        if let Some(pos_comp) = self.component_manager.get_pos_component(self.player) {
            Vector2Int {
                x: pos_comp.pos.0,
                y: pos_comp.pos.1,
            }
        } else {
            Vector2Int {
                x: -32768,
                y: -32768,
            }
        }
    }

    pub fn sync_hollow_grid(&mut self) {
        if !self.component_manager.is_synchronized() {
            self.global_event_helper
                .add(vivian_proto::SyncHollowGridMapsScNotify {
                    modify_entity_list: self
                        .component_manager
                        .get_changed_entities()
                        .iter()
                        .map(|&entity| {
                            self.entity_manager
                                .serialize_entity(entity, &self.component_manager)
                        })
                        .collect(),
                });

            self.component_manager.reset_change_state();
        }
    }

    pub fn as_client_proto(&self) -> vivian_proto::HollowScene {
        let player_pos = self
            .component_manager
            .get_pos_component(self.player)
            .map(|comp| Vector2Int {
                x: comp.pos.0,
                y: comp.pos.1,
            });

        vivian_proto::HollowScene {
            henshin_type: self.henshin.into(),
            cur_event_entity_info: self.global_event().map(|e| {
                self.entity_manager
                    .serialize_entity(e, &self.component_manager)
            }),
            hollow_grid_maps: Some(vivian_proto::HollowGridMapsInfo {
                hollow_grid_map_type: HollowGridMapType::Unknown1.into(),
                cur_section_id: self.cur_section.as_raw_u32(),
                cur_hollow_position: player_pos,
                hollow_section_list: self
                    .sections
                    .iter()
                    .map(|&section| vivian_proto::HollowSectionInfo {
                        section_id: section.as_raw_u32(),
                        time: String::from("Afternoon"),
                        weather: String::from("SunShine"),
                        hollow_vector_zero_1: Some(Vector2Int::default()),
                        hollow_vector_zero_2: Some(Vector2Int::default()),
                        hollow_vector_negative_1: Some(Vector2Int {
                            x: -32768,
                            y: -32768,
                        }),
                        hollow_vector_negative_2: Some(Vector2Int {
                            x: -32768,
                            y: -32768,
                        }),
                        hollow_objective_id: 1000204,
                        section_grid_map: Some(vivian_proto::HollowSectionGridMapInfo {
                            cur_grid_position: player_pos, // idk?
                            hollow_grid_map: Some(vivian_proto::HollowGridMap {
                                hollow_grid_list: self
                                    .entity_manager
                                    .serialize_entities(section, &self.component_manager),
                            }),
                        }),
                    })
                    .collect(),
            }),
        }
    }
}

pub struct EventGraphContext<'logic> {
    pub chessboard: &'logic mut Chessboard,
    pub listener: &'logic mut dyn LogicEventListener,
    pub event_specials: &'logic HashMap<String, i32>,
    #[expect(dead_code)]
    pub event_uid: u64,
}

impl ActionListener for Chessboard {
    fn should_execute_action(
        &mut self,
        event_uid: u64,
        action: &config::ConfigEventAction,
        listener: &mut dyn LogicEventListener,
        specials: &HashMap<String, i32>,
    ) -> bool {
        let mut context = EventGraphContext {
            chessboard: self,
            listener,
            event_uid,
            event_specials: specials,
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
        action: &config::ConfigEventAction,
        listener: &mut dyn LogicEventListener,
        specials: &HashMap<String, i32>,
    ) {
        action_executor::execute(
            &mut EventGraphContext {
                chessboard: self,
                listener,
                event_uid,
                event_specials: specials,
            },
            action,
        );

        self.sync_hollow_grid();
    }

    fn enqueue_client_action(
        &mut self,
        (event_uid, event): (EventUID, &Event),
        info: vivian_proto::ActionInfo,
    ) {
        self.global_event_helper.add(SectionEventScNotify {
            event_id: event_uid.event_id(),
            hollow_event_id: event_uid.event_id(),
            tag: event.tag,
            section_id: self.cur_section.as_raw_u32(),
            owner_type: event_uid.owner_type().into(),
            event_name: event.ty.to_string(),
            action_list: vec![info],
            section_event_unk_1: 1,
            section_event_unk_2: event_uid.event_id(),
            ..Default::default()
        });
    }

    fn is_immediate_mode(&self) -> bool {
        false
    }
}

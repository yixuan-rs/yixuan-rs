mod action;
pub(crate) mod event_util;
mod uid;

use std::collections::HashMap;

pub use action::ActionBase;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use tracing::error;
pub use uid::EventUID;

use config::{ConfigEvent, ConfigEventAction, GraphReference, SectionEvent};
use vivian_proto::ActionInfo;

use crate::listener::LogicEventListener;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum EventState {
    Initing,
    Running,
    Pause,
    WaitingMsg,
    WaitingClient,
    Finished,
    Error,
}

pub struct Event {
    pub ty: SectionEvent,
    pub graph: GraphReference,
    pub tag: u32,
    pub config: &'static ConfigEvent,
    pub cur_action_index: isize,
    pub state: EventState,
    pub specials: HashMap<String, i32>,
}

impl Event {
    pub fn new(
        ty: SectionEvent,
        tag: u32,
        graph: GraphReference,
        config: &'static ConfigEvent,
    ) -> Self {
        Self {
            ty,
            graph,
            tag,
            config,
            cur_action_index: -1,
            state: EventState::Initing,
            specials: HashMap::new(), // TODO: take initial ones from graph config
        }
    }

    pub fn add_parameter(&mut self, param_config_str: Option<&str>) {
        let Some((key, value)) = param_config_str.and_then(|p| {
            let mut p = p.split(':');
            Some((p.next()?, p.next()?))
        }) else {
            return;
        };

        let Ok(value) = value.parse::<i32>() else {
            error!("failed to parse parameter string, key: {key}, value: {value}");
            return;
        };

        self.specials.insert(key.to_string(), value);
    }

    pub fn wakeup(
        &mut self,
        own_uid: EventUID,
        action_listener: &mut dyn ActionListener,
        logic_listener: &mut dyn LogicEventListener,
    ) {
        self.state = EventState::Running;

        for (index, action) in self
            .config
            .actions
            .iter()
            .enumerate()
            .skip((self.cur_action_index + 1) as usize)
        {
            self.cur_action_index = index as isize;

            let uid = ((self.graph.id() as u64) << 32) | own_uid.event_id() as u64;
            if action_listener.should_execute_action(uid, action, logic_listener, &self.specials) {
                action_listener.execute_action(uid, action, logic_listener, &self.specials);
                if let Some(client_action_info) = action::action_to_proto(action) {
                    action_listener.enqueue_client_action((own_uid, self), client_action_info);

                    if action_listener.is_immediate_mode() || action::requires_client_input(action)
                    {
                        self.state = EventState::WaitingClient;
                        return;
                    }
                }
            }
        }

        self.state = EventState::Finished;
    }

    pub fn is_persistent(&self) -> bool {
        self.ty != SectionEvent::OnInteract
    }

    pub fn is_finished(&self) -> bool {
        self.state >= EventState::Finished
    }
}

pub trait ActionListener {
    fn should_execute_action(
        &mut self,
        event_uid: u64,
        action: &ConfigEventAction,
        listener: &mut dyn LogicEventListener,
        specials: &HashMap<String, i32>,
    ) -> bool;

    fn execute_action(
        &mut self,
        event_uid: u64,
        action: &ConfigEventAction,
        listener: &mut dyn LogicEventListener,
        specials: &HashMap<String, i32>,
    );

    fn enqueue_client_action(&mut self, by_event: (EventUID, &Event), info: ActionInfo);

    fn is_immediate_mode(&self) -> bool;
}

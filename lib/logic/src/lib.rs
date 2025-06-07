use config::{EventGraphCollection, TemplateCollection, world::LevelWorldConfig};
use fight::GameFightState;
use hall::GameHallState;
use hollow::GameHollowState;
use listener::NotifyListener;
use long_fight::GameLongFightState;
use big_scene::GameBigSceneState;

pub mod battle;
pub mod debug;
pub mod dungeon;
pub mod event;
pub mod fight;
pub mod hall;
pub mod hollow;
pub mod item;
pub mod listener;
pub mod long_fight;
pub mod math;
mod predicate_util;
pub mod scene;
pub mod system;
pub mod big_scene;

macro_rules! game_state {
    ($($state:ident;)*) => {
        ::paste::paste!(pub enum GameState {
            $($state(Box<[<Game $state State>]>),)*
        });

        impl GameState {
            pub fn flush_notifies(&mut self, listener: &mut dyn NotifyListener) {
                match self {
                    $(Self::$state(state) => state.flush_notifies(listener),)*
                }
            }
        }

        ::paste::paste!($(impl From<[<Game $state State>]> for GameState {
            fn from(value: [<Game $state State>]) -> Self {
                Self::$state(Box::new(value))
            }
        })*);
    };
}

game_state! {
    Hall;
    Hollow;
    Fight;
    LongFight;
    BigScene;
}

#[derive(Clone, Copy)]
pub struct LogicResources {
    pub template_collection: &'static TemplateCollection,
    pub event_graphs: &'static EventGraphCollection,
    pub level_world: &'static LevelWorldConfig,
}

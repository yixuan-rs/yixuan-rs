use crate::{player::Player, resources::NapResources};

use std::{collections::HashMap, sync::LazyLock};

use vivian_logic::{GameState, listener::NotifyListener};
use vivian_proto::{Message, NetCmd};

type PacketHandler = fn(&mut NetContext<'_>, u16, Vec<u8>);

pub struct NetContext<'logic> {
    pub player: &'logic mut Player,
    pub game_state: &'logic mut Option<GameState>,
    pub resources: &'static NapResources,
    pub notifies: NotifyQueue,
    pub response: Option<(u16, Vec<u8>)>,
}

handlers! {
    Player;
    Avatar;
    Item;
    Quest;
    Buddy;
    Misc;
    Gacha;
    Ramen;
    World;
    Social;
    Training;
    PhotoWall;
    MonthReward;
    Map;
}

#[derive(Default)]
pub struct NotifyQueue(Vec<(u16, Vec<u8>)>);

impl<'player> NetContext<'player> {
    pub fn new(
        player: &'player mut Player,
        game_state: &'player mut Option<GameState>,
        resources: &'static NapResources,
    ) -> Self {
        Self {
            player,
            game_state,
            resources,
            notifies: NotifyQueue::default(),
            response: None,
        }
    }
}

impl NotifyQueue {
    pub fn prepend_notify<Notify: NetCmd + Message>(&mut self, ntf: Notify) {
        self.0.insert(0, (ntf.get_cmd_id(), ntf.encode_to_vec()));
    }

    pub fn drain(&mut self) -> Vec<(u16, Vec<u8>)> {
        std::mem::take(&mut self.0)
    }
}

impl NotifyListener for NotifyQueue {
    fn push(&mut self, cmd_id: u16, body: Vec<u8>) {
        self.0.push((cmd_id, body));
    }
}

macro_rules! handlers {
    ($($module:ident;)*) => {
        $(
            ::paste::paste!(mod [<$module:snake>];);
        )*

        pub fn handle_command(context: &mut NetContext, cmd_id: u16, body: Vec<u8>) {
            static HANDLER_MAP: LazyLock<HashMap<u16, PacketHandler>> = LazyLock::new(|| {
                let mut map = HashMap::new();
                $(::paste::paste!([<$module:snake>]::[<$module Handler>]::register_handlers(&mut map););)*

                map
            });

            if let Some(handler_fn) = HANDLER_MAP.get(&cmd_id) {
                handler_fn(context, cmd_id, body);
            } else {
                if context.player.loading_state == crate::player::LoadingState::ExtendDataSync {
                    ::tracing::debug!("ExtendDataSync request ({cmd_id}) is unhandled, sending dummy one in response!");
                    context.response = Some((0, Vec::new()));
                }
                else {
                    ::tracing::warn!("unhandled client command: {cmd_id}");
                }
            }

            context.player.update(context.game_state);
        }
    };
}

use handlers;

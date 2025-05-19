use super::{
    chessboard::{EventGraphContext, GridReference},
    expression_util,
};

actions! {
    ConfigEventModification;
    ConfigPushWithDirection;
    ConfigSetMapState;
    ConfigReward;
    ConfigAddItem;
    ConfigSetHollowVariable;
    ConfigFinishHollow;
}

pub fn execute_config_event_modification(
    ctx: &mut EventGraphContext,
    cfg: &ConfigEventModification,
) {
    let grid_pos = match cfg.position {
        HollowPositionOffsetType::Absolute => GridReference::Absolute(cfg.x, cfg.y),
        HollowPositionOffsetType::Relative => GridReference::Relative(cfg.x, cfg.y),
        other => return warn!("ConfigEventModification offset {other:?} is not implemented yet"),
    };

    if cfg.target_event_id.is_empty() {
        if let Some(&set_event_id) = cfg.event_id.first() {
            ctx.chessboard.modify_grid_event(
                grid_pos,
                0,
                set_event_id,
                NodeState::try_from(i32::from(cfg.event_state)).unwrap(),
                NodeVisible::try_from(i32::from(cfg.visible_state)).unwrap(),
            );
        }
    } else {
        for (&target_event_id, &set_event_id) in cfg.target_event_id.iter().zip(cfg.event_id.iter())
        {
            if ctx.chessboard.modify_grid_event(
                grid_pos,
                target_event_id,
                set_event_id,
                NodeState::try_from(i32::from(cfg.event_state)).unwrap(),
                NodeVisible::try_from(i32::from(cfg.visible_state)).unwrap(),
            ) {
                break;
            }
        }
    }
}

pub fn execute_config_set_map_state(ctx: &mut EventGraphContext, cfg: &ConfigSetMapState) {
    let grid_pos = match cfg.position {
        HollowPositionOffsetType::Absolute => GridReference::Absolute(cfg.x, cfg.y),
        HollowPositionOffsetType::Relative => GridReference::Relative(cfg.x, cfg.y),
        other => return warn!("ConfigSetMapState offset {other:?} is not implemented yet"),
    };

    let mut visible_state_changes = cfg
        .from_visible_state
        .iter()
        .copied()
        .zip(cfg.to_visible_state.iter().copied())
        .peekable();

    let mut state_changes = cfg
        .from_state
        .iter()
        .copied()
        .zip(cfg.to_state.iter().copied())
        .peekable();

    while visible_state_changes.peek().is_some() || state_changes.peek().is_some() {
        if ctx.chessboard.set_map_state(
            grid_pos,
            visible_state_changes
                .next()
                .map(|(from_visible, to_visible)| {
                    (
                        NodeVisible::try_from(i32::from(from_visible)).unwrap(),
                        NodeVisible::try_from(i32::from(to_visible)).unwrap(),
                    )
                }),
            state_changes.next().map(|(from_state, to_state)| {
                (
                    NodeState::try_from(i32::from(from_state)).unwrap(),
                    NodeState::try_from(i32::from(to_state)).unwrap(),
                )
            }),
        ) {
            break;
        }
    }
}

pub fn execute_config_push_with_direction(
    ctx: &mut EventGraphContext,
    cfg: &ConfigPushWithDirection,
) {
    let pos = ctx.chessboard.cur_player_pos();
    let Ok(dir) =
        ConfigGridDir::try_from(expression_util::get_dynamic_integer(&cfg.direction, ctx))
    else {
        return;
    };

    let pos = match dir {
        ConfigGridDir::Left => Vector2Int {
            x: pos.x - 1,
            ..pos
        },
        ConfigGridDir::Right => Vector2Int {
            x: pos.x + 1,
            ..pos
        },
        ConfigGridDir::Up => Vector2Int {
            y: pos.y - 1,
            ..pos
        },
        ConfigGridDir::Down => Vector2Int {
            y: pos.y + 1,
            ..pos
        },
    };

    ctx.chessboard.player_move(pos, ctx.listener, true);
}

pub fn execute_config_reward(ctx: &mut EventGraphContext, cfg: &ConfigReward) {
    let once_reward_id = expression_util::get_dynamic_integer(&cfg.once_reward_id, ctx);
    if once_reward_id != 0 {
        ctx.listener.give_once_reward(once_reward_id as u32);
    }
}

pub fn execute_config_add_item(ctx: &mut EventGraphContext, cfg: &ConfigAddItem) {
    let count = expression_util::get_dynamic_integer(&cfg.count, ctx);
    ctx.listener.add_item(cfg.item_id, count as u32);
}

pub fn execute_config_set_hollow_variable(
    ctx: &mut EventGraphContext,
    cfg: &ConfigSetHollowVariable,
) {
    ctx.chessboard
        .hollow_variables
        .insert(cfg.key.clone(), cfg.value);
}

pub fn execute_config_finish_hollow(ctx: &mut EventGraphContext, _: &ConfigFinishHollow) {
    ctx.chessboard.finished = true;
}

macro_rules! actions {
    ($($cfg:ident;)*) => {
        pub fn execute(context: &mut EventGraphContext<'_>, cfg: &::config::ConfigEventAction) {
            ::paste::paste!(match cfg {
                $(::config::ConfigEventAction::$cfg(cfg) => [<execute_ $cfg:snake>](context, cfg),)*
                _ => (),
            });
        }
    };
}

use actions;
use config::{
    ConfigAddItem, ConfigEventModification, ConfigFinishHollow, ConfigGridDir,
    ConfigPushWithDirection, ConfigReward, ConfigSetHollowVariable, ConfigSetMapState,
    HollowPositionOffsetType,
};
use tracing::warn;
use vivian_proto::common::{NodeState, NodeVisible, Vector2Int};

use crate::predicate_util;

use super::chessboard::EventGraphContext;

predicates! {
    ConfigEventByMainCharacter;
    ConfigEventByHollowVariable;
    ConfigEventByChoiceServer;
}

fn execute_config_event_by_main_character(
    context: &mut EventGraphContext<'_>,
    predicate: &ConfigEventByMainCharacter,
) -> bool {
    predicate_util::run_compare_function(
        predicate.compare_type,
        context.chessboard.player_avatar_id,
        predicate.avatar_id,
    )
}

fn execute_config_event_by_hollow_variable(
    context: &mut EventGraphContext<'_>,
    predicate: &ConfigEventByHollowVariable,
) -> bool {
    predicate_util::run_compare_function(
        predicate.compare_type,
        context
            .chessboard
            .hollow_variables
            .get(&predicate.key)
            .copied()
            .unwrap_or_default(),
        predicate.count,
    )
}

fn execute_config_event_by_choice_server(
    context: &mut EventGraphContext<'_>,
    predicate: &ConfigEventByChoiceServer,
) -> bool {
    predicate_util::run_compare_function(
        predicate.compare_type,
        context.chessboard.event_choice_uid,
        predicate.uid,
    )
}

macro_rules! predicates {
    ($($cfg:ident;)*) => {
        pub fn execute(context: &mut EventGraphContext<'_>, cfg: &::config::ConfigPredicate) -> bool {
            ::paste::paste!(match cfg {
                $(::config::ConfigPredicate::$cfg(cfg) => [<execute_ $cfg:snake>](context, cfg),)*
            })
        }
    };
}

use config::{ConfigEventByChoiceServer, ConfigEventByHollowVariable, ConfigEventByMainCharacter};
use predicates;

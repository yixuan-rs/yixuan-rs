predicates! {
    ConfigEventByMainCharacter;
}

fn execute_config_event_by_main_character(
    context: &mut EventGraphContext<'_>,
    predicate: &ConfigEventByMainCharacter,
) -> bool {
    predicate_util::run_compare_function(
        predicate.compare_type,
        context.hall.player_avatar_id,
        predicate.avatar_id,
    )
}

macro_rules! predicates {
    ($($cfg:ident;)*) => {
        pub fn execute(context: &mut EventGraphContext<'_>, cfg: &::config::ConfigPredicate) -> bool {
            ::paste::paste!(match cfg {
                $(::config::ConfigPredicate::$cfg(cfg) => [<execute_ $cfg:snake>](context, cfg),)*
                _ => false,
            })
        }
    };
}

use config::ConfigEventByMainCharacter;
use predicates;

use crate::predicate_util;

use super::EventGraphContext;

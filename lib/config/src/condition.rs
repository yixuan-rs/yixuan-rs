use std::{collections::HashMap, str::FromStr};

use num_enum::TryFromPrimitive;

use crate::{ConditionConfigTemplate, EQuestState};

#[derive(Debug)]
pub enum Condition {
    HasExecutedHollowEvent {
        event_ids: Vec<u32>,
        target_num: u32,
    },
    QuestState {
        id: u32,
        state: EQuestState,
    },
    ClientSystemOpen {
        id: u32,
    },
    FinishNewbie {
        group_id: u32,
    },
    SignedNewsStandToday,
}

#[derive(Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum ConditionType {
    HasExecutedHollowEvent = 3003,
    QuestStateGreaterOrEqual = 3010,
    ClientSystemOpen = 3012,
    QuestStateEqual = 3014,
    FinishNewbie = 3080,
    SignedNewsStandToday = 3114,
}

#[derive(thiserror::Error, Debug)]
pub enum ConditionParseError {
    #[error("unknown condition type: {0}")]
    UnknownType(u32),
    #[error("missing condition argument: {0}")]
    MissingArgument(&'static str),
    #[error("invalid condition argument format: {0}")]
    InvalidArgumentFormat(&'static str),
}

impl Condition {
    pub fn parse_from_template(
        tmpl: &ConditionConfigTemplate,
    ) -> Result<Self, ConditionParseError> {
        type Error = ConditionParseError;

        let args = Self::parse_args_string(tmpl.args().unwrap_or_default());

        Ok(
            match ConditionType::try_from(tmpl.statis_type())
                .map_err(|_| Error::UnknownType(tmpl.statis_type()))?
            {
                ConditionType::HasExecutedHollowEvent => Condition::HasExecutedHollowEvent {
                    event_ids: (0..Self::named_arg_count(&args, "EventID")?)
                        .map(|i| Self::get_argument(&args, "EventID", i))
                        .collect::<Result<_, _>>()?,
                    target_num: tmpl.target_num(),
                },
                ConditionType::QuestStateEqual | ConditionType::QuestStateGreaterOrEqual => {
                    Condition::QuestState {
                        id: Self::get_argument(&args, "ID", 0)?,
                        state: EQuestState::try_from(Self::get_argument::<i32>(&args, "State", 0)?)
                            .unwrap(),
                    }
                }
                ConditionType::ClientSystemOpen => Condition::ClientSystemOpen {
                    id: Self::get_argument(&args, "ID", 0)?,
                },
                ConditionType::FinishNewbie => Condition::FinishNewbie {
                    group_id: Self::get_argument(&args, "GroupID", 0)?,
                },
                ConditionType::SignedNewsStandToday => Condition::SignedNewsStandToday,
            },
        )
    }

    fn get_argument<T: FromStr>(
        args: &HashMap<&str, Vec<&str>>,
        name: &'static str,
        i: usize,
    ) -> Result<T, ConditionParseError> {
        args.get(name)
            .ok_or(ConditionParseError::MissingArgument(name))?
            .get(i)
            .unwrap()
            .parse::<T>()
            .map_err(|_| ConditionParseError::InvalidArgumentFormat(name))
    }

    fn named_arg_count(
        args: &HashMap<&str, Vec<&str>>,
        name: &'static str,
    ) -> Result<usize, ConditionParseError> {
        Ok(args
            .get(name)
            .ok_or(ConditionParseError::MissingArgument(name))?
            .len())
    }

    fn parse_args_string(args: &str) -> HashMap<&str, Vec<&str>> {
        let mut map = HashMap::new();

        for entry in args.split('|') {
            if let [name, value] = entry.split(':').collect::<Vec<_>>().as_slice() {
                map.entry(*name).or_insert_with(Vec::new).push(*value);
            }
        }

        map
    }
}

pub enum BoundConditions {
    None,
    One(i32),
    All(Vec<i32>),
    Oneof(Vec<i32>),
}

impl BoundConditions {
    pub fn parse(s: &str) -> Self {
        if s.is_empty() {
            return Self::None;
        }

        if s.contains('|') {
            Self::Oneof(s.split('|').map(|s| s.parse().unwrap()).collect())
        } else if s.contains('&') {
            Self::All(s.split('&').map(|s| s.parse().unwrap()).collect())
        } else {
            Self::One(s.parse().unwrap())
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("failed to load condition {0}, cause: {1}")]
pub struct LoadConditionsError(i32, ConditionParseError);

pub fn load_all_conditions<'tb>(
    table: impl Iterator<Item = ConditionConfigTemplate<'tb>>,
) -> Result<HashMap<i32, Condition>, LoadConditionsError> {
    let mut map = HashMap::new();

    for tmpl in table {
        match Condition::parse_from_template(&tmpl) {
            Ok(condition) => {
                map.insert(tmpl.condition_id(), condition);
            }
            Err(ConditionParseError::UnknownType(_)) => (),
            Err(ConditionParseError::MissingArgument(_)) => (),
            Err(err) => return Err(LoadConditionsError(tmpl.condition_id(), err)),
        }
    }

    Ok(map)
}

use super::*;
use property::{Property, PropertyHashMap, PropertyHashSet};

#[derive(Model)]
pub struct HollowModel {
    pub hollow_groups: PropertyHashSet<u32>,
    pub new_hollow_groups: PropertyHashSet<u32>,
    pub unlocked_hollows: PropertyHashSet<u32>,
    pub new_unlocked_hollows: PropertyHashSet<u32>,
    pub hollows: PropertyHashMap<u32, Hollow>,
    pub executed_hollow_events: PropertyHashSet<u32>,
}

pub struct Hollow {
    pub hollow_quest_id: u32,
    pub finished: bool,
    pub acquired_hollow_challenge_reward: u32,
}

impl HollowModel {
    pub fn load_from_pb(pb: HollowData) -> Self {
        Self {
            hollow_groups: pb.hollow_group_list.into_iter().collect(),
            new_hollow_groups: pb.unlock_hollow_group_list.into_iter().collect(),
            unlocked_hollows: pb.hollow_id_list.into_iter().collect(),
            new_unlocked_hollows: pb.unlock_hollow_id_list.into_iter().collect(),
            hollows: pb
                .hollow_list
                .into_iter()
                .map(|hollow| {
                    (
                        hollow.hollow_quest_id,
                        Hollow {
                            hollow_quest_id: hollow.hollow_quest_id,
                            finished: hollow.finished,
                            acquired_hollow_challenge_reward: hollow
                                .acquired_hollow_challenge_reward,
                        },
                    )
                })
                .collect(),
            executed_hollow_events: pb.executed_hollow_event_id_list.into_iter().collect(),
        }
    }
}

impl Saveable for HollowModel {
    fn save_to_pb(&self, root: &mut yixuan_proto::server_only::PlayerData) {
        root.hollow = Some(HollowData {
            hollow_group_list: self.hollow_groups.iter().copied().collect(),
            unlock_hollow_group_list: self.new_hollow_groups.iter().copied().collect(),
            hollow_id_list: self.unlocked_hollows.iter().copied().collect(),
            unlock_hollow_id_list: self.new_unlocked_hollows.iter().copied().collect(),
            hollow_list: self
                .hollows
                .iter()
                .map(|(_, hollow)| HollowInfo {
                    hollow_quest_id: hollow.hollow_quest_id,
                    finished: hollow.finished,
                    acquired_hollow_challenge_reward: hollow.acquired_hollow_challenge_reward,
                })
                .collect(),
            executed_hollow_event_id_list: self.executed_hollow_events.iter().copied().collect(),
        });
    }
}

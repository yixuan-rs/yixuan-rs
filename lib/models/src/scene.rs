use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use super::*;
use config::{GraphReference, SectionEvent};
use property::{PrimitiveProperty, Property, PropertyHashMap};
use yixuan_logic::{
    dungeon::{Dungeon, DungeonEquipment},
    event::{EventState, EventUID, GraphID},
    hall::npc::InteractTarget,
    math::Scale,
};

#[derive(Model)]
pub struct SceneModel {
    pub cur_scene_uid: PrimitiveProperty<u64>,
    pub default_scene_uid: PrimitiveProperty<u64>,
    pub scene_snapshots: PropertyHashMap<u64, SceneSnapshot>,
    pub dungeons: PropertyHashMap<u64, Dungeon>,
}

pub struct SceneSnapshot {
    pub scene_id: u32,
    pub dungeon_uid: u64,
    pub back_scene_uid: u64,
    pub to_be_destroyed: bool,
    pub ext: SceneSnapshotExt,
}

pub enum SceneSnapshotExt {
    Hall(HallSceneSnapshot),
    Hollow(HollowSceneSnapshot),
    Fight(FightSceneSnapshot),
    LongFight(LongFightSceneSnapshot),
}

pub struct HallSceneSnapshot {
    pub cur_section_id: u32,
    pub sections: HashMap<u32, HallSectionSnapshot>,
    pub main_city_objects_state: HashMap<i32, i32>,
}

pub struct HallSectionSnapshot {
    pub section_id: u32,
    pub scene_units: Vec<HallSceneUnitSnapshot>,
    pub attached_graphs: HashSet<GraphReference>,
    pub event_snapshots: Vec<EventSnapshot>,
    pub already_executed_events: HashSet<u64>,
}

pub struct EventSnapshot {
    pub graph_id: u32,
    pub graph_type: GraphReferenceType,
    pub ty: SectionEvent,
    pub uid: EventUID,
    pub tag: u32,
    pub state: EventState,
    pub cur_action_idx: isize,
}

pub struct HallSceneUnitSnapshot {
    pub npc_tag_id: u32,
    pub is_interactable: bool,
    pub interacts: HashMap<u32, InteractSnapshot>,
}

pub struct InteractSnapshot {
    pub tag_id: i32,
    pub name: String,
    pub scale: Scale,
    pub participators: HashMap<u32, String>,
    pub targets: HashSet<InteractTarget>,
}

pub struct HollowSceneSnapshot {}

pub struct FightSceneSnapshot {
    pub time_period: String,
    pub weather: String,
}

pub struct LongFightSceneSnapshot {
    pub time_period: String,
    pub weather: String,
}

impl SceneModel {
    pub fn create_pure_fight_dungeon(
        &mut self,
        quest_id: u32,
        quest_type: u32,
        scene_id: u32,
        equip: DungeonEquipment,
    ) -> (u64, u64) {
        let dungeon_uid = self.next_dungeon_uid();
        let scene_uid = self.next_scene_uid();

        let mut dungeon = Dungeon::new(quest_id, quest_type);
        dungeon.inner_quest_id_list.push(quest_id);
        dungeon.equipment = equip;

        self.dungeons.insert(dungeon_uid, dungeon);
        self.scene_snapshots.insert(
            scene_uid,
            SceneSnapshot {
                scene_id,
                dungeon_uid,
                back_scene_uid: 0,
                to_be_destroyed: true,
                ext: SceneSnapshotExt::Fight(FightSceneSnapshot {
                    time_period: String::from("Morning"),
                    weather: String::from("SunShine"),
                }),
            },
        );

        (dungeon_uid, scene_uid)
    }

    pub fn create_hollow_dungeon(
        &mut self,
        quest_id: u32,
        scene_id: u32,
        equip: DungeonEquipment,
    ) -> (u64, u64) {
        let dungeon_uid = self.next_dungeon_uid();
        let scene_uid = self.next_scene_uid();

        let mut dungeon = Dungeon::new(quest_id, 3);
        dungeon.equipment = equip;

        self.dungeons.insert(dungeon_uid, dungeon);
        self.scene_snapshots.insert(
            scene_uid,
            SceneSnapshot {
                scene_id,
                dungeon_uid,
                back_scene_uid: 0,
                to_be_destroyed: true,
                ext: SceneSnapshotExt::Hollow(HollowSceneSnapshot {}),
            },
        );

        (dungeon_uid, scene_uid)
    }

    pub fn create_long_fight_dungeon(
        &mut self,
        quest_id: u32,
        scene_id: u32,
        equip: DungeonEquipment,
    ) -> (u64, u64) {
        let dungeon_uid = self.next_dungeon_uid();
        let scene_uid = self.next_scene_uid();

        let mut dungeon = Dungeon::new(quest_id, 3);
        dungeon.equipment = equip;
        dungeon.inner_quest_id_list.push(scene_id);

        self.dungeons.insert(dungeon_uid, dungeon);
        self.scene_snapshots.insert(
            scene_uid,
            SceneSnapshot {
                scene_id,
                dungeon_uid,
                back_scene_uid: 0,
                to_be_destroyed: true,
                ext: SceneSnapshotExt::LongFight(LongFightSceneSnapshot {
                    time_period: String::from("Morning"),
                    weather: String::from("SunShine"),
                }),
            },
        );

        (dungeon_uid, scene_uid)
    }

    pub fn clear_scene_data(&mut self, scene_uid: u64) {
        // Destroy scene, if needed

        if self
            .scene_snapshots
            .get(&scene_uid)
            .map(|s| s.to_be_destroyed)
            .unwrap_or(true)
        {
            if let Some(scene) = self.scene_snapshots.remove(&scene_uid) {
                if scene.dungeon_uid != 0
                    && !self
                        .scene_snapshots
                        .iter()
                        .any(|(_, s)| s.dungeon_uid == scene.dungeon_uid)
                {
                    // If no scenes referring this dungeon left, destroy it as well
                    self.dungeons.remove(&scene.dungeon_uid);
                }

                if scene.back_scene_uid != 0 {
                    self.cur_scene_uid.set(scene.back_scene_uid);
                    return;
                }
            }
        }

        self.cur_scene_uid.set(self.default_scene_uid.get());
    }

    pub fn load_from_pb(pb: SceneData) -> Self {
        use yixuan_proto::server_only::scene_info::Info;

        Self {
            cur_scene_uid: pb.cur_scene_uid.into(),
            default_scene_uid: pb.default_scene_uid.into(),
            scene_snapshots: pb
                .scenes
                .into_iter()
                .map(|(uid, info)| {
                    (
                        uid,
                        SceneSnapshot {
                            scene_id: info.id,
                            dungeon_uid: info.dungeon_uid,
                            back_scene_uid: info.back_scene_uid,
                            to_be_destroyed: info.to_be_destroyed,
                            ext: match info.info.unwrap() {
                                Info::Hall(hall) => {
                                    SceneSnapshotExt::Hall(HallSceneSnapshot::load_from_pb(hall))
                                }
                                Info::Hollow(hollow) => SceneSnapshotExt::Hollow(
                                    HollowSceneSnapshot::load_from_pb(hollow),
                                ),
                                Info::Fight(fight) => {
                                    SceneSnapshotExt::Fight(FightSceneSnapshot::load_from_pb(fight))
                                }
                                Info::LongFight(fight) => SceneSnapshotExt::LongFight(
                                    LongFightSceneSnapshot::load_from_pb(fight),
                                ),
                            },
                        },
                    )
                })
                .collect(),
            dungeons: pb
                .dungeons
                .into_iter()
                .map(|(uid, pb)| (uid, Dungeon::new(pb.quest_id, pb.quest_type)))
                .collect(),
        }
    }

    pub fn next_scene_uid(&self) -> u64 {
        self.scene_snapshots.keys().max().copied().unwrap_or(0) + 1
    }

    pub fn next_dungeon_uid(&self) -> u64 {
        self.dungeons.keys().max().copied().unwrap_or(0) + 1
    }
}

impl Saveable for SceneModel {
    fn save_to_pb(&self, root: &mut yixuan_proto::server_only::PlayerData) {
        use yixuan_proto::server_only::scene_info::Info;

        root.scene = Some(SceneData {
            cur_scene_uid: self.cur_scene_uid.get(),
            default_scene_uid: self.default_scene_uid.get(),
            scenes: self
                .scene_snapshots
                .iter()
                .map(|(&uid, snapshot)| {
                    (
                        uid,
                        SceneInfo {
                            id: snapshot.scene_id,
                            dungeon_uid: snapshot.dungeon_uid,
                            back_scene_uid: snapshot.back_scene_uid,
                            to_be_destroyed: snapshot.to_be_destroyed,
                            info: Some(match &snapshot.ext {
                                SceneSnapshotExt::Hall(hall) => Info::Hall(hall.save_to_pb()),
                                SceneSnapshotExt::Hollow(hollow) => {
                                    Info::Hollow(hollow.save_to_pb())
                                }
                                SceneSnapshotExt::Fight(fight) => Info::Fight(fight.save_to_pb()),
                                SceneSnapshotExt::LongFight(fight) => {
                                    Info::LongFight(fight.save_to_pb())
                                }
                            }),
                        },
                    )
                })
                .collect(),
            dungeons: self
                .dungeons
                .iter()
                // TODO: proper dungeon saving with all the inner objects
                .map(|(&uid, dungeon)| {
                    (
                        uid,
                        DungeonInfo {
                            quest_id: dungeon.quest_id,
                            quest_type: dungeon.quest_type,
                        },
                    )
                })
                .collect(),
        });
    }
}

impl HallSceneSnapshot {
    pub fn load_from_pb(pb: HallSceneInfo) -> Self {
        Self {
            cur_section_id: pb.cur_section_id,
            sections: pb
                .sections
                .into_iter()
                .map(|section| {
                    (
                        section.section_id,
                        HallSectionSnapshot {
                            section_id: section.section_id,
                            scene_units: section
                                .scene_unit_list
                                .into_iter()
                                .map(|unit| HallSceneUnitSnapshot {
                                    npc_tag_id: unit.npc_tag_id,
                                    is_interactable: unit.is_interactable,
                                    interacts: unit
                                        .interacts
                                        .into_iter()
                                        .map(|(id, interact)| {
                                            (
                                                id,
                                                InteractSnapshot {
                                                    name: interact.name,
                                                    scale: Scale(
                                                        interact.scale_x,
                                                        interact.scale_y,
                                                        interact.scale_z,
                                                        interact.scale_w,
                                                        interact.scale_r,
                                                    ),
                                                    tag_id: interact.tag_id,
                                                    participators: interact.participators.clone(),
                                                    targets: interact
                                                        .interact_target_list
                                                        .into_iter()
                                                        .filter_map(|tgt| {
                                                            InteractTarget::try_from(tgt).ok()
                                                        })
                                                        .collect(),
                                                },
                                            )
                                        })
                                        .collect(),
                                })
                                .collect(),
                            attached_graphs: section
                                .attached_graph_list
                                .into_iter()
                                .filter_map(|info| {
                                    Some(match info.reference_type() {
                                        GraphReferenceType::Interact => {
                                            GraphReference::Interact(info.reference_id)
                                        }
                                        GraphReferenceType::HollowEvent => {
                                            GraphReference::HollowEvent(info.reference_id)
                                        }
                                        GraphReferenceType::Quest => {
                                            GraphReference::Quest(info.reference_id)
                                        }
                                        GraphReferenceType::MainCitySection => {
                                            GraphReference::MainCitySection(info.reference_id)
                                        }
                                        GraphReferenceType::None => return None,
                                    })
                                })
                                .collect(),
                            event_snapshots: section
                                .event_state_list
                                .into_iter()
                                .map(|info| EventSnapshot {
                                    graph_id: info.graph_reference_id,
                                    graph_type: info.graph_reference_type(),
                                    ty: SectionEvent::from_str(&info.name).unwrap(),
                                    uid: info.event_uid.into(),
                                    state: EventState::try_from(info.event_state).unwrap(),
                                    cur_action_idx: info.cur_action_idx as isize,
                                    tag: info.tag,
                                })
                                .collect(),
                            already_executed_events: section
                                .already_executed_event_uid_list
                                .into_iter()
                                .collect(),
                        },
                    )
                })
                .collect(),
            main_city_objects_state: pb.main_city_objects_state.clone(),
        }
    }

    pub fn save_to_pb(&self) -> HallSceneInfo {
        HallSceneInfo {
            cur_section_id: self.cur_section_id,
            sections: self
                .sections
                .values()
                .map(|section| HallSectionInfo {
                    section_id: section.section_id,
                    scene_unit_list: section
                        .scene_units
                        .iter()
                        .map(|unit| HallSceneUnit {
                            npc_tag_id: unit.npc_tag_id,
                            is_interactable: unit.is_interactable,
                            interacts: unit
                                .interacts
                                .iter()
                                .map(|(&id, interact)| {
                                    (
                                        id,
                                        UnitInteract {
                                            name: interact.name.clone(),
                                            scale_x: interact.scale.0,
                                            scale_y: interact.scale.1,
                                            scale_z: interact.scale.2,
                                            scale_w: interact.scale.3,
                                            scale_r: interact.scale.4,
                                            tag_id: interact.tag_id,
                                            participators: interact.participators.clone(),
                                            interact_target_list: interact
                                                .targets
                                                .iter()
                                                .map(|&tgt| tgt.into())
                                                .collect(),
                                        },
                                    )
                                })
                                .collect(),
                        })
                        .collect(),
                    attached_graph_list: section
                        .attached_graphs
                        .iter()
                        .map(|graph_ref| {
                            let GraphID(id, ty) = GraphID::from(*graph_ref);
                            AttachedGraphInfo {
                                reference_id: id,
                                reference_type: ty.into(),
                            }
                        })
                        .collect(),
                    event_state_list: section
                        .event_snapshots
                        .iter()
                        .map(|event| EventStateInfo {
                            graph_reference_id: event.graph_id,
                            graph_reference_type: event.graph_type.into(),
                            name: event.ty.to_string(),
                            event_uid: *event.uid,
                            event_state: event.state.into(),
                            cur_action_idx: event.cur_action_idx as i32,
                            tag: event.tag,
                        })
                        .collect(),
                    already_executed_event_uid_list: section
                        .already_executed_events
                        .iter()
                        .copied()
                        .collect(),
                })
                .collect(),
            main_city_objects_state: self.main_city_objects_state.clone(),
        }
    }
}

impl HollowSceneSnapshot {
    pub fn load_from_pb(_pb: HollowSceneInfo) -> Self {
        Self {}
    }

    pub fn save_to_pb(&self) -> HollowSceneInfo {
        HollowSceneInfo {}
    }
}

impl FightSceneSnapshot {
    pub fn load_from_pb(pb: FightSceneInfo) -> Self {
        Self {
            time_period: pb.time_period,
            weather: pb.weather,
        }
    }

    pub fn save_to_pb(&self) -> FightSceneInfo {
        FightSceneInfo {
            time_period: self.time_period.clone(),
            weather: self.weather.clone(),
        }
    }
}

impl LongFightSceneSnapshot {
    pub fn load_from_pb(pb: LongFightSceneInfo) -> Self {
        Self {
            time_period: pb.time_period,
            weather: pb.weather,
        }
    }

    pub fn save_to_pb(&self) -> LongFightSceneInfo {
        LongFightSceneInfo {
            time_period: self.time_period.clone(),
            weather: self.weather.clone(),
        }
    }
}

impl EventSnapshot {
    pub fn graph_reference(&self) -> Option<GraphReference> {
        Some(match self.graph_type {
            GraphReferenceType::Interact => GraphReference::Interact(self.graph_id),
            GraphReferenceType::HollowEvent => GraphReference::HollowEvent(self.graph_id),
            GraphReferenceType::Quest => GraphReference::Quest(self.graph_id),
            GraphReferenceType::MainCitySection => GraphReference::MainCitySection(self.graph_id),
            GraphReferenceType::None => return None,
        })
    }
}

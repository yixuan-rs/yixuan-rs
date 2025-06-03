use std::collections::{HashMap, HashSet};

use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::math::Scale;

#[derive(Debug)]
pub struct SceneUnit {
    pub npc_id: u32,
    pub interacts: HashMap<u32, Interact>,
}

#[derive(Debug)]
pub struct Interact {
    pub id: u32,
    pub tag_id: i32,
    pub name: String,
    pub scale: Scale,
    pub targets: HashSet<InteractTarget>,
    pub participators: HashMap<u32, String>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum InteractTarget {
    TriggerBox = 1,
    Npc = 2,
}

impl SceneUnit {
    pub fn to_client_proto(&self) -> yixuan_proto::SceneUnitProtocolInfo {
        yixuan_proto::SceneUnitProtocolInfo {
            npc_id: self.npc_id,
            is_interactable: true,
            interacts_info: self
                .interacts
                .iter()
                .map(|(&id, interact)| {
                    (
                        id,
                        yixuan_proto::InteractInfo {
                            tag_id: interact.tag_id,
                            name: interact.name.clone(),
                            scale_x: interact.scale.0,
                            scale_y: interact.scale.1,
                            scale_z: interact.scale.2,
                            scale_w: interact.scale.3,
                            scale_r: interact.scale.4,
                            interact_target_list: interact
                                .targets
                                .iter()
                                .map(|target| match target {
                                    InteractTarget::Npc => yixuan_proto::InteractTarget::Npc.into(),
                                    InteractTarget::TriggerBox => {
                                        yixuan_proto::InteractTarget::TriggerBox.into()
                                    }
                                })
                                .collect(),
                            participators: interact.participators.clone(),
                            unk_interact_info_bool: true,
                        },
                    )
                })
                .collect(),
        }
    }
}

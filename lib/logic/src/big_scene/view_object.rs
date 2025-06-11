use std::collections::{HashMap, HashSet};

use config::{
    EntityType,
    world::{ConfigGroupMember, ViewObjectTrait},
};
use tracing::debug;
use yixuan_proto::{
    GroupMemberCreateScNotify, GroupOrderReadyScNotify, GroupOrderReason, GroupOrderReasonInfo,
    SceneEntityAppearScNotify, ViewType,
    common::{SceneAvatarInfo, SceneEntityInfo, SceneMonsterInfo, scene_entity_info},
};

use crate::{
    LogicResources,
    listener::{NotifyListener, NotifyListenerExt},
    math::Vector3i,
};

use super::{
    component::{
        ComponentContainer, ViewObjectExtension, VoBattleAvatarMemberComponent,
        VoMonsterProxyComponent,
    },
    event::ViewObjectEventBase,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ViewObjectHandle(pub u32);

pub struct GroupOrderReady {
    pub group_id: u32,
    pub view_object_handle_list: Vec<ViewObjectHandle>,
    pub view_object_handle_mark_set: HashSet<ViewObjectHandle>,
    pub is_all_ready: bool,
}

pub struct ViewObjectManager {
    resources: LogicResources,
    floor_id: u32,
    net_id_counter: u32,
    internal_view_object_map: HashMap<ViewObjectHandle, ViewObject>,
    group_order_ready_map: HashMap<u32, GroupOrderReady>,
    active_gameplay_tags: HashSet<String>,
    appear_objects: HashSet<ViewObjectHandle>,
    component_container: ComponentContainer,
    pub team_avatar_handle: Option<ViewObjectHandle>,
}

#[derive(Debug)]
pub struct ViewObject {
    pub net_id: u32,
    pub group_id: u32,
    pub config_id: u32,
    pub position: Vector3i,
    pub rotation: Vector3i,
}

impl ViewObjectManager {
    pub fn new(floor_id: u32, res: LogicResources) -> Self {
        Self {
            resources: res,
            floor_id,
            net_id_counter: 0,
            internal_view_object_map: HashMap::new(),
            group_order_ready_map: HashMap::new(),
            active_gameplay_tags: HashSet::new(),
            appear_objects: HashSet::new(),
            component_container: ComponentContainer::default(),
            team_avatar_handle: None,
        }
    }

    pub fn create_team_avatar(
        &mut self,
        avatar_id: u32,
        pos: Vector3i,
        rot: Vector3i,
    ) -> ViewObjectHandle {
        let net_id = self.next_net_id(EntityType::Avatar);
        let handle = ViewObjectHandle(net_id);

        self.internal_view_object_map.insert(
            handle,
            ViewObject {
                net_id,
                group_id: 0,
                config_id: 0,
                position: pos,
                rotation: rot,
            },
        );

        self.component_container.add_component(
            handle,
            VoBattleAvatarMemberComponent {
                current_avatar_id: avatar_id,
            },
        );

        self.appear_objects.insert(handle);

        handle
    }

    pub fn load_all_groups(&mut self) {
        let floor_config = self
            .resources
            .level_world
            .floors
            .get(&self.floor_id)
            .unwrap();

        floor_config
            .group_id_list
            .iter()
            .flat_map(|group_id| self.resources.level_world.groups.get(group_id))
            .for_each(|group| {
                let view_object_handle_list = group
                    .members
                    .iter()
                    .flat_map(|member| {
                        (self.is_default_new_mode_monster_object(member.view_object_id)
                            || self.is_active_tagged_monster_object(member.view_object_id))
                        .then(|| self.spawn_monster_object(group.group_id, member))
                    })
                    .collect::<Vec<_>>();

                if !view_object_handle_list.is_empty() {
                    self.group_order_ready_map.insert(
                        group.group_id,
                        GroupOrderReady {
                            group_id: group.group_id,
                            view_object_handle_list,
                            view_object_handle_mark_set: HashSet::new(),
                            is_all_ready: false,
                        },
                    );
                }
            });
    }

    fn spawn_monster_object(
        &mut self,
        group_id: u32,
        config: &ConfigGroupMember,
    ) -> ViewObjectHandle {
        let net_id = self.next_net_id(EntityType::Monster);
        let handle = ViewObjectHandle(net_id);

        self.internal_view_object_map.insert(
            handle,
            ViewObject {
                net_id,
                group_id,
                config_id: config.config_id,
                position: Vector3i::from_config(&config.member_position),
                rotation: Vector3i::from_config(&config.member_euler),
            },
        );

        self.component_container.add_component(
            handle,
            VoMonsterProxyComponent {
                monster_id: 11105, // TODO: random/configured ?
                level: 0,
            },
        );

        self.appear_objects.insert(handle);
        handle
    }

    pub fn notify_group_orders(&mut self, listener: &mut dyn NotifyListener) {
        self.group_order_ready_map
            .iter_mut()
            .filter(|(_, group_order_ready)| !group_order_ready.is_all_ready)
            .for_each(|(_, group_order_ready)| {
                group_order_ready
                    .view_object_handle_list
                    .iter()
                    .filter(|handle| {
                        group_order_ready
                            .view_object_handle_mark_set
                            .insert(**handle)
                    })
                    .for_each(|handle| {
                        if let Some(vo) = self.internal_view_object_map.get(handle) {
                            listener.add(GroupMemberCreateScNotify {
                                group_id: vo.group_id,
                                config_id: vo.config_id,
                            });
                        }
                    });

                listener.add(GroupOrderReadyScNotify {
                    group_id: group_order_ready.group_id,
                    member_config_id_list: group_order_ready
                        .view_object_handle_list
                        .iter()
                        .flat_map(|handle| {
                            self.internal_view_object_map
                                .get(handle)
                                .map(|vo| vo.config_id)
                        })
                        .collect(),
                    reason: Some(GroupOrderReasonInfo {
                        group_order_reason: GroupOrderReason::Unk1.into(),
                    }),
                });

                group_order_ready.is_all_ready = true;
            });
    }

    pub fn flush_scene_entity_appear(&mut self) -> Option<SceneEntityAppearScNotify> {
        (!self.appear_objects.is_empty()).then(|| SceneEntityAppearScNotify {
            param: 0,
            appear_type: ViewType::Born.into(),
            entity_list: std::mem::take(&mut self.appear_objects)
                .into_iter()
                .flat_map(|handle| self.serialize_object(handle))
                .collect(),
        })
    }

    pub fn serialize_object(&self, handle: ViewObjectHandle) -> Option<SceneEntityInfo> {
        self.internal_view_object_map
            .get(&handle)
            .map(|view_object| SceneEntityInfo {
                entity_id: view_object.net_id,
                group_id: view_object.group_id,
                config_id: view_object.config_id,
                position: Some(view_object.position.to_proto()),
                rotation: Some(view_object.rotation.to_proto()),
                entity: if let Some(VoBattleAvatarMemberComponent { current_avatar_id }) =
                    handle.find_component(&self.component_container)
                {
                    Some(scene_entity_info::Entity::Avatar(SceneAvatarInfo {
                        avatar_id: current_avatar_id as i32,
                    }))
                } else if let Some(VoMonsterProxyComponent { monster_id, level }) =
                    handle.find_component(&self.component_container)
                {
                    Some(scene_entity_info::Entity::Monster(SceneMonsterInfo {
                        monster_id: monster_id as i32,
                        level: level as i32,
                    }))
                } else {
                    None
                },
                ..Default::default()
            })
    }

    pub fn get_team_avatar_position(&self) -> Option<Vector3i> {
        self.team_avatar_handle.and_then(|handle| {
            self.internal_view_object_map
                .get(&handle)
                .map(|vo| vo.position.clone())
        })
    }

    pub fn send_event<Evt: ViewObjectEventBase>(&mut self, handle: ViewObjectHandle, evt: Evt) {
        debug!("ViewObjectManager::send_event: {evt:?}");

        if let Some(vo) = self.internal_view_object_map.get_mut(&handle) {
            let evt = evt.pack();
            self.component_container
                .for_each_component(handle, |component| component.receive_event(vo, &evt));
        }
    }

    fn is_default_new_mode_monster_object(&self, view_object_id: u32) -> bool {
        let Some(config) = self.resources.level_world.view_objects.get(&view_object_id) else {
            return false;
        };

        config.traits.iter().any(|t| {
            matches!(
                t,
                ViewObjectTrait::DefaultMonsterTrait { mute_ai: false, .. }
            )
        }) && config.traits.iter().any(|t| {
            matches!(
                t,
                ViewObjectTrait::MonsterAlertTrait {
                    use_new_mode: true,
                    ..
                }
            )
        }) && !config
            .traits
            .iter()
            .any(|t| matches!(t, ViewObjectTrait::GamePlayTagTrait { .. }))
    }

    fn is_active_tagged_monster_object(&self, view_object_id: u32) -> bool {
        let Some(config) = self.resources.level_world.view_objects.get(&view_object_id) else {
            return false;
        };

        config.traits.iter().any(|t| {
            matches!(
                t,
                ViewObjectTrait::DefaultMonsterTrait { mute_ai: false, .. }
            )
        }) && config.traits.iter().any(|t| {
            if let ViewObjectTrait::GamePlayTagTrait { entity_tag_list } = t {
                entity_tag_list
                    .iter()
                    .all(|tag| self.active_gameplay_tags.contains(&tag.tag_name))
            } else {
                false
            }
        })
    }

    fn next_net_id(&mut self, ty: EntityType) -> u32 {
        self.net_id_counter += 1;
        (u32::from(ty) << 24) | self.net_id_counter
    }
}

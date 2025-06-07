use std::collections::HashMap;

use config::EntityType;
use tracing::debug;
use yixuan_proto::common::{SceneAvatarInfo, SceneEntityInfo, scene_entity_info};

use crate::math::Vector3i;

use super::{
    component::{ComponentContainer, ViewObjectExtension, VoBattleAvatarMemberComponent},
    event::ViewObjectEventBase,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ViewObjectHandle(pub u32);

#[derive(Default)]
pub struct ViewObjectManager {
    net_id_counter: u32,
    internal_view_object_map: HashMap<ViewObjectHandle, ViewObject>,
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

        handle
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
                entity: if self.is_avatar(handle) {
                    let battle_avatar_member = self
                        .component_container
                        .get_component::<VoBattleAvatarMemberComponent>(handle)
                        .unwrap();

                    Some(scene_entity_info::Entity::Avatar(SceneAvatarInfo {
                        avatar_id: battle_avatar_member.current_avatar_id as i32,
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

    fn is_avatar(&self, handle: ViewObjectHandle) -> bool {
        handle.has_component::<VoBattleAvatarMemberComponent>(&self.component_container)
    }

    fn next_net_id(&mut self, ty: EntityType) -> u32 {
        self.net_id_counter += 1;
        (u32::from(ty) << 24) | self.net_id_counter
    }
}

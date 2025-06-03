use super::entity::HollowEntity;
use std::collections::{HashMap, HashSet};
use yixuan_proto::{HollowComponentType, HollowPosComponent, Message, common::Vector2Int};

mod behavior;
mod category;
mod entity_priority;
mod grid;
mod grid_state;
mod hollow_event;
mod id;
mod owner;
mod pos;

pub use behavior::*;
pub use category::*;
pub use entity_priority::*;
pub use grid::*;
pub use grid_state::*;
pub use hollow_event::*;
pub use id::*;
pub use owner::*;
pub use pos::*;

pub trait SerializableComponent<Proto> {
    fn component_type(&self) -> HollowComponentType;
    fn component_info(&self) -> Proto;
}

macro_rules! component_mgr {
    ($($name:ident;)*) => {
        ::paste::paste!(
            #[derive(Default)]
            pub struct HollowComponentManager {
                $([<$name:snake _map>]: HashMap<HollowEntity, $name>,)*
                changed_entities: HashSet<HollowEntity>,
            }
        );

        ::paste::paste!(
            #[allow(dead_code)]
            impl HollowComponentManager {
            $(
                pub fn [<insert_ $name:snake>](&mut self, entity: HollowEntity, component: $name) {
                    self.changed_entities.insert(entity);
                    self.[<$name:snake _map>].insert(entity, component);
                }

                pub fn [<get_ $name:snake>](&self, entity: HollowEntity) -> Option<$name> {
                    self.[<$name:snake _map>].get(&entity).cloned()
                }
            )*
                pub fn serialize_components(&self, entity: HollowEntity) -> Vec<yixuan_proto::HollowEntityComponentInfo> {
                    let mut components = Vec::new();

                    $(if let Some(component) = self.[<$name:snake _map>].get(&entity) {
                        components.push(yixuan_proto::HollowEntityComponentInfo {
                            r#type: i32::from(component.component_type()) as u32,
                            component_info: component.component_info().encode_to_vec(),
                        });
                    })*

                    components
                }

                pub fn is_synchronized(&self) -> bool {
                    self.changed_entities.is_empty()
                }

                pub fn get_changed_entities(&self) -> &HashSet<HollowEntity> {
                    &self.changed_entities
                }

                pub fn reset_change_state(&mut self) {
                    self.changed_entities.clear();
                }
            }
        );
    };
}

component_mgr! {
    IdComponent;
    PosComponent;
    HollowGridComponent;
    HollowEventComponent;
    CategoryComponent;
    BehaviorComponent;
    OwnerComponent;
    EntityPriorityComponent;
    GridStateComponent;
}

use super::{event::ViewObjectEvent, view_object::{ViewObject, ViewObjectHandle}};
use std::collections::HashMap;

pub trait ViewObjectComponentBase: Clone + ViewObjectComponent {
    const COMP_TYPE: VoComponentType;
    fn add(self, handle: ViewObjectHandle, cc: &mut ComponentContainer);
    fn get(handle: ViewObjectHandle, cc: &ComponentContainer) -> Option<Self>;
}

pub trait ViewObjectComponent {
    fn receive_event(&mut self, vo: &mut ViewObject, evt: &ViewObjectEvent);
}

macro_rules! components {
    ($($name:ident;)*) => {
        pub enum VoComponentType {
            $($name)*,
        }

        ::paste::paste!($(impl ViewObjectComponentBase for [<Vo $name Component>] {
            const COMP_TYPE: VoComponentType = VoComponentType::$name;

            fn add(self, handle: ViewObjectHandle, cc: &mut ComponentContainer) {
                cc.[<$name:snake _map>].insert(handle, self);
            }
            
            fn get(handle: ViewObjectHandle, cc: &ComponentContainer) -> Option<Self> {
                cc.[<$name:snake _map>].get(&handle).cloned()
            }
        })*);

        ::paste::paste!(
            #[derive(Default)]
            pub struct ComponentContainer {
                $([<$name:snake _map>]: HashMap<ViewObjectHandle, [<Vo $name Component>]>,)*
            }
        );

        ::paste::paste!(
            #[allow(dead_code)]
            impl ComponentContainer {
                pub fn get_component<VoComponent: ViewObjectComponentBase>(&self, handle: ViewObjectHandle) -> Option<VoComponent> {
                    VoComponent::get(handle, self)
                }

                pub fn add_component<VoComponent: ViewObjectComponentBase>(&mut self, handle: ViewObjectHandle, component: VoComponent) {
                    component.add(handle, self);
                }

                pub fn for_each_component(&mut self, handle: ViewObjectHandle, mut f: impl FnMut(&mut dyn ViewObjectComponent)) {
                    $(
                        if let Some(component) = self.[<$name:snake _map>].get_mut(&handle) {
                            f(component);
                        }
                    )*
                }

                fn has_component<VoComponent: ViewObjectComponentBase>(&self, handle: ViewObjectHandle) -> bool {
                    match VoComponent::COMP_TYPE {
                        $(VoComponentType::$name => self.[<$name:snake _map>].contains_key(&handle),)*
                    }
                }
            }
        );
    };
}

components!{ BattleAvatarMember; }

#[derive(Debug, Clone)]
pub struct VoBattleAvatarMemberComponent {
    pub current_avatar_id: u32,
}

impl ViewObjectComponent for VoBattleAvatarMemberComponent {
    fn receive_event(&mut self, vo: &mut ViewObject, evt: &ViewObjectEvent) {
        match evt {
            ViewObjectEvent::AvatarChangeEvt(evt) => {
                self.current_avatar_id = evt.current_avatar_id;
            }
            ViewObjectEvent::EntityMoveEvt(evt) => {
                vo.position = evt.position.clone();
                vo.rotation = evt.rotation.clone();
            }
        }
    }
} 

pub trait ViewObjectExtension {
    fn has_component<VoComponent: ViewObjectComponentBase>(&self, cc: &ComponentContainer) -> bool;
}

impl ViewObjectExtension for ViewObjectHandle {
    fn has_component<VoComponent: ViewObjectComponentBase>(&self, cc: &ComponentContainer) -> bool {
        cc.has_component::<VoComponent>(*self)
    }
} 

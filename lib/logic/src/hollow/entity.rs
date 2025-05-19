use std::collections::{HashMap, HashSet};

use vivian_proto::HollowEntityType;

use super::{component::HollowComponentManager, entity_util};

#[derive(Default)]
pub struct HollowEntityManager {
    uid_counter: HashMap<HollowEntityType, u32>,
    running_entities: HashSet<HollowEntity>,
    priorities: HashMap<i32, i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HollowEntity(u32);

impl HollowEntityManager {
    pub fn create(&mut self, ty: HollowEntityType) -> HollowEntity {
        let entity = HollowEntity(
            (i32::from(ty) as u32) * 1_000_000
                + *self
                    .uid_counter
                    .entry(ty)
                    .and_modify(|uid| *uid += 1)
                    .or_insert(1),
        );

        self.running_entities.insert(entity);
        entity
    }

    pub fn serialize_entity(
        &self,
        entity @ HollowEntity(uid): HollowEntity,
        hcm: &HollowComponentManager,
    ) -> vivian_proto::HollowEntityInfo {
        vivian_proto::HollowEntityInfo {
            uid,
            entity_type: uid / 1_000_000,
            entity_component_list: hcm.serialize_components(entity),
        }
    }

    pub fn serialize_entities(
        &self,
        section: HollowEntity,
        hcm: &HollowComponentManager,
    ) -> Vec<vivian_proto::HollowEntityInfo> {
        let mut entities = self
            .running_entities
            .iter()
            .filter(|&&entity| entity_util::belongs_to_section(entity, section, hcm))
            .map(
                |&entity @ HollowEntity(uid)| vivian_proto::HollowEntityInfo {
                    uid,
                    entity_type: uid / 1_000_000,
                    entity_component_list: hcm.serialize_components(entity),
                },
            )
            .collect::<Vec<_>>();

        entities.sort_by_key(|entity| entity.uid);
        entities
    }

    pub fn next_secondary_priority(&mut self, priority_tag: i32) -> i32 {
        *self
            .priorities
            .entry(priority_tag)
            .and_modify(|p| *p += 1)
            .or_default()
    }

    pub fn iter(&self) -> impl Iterator<Item = HollowEntity> {
        self.running_entities.iter().copied()
    }

    pub fn single(&self, ty: HollowEntityType) -> HollowEntity {
        let entities = self
            .running_entities
            .iter()
            .filter(|entity| entity.entity_type() == ty)
            .copied()
            .collect::<Vec<_>>();

        match entities[..] {
            [entity] => entity,
            _ => panic!("HollowEntityManager::single({ty:?}) failed"),
        }
    }

    pub fn uid_exists(&self, uid: u32) -> Option<HollowEntity> {
        self.running_entities
            .contains(&HollowEntity(uid))
            .then_some(HollowEntity(uid))
    }
}

impl HollowEntity {
    pub fn entity_type(&self) -> HollowEntityType {
        HollowEntityType::try_from((self.0 / 1_000_000) as i32).unwrap()
    }

    pub fn as_raw_u32(&self) -> u32 {
        self.0
    }
}

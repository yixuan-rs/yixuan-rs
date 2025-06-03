use config::ConfigEventOwner;
use yixuan_proto::HollowEntityType;

use super::{
    component::HollowComponentManager,
    entity::{HollowEntity, HollowEntityManager},
};

pub fn belongs_to_section(
    entity: HollowEntity,
    section: HollowEntity,
    cm: &HollowComponentManager,
) -> bool {
    match (cm.get_pos_component(entity), cm.get_owner_component(entity)) {
        (Some(pos), _) => pos.section == section,
        (None, Some(owner)) => belongs_to_section(owner.0, section, cm),
        (None, None) => false,
    }
}

pub fn get_grid_at(
    pos: (i32, i32),
    section: HollowEntity,
    em: &HollowEntityManager,
    cm: &HollowComponentManager,
) -> Option<HollowEntity> {
    em.iter()
        .filter(|entity| entity.entity_type() == HollowEntityType::Grid)
        .find(|&entity| {
            cm.get_pos_component(entity)
                .map(|comp| comp.pos == pos && comp.section == section)
                .unwrap_or(false)
        })
}

pub fn get_grid_event(
    grid: HollowEntity,
    em: &HollowEntityManager,
    cm: &HollowComponentManager,
) -> Option<HollowEntity> {
    em.iter().find(|&entity| {
        entity.entity_type() == HollowEntityType::Event
            && cm
                .get_owner_component(entity)
                .map(|owner| owner.0 == grid)
                .unwrap_or(false)
    })
}

pub fn get_owner_by_config(
    config: &ConfigEventOwner,
    section: HollowEntity,
    em: &HollowEntityManager,
    cm: &HollowComponentManager,
) -> Option<HollowEntity> {
    match config {
        ConfigEventOwner::Global => Some(em.single(HollowEntityType::Global)),
        ConfigEventOwner::Grid { position } => {
            get_grid_at((position.x, position.y), section, em, cm)
        }
    }
}

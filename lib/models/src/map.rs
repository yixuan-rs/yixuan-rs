use super::*;

use property::{Property, PropertyHashMap};

#[derive(Default)]
pub struct AreaGroupInfo {
    pub is_unlocked: bool,
    pub area_progress: u32,
}

#[derive(Default)]
pub struct AreaStreetInfo {
    pub is_unlocked: bool,
    pub area_progress: u32,
    pub location_pop_showed: bool,
    pub new_area_showed: bool,
}

#[derive(Model)]
pub struct MapModel {
    pub area_group_map: PropertyHashMap<u32, AreaGroupInfo>,
    pub area_street_map: PropertyHashMap<u32, AreaStreetInfo>,
}

impl MapModel {
    pub fn load_from_pb(pb: MapData) -> Self {
        Self {
            area_group_map: pb
                .area_group_list
                .into_iter()
                .map(|group| {
                    (
                        group.group_id,
                        AreaGroupInfo {
                            is_unlocked: group.is_unlocked.into(),
                            area_progress: group.area_progress.into(),
                        },
                    )
                })
                .collect(),
            area_street_map: pb
                .area_street_list
                .into_iter()
                .map(|street| {
                    (
                        street.area_id,
                        AreaStreetInfo {
                            is_unlocked: street.is_unlocked.into(),
                            area_progress: street.area_progress.into(),
                            location_pop_showed: street.location_pop_showed.into(),
                            new_area_showed: street.new_area_showed.into(),
                        },
                    )
                })
                .collect(),
        }
    }
}

impl Saveable for MapModel {
    fn save_to_pb(&self, root: &mut vivian_proto::server_only::PlayerData) {
        root.map = Some(MapData {
            area_group_list: self
                .area_group_map
                .iter()
                .map(|(&group_id, group)| MapAreaGroupInfo {
                    group_id,
                    is_unlocked: group.is_unlocked,
                    area_progress: group.area_progress,
                })
                .collect(),
            area_street_list: self
                .area_street_map
                .iter()
                .map(|(&area_id, street)| MapAreaStreetInfo {
                    area_id,
                    is_unlocked: street.is_unlocked,
                    area_progress: street.area_progress,
                    location_pop_showed: street.location_pop_showed,
                    new_area_showed: street.new_area_showed,
                })
                .collect(),
        });
    }
}

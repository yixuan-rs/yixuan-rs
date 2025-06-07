use config::EGadgetType;
use tracing::{debug, error};
use yixuan_codegen::handlers;
use yixuan_proto::{
    AreaGroupInfo, AreaMapData, AreaMapModStateCsReq, AreaMapModStateScRsp, AreaStreetInfo,
    FloorGroupMemberInfo, GetAreaMapDataCsReq, GetAreaMapDataScRsp, GetAreaPortalDataCsReq,
    GetAreaPortalDataScRsp, GetFloorActiveGroupListCsReq, GetFloorActiveGroupListScRsp,
    GetFloorGroupMemberListCsReq, GetFloorGroupMemberListScRsp,
};

use super::NetContext;

pub struct MapHandler;

#[handlers]
impl MapHandler {
    pub fn on_get_area_map_data_cs_req(
        context: &mut NetContext<'_>,
        _request: GetAreaMapDataCsReq,
    ) -> GetAreaMapDataScRsp {
        let mut sorted_groups: Vec<_> = context
            .player
            .map_model
            .area_group_map
            .iter()
            .map(|(&group_id, group)| AreaGroupInfo {
                group_id,
                area_progress: group.area_progress,
                is_unlocked: group.is_unlocked,
            })
            .collect();
        sorted_groups.sort_by_key(|g| g.group_id);

        let mut sorted_streets: Vec<_> = context
            .player
            .map_model
            .area_street_map
            .iter()
            .map(|(&area_id, street)| AreaStreetInfo {
                area_id,
                area_progress: street.area_progress,
                is_unlocked: street.is_unlocked,
                location_pop_showed: street.location_pop_showed,
                new_area_showed: street.new_area_showed,
                new_area_portals_showed: street.new_area_portals_showed,
            })
            .collect();
        sorted_streets.sort_by_key(|s| s.area_id);

        GetAreaMapDataScRsp {
            retcode: 0,
            data: Some(AreaMapData {
                group: sorted_groups,
                street: sorted_streets,
                ..Default::default()
            }),
        }
    }

    pub fn on_area_map_mod_state_cs_req(
        context: &mut NetContext<'_>,
        request: AreaMapModStateCsReq,
    ) -> AreaMapModStateScRsp {
        let Some(area) = context
            .player
            .map_model
            .area_street_map
            .get_mut(&request.area_id)
        else {
            return AreaMapModStateScRsp { retcode: 1 };
        };

        area.location_pop_showed |= request.location_pop_showed;
        area.new_area_showed |= request.new_area_showed;
        area.new_area_portals_showed |= request.new_area_portals_showed;

        AreaMapModStateScRsp { retcode: 0 }
    }

    pub fn on_get_area_portal_data_cs_req(
        _context: &mut NetContext<'_>,
        _request: GetAreaPortalDataCsReq,
    ) -> GetAreaPortalDataScRsp {
        // TODO: Send back IDs of portals that are NEW
        GetAreaPortalDataScRsp {
            retcode: 0,
            area_portal_id_list: Vec::new(),
        }
    }

    pub fn on_get_floor_active_group_list_cs_req(
        context: &mut NetContext<'_>,
        request: GetFloorActiveGroupListCsReq,
    ) -> GetFloorActiveGroupListScRsp {
        debug!("{request:?}");

        let Some(floor_config) = context.resources.level_world.floors.get(&request.floor_id) else {
            error!("floor with id {} doesn't exist", request.floor_id);
            return GetFloorActiveGroupListScRsp {
                retcode: 1,
                ..Default::default()
            };
        };

        GetFloorActiveGroupListScRsp {
            retcode: 0,
            group_list: floor_config.group_id_list.clone(),
        }
    }

    pub fn on_get_floor_group_member_list_cs_req(
        context: &mut NetContext<'_>,
        request: GetFloorGroupMemberListCsReq,
    ) -> GetFloorGroupMemberListScRsp {
        debug!("{request:?}");

        let Some(floor_config) = context.resources.level_world.floors.get(&request.floor_id) else {
            error!("floor with id {} doesn't exist", request.floor_id);
            return GetFloorGroupMemberListScRsp {
                retcode: 1,
                ..Default::default()
            };
        };

        GetFloorGroupMemberListScRsp {
            retcode: 0,
            floor_group_portal_list: floor_config
                .group_id_list
                .iter()
                .filter_map(|id| context.resources.level_world.groups.get(id))
                .flat_map(|group| {
                    [group.group_id]
                        .into_iter()
                        .cycle()
                        .zip(group.members.iter())
                })
                .filter(|(_, member)| {
                    member.gadget_server_metadata.gadget_type == EGadgetType::Portal
                })
                .map(|(group_id, member)| FloorGroupMemberInfo {
                    group_id,
                    config_id: member.config_id,
                })
                .collect(),
        }
    }
}

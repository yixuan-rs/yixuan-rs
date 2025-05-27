use vivian_codegen::handlers;
use vivian_proto::{
    AreaGroupInfo, AreaMapData, AreaMapModStateCsReq, AreaMapModStateScRsp, AreaStreetInfo, GetAreaMapDataCsReq, GetAreaMapDataScRsp, GetAreaPortalDataCsReq, GetAreaPortalDataScRsp
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
			})
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

		AreaMapModStateScRsp {
			retcode: 0,
		}
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
}

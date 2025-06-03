use yixuan_logic::{
    LogicResources,
    item::{AvatarItem, EAvatarSkillType},
    listener::{NotifyListener, NotifyListenerExt},
};

use super::*;

use crate::property::{Property, PropertyHashMap};

#[derive(Model)]
pub struct AvatarModel {
    pub avatar_map: PropertyHashMap<u32, AvatarItem>,
    pub add_avatar_perform_map:
        PropertyHashMap<u32, yixuan_proto::add_avatar_sc_notify::PerformType>,
}

impl AvatarModel {
    pub fn send_add_avatar_notify(&self, listener: &mut dyn NotifyListener) {
        self.add_avatar_perform_map
            .iter_changed_items()
            .for_each(|(&id, &perform_type)| {
                listener.add(yixuan_proto::AddAvatarScNotify {
                    avatar_id: id,
                    perform_type: perform_type.into(),
                    reward_list: Vec::new(),
                    lock: false,
                });
            });
    }

    pub fn is_avatar_unlocked(&self, id: u32) -> bool {
        self.avatar_map.contains_key(&id)
    }

    pub fn load_from_pb(pb: AvatarData, res: &LogicResources) -> Self {
        Self {
            avatar_map: pb
                .avatar_list
                .into_iter()
                .map(|avatar| {
                    (
                        avatar.id,
                        AvatarItem {
                            id: avatar.id,
                            level: avatar.level,
                            exp: avatar.exp,
                            rank: avatar.rank,
                            unlocked_talent_num: avatar.unlocked_talent_num,
                            talent_switch: avatar.talent_switch_list.try_into().unwrap_or_default(),
                            passive_skill_level: avatar.passive_skill_level,
                            skill_level_map: avatar
                                .skill_level_map
                                .into_iter()
                                .map(|(ty, level)| (EAvatarSkillType::try_from(ty).unwrap(), level))
                                .collect(),
                            weapon_uid: avatar.cur_weapon_uid,
                            show_weapon_type: avatar.show_weapon_type,
                            dressed_equip_map: avatar.dressed_equip_map,
                            first_get_time: avatar.first_get_time,
                            taken_rank_up_reward_list: avatar.taken_rank_up_reward_list,
                            avatar_skin_id: avatar.avatar_skin_id,
                            is_favorite: avatar.is_favorite,
                            is_awake_available: res
                                .template_collection
                                .avatar_battle_template_tb()
                                .find(|tmpl| tmpl.id() == avatar.id)
                                .and_then(|tmpl| tmpl.awake_ids())
                                .is_some(),
                            awake_id: avatar.awake_id,
                        },
                    )
                })
                .collect(),
            add_avatar_perform_map: PropertyHashMap::default(),
        }
    }
}

impl Saveable for AvatarModel {
    fn save_to_pb(&self, root: &mut yixuan_proto::server_only::PlayerData) {
        root.avatar = Some(AvatarData {
            avatar_list: self
                .avatar_map
                .iter()
                .map(|(_, avatar)| AvatarItemInfo {
                    id: avatar.id,
                    level: avatar.level,
                    exp: avatar.exp,
                    rank: avatar.rank,
                    unlocked_talent_num: avatar.unlocked_talent_num,
                    talent_switch_list: avatar.talent_switch.to_vec(),
                    passive_skill_level: avatar.passive_skill_level,
                    skill_level_map: avatar
                        .skill_level_map
                        .iter()
                        .map(|(&ty, &level)| (ty.into(), level))
                        .collect(),
                    cur_weapon_uid: avatar.weapon_uid,
                    show_weapon_type: avatar.show_weapon_type,
                    dressed_equip_map: avatar.dressed_equip_map.clone(),
                    first_get_time: avatar.first_get_time,
                    taken_rank_up_reward_list: avatar.taken_rank_up_reward_list.clone(),
                    avatar_skin_id: avatar.avatar_skin_id,
                    is_favorite: avatar.is_favorite,
                    awake_id: avatar.awake_id,
                })
                .collect(),
        });
    }
}

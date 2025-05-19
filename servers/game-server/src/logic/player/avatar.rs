use std::collections::HashMap;

use common::time_util;
use config::AvatarBaseTemplate;
use vivian_codegen::Model;
use vivian_logic::{
    item::{AvatarItem, EAvatarSkillType},
    listener::{NotifyListener, NotifyListenerExt},
};
use vivian_proto::server_only::{AvatarData, AvatarItemInfo};

use crate::{
    logic::{
        property::{Property, PropertyHashMap},
        sync::{LoginDataSyncComponent, PlayerSyncComponent, SyncType},
    },
    resources::NapResources,
};

use super::{Model, Saveable};

#[derive(Model)]
pub struct AvatarModel {
    pub avatar_map: PropertyHashMap<u32, AvatarItem>,
    pub add_avatar_perform_map:
        PropertyHashMap<u32, vivian_proto::add_avatar_sc_notify::PerformType>,
}

impl AvatarModel {
    pub fn on_first_login(&mut self, res: &NapResources) {
        const STARTING_AVATARS: &[u32] = &[1011, 1081];

        STARTING_AVATARS
            .iter()
            .filter_map(|id| {
                res.templates
                    .avatar_base_template_tb()
                    .find(|tmpl| tmpl.id() == *id)
            })
            .for_each(|tmpl| self.unlock_avatar(&tmpl, None));
    }

    pub fn send_add_avatar_notify(&self, listener: &mut dyn NotifyListener) {
        self.add_avatar_perform_map
            .iter_changed_items()
            .for_each(|(&id, &perform_type)| {
                listener.add(vivian_proto::AddAvatarScNotify {
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

    pub fn load_from_pb(pb: AvatarData) -> Self {
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
                            dressed_equip_map: avatar.dressed_equip_map,
                            first_get_time: avatar.first_get_time,
                            taken_rank_up_reward_list: avatar.taken_rank_up_reward_list,
                            avatar_skin_id: avatar.avatar_skin_id,
                            is_favorite: avatar.is_favorite,
                        },
                    )
                })
                .collect(),
            add_avatar_perform_map: PropertyHashMap::default(),
        }
    }

    pub fn unlock_avatar(
        &mut self,
        base_template: &AvatarBaseTemplate,
        perform_type: Option<vivian_proto::add_avatar_sc_notify::PerformType>,
    ) {
        const AVATAR_BLACKLIST: &[u32] = &[];

        let avatar_id = base_template.id();

        if base_template.camp() != 0
            && !self.avatar_map.contains_key(&avatar_id)
            && !AVATAR_BLACKLIST.contains(&avatar_id)
        {
            self.avatar_map.insert(
                avatar_id,
                AvatarItem {
                    id: avatar_id,
                    level: 1,
                    exp: 0,
                    rank: 1,
                    unlocked_talent_num: 0,
                    talent_switch: [false; 6],
                    passive_skill_level: 0,
                    skill_level_map: (0..EAvatarSkillType::EnumCount.into())
                        .map(|ty| EAvatarSkillType::try_from(ty).unwrap())
                        .zip([0].into_iter().cycle())
                        .collect(),
                    weapon_uid: 0,
                    dressed_equip_map: HashMap::new(),
                    first_get_time: time_util::unix_timestamp_seconds(),
                    taken_rank_up_reward_list: Vec::new(),
                    avatar_skin_id: 0,
                    is_favorite: false,
                },
            );

            if let Some(perform_type) = perform_type {
                self.add_avatar_perform_map.insert(avatar_id, perform_type);
            }
        }
    }
}

impl Saveable for AvatarModel {
    fn save_to_pb(&self, root: &mut vivian_proto::server_only::PlayerData) {
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
                    dressed_equip_map: avatar.dressed_equip_map.clone(),
                    first_get_time: avatar.first_get_time,
                    taken_rank_up_reward_list: avatar.taken_rank_up_reward_list.clone(),
                    avatar_skin_id: avatar.avatar_skin_id,
                    is_favorite: avatar.is_favorite,
                    show_weapon_type: 0,
                })
                .collect(),
        });
    }
}

impl LoginDataSyncComponent for AvatarModel {
    fn prepare_responses(
        &self,
        sync_helper: &mut crate::logic::sync::DataSyncHelper,
        _res: &NapResources,
    ) {
        sync_helper.add_response(
            SyncType::BasicData,
            vivian_proto::GetAvatarDataScRsp {
                retcode: 0,
                avatar_list: self
                    .avatar_map
                    .iter()
                    .map(|(_, avatar)| avatar.as_client_proto())
                    .collect(),
            },
        );
    }
}

impl PlayerSyncComponent for AvatarModel {
    fn add_changes_to_player_sync_notify(
        &self,
        player_sync_sc_notify: &mut vivian_proto::PlayerSyncScNotify,
    ) {
        player_sync_sc_notify
            .avatar
            .get_or_insert_default()
            .avatar_list
            .extend(
                self.avatar_map
                    .iter_changed_items()
                    .map(|(_, avatar)| avatar.as_client_proto()),
            );
    }

    fn supports_player_sync(&self) -> bool {
        true
    }
}

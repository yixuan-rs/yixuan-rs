#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(2233)]
pub struct KeepAliveNotify {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(3164)]
pub struct PlayerGetTokenCsReq {
    #[prost(uint32, tag = "3", xor = "5468")]
    pub uid: u32,
    #[prost(uint32, tag = "5", xor = "13807")]
    pub channel_id: u32,
    pub account_type: u32,
    #[prost(string, tag = "10")]
    pub account_uid: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub token: ::prost::alloc::string::String,
    pub platform: u32,
    #[prost(string, tag = "7")]
    pub device: ::prost::alloc::string::String,
    #[prost(uint32, tag = "14", xor = "12461")]
    pub rsa_ver: u32,
    #[prost(string, tag = "13")]
    pub client_rand_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
#[cmd_id(1568)]
pub struct PlayerGetTokenScRsp {
    #[prost(int32, tag = "15", xor = "2223")]
    pub retcode: i32,
    #[prost(string, tag = "9")]
    pub msg: ::prost::alloc::string::String,
    #[prost(uint32, tag = "8", xor = "6720")]
    pub uid: u32,
    #[prost(uint32, tag = "6", xor = "14913")]
    pub blacklist_reason: u32,
    #[prost(int64, tag = "1", xor = "906")]
    pub blacklist_end_timestamp: i64,
    #[prost(string, tag = "12")]
    pub server_rand_key: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub sign: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(7653)]
pub struct PlayerLoginCsReq {
    pub platform: u32,
    pub cps: ::prost::alloc::string::String,
    pub device_uuid: ::prost::alloc::string::String,
    pub device_info: ::prost::alloc::string::String,
    pub system_info: ::prost::alloc::string::String,
    pub language: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
#[cmd_id(3376)]
pub struct PlayerLoginScRsp {
    #[prost(int32, tag = "10", xor = "2872")]
    pub retcode: i32,
    pub timestamp: u64,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct PlayerLogoutCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(2308)]
pub struct GetSelfBasicInfoCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct SelfBasicInfo {
    #[prost(string, tag = "8")]
    pub nick_name: ::prost::alloc::string::String,
    #[prost(int64, tag = "5", xor = "954")]
    pub role_create_time: i64,
    #[prost(uint32, tag = "13", xor = "11664")]
    pub level: u32,
    #[prost(uint32, tag = "3", xor = "1009")]
    pub exp: u32,
    #[prost(uint32, tag = "6", xor = "9998")]
    pub avatar_id: u32,
    #[prost(uint32, tag = "11", xor = "14861")]
    pub player_avatar_id: u32,
    #[prost(uint32, tag = "1926", xor = "15600")]
    pub control_guise_avatar_id: u32,
    pub portrait_id: u32,
    #[prost(uint32, tag = "4", xor = "2773")]
    pub name_change_times: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
#[cmd_id(1706)]
pub struct GetSelfBasicInfoScRsp {
    #[prost(int32, tag = "12", xor = "3011")]
    pub retcode: i32,
    #[prost(message, optional, tag = "14")]
    pub self_basic_info: ::core::option::Option<SelfBasicInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GetPlayerTransactionCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GetPlayerTransactionScRsp {
    pub retcode: i32,
    pub transaction: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(8346)]
pub struct GetServerTimestampCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GetServerTimestampScRsp {
    pub retcode: i32,
    pub timestamp: u64,
    pub utc_offset: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(3601)]
pub struct GetAuthkeyCsReq {
    pub auth_appid: ::prost::alloc::string::String,
    pub sign_type: u32,
    pub authkey_ver: u32,
    pub offline_verify_value: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GetAuthkeyScRsp {
    pub retcode: i32,
    pub authkey: ::prost::alloc::string::String,
    pub auth_appid: ::prost::alloc::string::String,
    pub sign_type: u32,
    pub authkey_ver: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct ModNickNameCsReq {
    pub nick_name: ::prost::alloc::string::String,
    pub avatar_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct ModNickNameScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(3870)]
pub struct ModAvatarCsReq {
    #[prost(uint32, tag = "4", xor = "15083")]
    pub avatar_id: u32,
    #[prost(uint32, tag = "3", xor = "14843")]
    pub player_avatar_id: u32,
    #[prost(uint32, tag = "15", xor = "12885")]
    pub control_guise_avatar_id: u32,
    pub r#type: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct ModAvatarScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GetDisplayCaseDataCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GetDisplayCaseDataScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct ItemInfo {
    #[prost(uint32, tag = "15", xor = "1114")]
    pub id: u32,
    #[prost(int32, tag = "8", xor = "1782")]
    pub count: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct WeaponInfo {
    #[prost(uint32, tag = "11", xor = "14685")]
    pub uid: u32,
    #[prost(uint32, tag = "2", xor = "11479")]
    pub id: u32,
    #[prost(uint32, tag = "7", xor = "785")]
    pub level: u32,
    #[prost(uint32, tag = "8", xor = "2219")]
    pub exp: u32,
    #[prost(uint32, tag = "15", xor = "8282")]
    pub star: u32,
    #[prost(uint32, tag = "3", xor = "13657")]
    pub refine_level: u32,
    #[prost(bool, tag = "6")]
    pub lock: bool,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct EquipProperty {
    #[prost(uint32, tag = "1", xor = "10838")]
    pub key: u32,
    #[prost(uint32, tag = "5", xor = "7012")]
    pub base_value: u32,
    #[prost(uint32, tag = "8", xor = "5747")]
    pub add_value: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct EquipInfo {
    #[prost(uint32, tag = "4", xor = "14516")]
    pub uid: u32,
    #[prost(uint32, tag = "12", xor = "2416")]
    pub id: u32,
    #[prost(uint32, tag = "8", xor = "966")]
    pub level: u32,
    #[prost(uint32, tag = "1", xor = "2080")]
    pub exp: u32,
    #[prost(uint32, tag = "3", xor = "10240")]
    pub star: u32,
    #[prost(bool, tag = "5")]
    pub lock: bool,
    #[prost(message, repeated, tag = "9")]
    pub propertys: ::prost::alloc::vec::Vec<EquipProperty>,
    #[prost(message, repeated, tag = "14")]
    pub sub_propertys: ::prost::alloc::vec::Vec<EquipProperty>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct AutoRecoveryInfo {
    pub buy_times: u32,
    pub last_recovery_timestamp: i64,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct ItemRewardInfo {
    pub item_id: u32,
    pub amount: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct ItemReward {
    pub reward_list: ::prost::alloc::vec::Vec<ItemRewardInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct ItemChangedScNotify {
    pub item_reward_map: ::std::collections::HashMap<u32, ItemReward>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct ItemSync {
    #[prost(message, repeated, tag = "8")]
    pub item_list: ::prost::alloc::vec::Vec<ItemInfo>,
    #[prost(message, repeated, tag = "7")]
    pub weapon_list: ::prost::alloc::vec::Vec<WeaponInfo>,
    #[prost(message, repeated, tag = "5")]
    pub equip_list: ::prost::alloc::vec::Vec<EquipInfo>,
    pub item_changed: ::core::option::Option<ItemChangedScNotify>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct AvatarSync {
    #[prost(message, repeated, tag = "13")]
    pub avatar_list: ::prost::alloc::vec::Vec<AvatarInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct BossBattleSync {
    pub new_unlocked_levels: ::prost::alloc::vec::Vec<u32>,
    pub selected_level: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct DoubleEliteSync {
    pub new_unlocked_levels: ::prost::alloc::vec::Vec<u32>,
    pub progress_list: ::prost::alloc::vec::Vec<DoubleEliteProgress>,
    pub selected_difficulty: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct ActivityBattleSync {
    pub boss_battle: ::core::option::Option<BossBattleSync>,
    pub double_elite: ::core::option::Option<DoubleEliteSync>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct BigSceneSync {
    pub scene_avatar_list: ::prost::alloc::vec::Vec<super::common::BigSceneAvatarInfo>,
    pub cur_scene_avatar_list: ::prost::alloc::vec::Vec<
        super::common::BigSceneAvatarInfo,
    >,
    pub cur_avatar_id: u32,
    pub is_scene_team_replaced: bool,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(7313)]
pub struct PlayerSyncScNotify {
    #[prost(message, optional, tag = "7")]
    pub self_basic_info: ::core::option::Option<SelfBasicInfo>,
    #[prost(message, optional, tag = "10")]
    pub item: ::core::option::Option<ItemSync>,
    #[prost(message, optional, tag = "12")]
    pub avatar: ::core::option::Option<AvatarSync>,
    #[prost(message, optional, tag = "2")]
    pub misc: ::core::option::Option<MiscSync>,
    pub quest: ::core::option::Option<QuestSync>,
    pub hollow: ::core::option::Option<HollowSync>,
    pub activity_battle: ::core::option::Option<ActivityBattleSync>,
    pub big_scene: ::core::option::Option<BigSceneSync>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GetFriendListCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GetFriendListScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct AvatarSkillLevel {
    #[prost(uint32, tag = "14", xor = "11457")]
    pub skill_type: u32,
    #[prost(uint32, tag = "10", xor = "7743")]
    pub level: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct DressedEquip {
    #[prost(uint32, tag = "4", xor = "15849")]
    pub index: u32,
    #[prost(uint32, tag = "14", xor = "11673")]
    pub equip_uid: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct AvatarInfo {
    #[prost(uint32, tag = "1", xor = "9166")]
    pub id: u32,
    #[prost(uint32, tag = "4", xor = "3916")]
    pub level: u32,
    #[prost(uint32, tag = "14", xor = "10776")]
    pub exp: u32,
    #[prost(uint32, tag = "11", xor = "7748")]
    pub rank: u32,
    #[prost(uint32, tag = "3", xor = "2118")]
    pub unlocked_talent_num: u32,
    #[prost(uint32, tag = "12", xor = "2862")]
    pub cur_weapon_uid: u32,
    #[prost(uint32, tag = "9", xor = "7731")]
    pub passive_skill_level: u32,
    #[prost(message, repeated, tag = "8")]
    pub skill_type_level: ::prost::alloc::vec::Vec<AvatarSkillLevel>,
    #[prost(message, repeated, tag = "6")]
    pub dressed_equip_list: ::prost::alloc::vec::Vec<DressedEquip>,
    #[prost(enumeration = "AvatarShowWeaponType", tag = "7")]
    pub show_weapon_type: i32,
    #[prost(int64, tag = "13", xor = "7574")]
    pub first_get_time: i64,
    #[prost(bool, repeated, tag = "10")]
    pub talent_switch_list: ::prost::alloc::vec::Vec<bool>,
    #[prost(uint32, repeated, tag = "2")]
    pub taken_rank_up_reward_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(bool, tag = "15")]
    pub is_favorite: bool,
    #[prost(uint32, tag = "814", xor = "13997")]
    pub avatar_skin_id: u32,
    #[prost(bool, tag = "109")]
    pub is_awake_available: bool,
    #[prost(uint32, tag = "1431", xor = "11762")]
    pub awake_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(7993)]
pub struct GetQuestDataCsReq {
    #[prost(uint32, tag = "5", xor = "12279")]
    pub quest_type: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct TrackQuestNpcInfo {
    pub track_npc_id_list: ::prost::alloc::vec::Vec<u32>,
    pub quest_scene_info: ::core::option::Option<QuestSceneInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct QuestSceneInfo {
    pub quest_scene_unit_id_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct SpecialQuestInfo {
    pub previous_main_city_quest_id: u32,
    pub cur_main_city_quest_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct QuestInfo {
    #[prost(uint32, tag = "14", xor = "2933")]
    pub id: u32,
    #[prost(enumeration = "QuestState", tag = "9")]
    pub state: i32,
    #[prost(int64, tag = "8", xor = "15116")]
    pub unlock_time: i64,
    pub progress: u32,
    pub in_progress_time: i64,
    pub finish_condition_progress: ::std::collections::HashMap<u32, u32>,
    pub track_info: ::core::option::Option<TrackQuestNpcInfo>,
    pub special_quest_info: ::core::option::Option<SpecialQuestInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct TrackQuestSync {
    pub cur_main_quest_id: u32,
    pub cur_track_quest_id: u32,
    pub cur_track_special_quest_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct QuestSync {
    pub quest_list: ::prost::alloc::vec::Vec<QuestInfo>,
    pub finished_quest_id_list: ::prost::alloc::vec::Vec<u32>,
    pub new_hollow_quest_id_list: ::prost::alloc::vec::Vec<u32>,
    pub track_quest_sync: ::core::option::Option<TrackQuestSync>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct TrackQuestInfo {
    pub cur_main_quest_id: u32,
    pub cur_track_quest_id: u32,
    pub cur_track_special_quest_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct QuestCollection {
    #[prost(uint32, tag = "15", xor = "13398")]
    pub quest_type: u32,
    #[prost(message, repeated, tag = "6")]
    pub quest_list: ::prost::alloc::vec::Vec<QuestInfo>,
    #[prost(uint32, repeated, tag = "11")]
    pub finished_quest_id_list: ::prost::alloc::vec::Vec<u32>,
    pub track_info: ::core::option::Option<TrackQuestInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct QuestData {
    #[prost(message, repeated, tag = "9")]
    pub quest_collection_list: ::prost::alloc::vec::Vec<QuestCollection>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
#[cmd_id(9469)]
pub struct GetQuestDataScRsp {
    #[prost(int32, tag = "3", xor = "15829")]
    pub retcode: i32,
    #[prost(uint32, tag = "7", xor = "16009")]
    pub quest_type: u32,
    #[prost(message, optional, tag = "1")]
    pub quest_data: ::core::option::Option<QuestData>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(9329)]
pub struct GetArchiveDataCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct VideotapeInfo {
    pub archive_file_id: u32,
    pub finished: bool,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct ArchiveData {
    pub hollow_archive_id_list: ::prost::alloc::vec::Vec<u32>,
    pub videotape_list: ::prost::alloc::vec::Vec<VideotapeInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
#[cmd_id(367)]
pub struct GetArchiveDataScRsp {
    #[prost(int32, tag = "2", xor = "4681")]
    pub retcode: i32,
    #[prost(message, optional, tag = "10")]
    pub archive_data: ::core::option::Option<ArchiveData>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(9825)]
pub struct GetHollowDataCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HollowInfo {
    pub hollow_quest_id: u32,
    pub unk_hollow_info_100: u32,
    pub acquired_hollow_challenge_reward: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HollowData {
    pub unlock_hollow_group_list: ::prost::alloc::vec::Vec<u32>,
    pub hollow_group_list: ::prost::alloc::vec::Vec<u32>,
    pub unlock_hollow_id_list: ::prost::alloc::vec::Vec<u32>,
    pub hollow_info_list: ::prost::alloc::vec::Vec<HollowInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HollowSync {
    pub unlock_hollow_group_list: ::prost::alloc::vec::Vec<u32>,
    pub hollow_group_list: ::prost::alloc::vec::Vec<u32>,
    pub unlock_hollow_id_list: ::prost::alloc::vec::Vec<u32>,
    pub hollow_info_list: ::prost::alloc::vec::Vec<HollowInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
#[cmd_id(4922)]
pub struct GetHollowDataScRsp {
    #[prost(int32, tag = "5", xor = "12982")]
    pub retcode: i32,
    #[prost(message, optional, tag = "2")]
    pub hollow_data: ::core::option::Option<HollowData>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(1934)]
pub struct AbyssGetDataCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct AbyssData {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct AbyssDungeon {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct AbyssGroup {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct AbyssGetDataScRsp {
    pub retcode: i32,
    pub abyss_data: ::core::option::Option<AbyssData>,
    pub abyss_dungeon_list: ::prost::alloc::vec::Vec<AbyssDungeon>,
    pub abyss_group_list: ::prost::alloc::vec::Vec<AbyssGroup>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(3394)]
pub struct AbyssArpeggioGetDataCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct AbyssArpeggioGetDataScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(8146)]
pub struct StartTrainingQuestCsReq {
    #[prost(uint32, tag = "11", xor = "14951")]
    pub quest_id: u32,
    #[prost(uint32, repeated, tag = "13")]
    pub avatar_id_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct StartTrainingQuestScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct StartHollowQuestCsReq {
    pub quest_id: u32,
    pub avatar_id_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HollowQuestProgressCsReq {
    pub quest_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct HollowQuestProgressScRsp {
    pub retcode: i32,
    pub new_progress: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct StartHollowQuestScRsp {
    pub retcode: i32,
    pub quest_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct RestartActivityBattleCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct RestartActivityBattleScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct BeginActivityBattleCsReq {
    pub quest_id: u32,
    pub avatar_id_list: ::prost::alloc::vec::Vec<u32>,
    pub level: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct BeginActivityBattleScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct AvatarContributionInfo {
    pub avatar_id: u32,
    pub contribution: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct DungeonQuestFinishedScNotify {
    pub quest_id: u32,
    pub result: u32,
    pub rank: u32,
    pub statistics: ::std::collections::HashMap<u32, u64>,
    pub battle_reward_map: ::std::collections::HashMap<u32, ItemReward>,
    pub avatar_contribution_list: ::prost::alloc::vec::Vec<AvatarContributionInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct MainCityQuestTrackNpcScNotify {
    pub quest_id: u32,
    pub track_npc_id_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(5783)]
pub struct EnterWorldCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
#[cmd_id(5834)]
pub struct EnterWorldScRsp {
    #[prost(int32, tag = "13", xor = "2001")]
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct InteractInfo {
    pub tag_id: i32,
    pub name: ::prost::alloc::string::String,
    pub scale_x: f64,
    pub scale_y: f64,
    pub scale_z: f64,
    pub scale_w: f64,
    pub scale_r: f64,
    pub participators: ::std::collections::HashMap<u32, ::prost::alloc::string::String>,
    pub interact_target_list: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct SceneUnitProtocolInfo {
    pub npc_id: u32,
    pub is_interactable: bool,
    pub interacts_info: ::std::collections::HashMap<u32, InteractInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HallSceneData {
    #[prost(uint32, tag = "7", xor = "16280")]
    pub section_id: u32,
    #[prost(message, optional, tag = "4")]
    pub position: ::core::option::Option<super::common::Transform>,
    pub scene_unit_list: ::prost::alloc::vec::Vec<SceneUnitProtocolInfo>,
    pub main_city_objects_state: ::std::collections::HashMap<i32, i32>,
    pub hall_unknown_map_string_int: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        i32,
    >,
    pub time_period: u32,
    pub time_of_day: u32,
    pub bgm_id: u32,
    pub day_of_week: u32,
    pub hall_unknown_map_uint_uint: ::std::collections::HashMap<u32, u32>,
    #[prost(uint32, tag = "367", xor = "10098")]
    pub player_avatar_id: u32,
    #[prost(string, tag = "91")]
    pub transform_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "848", xor = "5580")]
    pub control_guise_avatar_id: u32,
    pub main_city_quest_id_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct ScenePerformInfo {
    pub time: ::prost::alloc::string::String,
    pub weather: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct DropItems {
    pub drop_items: ::std::collections::HashMap<u32, u32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct BattleRewardInfo {
    pub reward_id: u32,
    pub battle_reward_map: ::std::collections::HashMap<u32, DropItems>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct SceneRewardInfo {
    pub normal_drop_reward_list: ::prost::alloc::vec::Vec<BattleRewardInfo>,
    pub special_drop_reward_list: ::prost::alloc::vec::Vec<BattleRewardInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct FightSceneData {
    #[prost(message, optional, tag = "3")]
    pub scene_perform: ::core::option::Option<ScenePerformInfo>,
    #[prost(message, optional, tag = "2")]
    pub scene_reward: ::core::option::Option<SceneRewardInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct PublicVariable {
    pub var_int: i64,
    pub r#type: u32,
    pub var_number: f64,
    pub var_str: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct QuestCondProgress {
    pub public_variables: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        PublicVariable,
    >,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct LongFightProgressInfo {
    pub quest_cond_progress: ::core::option::Option<QuestCondProgress>,
    pub quest_variables_info: ::core::option::Option<QuestVariablesInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct QuestVariablesInfo {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct LongFightSceneData {
    pub scene_perform: ::core::option::Option<ScenePerformInfo>,
    pub scene_reward: ::core::option::Option<SceneRewardInfo>,
    pub scene_progress: ::core::option::Option<LongFightProgressInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct IdComponent {
    pub id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HollowPosComponent {
    pub pos: ::core::option::Option<super::common::Vector2Int>,
    pub section_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HollowGridComponent {
    pub grid_type: i32,
    pub grid_link: i32,
    pub grid_unk_enum: i32,
    pub prev_grid_state: ::core::option::Option<super::common::HollowGridState>,
    pub cur_grid_state: ::core::option::Option<super::common::HollowGridState>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HollowEventComponent {
    pub icon_texture_sheet_id: i32,
    pub interact_texture_sheet_id: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct CategoryComponent {
    pub category_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct BehaviorComponent {
    pub unk_behavior_state: bool,
    pub int_specials: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
    pub str_specials: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct OwnerComponent {
    pub owner_entity_uid: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct EntityPriorityComponent {
    pub priority: i32,
    pub secondary_priority: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GridStateComponent {
    pub cur_grid_state: ::core::option::Option<super::common::HollowGridState>,
    pub prev_grid_state: ::core::option::Option<super::common::HollowGridState>,
    pub grid_state_component_unknown: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HollowEntityComponentInfo {
    pub r#type: u32,
    pub component_info: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HollowEntityInfo {
    pub uid: u32,
    pub entity_type: u32,
    pub entity_component_list: ::prost::alloc::vec::Vec<HollowEntityComponentInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HollowGridMap {
    pub hollow_grid_list: ::prost::alloc::vec::Vec<HollowEntityInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HollowSectionGridMapInfo {
    pub cur_grid_position: ::core::option::Option<super::common::Vector2Int>,
    pub hollow_grid_map: ::core::option::Option<HollowGridMap>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HollowSectionInfo {
    pub section_id: u32,
    pub hollow_objective_id: i32,
    pub time: ::prost::alloc::string::String,
    pub weather: ::prost::alloc::string::String,
    pub section_grid_map: ::core::option::Option<HollowSectionGridMapInfo>,
    pub hollow_vector_zero_1: ::core::option::Option<super::common::Vector2Int>,
    pub hollow_vector_zero_2: ::core::option::Option<super::common::Vector2Int>,
    pub hollow_vector_negative_1: ::core::option::Option<super::common::Vector2Int>,
    pub hollow_vector_negative_2: ::core::option::Option<super::common::Vector2Int>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HollowGridMapsInfo {
    pub cur_hollow_position: ::core::option::Option<super::common::Vector2Int>,
    pub cur_section_id: u32,
    pub hollow_grid_map_type: i32,
    pub hollow_section_list: ::prost::alloc::vec::Vec<HollowSectionInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HollowScene {
    pub henshin_type: i32,
    pub cur_event_entity_info: ::core::option::Option<HollowEntityInfo>,
    pub hollow_grid_maps: ::core::option::Option<HollowGridMapsInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HollowSceneData {
    pub hollow_scene: ::core::option::Option<HollowScene>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct SceneData {
    #[prost(uint32, tag = "4", xor = "14376")]
    pub scene_type: u32,
    #[prost(uint32, tag = "15", xor = "9605")]
    pub scene_id: u32,
    #[prost(uint32, tag = "7", xor = "4335")]
    pub play_type: u32,
    #[prost(message, optional, tag = "10")]
    pub hall_scene_data: ::core::option::Option<HallSceneData>,
    #[prost(message, optional, tag = "14")]
    pub fight_scene_data: ::core::option::Option<FightSceneData>,
    pub long_fight_scene_data: ::core::option::Option<LongFightSceneData>,
    pub hollow_scene_data: ::core::option::Option<HollowSceneData>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct DungeonQuestInfo {
    pub inner_quest_id_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct DungeonStatistics {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct DungeonEquipInfo {
    #[prost(message, repeated, tag = "8")]
    pub avatar_list: ::prost::alloc::vec::Vec<AvatarInfo>,
    #[prost(message, repeated, tag = "12")]
    pub weapon_list: ::prost::alloc::vec::Vec<WeaponInfo>,
    #[prost(message, repeated, tag = "10")]
    pub equip_list: ::prost::alloc::vec::Vec<EquipInfo>,
    pub buddy_list: ::prost::alloc::vec::Vec<BuddyInfo>,
    pub buddy: ::core::option::Option<BuddyInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct BigBossInfo {
    pub difficulty: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct DoubleEliteInfo {
    pub difficulty: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct DungeonInfo {
    #[prost(uint32, tag = "4", xor = "12540")]
    pub quest_id: u32,
    #[prost(message, optional, tag = "12")]
    pub dungeon_equip_info: ::core::option::Option<DungeonEquipInfo>,
    #[prost(message, repeated, tag = "9")]
    pub avatar_list: ::prost::alloc::vec::Vec<super::common::AvatarUnitInfo>,
    #[prost(uint32, tag = "338", xor = "6264")]
    pub quest_type: u32,
    pub dungeon_statistics: ::core::option::Option<DungeonStatistics>,
    pub begin_time: i64,
    pub dungeon_quest_info: ::core::option::Option<DungeonQuestInfo>,
    pub big_boss_info: ::core::option::Option<BigBossInfo>,
    pub double_elite_info: ::core::option::Option<DoubleEliteInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(9275)]
pub struct EnterSceneScNotify {
    #[prost(message, optional, tag = "15")]
    pub scene: ::core::option::Option<SceneData>,
    #[prost(message, optional, tag = "2")]
    pub dungeon: ::core::option::Option<DungeonInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(7544)]
pub struct EnterSectionCompleteCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
#[cmd_id(2000)]
pub struct EnterSectionCompleteScRsp {
    #[prost(int32, tag = "3", xor = "730")]
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HallRefreshScNotify {
    pub force_refresh: bool,
    pub section_id: u32,
    pub position: ::core::option::Option<super::common::Transform>,
    pub scene_unit_list: ::prost::alloc::vec::Vec<SceneUnitProtocolInfo>,
    pub main_city_objects_state: ::std::collections::HashMap<i32, i32>,
    pub hall_unknown_map_string_int: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        i32,
    >,
    pub time_period: u32,
    pub time_of_day: u32,
    pub bgm_id: u32,
    pub day_of_week: u32,
    pub hall_unknown_map_uint_uint: ::std::collections::HashMap<u32, u32>,
    pub player_avatar_id: u32,
    pub transform_id: ::prost::alloc::string::String,
    pub control_guise_avatar_id: u32,
    pub main_city_quest_id_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct ModMainCityTimeCsReq {
    pub time_period: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct ModMainCityTimeScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GetTimeInfoCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct TimeInfo {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GetTimeInfoScRsp {
    pub retcode: i32,
    pub time_info: ::core::option::Option<TimeInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct SceneTransitionCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct SceneTransitionScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(9870)]
pub struct SavePosInMainCityCsReq {
    #[prost(uint32, tag = "12", xor = "14823")]
    pub section_id: u32,
    #[prost(message, optional, tag = "7")]
    pub position: ::core::option::Option<super::common::Transform>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct SavePosInMainCityScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct TriggerInteractCsReq {
    pub interact_id: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct TriggerInteractScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct InteractWithUnitCsReq {
    pub npc_tag_id: i32,
    pub interact_id: i32,
    pub r#type: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct InteractWithUnitScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct EndNpcTalkCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct EndNpcTalkScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct ActionInfo {
    pub action_id: u32,
    pub action_type: i32,
    pub body: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct SectionEventScNotify {
    pub event_id: u32,
    pub section_id: u32,
    pub tag: u32,
    pub section_event_unk_1: u32,
    pub section_event_unk_2: u32,
    pub section_event_unk_3: u32,
    pub section_event_unk_4: u32,
    pub int_specials: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
    pub str_specials: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    pub section_event_map_str_int_2: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        i32,
    >,
    pub section_event_map_str_str_2: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    pub owner_type: i32,
    pub action_list: ::prost::alloc::vec::Vec<ActionInfo>,
    pub section_event_unk_string_list: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    pub hollow_event_id: u32,
    pub event_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct RunEventGraphCsReq {
    pub event_id: u32,
    pub section_id: u32,
    pub tag: u32,
    pub section_event_unk_1: u32,
    pub section_event_unk_2: u32,
    pub owner_type: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct RunEventGraphScRsp {
    pub retcode: i32,
    pub finish_event: bool,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct RunEventActionCsReq {
    pub event_id: u32,
    pub tag: u32,
    pub section_event_unk_1: u32,
    pub section_event_unk_2: u32,
    pub section_id: u32,
    pub owner_type: i32,
    pub action_id: u32,
    pub action_type: i32,
    pub action_body: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct RunEventActionScRsp {
    pub retcode: i32,
    pub finish_event: bool,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct FinishEventGraphScNotify {
    pub tag: u32,
    pub section_event_unk_2: u32,
    pub owner_type: i32,
    pub event_name: ::prost::alloc::string::String,
    pub event_uid: u32,
    pub finish_event_unk_1: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct EnterSectionCsReq {
    pub section_id: u32,
    pub transform_id: ::prost::alloc::string::String,
    pub tag: u32,
    pub owner_type: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct EnterSectionScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct CollectHollowRewardCsReq {
    pub reward_id_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct CollectHollowRewardScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(331)]
pub struct EndBattleCsReq {
    #[prost(message, optional, tag = "14")]
    pub fight_result: ::core::option::Option<super::common::FightResult>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct FightSettle {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
#[cmd_id(8037)]
pub struct EndBattleScRsp {
    #[prost(int32, tag = "11", xor = "12497")]
    pub retcode: i32,
    #[prost(message, optional, tag = "15")]
    pub fight_settle: ::core::option::Option<FightSettle>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct SyncLongFightProgressCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct SyncLongFightProgressScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct SyncGlobalVariablesCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct SyncGlobalVariablesScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(7539)]
pub struct LeaveCurSceneCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct LeaveCurSceneScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct SectionRefreshCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct SectionRefreshScRsp {
    pub retcode: i32,
    pub refresh_status: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct ServerPerformMsg {
    pub cmd_id: u32,
    pub content: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HollowPerformScNotify {
    pub msg_list: ::prost::alloc::vec::Vec<ServerPerformMsg>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HollowTickCsReq {
    pub quest_id: u32,
    pub unknown_hollow_tick: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct HollowTickScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HollowMoveCsReq {
    pub move_path: ::prost::alloc::vec::Vec<super::common::Vector2Int>,
    pub hollow_move_unknown: ::core::option::Option<super::common::Vector2Int>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct HollowMoveScRsp {
    pub retcode: i32,
    pub section_id: u32,
    pub new_position: ::core::option::Option<super::common::Vector2Int>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HollowEventReportCsReq {
    pub hollow_event_report_int_list: ::prost::alloc::vec::Vec<u32>,
    pub hollow_event_report_str_list: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct HollowEventReportScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct SyncHollowGridMapsScNotify {
    pub modify_entity_list: ::prost::alloc::vec::Vec<HollowEntityInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct HollowPushScNotify {
    pub prev_position: ::core::option::Option<super::common::Vector2Int>,
    pub cur_position: ::core::option::Option<super::common::Vector2Int>,
    pub cur_section_id: u32,
    pub entity_type: i32,
    pub reason: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct TriggerHollowEventCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct TriggerHollowEventScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(5866)]
pub struct GetWeaponDataCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
#[cmd_id(7353)]
pub struct GetWeaponDataScRsp {
    #[prost(int32, tag = "9", xor = "9803")]
    pub retcode: i32,
    #[prost(message, repeated, tag = "8")]
    pub weapon_list: ::prost::alloc::vec::Vec<WeaponInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(922)]
pub struct GetItemDataCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
#[cmd_id(3535)]
pub struct GetItemDataScRsp {
    #[prost(int32, tag = "10", xor = "4971")]
    pub retcode: i32,
    #[prost(message, repeated, tag = "6")]
    pub item_list: ::prost::alloc::vec::Vec<ItemInfo>,
    pub auto_recovery_info: ::std::collections::HashMap<u32, AutoRecoveryInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(8273)]
pub struct GetAvatarDataCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
#[cmd_id(6298)]
pub struct GetAvatarDataScRsp {
    #[prost(int32, tag = "12", xor = "49")]
    pub retcode: i32,
    #[prost(message, repeated, tag = "3")]
    pub avatar_list: ::prost::alloc::vec::Vec<AvatarInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct AddAvatarScNotify {
    pub avatar_id: u32,
    pub perform_type: i32,
    pub reward_list: ::prost::alloc::vec::Vec<ItemRewardInfo>,
    pub lock: bool,
}
/// Nested message and enum types in `AddAvatarScNotify`.
pub mod add_avatar_sc_notify {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PerformType {
        PerformNone = 0,
        PerformPopUp = 1,
        PerformAnimation = 2,
    }
    impl PerformType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::PerformNone => "PERFORM_NONE",
                Self::PerformPopUp => "PERFORM_POP_UP",
                Self::PerformAnimation => "PERFORM_ANIMATION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PERFORM_NONE" => Some(Self::PerformNone),
                "PERFORM_POP_UP" => Some(Self::PerformPopUp),
                "PERFORM_ANIMATION" => Some(Self::PerformAnimation),
                _ => None,
            }
        }
    }
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct AvatarLevelUpCsReq {
    pub avatar_id: u32,
    pub exp_materials: ::std::collections::HashMap<u32, u32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct AvatarLevelUpScRsp {
    pub retcode: i32,
    pub return_item_list: ::prost::alloc::vec::Vec<ItemRewardInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(6953)]
pub struct EquipmentDressCsReq {
    #[prost(uint32, tag = "14", xor = "10771")]
    pub avatar_id: u32,
    #[prost(uint32, tag = "7", xor = "9850")]
    pub equip_uid: u32,
    #[prost(uint32, tag = "4", xor = "5174")]
    pub dress_index: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct EquipmentDressScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(942)]
pub struct EquipmentUnDressCsReq {
    #[prost(uint32, tag = "7", xor = "6869")]
    pub avatar_id: u32,
    #[prost(uint32, repeated, tag = "4")]
    pub undress_index_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct EquipmentUnDressScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct EquipmentDressParam {
    #[prost(uint32, tag = "1", xor = "14812")]
    pub equip_uid: u32,
    #[prost(uint32, tag = "9", xor = "7613")]
    pub dress_index: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(1193)]
pub struct EquipmentSuitDressCsReq {
    #[prost(uint32, tag = "4", xor = "16013")]
    pub avatar_id: u32,
    #[prost(message, repeated, tag = "6")]
    pub param_list: ::prost::alloc::vec::Vec<EquipmentDressParam>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct EquipmentSuitDressScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(3478)]
pub struct WeaponDressCsReq {
    #[prost(uint32, tag = "12", xor = "7005")]
    pub avatar_id: u32,
    #[prost(uint32, tag = "1", xor = "15967")]
    pub weapon_uid: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct WeaponDressScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(2401)]
pub struct WeaponUnDressCsReq {
    #[prost(uint32, tag = "13", xor = "8114")]
    pub avatar_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct WeaponUnDressScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct TalentSwitchCsReq {
    pub avatar_id: u32,
    pub talent_switch_list: ::prost::alloc::vec::Vec<bool>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct TalentSwitchScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct AvatarSetAwakeCsReq {
    pub avatar_id: u32,
    pub awake_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct AvatarSetAwakeScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct AvatarShowWeaponCsReq {
    pub avatar_id: u32,
    pub show_weapon_type: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct AvatarShowWeaponScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(438)]
pub struct AvatarFavoriteCsReq {
    #[prost(uint32, tag = "1", xor = "6652")]
    pub avatar_id: u32,
    #[prost(bool, tag = "9")]
    pub is_favorite: bool,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct AvatarFavoriteScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(1473)]
pub struct AvatarSkinDressCsReq {
    #[prost(uint32, tag = "5", xor = "2734")]
    pub avatar_id: u32,
    #[prost(uint32, tag = "4", xor = "4683")]
    pub avatar_skin_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct AvatarSkinDressScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(9669)]
pub struct AvatarSkinUnDressCsReq {
    #[prost(uint32, tag = "1", xor = "12685")]
    pub avatar_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct AvatarSkinUnDressScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GetAvatarRecommendEquipCsReq {
    pub avatar_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GetAvatarRecommendEquipScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(1353)]
pub struct GetEquipDataCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
#[cmd_id(6775)]
pub struct GetEquipDataScRsp {
    #[prost(int32, tag = "6", xor = "2761")]
    pub retcode: i32,
    #[prost(message, repeated, tag = "10")]
    pub equip_list: ::prost::alloc::vec::Vec<EquipInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(8013)]
pub struct GetWishlistDataCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
#[cmd_id(902)]
pub struct GetWishlistDataScRsp {
    #[prost(int32, tag = "11", xor = "15482")]
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(8551)]
pub struct GetBuddyDataCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct BuddySkillLevel {
    pub skill_type: u32,
    pub level: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct BuddyInfo {
    pub id: u32,
    pub level: u32,
    pub exp: u32,
    pub rank: u32,
    pub star: u32,
    pub first_get_time: i64,
    pub skill_type_level: ::prost::alloc::vec::Vec<BuddySkillLevel>,
    pub taken_rank_up_reward_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GetBuddyDataScRsp {
    pub retcode: i32,
    pub buddy_list: ::prost::alloc::vec::Vec<BuddyInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GetAvatarRecommendItemsCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GetAvatarRecommendItemsScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GetGachaDataCsReq {
    pub gacha_type: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GachaMaterial {
    pub material_id: u32,
    pub num: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct Gacha {
    pub gacha_type: u32,
    pub gacha_id: u32,
    pub gacha_schedule_id: u32,
    pub begin_time: i64,
    pub end_time: i64,
    pub up_item_id_list: ::prost::alloc::vec::Vec<u32>,
    pub optional_up_item_id_list: ::prost::alloc::vec::Vec<u32>,
    pub gacha_material_list: ::prost::alloc::vec::Vec<GachaMaterial>,
    pub gacha_info_webview: ::prost::alloc::string::String,
    pub gacha_log_webview: ::prost::alloc::string::String,
    pub remain_up_item_times: u32,
    pub remain_optional_up_item_times: u32,
    pub newbie_avatar_id_list: ::prost::alloc::vec::Vec<u32>,
    pub newbie_remain_up_item_times: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GachaInfo {
    pub gacha_list: ::prost::alloc::vec::Vec<Gacha>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GachaDisplayData {
    pub gacha_random: u32,
    pub gacha_info: ::core::option::Option<GachaInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GetGachaDataScRsp {
    pub retcode: i32,
    pub gacha_type: u32,
    pub display: ::core::option::Option<GachaDisplayData>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct DoGachaCsReq {
    pub gacha_type: u32,
    pub gacha_id: u32,
    pub gacha_random: u32,
    pub times: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct DropItem {
    pub item_id: u32,
    pub uid: u32,
    pub count: u32,
    pub point_item_id: u32,
    pub point_item_count: u32,
    pub lock: bool,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct DoGachaScRsp {
    pub retcode: i32,
    pub times: u32,
    pub display: ::core::option::Option<GachaDisplayData>,
    pub drop_item_list: ::prost::alloc::vec::Vec<DropItem>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GachaBuyMaterialCsReq {
    pub buy_material_id: u32,
    pub count: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GachaBuyMaterialScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GachaSetNewbieAvatarCsReq {
    pub gacha_id: u32,
    pub avatar_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GachaSetNewbieAvatarScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(9471)]
pub struct VideoGetInfoCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
#[cmd_id(9262)]
pub struct VideoGetInfoScRsp {
    #[prost(int32, tag = "14", xor = "11884")]
    pub retcode: i32,
    #[prost(map = "uint32, uint64", tag = "4")]
    pub video_key_map: ::std::collections::HashMap<u32, u64>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(5260)]
pub struct SavePlayerAccessoryCsReq {
    #[prost(message, optional, tag = "10")]
    pub player_accessory: ::core::option::Option<PlayerAccessoryInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct SavePlayerAccessoryScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct SystemSwitchStateInfo {
    pub r#type: u32,
    pub switch_state: bool,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct InputSettingInfo {
    pub input_type_map: ::std::collections::HashMap<u32, i32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct SavePlayerSystemSettingCsReq {
    pub r#type: u32,
    pub params: u32,
    pub system_switch_state: ::core::option::Option<SystemSwitchStateInfo>,
    pub input_setting: ::core::option::Option<InputSettingInfo>,
    pub setting_content_map: ::std::collections::HashMap<u32, u32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct SavePlayerSystemSettingScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GetSwitchDataCsReq {
    pub r#type: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct SwitchData {
    pub open_system_id_list: ::prost::alloc::vec::Vec<u32>,
    pub switch_of_qte: bool,
    pub switch_of_story_mode: bool,
    pub system_switch_state_list: ::prost::alloc::vec::Vec<SystemSwitchStateInfo>,
    pub input_setting_map: ::std::collections::HashMap<u32, InputSettingInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GetSwitchDataScRsp {
    pub retcode: i32,
    pub r#type: u32,
    pub switch_data: ::core::option::Option<SwitchData>,
    pub setting_switch_map: ::std::collections::HashMap<u32, u32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[cmd_id(2731)]
pub struct GetMiscDataCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct QuickAccessInfo {
    pub quick_access_id: u32,
    pub quick_access_index: u32,
    pub r#type: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct UnlockInfo {
    #[prost(int32, repeated, tag = "7")]
    pub unlocked_list: ::prost::alloc::vec::Vec<i32>,
    pub quick_access_list: ::prost::alloc::vec::Vec<QuickAccessInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct TeleportUnlockInfo {
    pub unlocked_list: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct NewsStandData {
    pub cur_style: i32,
    pub normal_news_id_list: ::prost::alloc::vec::Vec<u32>,
    pub head_lines_id_list: ::prost::alloc::vec::Vec<u32>,
    pub advertisement_id_list: ::prost::alloc::vec::Vec<u32>,
    pub news_read_state: bool,
    pub can_sign: bool,
    pub current_sign_id: u32,
    pub sign_count: u32,
    pub sign_refresh_time: i64,
    pub last_sign_time: i64,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct PostGirlItem {
    #[prost(uint32, tag = "14", xor = "5999")]
    pub id: u32,
    #[prost(int64, tag = "1", xor = "3271")]
    pub unlock_time: i64,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct PostGirlInfo {
    #[prost(message, repeated, tag = "12")]
    pub post_girl_item_list: ::prost::alloc::vec::Vec<PostGirlItem>,
    #[prost(uint32, repeated, tag = "2")]
    pub selected_post_girl_id_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(bool, tag = "9")]
    pub post_girl_random_toggle: bool,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct BusinessCardData {
    #[prost(uint32, repeated, tag = "15")]
    pub unlocked_business_card_id_list: ::prost::alloc::vec::Vec<u32>,
    pub selected_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct PlayerSkinInfo {
    #[prost(uint32, tag = "1", xor = "16079")]
    pub player_skin_id: u32,
    #[prost(uint32, repeated, tag = "14")]
    pub equipped_accessory_id_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct PlayerAccessoryInfo {
    #[prost(uint32, tag = "4", xor = "7858")]
    pub avatar_id: u32,
    #[prost(uint32, tag = "9", xor = "1443")]
    pub avatar_skin_id: u32,
    #[prost(message, repeated, tag = "13")]
    pub player_skin_list: ::prost::alloc::vec::Vec<PlayerSkinInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct PlayerAccessoryData {
    #[prost(message, repeated, tag = "12")]
    pub player_accessory_list: ::prost::alloc::vec::Vec<PlayerAccessoryInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct MiscData {
    #[prost(message, optional, tag = "3")]
    pub unlock: ::core::option::Option<UnlockInfo>,
    pub teleport: ::core::option::Option<TeleportUnlockInfo>,
    pub news_stand: ::core::option::Option<NewsStandData>,
    #[prost(message, optional, tag = "6")]
    pub post_girl: ::core::option::Option<PostGirlInfo>,
    #[prost(message, optional, tag = "2030")]
    pub business_card: ::core::option::Option<BusinessCardData>,
    #[prost(message, optional, tag = "590")]
    pub player_accessory: ::core::option::Option<PlayerAccessoryData>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
#[cmd_id(2434)]
pub struct GetMiscDataScRsp {
    #[prost(int32, tag = "11", xor = "9395")]
    pub retcode: i32,
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<MiscData>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct NewsStandSync {
    pub current_sign_id: u32,
    pub can_sign: bool,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct PostGirlSync {
    pub new_post_girl_item_list: ::prost::alloc::vec::Vec<PostGirlItem>,
    pub selected_post_girl_id_list: ::prost::alloc::vec::Vec<u32>,
    pub post_girl_random_toggle: bool,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct BusinessCardSync {
    pub unlocked_business_card_id_list: ::prost::alloc::vec::Vec<u32>,
    pub selected_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct PlayerAccessorySync {
    #[prost(message, repeated, tag = "9")]
    pub player_accessory_list: ::prost::alloc::vec::Vec<PlayerAccessoryInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct MiscSync {
    pub trigger_newbie_group_list: ::prost::alloc::vec::Vec<i32>,
    pub quick_access_list: ::prost::alloc::vec::Vec<QuickAccessInfo>,
    pub news_stand: ::core::option::Option<NewsStandSync>,
    #[prost(message, optional, tag = "1574")]
    pub post_girl: ::core::option::Option<PostGirlSync>,
    #[prost(message, optional, tag = "91")]
    pub business_card: ::core::option::Option<BusinessCardSync>,
    #[prost(message, optional, tag = "950")]
    pub player_accessory: ::core::option::Option<PlayerAccessorySync>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct SelectPostGirlCsReq {
    pub post_girl_id_list: ::prost::alloc::vec::Vec<u32>,
    pub post_girl_random_toggle: bool,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct SelectPostGirlScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GameLogReportCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GameLogReportScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct PlayerOperationCsReq {
    pub system: u32,
    pub operator: u32,
    pub param: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct PlayerOperationScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct EndNewbieCsReq {
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct EndNewbieScRsp {
    pub retcode: i32,
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct FinishNewbieGroupCsReq {
    pub group_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct FinishNewbieGroupScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GetBattleDataCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GetBattleDataScRsp {
    pub retcode: i32,
    pub battle_data: ::core::option::Option<BattleData>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct BattleData {
    pub battle_data: ::core::option::Option<ActivityBattleData>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct ActivityBattleData {
    pub boss_battle: ::core::option::Option<BossBattleData>,
    pub double_elite: ::core::option::Option<DoubleEliteData>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct BossBattleData {
    pub unlocked_levels: ::prost::alloc::vec::Vec<u32>,
    pub selected_level: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct DoubleEliteProgress {
    pub quest_id: u32,
    pub unlocked: bool,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct DoubleEliteData {
    pub unlocked_levels: ::prost::alloc::vec::Vec<u32>,
    pub progress_list: ::prost::alloc::vec::Vec<DoubleEliteProgress>,
    pub selected_difficulty: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GetNewsStandDataCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GetNewsStandDataScRsp {
    pub retcode: i32,
    pub news_stand_data: ::core::option::Option<NewsStandData>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct ReadNewsCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct ReadNewsScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct NewsStandSignCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct NewsStandSignScRsp {
    pub retcode: i32,
    pub sign_count: i32,
    pub last_sign_time: i64,
    pub reward_list: ::prost::alloc::vec::Vec<ItemRewardInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct ReportUiLayoutPlatformCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct ReportUiLayoutPlatformScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct BattleReportCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct BattleReportScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GetRamenDataCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct RamenData {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GetRamenDataScRsp {
    pub retcode: i32,
    pub ramen_data: ::core::option::Option<RamenData>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct PerformTriggerCsReq {
    pub perform_id: i32,
    pub perform_type: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct PerformTriggerScRsp {
    pub retcode: i32,
    pub perform_uid: i64,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct PerformEndCsReq {
    pub perform_id: i32,
    pub perform_type: i32,
    pub perform_uid: i64,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct PerformEndScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct PerformJumpCsReq {
    pub perform_id: i32,
    pub perform_type: i32,
    pub perform_uid: i64,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct PerformJumpScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct StartTrainingCsReq {
    pub avatar_list: ::prost::alloc::vec::Vec<i32>,
    pub special_training_id: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct StartTrainingScRsp {
    pub retcode: i32,
    pub training_uid: i64,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GetPhotoWallDataCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GetPhotoWallDataScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GetMonthDailyRewardListCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GetMonthDailyRewardListScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GetAreaMapDataCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct AreaMapData {
    pub group: ::prost::alloc::vec::Vec<AreaGroupInfo>,
    pub collect: ::prost::alloc::vec::Vec<AreaCollectInfo>,
    pub street: ::prost::alloc::vec::Vec<AreaStreetInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct AreaGroupInfo {
    pub group_id: u32,
    pub is_unlocked: bool,
    pub area_progress: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct AreaCollectInfo {
    pub r#type: u32,
    pub owner_type: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct AreaStreetInfo {
    pub area_id: u32,
    pub is_unlocked: bool,
    pub area_progress: u32,
    pub location_pop_showed: bool,
    pub new_area_showed: bool,
    pub new_area_portals_showed: bool,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GetAreaMapDataScRsp {
    pub retcode: i32,
    pub data: ::core::option::Option<AreaMapData>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct AreaMapModStateCsReq {
    pub area_id: u32,
    pub location_pop_showed: bool,
    pub new_area_showed: bool,
    pub new_area_portals_showed: bool,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct AreaMapModStateScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GetAreaPortalDataCsReq {
    pub area_portal_id_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GetAreaPortalDataScRsp {
    pub retcode: i32,
    pub area_portal_id_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GetFloorActiveGroupListCsReq {
    pub floor_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GetFloorActiveGroupListScRsp {
    pub retcode: i32,
    pub group_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GetFloorGroupMemberListCsReq {
    pub floor_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct FloorGroupMemberInfo {
    pub group_id: u32,
    pub config_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GetFloorGroupMemberListScRsp {
    pub retcode: i32,
    pub floor_group_portal_list: ::prost::alloc::vec::Vec<FloorGroupMemberInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct GetBigSceneDataCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct BigSceneTeamData {
    pub scene_avatar_list: ::prost::alloc::vec::Vec<super::common::BigSceneAvatarInfo>,
    pub cur_scene_avatar_list: ::prost::alloc::vec::Vec<
        super::common::BigSceneAvatarInfo,
    >,
    pub cur_avatar_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct GetBigSceneDataScRsp {
    pub retcode: i32,
    pub big_scene_team: ::core::option::Option<BigSceneTeamData>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct BigSceneTeamReplaceCsReq {
    pub scene_avatar_id_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct BigSceneTeamReplaceScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct FloorPositionInfo {
    pub player_pos: ::core::option::Option<super::common::Vector3>,
    pub player_rot: ::core::option::Option<super::common::Vector3>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct EnterBigSceneFloorCsReq {
    pub floor_id: u32,
    pub floor_version: u32,
    pub floor_md5: ::prost::alloc::string::String,
    pub floor_position: ::core::option::Option<FloorPositionInfo>,
    pub rollback_point: ::core::option::Option<super::common::RollbackPointInfo>,
    pub player_pos: ::core::option::Option<super::common::Vector3>,
    pub player_rot: ::core::option::Option<super::common::Vector3>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct EnterBigSceneFloorScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct EnterBigSceneScNotify {
    pub scene_id: u32,
    pub floor_id: u32,
    pub pos: ::core::option::Option<super::common::Vector3>,
    pub floor_position: ::core::option::Option<FloorPositionInfo>,
    pub rollback_point: ::core::option::Option<super::common::RollbackPointInfo>,
    pub cur_entity_info: ::core::option::Option<super::common::SceneEntityInfo>,
    pub day_of_week: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct EnterFloorDoneCsReq {
    pub floor_version: u32,
    pub floor_md5: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct EnterFloorDoneScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct SaveBigSceneVariablesCsReq {}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct SaveBigSceneVariablesScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct BigSceneAvatarChangeCsReq {
    pub avatar_id: u32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct BigSceneAvatarChangeScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct SceneEntityPositionInfo {
    pub entity_id: u32,
    pub position: ::core::option::Option<super::common::Vector3>,
    pub rotation: ::core::option::Option<super::common::Vector3>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct SyncSceneEntityPositionCsReq {
    pub entity_position_list: ::prost::alloc::vec::Vec<SceneEntityPositionInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct SyncSceneEntityPositionScRsp {
    pub retcode: i32,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
pub struct ActiveRollbackPointCsReq {
    pub group_id: u32,
    pub rollback_point: ::core::option::Option<super::common::RollbackPointInfo>,
}
#[derive(Clone, PartialEq, ::proto_derive::Message, ::proto_derive::NetCmd)]
#[derive(::proto_derive::NetResponse)]
pub struct ActiveRollbackPointScRsp {
    pub retcode: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ModAvatarType {
    Unk0 = 0,
    Unk1 = 1,
    Unk2 = 2,
}
impl ModAvatarType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unk0 => "MOD_AVATAR_TYPE_UNK_0",
            Self::Unk1 => "MOD_AVATAR_TYPE_UNK_1",
            Self::Unk2 => "MOD_AVATAR_TYPE_UNK_2",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MOD_AVATAR_TYPE_UNK_0" => Some(Self::Unk0),
            "MOD_AVATAR_TYPE_UNK_1" => Some(Self::Unk1),
            "MOD_AVATAR_TYPE_UNK_2" => Some(Self::Unk2),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BigSceneTeamStatus {
    None = 0,
    Modified = 1,
}
impl BigSceneTeamStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "BIG_SCENE_TEAM_STATUS_NONE",
            Self::Modified => "BIG_SCENE_TEAM_STATUS_MODIFIED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BIG_SCENE_TEAM_STATUS_NONE" => Some(Self::None),
            "BIG_SCENE_TEAM_STATUS_MODIFIED" => Some(Self::Modified),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AvatarShowWeaponType {
    ShowWeaponLock = 0,
    ShowWeaponActive = 1,
    ShowWeaponInactive = 2,
}
impl AvatarShowWeaponType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::ShowWeaponLock => "SHOW_WEAPON_LOCK",
            Self::ShowWeaponActive => "SHOW_WEAPON_ACTIVE",
            Self::ShowWeaponInactive => "SHOW_WEAPON_INACTIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SHOW_WEAPON_LOCK" => Some(Self::ShowWeaponLock),
            "SHOW_WEAPON_ACTIVE" => Some(Self::ShowWeaponActive),
            "SHOW_WEAPON_INACTIVE" => Some(Self::ShowWeaponInactive),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum QuestState {
    Unlock = 0,
    RewardGot = 1,
    NotFetch = 2,
    NotFinished = 3,
    FinishedNotGot = 4,
}
impl QuestState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unlock => "QUEST_STATE_UNLOCK",
            Self::RewardGot => "QUEST_STATE_REWARD_GOT",
            Self::NotFetch => "QUEST_STATE_NOT_FETCH",
            Self::NotFinished => "QUEST_STATE_NOT_FINISHED",
            Self::FinishedNotGot => "QUEST_STATE_FINISHED_NOT_GOT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "QUEST_STATE_UNLOCK" => Some(Self::Unlock),
            "QUEST_STATE_REWARD_GOT" => Some(Self::RewardGot),
            "QUEST_STATE_NOT_FETCH" => Some(Self::NotFetch),
            "QUEST_STATE_NOT_FINISHED" => Some(Self::NotFinished),
            "QUEST_STATE_FINISHED_NOT_GOT" => Some(Self::FinishedNotGot),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InteractTarget {
    None = 0,
    TriggerBox = 1,
    Npc = 2,
}
impl InteractTarget {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "INTERACT_TARGET_NONE",
            Self::TriggerBox => "INTERACT_TARGET_TRIGGER_BOX",
            Self::Npc => "INTERACT_TARGET_NPC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INTERACT_TARGET_NONE" => Some(Self::None),
            "INTERACT_TARGET_TRIGGER_BOX" => Some(Self::TriggerBox),
            "INTERACT_TARGET_NPC" => Some(Self::Npc),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HenshinType {
    None = 0,
    BomberTick5 = 10,
    Tugger = 11,
    SpeedupMove = 19,
    SpeedupRush = 20,
    BomberTick0 = 5,
    RinaBangboo1 = 13,
    Pacmanv2 = 3,
    BomberTick1 = 6,
    QinYi = 16,
    AvatarNekomiya = 18,
    RinaBangboo2 = 14,
    BomberTick3 = 8,
    DefaultPlayer = 1,
    Pacmanv1 = 2,
    Bomber = 4,
    AvatarCorin = 17,
    ActivityHacker = 21,
    BomberTick2 = 7,
    GoodsLoader = 15,
    BomberTick4 = 9,
    NightMode = 12,
}
impl HenshinType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "HenshinType_NONE",
            Self::BomberTick5 => "HenshinType_Bomber_Tick5",
            Self::Tugger => "HenshinType_Tugger",
            Self::SpeedupMove => "HenshinType_Speedup_Move",
            Self::SpeedupRush => "HenshinType_Speedup_Rush",
            Self::BomberTick0 => "HenshinType_Bomber_Tick0",
            Self::RinaBangboo1 => "HenshinType_Rina_Bangboo1",
            Self::Pacmanv2 => "HenshinType_PACMANV2",
            Self::BomberTick1 => "HenshinType_Bomber_Tick1",
            Self::QinYi => "HenshinType_QinYi",
            Self::AvatarNekomiya => "HenshinType_Avatar_Nekomiya",
            Self::RinaBangboo2 => "HenshinType_Rina_Bangboo2",
            Self::BomberTick3 => "HenshinType_Bomber_Tick3",
            Self::DefaultPlayer => "HenshinType_DEFAULT_PLAYER",
            Self::Pacmanv1 => "HenshinType_PACMANV1",
            Self::Bomber => "HenshinType_BOMBER",
            Self::AvatarCorin => "HenshinType_Avatar_Corin",
            Self::ActivityHacker => "HenshinType_Activity_Hacker",
            Self::BomberTick2 => "HenshinType_Bomber_Tick2",
            Self::GoodsLoader => "HenshinType_GoodsLoader",
            Self::BomberTick4 => "HenshinType_Bomber_Tick4",
            Self::NightMode => "HenshinType_NIGHT_MODE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HenshinType_NONE" => Some(Self::None),
            "HenshinType_Bomber_Tick5" => Some(Self::BomberTick5),
            "HenshinType_Tugger" => Some(Self::Tugger),
            "HenshinType_Speedup_Move" => Some(Self::SpeedupMove),
            "HenshinType_Speedup_Rush" => Some(Self::SpeedupRush),
            "HenshinType_Bomber_Tick0" => Some(Self::BomberTick0),
            "HenshinType_Rina_Bangboo1" => Some(Self::RinaBangboo1),
            "HenshinType_PACMANV2" => Some(Self::Pacmanv2),
            "HenshinType_Bomber_Tick1" => Some(Self::BomberTick1),
            "HenshinType_QinYi" => Some(Self::QinYi),
            "HenshinType_Avatar_Nekomiya" => Some(Self::AvatarNekomiya),
            "HenshinType_Rina_Bangboo2" => Some(Self::RinaBangboo2),
            "HenshinType_Bomber_Tick3" => Some(Self::BomberTick3),
            "HenshinType_DEFAULT_PLAYER" => Some(Self::DefaultPlayer),
            "HenshinType_PACMANV1" => Some(Self::Pacmanv1),
            "HenshinType_BOMBER" => Some(Self::Bomber),
            "HenshinType_Avatar_Corin" => Some(Self::AvatarCorin),
            "HenshinType_Activity_Hacker" => Some(Self::ActivityHacker),
            "HenshinType_Bomber_Tick2" => Some(Self::BomberTick2),
            "HenshinType_GoodsLoader" => Some(Self::GoodsLoader),
            "HenshinType_Bomber_Tick4" => Some(Self::BomberTick4),
            "HenshinType_NIGHT_MODE" => Some(Self::NightMode),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HollowEntityType {
    None = 0,
    Global = 9,
    Npc = 2,
    Pin = 6,
    Light = 5,
    Player = 1,
    Barrier = 10,
    Grid = 3,
    SectionEvent = 11,
    Section = 7,
    Terrain = 12,
    Event = 4,
}
impl HollowEntityType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "HOLLOW_ENTITY_TYPE_NONE",
            Self::Global => "HOLLOW_ENTITY_TYPE_GLOBAL",
            Self::Npc => "HOLLOW_ENTITY_TYPE_NPC",
            Self::Pin => "HOLLOW_ENTITY_TYPE_PIN",
            Self::Light => "HOLLOW_ENTITY_TYPE_LIGHT",
            Self::Player => "HOLLOW_ENTITY_TYPE_PLAYER",
            Self::Barrier => "HOLLOW_ENTITY_TYPE_BARRIER",
            Self::Grid => "HOLLOW_ENTITY_TYPE_GRID",
            Self::SectionEvent => "HOLLOW_ENTITY_TYPE_SECTION_EVENT",
            Self::Section => "HOLLOW_ENTITY_TYPE_SECTION",
            Self::Terrain => "HOLLOW_ENTITY_TYPE_TERRAIN",
            Self::Event => "HOLLOW_ENTITY_TYPE_EVENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HOLLOW_ENTITY_TYPE_NONE" => Some(Self::None),
            "HOLLOW_ENTITY_TYPE_GLOBAL" => Some(Self::Global),
            "HOLLOW_ENTITY_TYPE_NPC" => Some(Self::Npc),
            "HOLLOW_ENTITY_TYPE_PIN" => Some(Self::Pin),
            "HOLLOW_ENTITY_TYPE_LIGHT" => Some(Self::Light),
            "HOLLOW_ENTITY_TYPE_PLAYER" => Some(Self::Player),
            "HOLLOW_ENTITY_TYPE_BARRIER" => Some(Self::Barrier),
            "HOLLOW_ENTITY_TYPE_GRID" => Some(Self::Grid),
            "HOLLOW_ENTITY_TYPE_SECTION_EVENT" => Some(Self::SectionEvent),
            "HOLLOW_ENTITY_TYPE_SECTION" => Some(Self::Section),
            "HOLLOW_ENTITY_TYPE_TERRAIN" => Some(Self::Terrain),
            "HOLLOW_ENTITY_TYPE_EVENT" => Some(Self::Event),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HollowGridMapType {
    None = 0,
    Unknown1 = 1,
    Unknown2 = 2,
}
impl HollowGridMapType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "HOLLOW_GRID_MAP_TYPE_NONE",
            Self::Unknown1 => "HOLLOW_GRID_MAP_TYPE_UNKNOWN_1",
            Self::Unknown2 => "HOLLOW_GRID_MAP_TYPE_UNKNOWN_2",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HOLLOW_GRID_MAP_TYPE_NONE" => Some(Self::None),
            "HOLLOW_GRID_MAP_TYPE_UNKNOWN_1" => Some(Self::Unknown1),
            "HOLLOW_GRID_MAP_TYPE_UNKNOWN_2" => Some(Self::Unknown2),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HollowComponentType {
    None = 0,
    IdComponent = 1,
    PosComponent = 2,
    HollowGridComponent = 3,
    LightInteractionComponent = 4,
    OwnedEntityComponent = 5,
    ChessUiComponent = 6,
    HollowEventComponent = 7,
    CategoryComponent = 8,
    BehaviorComponent = 9,
    OwnerComponent = 10,
    HollowNpcComponent = 11,
    HollowSnakeComponent = 12,
    HollowLightComponent = 13,
    ExtScriptVariableComponent = 14,
    ConwayLifeGameLifeStateComponent = 15,
    EntityPriorityComponent = 16,
    BigTvChessUiComponent = 17,
    GridStateComponent = 18,
    SpringComponent = 19,
    BlockComponent = 20,
    ConwayLifeGameMgrComponent = 21,
    HollowScriptSequenceComponent = 22,
    HollowSnapshotComponent = 23,
    HollowMapComponent = 24,
    HollowPluginCollectionComponent = 25,
    InnerWorldPlugin = 26,
    HollowLightPlugin = 27,
    HollowNpcMgrComponent = 28,
    HollowTimeRewindComponent = 29,
    NpcPosExt = 30,
    ClientStateComponent = 31,
    PlayerPosExt = 33,
    HollowRepairZoneComponent = 34,
    HollowGlobalComponent = 35,
    AimRectComponent = 36,
    SignalMgrComponent = 37,
    HollowFloorMgrComponent = 39,
    AreaCameraComponent = 40,
    GridInnerWorldComponent = 41,
    HollowSectionComponent = 42,
    BigWorldComponent = 44,
    ElevatorGridComponent = 45,
    HideComponent = 46,
    HollowSpawnerComponent = 47,
    HollowHackerGameComponent = 48,
    PopInteractComponent = 49,
    AbilityMarkComponent = 50,
}
impl HollowComponentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "HOLLOW_COMPONENT_TYPE_NONE",
            Self::IdComponent => "HOLLOW_COMPONENT_TYPE_ID_COMPONENT",
            Self::PosComponent => "HOLLOW_COMPONENT_TYPE_POS_COMPONENT",
            Self::HollowGridComponent => "HOLLOW_COMPONENT_TYPE_HOLLOW_GRID_COMPONENT",
            Self::LightInteractionComponent => {
                "HOLLOW_COMPONENT_TYPE_LIGHT_INTERACTION_COMPONENT"
            }
            Self::OwnedEntityComponent => "HOLLOW_COMPONENT_TYPE_OWNED_ENTITY_COMPONENT",
            Self::ChessUiComponent => "HOLLOW_COMPONENT_TYPE_CHESS_UI_COMPONENT",
            Self::HollowEventComponent => "HOLLOW_COMPONENT_TYPE_HOLLOW_EVENT_COMPONENT",
            Self::CategoryComponent => "HOLLOW_COMPONENT_TYPE_CATEGORY_COMPONENT",
            Self::BehaviorComponent => "HOLLOW_COMPONENT_TYPE_BEHAVIOR_COMPONENT",
            Self::OwnerComponent => "HOLLOW_COMPONENT_TYPE_OWNER_COMPONENT",
            Self::HollowNpcComponent => "HOLLOW_COMPONENT_TYPE_HOLLOW_NPC_COMPONENT",
            Self::HollowSnakeComponent => "HOLLOW_COMPONENT_TYPE_HOLLOW_SNAKE_COMPONENT",
            Self::HollowLightComponent => "HOLLOW_COMPONENT_TYPE_HOLLOW_LIGHT_COMPONENT",
            Self::ExtScriptVariableComponent => {
                "HOLLOW_COMPONENT_TYPE_EXT_SCRIPT_VARIABLE_COMPONENT"
            }
            Self::ConwayLifeGameLifeStateComponent => {
                "HOLLOW_COMPONENT_TYPE_CONWAY_LIFE_GAME_LIFE_STATE_COMPONENT"
            }
            Self::EntityPriorityComponent => {
                "HOLLOW_COMPONENT_TYPE_ENTITY_PRIORITY_COMPONENT"
            }
            Self::BigTvChessUiComponent => {
                "HOLLOW_COMPONENT_TYPE_BIG_TV_CHESS_UI_COMPONENT"
            }
            Self::GridStateComponent => "HOLLOW_COMPONENT_TYPE_GRID_STATE_COMPONENT",
            Self::SpringComponent => "HOLLOW_COMPONENT_TYPE_SPRING_COMPONENT",
            Self::BlockComponent => "HOLLOW_COMPONENT_TYPE_BLOCK_COMPONENT",
            Self::ConwayLifeGameMgrComponent => {
                "HOLLOW_COMPONENT_TYPE_CONWAY_LIFE_GAME_MGR_COMPONENT"
            }
            Self::HollowScriptSequenceComponent => {
                "HOLLOW_COMPONENT_TYPE_HOLLOW_SCRIPT_SEQUENCE_COMPONENT"
            }
            Self::HollowSnapshotComponent => {
                "HOLLOW_COMPONENT_TYPE_HOLLOW_SNAPSHOT_COMPONENT"
            }
            Self::HollowMapComponent => "HOLLOW_COMPONENT_TYPE_HOLLOW_MAP_COMPONENT",
            Self::HollowPluginCollectionComponent => {
                "HOLLOW_COMPONENT_TYPE_HOLLOW_PLUGIN_COLLECTION_COMPONENT"
            }
            Self::InnerWorldPlugin => "HOLLOW_COMPONENT_TYPE_INNER_WORLD_PLUGIN",
            Self::HollowLightPlugin => "HOLLOW_COMPONENT_TYPE_HOLLOW_LIGHT_PLUGIN",
            Self::HollowNpcMgrComponent => {
                "HOLLOW_COMPONENT_TYPE_HOLLOW_NPC_MGR_COMPONENT"
            }
            Self::HollowTimeRewindComponent => {
                "HOLLOW_COMPONENT_TYPE_HOLLOW_TIME_REWIND_COMPONENT"
            }
            Self::NpcPosExt => "HOLLOW_COMPONENT_TYPE_NPC_POS_EXT",
            Self::ClientStateComponent => "HOLLOW_COMPONENT_TYPE_CLIENT_STATE_COMPONENT",
            Self::PlayerPosExt => "HOLLOW_COMPONENT_TYPE_PLAYER_POS_EXT",
            Self::HollowRepairZoneComponent => {
                "HOLLOW_COMPONENT_TYPE_HOLLOW_REPAIR_ZONE_COMPONENT"
            }
            Self::HollowGlobalComponent => {
                "HOLLOW_COMPONENT_TYPE_HOLLOW_GLOBAL_COMPONENT"
            }
            Self::AimRectComponent => "HOLLOW_COMPONENT_TYPE_AIM_RECT_COMPONENT",
            Self::SignalMgrComponent => "HOLLOW_COMPONENT_TYPE_SIGNAL_MGR_COMPONENT",
            Self::HollowFloorMgrComponent => {
                "HOLLOW_COMPONENT_TYPE_HOLLOW_FLOOR_MGR_COMPONENT"
            }
            Self::AreaCameraComponent => "HOLLOW_COMPONENT_TYPE_AREA_CAMERA_COMPONENT",
            Self::GridInnerWorldComponent => {
                "HOLLOW_COMPONENT_TYPE_GRID_INNER_WORLD_COMPONENT"
            }
            Self::HollowSectionComponent => {
                "HOLLOW_COMPONENT_TYPE_HOLLOW_SECTION_COMPONENT"
            }
            Self::BigWorldComponent => "HOLLOW_COMPONENT_TYPE_BIG_WORLD_COMPONENT",
            Self::ElevatorGridComponent => {
                "HOLLOW_COMPONENT_TYPE_ELEVATOR_GRID_COMPONENT"
            }
            Self::HideComponent => "HOLLOW_COMPONENT_TYPE_HIDE_COMPONENT",
            Self::HollowSpawnerComponent => {
                "HOLLOW_COMPONENT_TYPE_HOLLOW_SPAWNER_COMPONENT"
            }
            Self::HollowHackerGameComponent => {
                "HOLLOW_COMPONENT_TYPE_HOLLOW_HACKER_GAME_COMPONENT"
            }
            Self::PopInteractComponent => "HOLLOW_COMPONENT_TYPE_POP_INTERACT_COMPONENT",
            Self::AbilityMarkComponent => "HOLLOW_COMPONENT_TYPE_ABILITY_MARK_COMPONENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HOLLOW_COMPONENT_TYPE_NONE" => Some(Self::None),
            "HOLLOW_COMPONENT_TYPE_ID_COMPONENT" => Some(Self::IdComponent),
            "HOLLOW_COMPONENT_TYPE_POS_COMPONENT" => Some(Self::PosComponent),
            "HOLLOW_COMPONENT_TYPE_HOLLOW_GRID_COMPONENT" => {
                Some(Self::HollowGridComponent)
            }
            "HOLLOW_COMPONENT_TYPE_LIGHT_INTERACTION_COMPONENT" => {
                Some(Self::LightInteractionComponent)
            }
            "HOLLOW_COMPONENT_TYPE_OWNED_ENTITY_COMPONENT" => {
                Some(Self::OwnedEntityComponent)
            }
            "HOLLOW_COMPONENT_TYPE_CHESS_UI_COMPONENT" => Some(Self::ChessUiComponent),
            "HOLLOW_COMPONENT_TYPE_HOLLOW_EVENT_COMPONENT" => {
                Some(Self::HollowEventComponent)
            }
            "HOLLOW_COMPONENT_TYPE_CATEGORY_COMPONENT" => Some(Self::CategoryComponent),
            "HOLLOW_COMPONENT_TYPE_BEHAVIOR_COMPONENT" => Some(Self::BehaviorComponent),
            "HOLLOW_COMPONENT_TYPE_OWNER_COMPONENT" => Some(Self::OwnerComponent),
            "HOLLOW_COMPONENT_TYPE_HOLLOW_NPC_COMPONENT" => {
                Some(Self::HollowNpcComponent)
            }
            "HOLLOW_COMPONENT_TYPE_HOLLOW_SNAKE_COMPONENT" => {
                Some(Self::HollowSnakeComponent)
            }
            "HOLLOW_COMPONENT_TYPE_HOLLOW_LIGHT_COMPONENT" => {
                Some(Self::HollowLightComponent)
            }
            "HOLLOW_COMPONENT_TYPE_EXT_SCRIPT_VARIABLE_COMPONENT" => {
                Some(Self::ExtScriptVariableComponent)
            }
            "HOLLOW_COMPONENT_TYPE_CONWAY_LIFE_GAME_LIFE_STATE_COMPONENT" => {
                Some(Self::ConwayLifeGameLifeStateComponent)
            }
            "HOLLOW_COMPONENT_TYPE_ENTITY_PRIORITY_COMPONENT" => {
                Some(Self::EntityPriorityComponent)
            }
            "HOLLOW_COMPONENT_TYPE_BIG_TV_CHESS_UI_COMPONENT" => {
                Some(Self::BigTvChessUiComponent)
            }
            "HOLLOW_COMPONENT_TYPE_GRID_STATE_COMPONENT" => {
                Some(Self::GridStateComponent)
            }
            "HOLLOW_COMPONENT_TYPE_SPRING_COMPONENT" => Some(Self::SpringComponent),
            "HOLLOW_COMPONENT_TYPE_BLOCK_COMPONENT" => Some(Self::BlockComponent),
            "HOLLOW_COMPONENT_TYPE_CONWAY_LIFE_GAME_MGR_COMPONENT" => {
                Some(Self::ConwayLifeGameMgrComponent)
            }
            "HOLLOW_COMPONENT_TYPE_HOLLOW_SCRIPT_SEQUENCE_COMPONENT" => {
                Some(Self::HollowScriptSequenceComponent)
            }
            "HOLLOW_COMPONENT_TYPE_HOLLOW_SNAPSHOT_COMPONENT" => {
                Some(Self::HollowSnapshotComponent)
            }
            "HOLLOW_COMPONENT_TYPE_HOLLOW_MAP_COMPONENT" => {
                Some(Self::HollowMapComponent)
            }
            "HOLLOW_COMPONENT_TYPE_HOLLOW_PLUGIN_COLLECTION_COMPONENT" => {
                Some(Self::HollowPluginCollectionComponent)
            }
            "HOLLOW_COMPONENT_TYPE_INNER_WORLD_PLUGIN" => Some(Self::InnerWorldPlugin),
            "HOLLOW_COMPONENT_TYPE_HOLLOW_LIGHT_PLUGIN" => Some(Self::HollowLightPlugin),
            "HOLLOW_COMPONENT_TYPE_HOLLOW_NPC_MGR_COMPONENT" => {
                Some(Self::HollowNpcMgrComponent)
            }
            "HOLLOW_COMPONENT_TYPE_HOLLOW_TIME_REWIND_COMPONENT" => {
                Some(Self::HollowTimeRewindComponent)
            }
            "HOLLOW_COMPONENT_TYPE_NPC_POS_EXT" => Some(Self::NpcPosExt),
            "HOLLOW_COMPONENT_TYPE_CLIENT_STATE_COMPONENT" => {
                Some(Self::ClientStateComponent)
            }
            "HOLLOW_COMPONENT_TYPE_PLAYER_POS_EXT" => Some(Self::PlayerPosExt),
            "HOLLOW_COMPONENT_TYPE_HOLLOW_REPAIR_ZONE_COMPONENT" => {
                Some(Self::HollowRepairZoneComponent)
            }
            "HOLLOW_COMPONENT_TYPE_HOLLOW_GLOBAL_COMPONENT" => {
                Some(Self::HollowGlobalComponent)
            }
            "HOLLOW_COMPONENT_TYPE_AIM_RECT_COMPONENT" => Some(Self::AimRectComponent),
            "HOLLOW_COMPONENT_TYPE_SIGNAL_MGR_COMPONENT" => {
                Some(Self::SignalMgrComponent)
            }
            "HOLLOW_COMPONENT_TYPE_HOLLOW_FLOOR_MGR_COMPONENT" => {
                Some(Self::HollowFloorMgrComponent)
            }
            "HOLLOW_COMPONENT_TYPE_AREA_CAMERA_COMPONENT" => {
                Some(Self::AreaCameraComponent)
            }
            "HOLLOW_COMPONENT_TYPE_GRID_INNER_WORLD_COMPONENT" => {
                Some(Self::GridInnerWorldComponent)
            }
            "HOLLOW_COMPONENT_TYPE_HOLLOW_SECTION_COMPONENT" => {
                Some(Self::HollowSectionComponent)
            }
            "HOLLOW_COMPONENT_TYPE_BIG_WORLD_COMPONENT" => Some(Self::BigWorldComponent),
            "HOLLOW_COMPONENT_TYPE_ELEVATOR_GRID_COMPONENT" => {
                Some(Self::ElevatorGridComponent)
            }
            "HOLLOW_COMPONENT_TYPE_HIDE_COMPONENT" => Some(Self::HideComponent),
            "HOLLOW_COMPONENT_TYPE_HOLLOW_SPAWNER_COMPONENT" => {
                Some(Self::HollowSpawnerComponent)
            }
            "HOLLOW_COMPONENT_TYPE_HOLLOW_HACKER_GAME_COMPONENT" => {
                Some(Self::HollowHackerGameComponent)
            }
            "HOLLOW_COMPONENT_TYPE_POP_INTERACT_COMPONENT" => {
                Some(Self::PopInteractComponent)
            }
            "HOLLOW_COMPONENT_TYPE_ABILITY_MARK_COMPONENT" => {
                Some(Self::AbilityMarkComponent)
            }
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GridType {
    None = 0,
    CommonGrid = 1,
    MiniGame = 2,
    TvExit = 3,
    Unk4 = 4,
}
impl GridType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "GRID_TYPE_NONE",
            Self::CommonGrid => "GRID_TYPE_COMMON_GRID",
            Self::MiniGame => "GRID_TYPE_MINI_GAME",
            Self::TvExit => "GRID_TYPE_TV_EXIT",
            Self::Unk4 => "GRID_TYPE_UNK_4",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GRID_TYPE_NONE" => Some(Self::None),
            "GRID_TYPE_COMMON_GRID" => Some(Self::CommonGrid),
            "GRID_TYPE_MINI_GAME" => Some(Self::MiniGame),
            "GRID_TYPE_TV_EXIT" => Some(Self::TvExit),
            "GRID_TYPE_UNK_4" => Some(Self::Unk4),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GridLink {
    None = 0,
    Up = 1,
    Down = 2,
    Right = 4,
    Left = 8,
}
impl GridLink {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "GRID_LINK_NONE",
            Self::Up => "GRID_LINK_UP",
            Self::Down => "GRID_LINK_DOWN",
            Self::Right => "GRID_LINK_RIGHT",
            Self::Left => "GRID_LINK_LEFT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GRID_LINK_NONE" => Some(Self::None),
            "GRID_LINK_UP" => Some(Self::Up),
            "GRID_LINK_DOWN" => Some(Self::Down),
            "GRID_LINK_RIGHT" => Some(Self::Right),
            "GRID_LINK_LEFT" => Some(Self::Left),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GridUnkEnum {
    None = 0,
    Unk1 = 1,
    Unk2 = 2,
    Unk3 = 3,
}
impl GridUnkEnum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "GRID_UNK_ENUM_NONE",
            Self::Unk1 => "UNK_1",
            Self::Unk2 => "UNK_2",
            Self::Unk3 => "UNK_3",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GRID_UNK_ENUM_NONE" => Some(Self::None),
            "UNK_1" => Some(Self::Unk1),
            "UNK_2" => Some(Self::Unk2),
            "UNK_3" => Some(Self::Unk3),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventGraphOwnerType {
    None = 0,
    Scene = 1,
    Section = 2,
    SceneUnit = 3,
    Hollow = 4,
    Unknown5 = 5,
}
impl EventGraphOwnerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "EVENT_GRAPH_OWNER_TYPE_NONE",
            Self::Scene => "EVENT_GRAPH_OWNER_TYPE_SCENE",
            Self::Section => "EVENT_GRAPH_OWNER_TYPE_SECTION",
            Self::SceneUnit => "EVENT_GRAPH_OWNER_TYPE_SCENE_UNIT",
            Self::Hollow => "EVENT_GRAPH_OWNER_TYPE_HOLLOW",
            Self::Unknown5 => "EVENT_GRAPH_OWNER_TYPE_UNKNOWN_5",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EVENT_GRAPH_OWNER_TYPE_NONE" => Some(Self::None),
            "EVENT_GRAPH_OWNER_TYPE_SCENE" => Some(Self::Scene),
            "EVENT_GRAPH_OWNER_TYPE_SECTION" => Some(Self::Section),
            "EVENT_GRAPH_OWNER_TYPE_SCENE_UNIT" => Some(Self::SceneUnit),
            "EVENT_GRAPH_OWNER_TYPE_HOLLOW" => Some(Self::Hollow),
            "EVENT_GRAPH_OWNER_TYPE_UNKNOWN_5" => Some(Self::Unknown5),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HollowPushReason {
    None = 0,
    ConveyorBelt = 3,
}
impl HollowPushReason {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "HOLLOW_PUSH_REASON_NONE",
            Self::ConveyorBelt => "HOLLOW_PUSH_REASON_CONVEYOR_BELT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HOLLOW_PUSH_REASON_NONE" => Some(Self::None),
            "HOLLOW_PUSH_REASON_CONVEYOR_BELT" => Some(Self::ConveyorBelt),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum QuickAccessType {
    None = 0,
    Direct = 1,
    QuickMenu = 2,
}
impl QuickAccessType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "QUICK_ACCESS_TYPE_NONE",
            Self::Direct => "QUICK_ACCESS_TYPE_DIRECT",
            Self::QuickMenu => "QUICK_ACCESS_TYPE_QUICK_MENU",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "QUICK_ACCESS_TYPE_NONE" => Some(Self::None),
            "QUICK_ACCESS_TYPE_DIRECT" => Some(Self::Direct),
            "QUICK_ACCESS_TYPE_QUICK_MENU" => Some(Self::QuickMenu),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ActionType {
    None = 0,
    WaitSeconds = 1,
    LogText = 2,
    Wait = 3,
    Log = 4,
    OpenUi = 5,
    SwitchSection = 6,
    AnimCtrlerParam = 7,
    ShowTip = 8,
    ShowPopWindow = 9,
    WalkFarAway = 10,
    OpenDialogHollow = 12,
    CloseDialogHollow = 13,
    PlayAnimHollow = 14,
    CameraStretch = 15,
    CameraMove = 16,
    CameraMoveV2 = 17,
    ShowTipHollow = 18,
    ShowPopWindowHollow = 19,
    PlayPostEffect = 20,
    EnterHollowQuest = 21,
    EnterArchiveFileQuest = 22,
    Preset = 23,
    BlackMask = 24,
    PlaySound = 25,
    CloseTip = 26,
    ReconectChessboard = 27,
    HollowSetSwitchEffect = 28,
    OutDoor = 29,
    FreezeChessboardCamera = 30,
    ShowVhsStoreLevelTips = 31,
    InteractNpcWithAnim = 32,
    ChangeHollowBg = 33,
    TrashGachaGetData = 34,
    TrashGacha = 35,
    ShowQuestTip = 36,
    TeleportUi = 37,
    CameraActive = 38,
    CameraReset = 39,
    CreateFc = 40,
    BehaviourFc = 41,
    SendEventFc = 42,
    ReadFcBlackBoardData = 43,
    WriteFcBlackBoardData = 44,
    ChangeSoundState = 45,
    AfkHollow = 46,
    SwitchBigTv = 49,
    TriggerInteract = 52,
    StopAnim = 53,
    GetTrust = 54,
    PlayDialogAnim = 56,
    UnfreezeChessboardCamera = 57,
    WaitTipsEnd = 58,
    BeginTutorialGuide = 59,
    FocusCamera = 60,
    UnlockClue = 61,
    AvatarTipsSwitch = 62,
    FinishRescue = 63,
    PlayTvEffect = 64,
    SetInteractPoint = 65,
    HideMainCityUi = 66,
    ChatCamera = 67,
    CreateClientEntity = 68,
    SetNpcVisibleClient = 69,
    GachaItemPerform = 70,
    SetMessageClient = 71,
    ModMainCityTimeClient = 72,
    ModifyLightLevelPerform = 73,
    SetPosition = 74,
    SetChessboardPerformMode = 75,
    Transition = 76,
    WaitUntilUiClose = 77,
    WaitTransitionEnd = 78,
    CloseUi = 79,
    QuitPhoto = 80,
    ShowTeleportUi = 81,
    ModifyCameraTargetSection = 82,
    CameraBackToPlayer = 83,
    ResetSceneObj = 84,
    ManualAccelerate = 85,
    BreakNavigate = 86,
    ShowExitButtonNew = 88,
    ShowBottomTipHollow = 89,
    ShowChapterTip = 90,
    EnterDungeonQuest = 91,
    DownloadFullResource = 92,
    AreaTips = 93,
    ClientPerform = 94,
    ShowItem = 95,
    SwitchOva = 96,
    SetLiftStatus = 97,
    AreaCameraModify = 98,
    TriggerPerformBehavior = 99,
    SwitchAtmosphere = 100,
    ModifyLightDiffusionPoints = 101,
    ModCatName = 102,
    OpenUiGame = 103,
    OpenDialogHollowV2 = 104,
    PlayDialogAnimV2 = 105,
    CloseDialogHollowV2 = 106,
    BreakDialogAnimV2 = 107,
    WaitAnimEnd = 108,
    PlayAnimSequence = 109,
    EndOverlordfeastGame = 110,
    PlayAimSequence = 111,
    ClientSwitchDelay = 112,
    BeginPhoto = 113,
    ChessboardGameHenshin = 114,
    SwitchGuiseAvatar = 115,
    If = 1001,
    StartLoop = 1002,
    EndLoop = 1003,
    CallFunction = 1004,
    Return = 1005,
    ResetEvent = 1006,
    AddItem = 1007,
    SetVariable = 1008,
    SetConditionProgress = 1009,
    RandomVariableValue = 1010,
    ListSpecialOpt = 1011,
    FinishQuest = 1012,
    RandomWithWeight = 1013,
    Perform = 1014,
    Reward = 1015,
    SetList = 1016,
    GetList = 1017,
    StartAction = 1018,
    SetString = 1019,
    SendCustomEventTracking = 1020,
    EmptyAction = 1021,
    SetVector2 = 1022,
    Switch = 1023,
    SwitchCompareInt = 1024,
    Draw = 1025,
    SetVec2List = 1026,
    GetVec2List = 1027,
    CallFunctionV2 = 1028,
    EnterHollowShop = 2001,
    MakeChoice = 2002,
    ModifySceneProperty = 2003,
    FinishEvent = 2004,
    TriggerBattle = 2005,
    AverageAvatarHp = 2006,
    RemoveCard = 2007,
    DropPool = 2009,
    Transfer = 2011,
    FinishHollow = 2012,
    RandomItemCard = 2014,
    EventModification = 2015,
    ChangeAvatarState = 2016,
    DropPack = 2017,
    SetMapState = 2018,
    DropCurse = 2019,
    LogHollow = 2020,
    DropCard = 2021,
    ChangeHollowEventWeight = 2022,
    RemoveCurse = 2023,
    HideNode = 2024,
    SetChallenge = 2025,
    DropChallengeId = 2026,
    GetAvatarInfo = 2027,
    SetHollowItem = 2028,
    ChangeCharacter = 2029,
    NewHollow = 2030,
    SlotMachine = 2033,
    SetHollowBlackOut = 2034,
    FinishBlackOut = 2035,
    SetHollowSystemState = 2036,
    AddCharacter = 2037,
    LockCurse = 2038,
    HollowDistance = 2039,
    PushBack = 2040,
    ApplyAbility = 2041,
    RemoveAbility = 2042,
    RandomBattleId = 2044,
    GetIndexByFilter = 2046,
    SetBattleType = 2048,
    GetPosition = 2049,
    StartMiniGame = 2050,
    SetHollowItemSlot = 2051,
    GetHollowItem = 2052,
    SearchGrid = 2053,
    SetNpcState = 2054,
    GetNpcInstanceId = 2055,
    DestoryNpc = 2056,
    AddCharacterAbyss = 2057,
    ChangeCharacterAbyss = 2058,
    GetCharacterPoolAbyss = 2059,
    AbyssDropCharacterPool = 2060,
    GetLeaderOfHollowNpc = 2061,
    SetLeaderOfHollowNpc = 2062,
    UpdateSaveNpcNum = 2063,
    PushWithDirection = 2064,
    HollowNpcFindPath = 2065,
    HollowNpcMove = 2066,
    NewChessboard = 2067,
    GoToNextLayer = 2068,
    GoToChessboard = 2069,
    GetPreChessboard = 2070,
    TriggerHollowNpcBehavior = 2071,
    ShowLayerResult = 2072,
    Henshin = 2073,
    CreateHollowNpc = 2074,
    DropChessboardId = 2075,
    MakeDialogChoice = 2076,
    GetEventId = 2077,
    CountDropPool = 2078,
    MakeItemChoice = 2079,
    HpActHollow = 2080,
    BanHollowEvent = 2081,
    CoordinateTransform = 2082,
    RegisterVariableCondition = 2083,
    OnOffCategory = 2084,
    ResetBigTvSnapshot = 2087,
    BigTvSupportSnapshot = 2088,
    SetEventIcon = 2089,
    GetAnimSheetId = 2090,
    HollowNpcHenshin = 2091,
    HollowNpcTransfer = 2092,
    BindBigTv = 2093,
    MoveNpcToSection = 2098,
    GetNpcId = 2099,
    SearchHollowNpc = 2100,
    Boom = 2101,
    TriggerHollowEvent = 2102,
    BreakDialogAnim = 2103,
    MoveBigTv = 2104,
    SetNextLayerChessboardId = 2105,
    GetBossBattleEvent = 2106,
    CreateHollowSnake = 2107,
    SetGridStaminaState = 2108,
    DisplayBigTvChessboard = 2109,
    SplitHollowSnake = 2110,
    GetHollowSnakeInfo = 2111,
    ModifyHollowSnake = 2112,
    ChangeHollowNpcApperance = 2113,
    OpenBigTvSokobanGame = 2114,
    SetInterconnectedStoryEvent = 2115,
    HollowNpcImitate = 2116,
    TriggerHollowNpcEarlyAct = 2117,
    GetAvatarByTag = 2118,
    SetBattleTypeAbyss = 2119,
    RemoveEventIdFromRandomPool = 2120,
    RecycleHollowItem = 2121,
    CopyEvent = 2122,
    BanCharacter = 2123,
    RemoveCharacter = 2124,
    SetNpcAttr = 2125,
    GetNpcAttr = 2126,
    HitNpc = 2127,
    GetPlayerHollowMovePath = 2128,
    GetBigTvIndex = 2129,
    ClearNpc = 2130,
    SaveMiniSnapshot = 2131,
    GetPossessHollowItem = 2132,
    ResetHollowLineQuest = 2133,
    ModifyLightLevel = 2134,
    GetLightLevel = 2135,
    AddHollowLight = 2136,
    RemoveHollowLight = 2137,
    ModifyChessboardPlugin = 2138,
    GetLightUid = 2139,
    NewBoom = 2140,
    SetEntityParam = 2141,
    GetEntityParam = 2142,
    RepairZone = 2143,
    PushRepairZone = 2144,
    SetInnerWorldMapState = 2145,
    ListConvert = 2146,
    AbyssGetBattleEvent = 2147,
    TriggerEntityBasicBehavior = 2148,
    TriggerEntityMove = 2149,
    TriggerEntityTransfer = 2150,
    TriggerEntityInteract = 2151,
    UpgradeCard = 2152,
    NewTimeRewind = 2153,
    EnterTimeRewind = 2154,
    InitTimeSegment = 2155,
    ModifyTimeSegment = 2156,
    ModifyTimeRewind = 2157,
    GetTimeRewindInfo = 2158,
    FinishKeySegment = 2159,
    ActivateGridInSegment = 2160,
    CountCardPool = 2161,
    MakeBangbooChoice = 2162,
    ChangeBangbooChoice = 2163,
    TriggerEntityScript = 2164,
    AddStressPunishCurse = 2165,
    PopupTip = 2166,
    HideHollowEntity = 2167,
    GetEntityPriorityList = 2168,
    ChessUiController = 2169,
    GetEntityPriority = 2170,
    CreateEntity = 2171,
    DestroyEntityByUid = 2172,
    InteractWithEntity = 2173,
    SearchPosition = 2174,
    FilterHollowEntity = 2175,
    ModifyStackingOrder = 2176,
    InitConwayLifeGame = 2177,
    IterateConwayLifeGame = 2178,
    ChangeConwayLifeGameGridState = 2179,
    BigTvChessUiController = 2180,
    SetEntityState = 2181,
    RemoveEntityState = 2182,
    GetEventTexture = 2183,
    ModifyComponent = 2184,
    ChangeHollowSoundState = 2185,
    SetEntityScriptVariable = 2186,
    CreateSignal = 2187,
    SubscribeSignal = 2188,
    UnsubscribeSignal = 2189,
    SendSignal = 2190,
    DestroySignal = 2191,
    SetMultiHollowOutSection = 2192,
    GetEntityScriptVariable = 2193,
    RemoveChessboard = 2194,
    BeginTutorialGuideInteract = 2195,
    TimeRewindInteract = 2196,
    LimboAvatarCard = 2197,
    LimboCampEvent = 2198,
    ModifyAimRectComponent = 2199,
    RemoveFromPool = 2200,
    ActivateSegmentInteract = 2201,
    RecordUseInitiativeItem = 2202,
    ModifyMultiHollowOutFloor = 2203,
    SetMultiHollowOutView = 2204,
    MarkGridAsElevator = 2205,
    MoveElevatorToSection = 2206,
    NextDropClueEvent = 2207,
    MoveHollowEvent = 2208,
    GetFocusCameraParam = 2209,
    ReplaceCard = 2210,
    LoadEventParam = 2211,
    SetLayerType = 2212,
    CreateHollowSpawner = 2213,
    SetHollowSpawner = 2214,
    GetHollowSpawner = 2215,
    RunHollowSpawner = 2216,
    PlayHollowQteGame = 2217,
    SetHollowPlayUi = 2218,
    SetHollowActivityParam = 2219,
    GetHollowActivityParam = 2220,
    RewardWithPerform = 2221,
    InitHackerGame = 2222,
    ModifyHackerGameParam = 2223,
    ModifyPopInteractComponent = 2224,
    SetLevelGlobalVariable = 2225,
    EventModificationByFalling = 2226,
    TryMoveElevator = 2227,
    GetEventPoolEvent = 2228,
    ChessUi3dController = 2229,
    HollowGameFinishToLevel = 2230,
    ChessboardSokobanUiInfo = 2231,
    CreateNpc = 3001,
    SetQuestPhase = 3002,
    ChangeInteract = 3003,
    InteractFinish = 3004,
    RemoveMainCityQuestNpc = 3005,
    RemoveMainCityQuestInteract = 3006,
    ChangeBackSceneInfo = 3007,
    ResetMainCityQuestGroup = 3008,
    UnlockHollowQuest = 3010,
    SetNpcVisible = 3011,
    RemoveInteract = 3012,
    RemoveNpc = 3013,
    SetVhsStoreLevel = 3014,
    SetVhsStoreTrendState = 3015,
    SwitchMainCityTime = 3016,
    TheWorld = 3017,
    ForceRefresh = 3018,
    ForbidAfk = 3019,
    SwitchMainCharacter = 3020,
    SetLandEventFinish = 3021,
    SetBgm = 3022,
    SetMainCityObjectState = 3023,
    EventChoice = 3024,
    CreateMoveNpc = 3025,
    ChangeGuidePoint = 3026,
    AddDailyQuest = 3027,
    AddMicroTask = 3028,
    SetFirstMeet = 3029,
    CreateCameraZone = 3030,
    SetMainCityTime = 3031,
    NextMainCityTimePeriod = 3032,
    PlayerSwitchMainCharacter = 3033,
    EndTransition = 3034,
    AddVhsFlowBuff = 3035,
    ActivatePhotoId = 3036,
    AccelerateMainCityTime = 3037,
    SetTrashNewFlag = 3038,
    UseLastTime = 3039,
    OccupyOvernight = 3040,
    ShowPhotoQuestFinishTip = 3041,
    AddSoundAmb = 3042,
    SubmitItem = 3043,
    ModTrust = 3044,
    SetPartnerEventState = 3045,
    SendMessage = 3046,
    SwitchTrackQuest = 3047,
    ModNpc = 3048,
    AcceptOvernight = 3049,
    ActiveTrigger = 3050,
    ModObjState = 3051,
    ModSceneObj = 3052,
    FansSettle = 3053,
    OpenHallGame = 3054,
    AddPartnerEvent = 3055,
    ExecOvernightEvent = 3056,
    SofaRestNextTimePeriod = 3057,
    BeginUiGame = 3058,
    PrepareData = 3059,
    ClearRpRecommendResult = 3060,
    DoMainCityGame = 3061,
    ShowInTodo = 3062,
    ChangeNpcName = 3063,
    CreateOva = 3064,
    SetOvaState = 3065,
    SwitchMainCharacterGuise = 3066,
    CompleteHallGame = 3068,
    HideMainControlAvatar = 3069,
    EatRamen = 3070,
    OngoingTips = 3071,
    SetSound = 3072,
    GenCampIdleDynamicTextItem = 3073,
    MapChooseByEvent = 4001,
    MapChooseByLayer = 4002,
    MapChooseByNum = 4003,
    MapChooseByRange = 4004,
    MapClearPool = 4005,
    MapSetEvent = 4007,
    MapSetLayer = 4008,
    MapSetTag = 4009,
}
impl ActionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "ACTION_TYPE_NONE",
            Self::WaitSeconds => "ACTION_TYPE_WAIT_SECONDS",
            Self::LogText => "ACTION_TYPE_LOG_TEXT",
            Self::Wait => "ACTION_TYPE_WAIT",
            Self::Log => "ACTION_TYPE_LOG",
            Self::OpenUi => "ACTION_TYPE_OPEN_UI",
            Self::SwitchSection => "ACTION_TYPE_SWITCH_SECTION",
            Self::AnimCtrlerParam => "ACTION_TYPE_ANIM_CTRLER_PARAM",
            Self::ShowTip => "ACTION_TYPE_SHOW_TIP",
            Self::ShowPopWindow => "ACTION_TYPE_SHOW_POP_WINDOW",
            Self::WalkFarAway => "ACTION_TYPE_WALK_FAR_AWAY",
            Self::OpenDialogHollow => "ACTION_TYPE_OPEN_DIALOG_HOLLOW",
            Self::CloseDialogHollow => "ACTION_TYPE_CLOSE_DIALOG_HOLLOW",
            Self::PlayAnimHollow => "ACTION_TYPE_PLAY_ANIM_HOLLOW",
            Self::CameraStretch => "ACTION_TYPE_CAMERA_STRETCH",
            Self::CameraMove => "ACTION_TYPE_CAMERA_MOVE",
            Self::CameraMoveV2 => "ACTION_TYPE_CAMERA_MOVE_V2",
            Self::ShowTipHollow => "ACTION_TYPE_SHOW_TIP_HOLLOW",
            Self::ShowPopWindowHollow => "ACTION_TYPE_SHOW_POP_WINDOW_HOLLOW",
            Self::PlayPostEffect => "ACTION_TYPE_PLAY_POST_EFFECT",
            Self::EnterHollowQuest => "ACTION_TYPE_ENTER_HOLLOW_QUEST",
            Self::EnterArchiveFileQuest => "ACTION_TYPE_ENTER_ARCHIVE_FILE_QUEST",
            Self::Preset => "ACTION_TYPE_PRESET",
            Self::BlackMask => "ACTION_TYPE_BLACK_MASK",
            Self::PlaySound => "ACTION_TYPE_PLAY_SOUND",
            Self::CloseTip => "ACTION_TYPE_CLOSE_TIP",
            Self::ReconectChessboard => "ACTION_TYPE_RECONECT_CHESSBOARD",
            Self::HollowSetSwitchEffect => "ACTION_TYPE_HOLLOW_SET_SWITCH_EFFECT",
            Self::OutDoor => "ACTION_TYPE_OUT_DOOR",
            Self::FreezeChessboardCamera => "ACTION_TYPE_FREEZE_CHESSBOARD_CAMERA",
            Self::ShowVhsStoreLevelTips => "ACTION_TYPE_SHOW_VHS_STORE_LEVEL_TIPS",
            Self::InteractNpcWithAnim => "ACTION_TYPE_INTERACT_NPC_WITH_ANIM",
            Self::ChangeHollowBg => "ACTION_TYPE_CHANGE_HOLLOW_BG",
            Self::TrashGachaGetData => "ACTION_TYPE_TRASH_GACHA_GET_DATA",
            Self::TrashGacha => "ACTION_TYPE_TRASH_GACHA",
            Self::ShowQuestTip => "ACTION_TYPE_SHOW_QUEST_TIP",
            Self::TeleportUi => "ACTION_TYPE_TELEPORT_UI",
            Self::CameraActive => "ACTION_TYPE_CAMERA_ACTIVE",
            Self::CameraReset => "ACTION_TYPE_CAMERA_RESET",
            Self::CreateFc => "ACTION_TYPE_CREATE_FC",
            Self::BehaviourFc => "ACTION_TYPE_BEHAVIOUR_FC",
            Self::SendEventFc => "ACTION_TYPE_SEND_EVENT_FC",
            Self::ReadFcBlackBoardData => "ACTION_TYPE_READ_FC_BLACK_BOARD_DATA",
            Self::WriteFcBlackBoardData => "ACTION_TYPE_WRITE_FC_BLACK_BOARD_DATA",
            Self::ChangeSoundState => "ACTION_TYPE_CHANGE_SOUND_STATE",
            Self::AfkHollow => "ACTION_TYPE_AFK_HOLLOW",
            Self::SwitchBigTv => "ACTION_TYPE_SWITCH_BIG_TV",
            Self::TriggerInteract => "ACTION_TYPE_TRIGGER_INTERACT",
            Self::StopAnim => "ACTION_TYPE_STOP_ANIM",
            Self::GetTrust => "ACTION_TYPE_GET_TRUST",
            Self::PlayDialogAnim => "ACTION_TYPE_PLAY_DIALOG_ANIM",
            Self::UnfreezeChessboardCamera => "ACTION_TYPE_UNFREEZE_CHESSBOARD_CAMERA",
            Self::WaitTipsEnd => "ACTION_TYPE_WAIT_TIPS_END",
            Self::BeginTutorialGuide => "ACTION_TYPE_BEGIN_TUTORIAL_GUIDE",
            Self::FocusCamera => "ACTION_TYPE_FOCUS_CAMERA",
            Self::UnlockClue => "ACTION_TYPE_UNLOCK_CLUE",
            Self::AvatarTipsSwitch => "ACTION_TYPE_AVATAR_TIPS_SWITCH",
            Self::FinishRescue => "ACTION_TYPE_FINISH_RESCUE",
            Self::PlayTvEffect => "ACTION_TYPE_PLAY_TV_EFFECT",
            Self::SetInteractPoint => "ACTION_TYPE_SET_INTERACT_POINT",
            Self::HideMainCityUi => "ACTION_TYPE_HIDE_MAIN_CITY_UI",
            Self::ChatCamera => "ACTION_TYPE_CHAT_CAMERA",
            Self::CreateClientEntity => "ACTION_TYPE_CREATE_CLIENT_ENTITY",
            Self::SetNpcVisibleClient => "ACTION_TYPE_SET_NPC_VISIBLE_CLIENT",
            Self::GachaItemPerform => "ACTION_TYPE_GACHA_ITEM_PERFORM",
            Self::SetMessageClient => "ACTION_TYPE_SET_MESSAGE_CLIENT",
            Self::ModMainCityTimeClient => "ACTION_TYPE_MOD_MAIN_CITY_TIME_CLIENT",
            Self::ModifyLightLevelPerform => "ACTION_TYPE_MODIFY_LIGHT_LEVEL_PERFORM",
            Self::SetPosition => "ACTION_TYPE_SET_POSITION",
            Self::SetChessboardPerformMode => "ACTION_TYPE_SET_CHESSBOARD_PERFORM_MODE",
            Self::Transition => "ACTION_TYPE_TRANSITION",
            Self::WaitUntilUiClose => "ACTION_TYPE_WAIT_UNTIL_UI_CLOSE",
            Self::WaitTransitionEnd => "ACTION_TYPE_WAIT_TRANSITION_END",
            Self::CloseUi => "ACTION_TYPE_CLOSE_UI",
            Self::QuitPhoto => "ACTION_TYPE_QUIT_PHOTO",
            Self::ShowTeleportUi => "ACTION_TYPE_SHOW_TELEPORT_UI",
            Self::ModifyCameraTargetSection => "ACTION_TYPE_MODIFY_CAMERA_TARGET_SECTION",
            Self::CameraBackToPlayer => "ACTION_TYPE_CAMERA_BACK_TO_PLAYER",
            Self::ResetSceneObj => "ACTION_TYPE_RESET_SCENE_OBJ",
            Self::ManualAccelerate => "ACTION_TYPE_MANUAL_ACCELERATE",
            Self::BreakNavigate => "ACTION_TYPE_BREAK_NAVIGATE",
            Self::ShowExitButtonNew => "ACTION_TYPE_SHOW_EXIT_BUTTON_NEW",
            Self::ShowBottomTipHollow => "ACTION_TYPE_SHOW_BOTTOM_TIP_HOLLOW",
            Self::ShowChapterTip => "ACTION_TYPE_SHOW_CHAPTER_TIP",
            Self::EnterDungeonQuest => "ACTION_TYPE_ENTER_DUNGEON_QUEST",
            Self::DownloadFullResource => "ACTION_TYPE_DOWNLOAD_FULL_RESOURCE",
            Self::AreaTips => "ACTION_TYPE_AREA_TIPS",
            Self::ClientPerform => "ACTION_TYPE_CLIENT_PERFORM",
            Self::ShowItem => "ACTION_TYPE_SHOW_ITEM",
            Self::SwitchOva => "ACTION_TYPE_SWITCH_OVA",
            Self::SetLiftStatus => "ACTION_TYPE_SET_LIFT_STATUS",
            Self::AreaCameraModify => "ACTION_TYPE_AREA_CAMERA_MODIFY",
            Self::TriggerPerformBehavior => "ACTION_TYPE_TRIGGER_PERFORM_BEHAVIOR",
            Self::SwitchAtmosphere => "ACTION_TYPE_SWITCH_ATMOSPHERE",
            Self::ModifyLightDiffusionPoints => {
                "ACTION_TYPE_MODIFY_LIGHT_DIFFUSION_POINTS"
            }
            Self::ModCatName => "ACTION_TYPE_MOD_CAT_NAME",
            Self::OpenUiGame => "ACTION_TYPE_OPEN_UI_GAME",
            Self::OpenDialogHollowV2 => "ACTION_TYPE_OPEN_DIALOG_HOLLOW_V2",
            Self::PlayDialogAnimV2 => "ACTION_TYPE_PLAY_DIALOG_ANIM_V2",
            Self::CloseDialogHollowV2 => "ACTION_TYPE_CLOSE_DIALOG_HOLLOW_V2",
            Self::BreakDialogAnimV2 => "ACTION_TYPE_BREAK_DIALOG_ANIM_V2",
            Self::WaitAnimEnd => "ACTION_TYPE_WAIT_ANIM_END",
            Self::PlayAnimSequence => "ACTION_TYPE_PLAY_ANIM_SEQUENCE",
            Self::EndOverlordfeastGame => "ACTION_TYPE_END_OVERLORDFEAST_GAME",
            Self::PlayAimSequence => "ACTION_TYPE_PLAY_AIM_SEQUENCE",
            Self::ClientSwitchDelay => "ACTION_TYPE_CLIENT_SWITCH_DELAY",
            Self::BeginPhoto => "ACTION_TYPE_BEGIN_PHOTO",
            Self::ChessboardGameHenshin => "ACTION_TYPE_CHESSBOARD_GAME_HENSHIN",
            Self::SwitchGuiseAvatar => "ACTION_TYPE_SWITCH_GUISE_AVATAR",
            Self::If => "ACTION_TYPE_IF",
            Self::StartLoop => "ACTION_TYPE_START_LOOP",
            Self::EndLoop => "ACTION_TYPE_END_LOOP",
            Self::CallFunction => "ACTION_TYPE_CALL_FUNCTION",
            Self::Return => "ACTION_TYPE_RETURN",
            Self::ResetEvent => "ACTION_TYPE_RESET_EVENT",
            Self::AddItem => "ACTION_TYPE_ADD_ITEM",
            Self::SetVariable => "ACTION_TYPE_SET_VARIABLE",
            Self::SetConditionProgress => "ACTION_TYPE_SET_CONDITION_PROGRESS",
            Self::RandomVariableValue => "ACTION_TYPE_RANDOM_VARIABLE_VALUE",
            Self::ListSpecialOpt => "ACTION_TYPE_LIST_SPECIAL_OPT",
            Self::FinishQuest => "ACTION_TYPE_FINISH_QUEST",
            Self::RandomWithWeight => "ACTION_TYPE_RANDOM_WITH_WEIGHT",
            Self::Perform => "ACTION_TYPE_PERFORM",
            Self::Reward => "ACTION_TYPE_REWARD",
            Self::SetList => "ACTION_TYPE_SET_LIST",
            Self::GetList => "ACTION_TYPE_GET_LIST",
            Self::StartAction => "ACTION_TYPE_START_ACTION",
            Self::SetString => "ACTION_TYPE_SET_STRING",
            Self::SendCustomEventTracking => "ACTION_TYPE_SEND_CUSTOM_EVENT_TRACKING",
            Self::EmptyAction => "ACTION_TYPE_EMPTY_ACTION",
            Self::SetVector2 => "ACTION_TYPE_SET_VECTOR2",
            Self::Switch => "ACTION_TYPE_SWITCH",
            Self::SwitchCompareInt => "ACTION_TYPE_SWITCH_COMPARE_INT",
            Self::Draw => "ACTION_TYPE_DRAW",
            Self::SetVec2List => "ACTION_TYPE_SET_VEC2_LIST",
            Self::GetVec2List => "ACTION_TYPE_GET_VEC2_LIST",
            Self::CallFunctionV2 => "ACTION_TYPE_CALL_FUNCTION_V2",
            Self::EnterHollowShop => "ACTION_TYPE_ENTER_HOLLOW_SHOP",
            Self::MakeChoice => "ACTION_TYPE_MAKE_CHOICE",
            Self::ModifySceneProperty => "ACTION_TYPE_MODIFY_SCENE_PROPERTY",
            Self::FinishEvent => "ACTION_TYPE_FINISH_EVENT",
            Self::TriggerBattle => "ACTION_TYPE_TRIGGER_BATTLE",
            Self::AverageAvatarHp => "ACTION_TYPE_AVERAGE_AVATAR_HP",
            Self::RemoveCard => "ACTION_TYPE_REMOVE_CARD",
            Self::DropPool => "ACTION_TYPE_DROP_POOL",
            Self::Transfer => "ACTION_TYPE_TRANSFER",
            Self::FinishHollow => "ACTION_TYPE_FINISH_HOLLOW",
            Self::RandomItemCard => "ACTION_TYPE_RANDOM_ITEM_CARD",
            Self::EventModification => "ACTION_TYPE_EVENT_MODIFICATION",
            Self::ChangeAvatarState => "ACTION_TYPE_CHANGE_AVATAR_STATE",
            Self::DropPack => "ACTION_TYPE_DROP_PACK",
            Self::SetMapState => "ACTION_TYPE_SET_MAP_STATE",
            Self::DropCurse => "ACTION_TYPE_DROP_CURSE",
            Self::LogHollow => "ACTION_TYPE_LOG_HOLLOW",
            Self::DropCard => "ACTION_TYPE_DROP_CARD",
            Self::ChangeHollowEventWeight => "ACTION_TYPE_CHANGE_HOLLOW_EVENT_WEIGHT",
            Self::RemoveCurse => "ACTION_TYPE_REMOVE_CURSE",
            Self::HideNode => "ACTION_TYPE_HIDE_NODE",
            Self::SetChallenge => "ACTION_TYPE_SET_CHALLENGE",
            Self::DropChallengeId => "ACTION_TYPE_DROP_CHALLENGE_ID",
            Self::GetAvatarInfo => "ACTION_TYPE_GET_AVATAR_INFO",
            Self::SetHollowItem => "ACTION_TYPE_SET_HOLLOW_ITEM",
            Self::ChangeCharacter => "ACTION_TYPE_CHANGE_CHARACTER",
            Self::NewHollow => "ACTION_TYPE_NEW_HOLLOW",
            Self::SlotMachine => "ACTION_TYPE_SLOT_MACHINE",
            Self::SetHollowBlackOut => "ACTION_TYPE_SET_HOLLOW_BLACK_OUT",
            Self::FinishBlackOut => "ACTION_TYPE_FINISH_BLACK_OUT",
            Self::SetHollowSystemState => "ACTION_TYPE_SET_HOLLOW_SYSTEM_STATE",
            Self::AddCharacter => "ACTION_TYPE_ADD_CHARACTER",
            Self::LockCurse => "ACTION_TYPE_LOCK_CURSE",
            Self::HollowDistance => "ACTION_TYPE_HOLLOW_DISTANCE",
            Self::PushBack => "ACTION_TYPE_PUSH_BACK",
            Self::ApplyAbility => "ACTION_TYPE_APPLY_ABILITY",
            Self::RemoveAbility => "ACTION_TYPE_REMOVE_ABILITY",
            Self::RandomBattleId => "ACTION_TYPE_RANDOM_BATTLE_ID",
            Self::GetIndexByFilter => "ACTION_TYPE_GET_INDEX_BY_FILTER",
            Self::SetBattleType => "ACTION_TYPE_SET_BATTLE_TYPE",
            Self::GetPosition => "ACTION_TYPE_GET_POSITION",
            Self::StartMiniGame => "ACTION_TYPE_START_MINI_GAME",
            Self::SetHollowItemSlot => "ACTION_TYPE_SET_HOLLOW_ITEM_SLOT",
            Self::GetHollowItem => "ACTION_TYPE_GET_HOLLOW_ITEM",
            Self::SearchGrid => "ACTION_TYPE_SEARCH_GRID",
            Self::SetNpcState => "ACTION_TYPE_SET_NPC_STATE",
            Self::GetNpcInstanceId => "ACTION_TYPE_GET_NPC_INSTANCE_ID",
            Self::DestoryNpc => "ACTION_TYPE_DESTORY_NPC",
            Self::AddCharacterAbyss => "ACTION_TYPE_ADD_CHARACTER_ABYSS",
            Self::ChangeCharacterAbyss => "ACTION_TYPE_CHANGE_CHARACTER_ABYSS",
            Self::GetCharacterPoolAbyss => "ACTION_TYPE_GET_CHARACTER_POOL_ABYSS",
            Self::AbyssDropCharacterPool => "ACTION_TYPE_ABYSS_DROP_CHARACTER_POOL",
            Self::GetLeaderOfHollowNpc => "ACTION_TYPE_GET_LEADER_OF_HOLLOW_NPC",
            Self::SetLeaderOfHollowNpc => "ACTION_TYPE_SET_LEADER_OF_HOLLOW_NPC",
            Self::UpdateSaveNpcNum => "ACTION_TYPE_UPDATE_SAVE_NPC_NUM",
            Self::PushWithDirection => "ACTION_TYPE_PUSH_WITH_DIRECTION",
            Self::HollowNpcFindPath => "ACTION_TYPE_HOLLOW_NPC_FIND_PATH",
            Self::HollowNpcMove => "ACTION_TYPE_HOLLOW_NPC_MOVE",
            Self::NewChessboard => "ACTION_TYPE_NEW_CHESSBOARD",
            Self::GoToNextLayer => "ACTION_TYPE_GO_TO_NEXT_LAYER",
            Self::GoToChessboard => "ACTION_TYPE_GO_TO_CHESSBOARD",
            Self::GetPreChessboard => "ACTION_TYPE_GET_PRE_CHESSBOARD",
            Self::TriggerHollowNpcBehavior => "ACTION_TYPE_TRIGGER_HOLLOW_NPC_BEHAVIOR",
            Self::ShowLayerResult => "ACTION_TYPE_SHOW_LAYER_RESULT",
            Self::Henshin => "ACTION_TYPE_HENSHIN",
            Self::CreateHollowNpc => "ACTION_TYPE_CREATE_HOLLOW_NPC",
            Self::DropChessboardId => "ACTION_TYPE_DROP_CHESSBOARD_ID",
            Self::MakeDialogChoice => "ACTION_TYPE_MAKE_DIALOG_CHOICE",
            Self::GetEventId => "ACTION_TYPE_GET_EVENT_ID",
            Self::CountDropPool => "ACTION_TYPE_COUNT_DROP_POOL",
            Self::MakeItemChoice => "ACTION_TYPE_MAKE_ITEM_CHOICE",
            Self::HpActHollow => "ACTION_TYPE_HP_ACT_HOLLOW",
            Self::BanHollowEvent => "ACTION_TYPE_BAN_HOLLOW_EVENT",
            Self::CoordinateTransform => "ACTION_TYPE_COORDINATE_TRANSFORM",
            Self::RegisterVariableCondition => "ACTION_TYPE_REGISTER_VARIABLE_CONDITION",
            Self::OnOffCategory => "ACTION_TYPE_ON_OFF_CATEGORY",
            Self::ResetBigTvSnapshot => "ACTION_TYPE_RESET_BIG_TV_SNAPSHOT",
            Self::BigTvSupportSnapshot => "ACTION_TYPE_BIG_TV_SUPPORT_SNAPSHOT",
            Self::SetEventIcon => "ACTION_TYPE_SET_EVENT_ICON",
            Self::GetAnimSheetId => "ACTION_TYPE_GET_ANIM_SHEET_ID",
            Self::HollowNpcHenshin => "ACTION_TYPE_HOLLOW_NPC_HENSHIN",
            Self::HollowNpcTransfer => "ACTION_TYPE_HOLLOW_NPC_TRANSFER",
            Self::BindBigTv => "ACTION_TYPE_BIND_BIG_TV",
            Self::MoveNpcToSection => "ACTION_TYPE_MOVE_NPC_TO_SECTION",
            Self::GetNpcId => "ACTION_TYPE_GET_NPC_ID",
            Self::SearchHollowNpc => "ACTION_TYPE_SEARCH_HOLLOW_NPC",
            Self::Boom => "ACTION_TYPE_BOOM",
            Self::TriggerHollowEvent => "ACTION_TYPE_TRIGGER_HOLLOW_EVENT",
            Self::BreakDialogAnim => "ACTION_TYPE_BREAK_DIALOG_ANIM",
            Self::MoveBigTv => "ACTION_TYPE_MOVE_BIG_TV",
            Self::SetNextLayerChessboardId => "ACTION_TYPE_SET_NEXT_LAYER_CHESSBOARD_ID",
            Self::GetBossBattleEvent => "ACTION_TYPE_GET_BOSS_BATTLE_EVENT",
            Self::CreateHollowSnake => "ACTION_TYPE_CREATE_HOLLOW_SNAKE",
            Self::SetGridStaminaState => "ACTION_TYPE_SET_GRID_STAMINA_STATE",
            Self::DisplayBigTvChessboard => "ACTION_TYPE_DISPLAY_BIG_TV_CHESSBOARD",
            Self::SplitHollowSnake => "ACTION_TYPE_SPLIT_HOLLOW_SNAKE",
            Self::GetHollowSnakeInfo => "ACTION_TYPE_GET_HOLLOW_SNAKE_INFO",
            Self::ModifyHollowSnake => "ACTION_TYPE_MODIFY_HOLLOW_SNAKE",
            Self::ChangeHollowNpcApperance => "ACTION_TYPE_CHANGE_HOLLOW_NPC_APPERANCE",
            Self::OpenBigTvSokobanGame => "ACTION_TYPE_OPEN_BIG_TV_SOKOBAN_GAME",
            Self::SetInterconnectedStoryEvent => {
                "ACTION_TYPE_SET_INTERCONNECTED_STORY_EVENT"
            }
            Self::HollowNpcImitate => "ACTION_TYPE_HOLLOW_NPC_IMITATE",
            Self::TriggerHollowNpcEarlyAct => "ACTION_TYPE_TRIGGER_HOLLOW_NPC_EARLY_ACT",
            Self::GetAvatarByTag => "ACTION_TYPE_GET_AVATAR_BY_TAG",
            Self::SetBattleTypeAbyss => "ACTION_TYPE_SET_BATTLE_TYPE_ABYSS",
            Self::RemoveEventIdFromRandomPool => {
                "ACTION_TYPE_REMOVE_EVENT_ID_FROM_RANDOM_POOL"
            }
            Self::RecycleHollowItem => "ACTION_TYPE_RECYCLE_HOLLOW_ITEM",
            Self::CopyEvent => "ACTION_TYPE_COPY_EVENT",
            Self::BanCharacter => "ACTION_TYPE_BAN_CHARACTER",
            Self::RemoveCharacter => "ACTION_TYPE_REMOVE_CHARACTER",
            Self::SetNpcAttr => "ACTION_TYPE_SET_NPC_ATTR",
            Self::GetNpcAttr => "ACTION_TYPE_GET_NPC_ATTR",
            Self::HitNpc => "ACTION_TYPE_HIT_NPC",
            Self::GetPlayerHollowMovePath => "ACTION_TYPE_GET_PLAYER_HOLLOW_MOVE_PATH",
            Self::GetBigTvIndex => "ACTION_TYPE_GET_BIG_TV_INDEX",
            Self::ClearNpc => "ACTION_TYPE_CLEAR_NPC",
            Self::SaveMiniSnapshot => "ACTION_TYPE_SAVE_MINI_SNAPSHOT",
            Self::GetPossessHollowItem => "ACTION_TYPE_GET_POSSESS_HOLLOW_ITEM",
            Self::ResetHollowLineQuest => "ACTION_TYPE_RESET_HOLLOW_LINE_QUEST",
            Self::ModifyLightLevel => "ACTION_TYPE_MODIFY_LIGHT_LEVEL",
            Self::GetLightLevel => "ACTION_TYPE_GET_LIGHT_LEVEL",
            Self::AddHollowLight => "ACTION_TYPE_ADD_HOLLOW_LIGHT",
            Self::RemoveHollowLight => "ACTION_TYPE_REMOVE_HOLLOW_LIGHT",
            Self::ModifyChessboardPlugin => "ACTION_TYPE_MODIFY_CHESSBOARD_PLUGIN",
            Self::GetLightUid => "ACTION_TYPE_GET_LIGHT_UID",
            Self::NewBoom => "ACTION_TYPE_NEW_BOOM",
            Self::SetEntityParam => "ACTION_TYPE_SET_ENTITY_PARAM",
            Self::GetEntityParam => "ACTION_TYPE_GET_ENTITY_PARAM",
            Self::RepairZone => "ACTION_TYPE_REPAIR_ZONE",
            Self::PushRepairZone => "ACTION_TYPE_PUSH_REPAIR_ZONE",
            Self::SetInnerWorldMapState => "ACTION_TYPE_SET_INNER_WORLD_MAP_STATE",
            Self::ListConvert => "ACTION_TYPE_LIST_CONVERT",
            Self::AbyssGetBattleEvent => "ACTION_TYPE_ABYSS_GET_BATTLE_EVENT",
            Self::TriggerEntityBasicBehavior => {
                "ACTION_TYPE_TRIGGER_ENTITY_BASIC_BEHAVIOR"
            }
            Self::TriggerEntityMove => "ACTION_TYPE_TRIGGER_ENTITY_MOVE",
            Self::TriggerEntityTransfer => "ACTION_TYPE_TRIGGER_ENTITY_TRANSFER",
            Self::TriggerEntityInteract => "ACTION_TYPE_TRIGGER_ENTITY_INTERACT",
            Self::UpgradeCard => "ACTION_TYPE_UPGRADE_CARD",
            Self::NewTimeRewind => "ACTION_TYPE_NEW_TIME_REWIND",
            Self::EnterTimeRewind => "ACTION_TYPE_ENTER_TIME_REWIND",
            Self::InitTimeSegment => "ACTION_TYPE_INIT_TIME_SEGMENT",
            Self::ModifyTimeSegment => "ACTION_TYPE_MODIFY_TIME_SEGMENT",
            Self::ModifyTimeRewind => "ACTION_TYPE_MODIFY_TIME_REWIND",
            Self::GetTimeRewindInfo => "ACTION_TYPE_GET_TIME_REWIND_INFO",
            Self::FinishKeySegment => "ACTION_TYPE_FINISH_KEY_SEGMENT",
            Self::ActivateGridInSegment => "ACTION_TYPE_ACTIVATE_GRID_IN_SEGMENT",
            Self::CountCardPool => "ACTION_TYPE_COUNT_CARD_POOL",
            Self::MakeBangbooChoice => "ACTION_TYPE_MAKE_BANGBOO_CHOICE",
            Self::ChangeBangbooChoice => "ACTION_TYPE_CHANGE_BANGBOO_CHOICE",
            Self::TriggerEntityScript => "ACTION_TYPE_TRIGGER_ENTITY_SCRIPT",
            Self::AddStressPunishCurse => "ACTION_TYPE_ADD_STRESS_PUNISH_CURSE",
            Self::PopupTip => "ACTION_TYPE_POPUP_TIP",
            Self::HideHollowEntity => "ACTION_TYPE_HIDE_HOLLOW_ENTITY",
            Self::GetEntityPriorityList => "ACTION_TYPE_GET_ENTITY_PRIORITY_LIST",
            Self::ChessUiController => "ACTION_TYPE_CHESS_UI_CONTROLLER",
            Self::GetEntityPriority => "ACTION_TYPE_GET_ENTITY_PRIORITY",
            Self::CreateEntity => "ACTION_TYPE_CREATE_ENTITY",
            Self::DestroyEntityByUid => "ACTION_TYPE_DESTROY_ENTITY_BY_UID",
            Self::InteractWithEntity => "ACTION_TYPE_INTERACT_WITH_ENTITY",
            Self::SearchPosition => "ACTION_TYPE_SEARCH_POSITION",
            Self::FilterHollowEntity => "ACTION_TYPE_FILTER_HOLLOW_ENTITY",
            Self::ModifyStackingOrder => "ACTION_TYPE_MODIFY_STACKING_ORDER",
            Self::InitConwayLifeGame => "ACTION_TYPE_INIT_CONWAY_LIFE_GAME",
            Self::IterateConwayLifeGame => "ACTION_TYPE_ITERATE_CONWAY_LIFE_GAME",
            Self::ChangeConwayLifeGameGridState => {
                "ACTION_TYPE_CHANGE_CONWAY_LIFE_GAME_GRID_STATE"
            }
            Self::BigTvChessUiController => "ACTION_TYPE_BIG_TV_CHESS_UI_CONTROLLER",
            Self::SetEntityState => "ACTION_TYPE_SET_ENTITY_STATE",
            Self::RemoveEntityState => "ACTION_TYPE_REMOVE_ENTITY_STATE",
            Self::GetEventTexture => "ACTION_TYPE_GET_EVENT_TEXTURE",
            Self::ModifyComponent => "ACTION_TYPE_MODIFY_COMPONENT",
            Self::ChangeHollowSoundState => "ACTION_TYPE_CHANGE_HOLLOW_SOUND_STATE",
            Self::SetEntityScriptVariable => "ACTION_TYPE_SET_ENTITY_SCRIPT_VARIABLE",
            Self::CreateSignal => "ACTION_TYPE_CREATE_SIGNAL",
            Self::SubscribeSignal => "ACTION_TYPE_SUBSCRIBE_SIGNAL",
            Self::UnsubscribeSignal => "ACTION_TYPE_UNSUBSCRIBE_SIGNAL",
            Self::SendSignal => "ACTION_TYPE_SEND_SIGNAL",
            Self::DestroySignal => "ACTION_TYPE_DESTROY_SIGNAL",
            Self::SetMultiHollowOutSection => "ACTION_TYPE_SET_MULTI_HOLLOW_OUT_SECTION",
            Self::GetEntityScriptVariable => "ACTION_TYPE_GET_ENTITY_SCRIPT_VARIABLE",
            Self::RemoveChessboard => "ACTION_TYPE_REMOVE_CHESSBOARD",
            Self::BeginTutorialGuideInteract => {
                "ACTION_TYPE_BEGIN_TUTORIAL_GUIDE_INTERACT"
            }
            Self::TimeRewindInteract => "ACTION_TYPE_TIME_REWIND_INTERACT",
            Self::LimboAvatarCard => "ACTION_TYPE_LIMBO_AVATAR_CARD",
            Self::LimboCampEvent => "ACTION_TYPE_LIMBO_CAMP_EVENT",
            Self::ModifyAimRectComponent => "ACTION_TYPE_MODIFY_AIM_RECT_COMPONENT",
            Self::RemoveFromPool => "ACTION_TYPE_REMOVE_FROM_POOL",
            Self::ActivateSegmentInteract => "ACTION_TYPE_ACTIVATE_SEGMENT_INTERACT",
            Self::RecordUseInitiativeItem => "ACTION_TYPE_RECORD_USE_INITIATIVE_ITEM",
            Self::ModifyMultiHollowOutFloor => {
                "ACTION_TYPE_MODIFY_MULTI_HOLLOW_OUT_FLOOR"
            }
            Self::SetMultiHollowOutView => "ACTION_TYPE_SET_MULTI_HOLLOW_OUT_VIEW",
            Self::MarkGridAsElevator => "ACTION_TYPE_MARK_GRID_AS_ELEVATOR",
            Self::MoveElevatorToSection => "ACTION_TYPE_MOVE_ELEVATOR_TO_SECTION",
            Self::NextDropClueEvent => "ACTION_TYPE_NEXT_DROP_CLUE_EVENT",
            Self::MoveHollowEvent => "ACTION_TYPE_MOVE_HOLLOW_EVENT",
            Self::GetFocusCameraParam => "ACTION_TYPE_GET_FOCUS_CAMERA_PARAM",
            Self::ReplaceCard => "ACTION_TYPE_REPLACE_CARD",
            Self::LoadEventParam => "ACTION_TYPE_LOAD_EVENT_PARAM",
            Self::SetLayerType => "ACTION_TYPE_SET_LAYER_TYPE",
            Self::CreateHollowSpawner => "ACTION_TYPE_CREATE_HOLLOW_SPAWNER",
            Self::SetHollowSpawner => "ACTION_TYPE_SET_HOLLOW_SPAWNER",
            Self::GetHollowSpawner => "ACTION_TYPE_GET_HOLLOW_SPAWNER",
            Self::RunHollowSpawner => "ACTION_TYPE_RUN_HOLLOW_SPAWNER",
            Self::PlayHollowQteGame => "ACTION_TYPE_PLAY_HOLLOW_QTE_GAME",
            Self::SetHollowPlayUi => "ACTION_TYPE_SET_HOLLOW_PLAY_UI",
            Self::SetHollowActivityParam => "ACTION_TYPE_SET_HOLLOW_ACTIVITY_PARAM",
            Self::GetHollowActivityParam => "ACTION_TYPE_GET_HOLLOW_ACTIVITY_PARAM",
            Self::RewardWithPerform => "ACTION_TYPE_REWARD_WITH_PERFORM",
            Self::InitHackerGame => "ACTION_TYPE_INIT_HACKER_GAME",
            Self::ModifyHackerGameParam => "ACTION_TYPE_MODIFY_HACKER_GAME_PARAM",
            Self::ModifyPopInteractComponent => {
                "ACTION_TYPE_MODIFY_POP_INTERACT_COMPONENT"
            }
            Self::SetLevelGlobalVariable => "ACTION_TYPE_SET_LEVEL_GLOBAL_VARIABLE",
            Self::EventModificationByFalling => {
                "ACTION_TYPE_EVENT_MODIFICATION_BY_FALLING"
            }
            Self::TryMoveElevator => "ACTION_TYPE_TRY_MOVE_ELEVATOR",
            Self::GetEventPoolEvent => "ACTION_TYPE_GET_EVENT_POOL_EVENT",
            Self::ChessUi3dController => "ACTION_TYPE_CHESS_UI_3D_CONTROLLER",
            Self::HollowGameFinishToLevel => "ACTION_TYPE_HOLLOW_GAME_FINISH_TO_LEVEL",
            Self::ChessboardSokobanUiInfo => "ACTION_TYPE_CHESSBOARD_SOKOBAN_UI_INFO",
            Self::CreateNpc => "ACTION_TYPE_CREATE_NPC",
            Self::SetQuestPhase => "ACTION_TYPE_SET_QUEST_PHASE",
            Self::ChangeInteract => "ACTION_TYPE_CHANGE_INTERACT",
            Self::InteractFinish => "ACTION_TYPE_INTERACT_FINISH",
            Self::RemoveMainCityQuestNpc => "ACTION_TYPE_REMOVE_MAIN_CITY_QUEST_NPC",
            Self::RemoveMainCityQuestInteract => {
                "ACTION_TYPE_REMOVE_MAIN_CITY_QUEST_INTERACT"
            }
            Self::ChangeBackSceneInfo => "ACTION_TYPE_CHANGE_BACK_SCENE_INFO",
            Self::ResetMainCityQuestGroup => "ACTION_TYPE_RESET_MAIN_CITY_QUEST_GROUP",
            Self::UnlockHollowQuest => "ACTION_TYPE_UNLOCK_HOLLOW_QUEST",
            Self::SetNpcVisible => "ACTION_TYPE_SET_NPC_VISIBLE",
            Self::RemoveInteract => "ACTION_TYPE_REMOVE_INTERACT",
            Self::RemoveNpc => "ACTION_TYPE_REMOVE_NPC",
            Self::SetVhsStoreLevel => "ACTION_TYPE_SET_VHS_STORE_LEVEL",
            Self::SetVhsStoreTrendState => "ACTION_TYPE_SET_VHS_STORE_TREND_STATE",
            Self::SwitchMainCityTime => "ACTION_TYPE_SWITCH_MAIN_CITY_TIME",
            Self::TheWorld => "ACTION_TYPE_THE_WORLD",
            Self::ForceRefresh => "ACTION_TYPE_FORCE_REFRESH",
            Self::ForbidAfk => "ACTION_TYPE_FORBID_AFK",
            Self::SwitchMainCharacter => "ACTION_TYPE_SWITCH_MAIN_CHARACTER",
            Self::SetLandEventFinish => "ACTION_TYPE_SET_LAND_EVENT_FINISH",
            Self::SetBgm => "ACTION_TYPE_SET_BGM",
            Self::SetMainCityObjectState => "ACTION_TYPE_SET_MAIN_CITY_OBJECT_STATE",
            Self::EventChoice => "ACTION_TYPE_EVENT_CHOICE",
            Self::CreateMoveNpc => "ACTION_TYPE_CREATE_MOVE_NPC",
            Self::ChangeGuidePoint => "ACTION_TYPE_CHANGE_GUIDE_POINT",
            Self::AddDailyQuest => "ACTION_TYPE_ADD_DAILY_QUEST",
            Self::AddMicroTask => "ACTION_TYPE_ADD_MICRO_TASK",
            Self::SetFirstMeet => "ACTION_TYPE_SET_FIRST_MEET",
            Self::CreateCameraZone => "ACTION_TYPE_CREATE_CAMERA_ZONE",
            Self::SetMainCityTime => "ACTION_TYPE_SET_MAIN_CITY_TIME",
            Self::NextMainCityTimePeriod => "ACTION_TYPE_NEXT_MAIN_CITY_TIME_PERIOD",
            Self::PlayerSwitchMainCharacter => "ACTION_TYPE_PLAYER_SWITCH_MAIN_CHARACTER",
            Self::EndTransition => "ACTION_TYPE_END_TRANSITION",
            Self::AddVhsFlowBuff => "ACTION_TYPE_ADD_VHS_FLOW_BUFF",
            Self::ActivatePhotoId => "ACTION_TYPE_ACTIVATE_PHOTO_ID",
            Self::AccelerateMainCityTime => "ACTION_TYPE_ACCELERATE_MAIN_CITY_TIME",
            Self::SetTrashNewFlag => "ACTION_TYPE_SET_TRASH_NEW_FLAG",
            Self::UseLastTime => "ACTION_TYPE_USE_LAST_TIME",
            Self::OccupyOvernight => "ACTION_TYPE_OCCUPY_OVERNIGHT",
            Self::ShowPhotoQuestFinishTip => "ACTION_TYPE_SHOW_PHOTO_QUEST_FINISH_TIP",
            Self::AddSoundAmb => "ACTION_TYPE_ADD_SOUND_AMB",
            Self::SubmitItem => "ACTION_TYPE_SUBMIT_ITEM",
            Self::ModTrust => "ACTION_TYPE_MOD_TRUST",
            Self::SetPartnerEventState => "ACTION_TYPE_SET_PARTNER_EVENT_STATE",
            Self::SendMessage => "ACTION_TYPE_SEND_MESSAGE",
            Self::SwitchTrackQuest => "ACTION_TYPE_SWITCH_TRACK_QUEST",
            Self::ModNpc => "ACTION_TYPE_MOD_NPC",
            Self::AcceptOvernight => "ACTION_TYPE_ACCEPT_OVERNIGHT",
            Self::ActiveTrigger => "ACTION_TYPE_ACTIVE_TRIGGER",
            Self::ModObjState => "ACTION_TYPE_MOD_OBJ_STATE",
            Self::ModSceneObj => "ACTION_TYPE_MOD_SCENE_OBJ",
            Self::FansSettle => "ACTION_TYPE_FANS_SETTLE",
            Self::OpenHallGame => "ACTION_TYPE_OPEN_HALL_GAME",
            Self::AddPartnerEvent => "ACTION_TYPE_ADD_PARTNER_EVENT",
            Self::ExecOvernightEvent => "ACTION_TYPE_EXEC_OVERNIGHT_EVENT",
            Self::SofaRestNextTimePeriod => "ACTION_TYPE_SOFA_REST_NEXT_TIME_PERIOD",
            Self::BeginUiGame => "ACTION_TYPE_BEGIN_UI_GAME",
            Self::PrepareData => "ACTION_TYPE_PREPARE_DATA",
            Self::ClearRpRecommendResult => "ACTION_TYPE_CLEAR_RP_RECOMMEND_RESULT",
            Self::DoMainCityGame => "ACTION_TYPE_DO_MAIN_CITY_GAME",
            Self::ShowInTodo => "ACTION_TYPE_SHOW_IN_TODO",
            Self::ChangeNpcName => "ACTION_TYPE_CHANGE_NPC_NAME",
            Self::CreateOva => "ACTION_TYPE_CREATE_OVA",
            Self::SetOvaState => "ACTION_TYPE_SET_OVA_STATE",
            Self::SwitchMainCharacterGuise => "ACTION_TYPE_SWITCH_MAIN_CHARACTER_GUISE",
            Self::CompleteHallGame => "ACTION_TYPE_COMPLETE_HALL_GAME",
            Self::HideMainControlAvatar => "ACTION_TYPE_HIDE_MAIN_CONTROL_AVATAR",
            Self::EatRamen => "ACTION_TYPE_EAT_RAMEN",
            Self::OngoingTips => "ACTION_TYPE_ONGOING_TIPS",
            Self::SetSound => "ACTION_TYPE_SET_SOUND",
            Self::GenCampIdleDynamicTextItem => {
                "ACTION_TYPE_GEN_CAMP_IDLE_DYNAMIC_TEXT_ITEM"
            }
            Self::MapChooseByEvent => "ACTION_TYPE_MAP_CHOOSE_BY_EVENT",
            Self::MapChooseByLayer => "ACTION_TYPE_MAP_CHOOSE_BY_LAYER",
            Self::MapChooseByNum => "ACTION_TYPE_MAP_CHOOSE_BY_NUM",
            Self::MapChooseByRange => "ACTION_TYPE_MAP_CHOOSE_BY_RANGE",
            Self::MapClearPool => "ACTION_TYPE_MAP_CLEAR_POOL",
            Self::MapSetEvent => "ACTION_TYPE_MAP_SET_EVENT",
            Self::MapSetLayer => "ACTION_TYPE_MAP_SET_LAYER",
            Self::MapSetTag => "ACTION_TYPE_MAP_SET_TAG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACTION_TYPE_NONE" => Some(Self::None),
            "ACTION_TYPE_WAIT_SECONDS" => Some(Self::WaitSeconds),
            "ACTION_TYPE_LOG_TEXT" => Some(Self::LogText),
            "ACTION_TYPE_WAIT" => Some(Self::Wait),
            "ACTION_TYPE_LOG" => Some(Self::Log),
            "ACTION_TYPE_OPEN_UI" => Some(Self::OpenUi),
            "ACTION_TYPE_SWITCH_SECTION" => Some(Self::SwitchSection),
            "ACTION_TYPE_ANIM_CTRLER_PARAM" => Some(Self::AnimCtrlerParam),
            "ACTION_TYPE_SHOW_TIP" => Some(Self::ShowTip),
            "ACTION_TYPE_SHOW_POP_WINDOW" => Some(Self::ShowPopWindow),
            "ACTION_TYPE_WALK_FAR_AWAY" => Some(Self::WalkFarAway),
            "ACTION_TYPE_OPEN_DIALOG_HOLLOW" => Some(Self::OpenDialogHollow),
            "ACTION_TYPE_CLOSE_DIALOG_HOLLOW" => Some(Self::CloseDialogHollow),
            "ACTION_TYPE_PLAY_ANIM_HOLLOW" => Some(Self::PlayAnimHollow),
            "ACTION_TYPE_CAMERA_STRETCH" => Some(Self::CameraStretch),
            "ACTION_TYPE_CAMERA_MOVE" => Some(Self::CameraMove),
            "ACTION_TYPE_CAMERA_MOVE_V2" => Some(Self::CameraMoveV2),
            "ACTION_TYPE_SHOW_TIP_HOLLOW" => Some(Self::ShowTipHollow),
            "ACTION_TYPE_SHOW_POP_WINDOW_HOLLOW" => Some(Self::ShowPopWindowHollow),
            "ACTION_TYPE_PLAY_POST_EFFECT" => Some(Self::PlayPostEffect),
            "ACTION_TYPE_ENTER_HOLLOW_QUEST" => Some(Self::EnterHollowQuest),
            "ACTION_TYPE_ENTER_ARCHIVE_FILE_QUEST" => Some(Self::EnterArchiveFileQuest),
            "ACTION_TYPE_PRESET" => Some(Self::Preset),
            "ACTION_TYPE_BLACK_MASK" => Some(Self::BlackMask),
            "ACTION_TYPE_PLAY_SOUND" => Some(Self::PlaySound),
            "ACTION_TYPE_CLOSE_TIP" => Some(Self::CloseTip),
            "ACTION_TYPE_RECONECT_CHESSBOARD" => Some(Self::ReconectChessboard),
            "ACTION_TYPE_HOLLOW_SET_SWITCH_EFFECT" => Some(Self::HollowSetSwitchEffect),
            "ACTION_TYPE_OUT_DOOR" => Some(Self::OutDoor),
            "ACTION_TYPE_FREEZE_CHESSBOARD_CAMERA" => Some(Self::FreezeChessboardCamera),
            "ACTION_TYPE_SHOW_VHS_STORE_LEVEL_TIPS" => Some(Self::ShowVhsStoreLevelTips),
            "ACTION_TYPE_INTERACT_NPC_WITH_ANIM" => Some(Self::InteractNpcWithAnim),
            "ACTION_TYPE_CHANGE_HOLLOW_BG" => Some(Self::ChangeHollowBg),
            "ACTION_TYPE_TRASH_GACHA_GET_DATA" => Some(Self::TrashGachaGetData),
            "ACTION_TYPE_TRASH_GACHA" => Some(Self::TrashGacha),
            "ACTION_TYPE_SHOW_QUEST_TIP" => Some(Self::ShowQuestTip),
            "ACTION_TYPE_TELEPORT_UI" => Some(Self::TeleportUi),
            "ACTION_TYPE_CAMERA_ACTIVE" => Some(Self::CameraActive),
            "ACTION_TYPE_CAMERA_RESET" => Some(Self::CameraReset),
            "ACTION_TYPE_CREATE_FC" => Some(Self::CreateFc),
            "ACTION_TYPE_BEHAVIOUR_FC" => Some(Self::BehaviourFc),
            "ACTION_TYPE_SEND_EVENT_FC" => Some(Self::SendEventFc),
            "ACTION_TYPE_READ_FC_BLACK_BOARD_DATA" => Some(Self::ReadFcBlackBoardData),
            "ACTION_TYPE_WRITE_FC_BLACK_BOARD_DATA" => Some(Self::WriteFcBlackBoardData),
            "ACTION_TYPE_CHANGE_SOUND_STATE" => Some(Self::ChangeSoundState),
            "ACTION_TYPE_AFK_HOLLOW" => Some(Self::AfkHollow),
            "ACTION_TYPE_SWITCH_BIG_TV" => Some(Self::SwitchBigTv),
            "ACTION_TYPE_TRIGGER_INTERACT" => Some(Self::TriggerInteract),
            "ACTION_TYPE_STOP_ANIM" => Some(Self::StopAnim),
            "ACTION_TYPE_GET_TRUST" => Some(Self::GetTrust),
            "ACTION_TYPE_PLAY_DIALOG_ANIM" => Some(Self::PlayDialogAnim),
            "ACTION_TYPE_UNFREEZE_CHESSBOARD_CAMERA" => {
                Some(Self::UnfreezeChessboardCamera)
            }
            "ACTION_TYPE_WAIT_TIPS_END" => Some(Self::WaitTipsEnd),
            "ACTION_TYPE_BEGIN_TUTORIAL_GUIDE" => Some(Self::BeginTutorialGuide),
            "ACTION_TYPE_FOCUS_CAMERA" => Some(Self::FocusCamera),
            "ACTION_TYPE_UNLOCK_CLUE" => Some(Self::UnlockClue),
            "ACTION_TYPE_AVATAR_TIPS_SWITCH" => Some(Self::AvatarTipsSwitch),
            "ACTION_TYPE_FINISH_RESCUE" => Some(Self::FinishRescue),
            "ACTION_TYPE_PLAY_TV_EFFECT" => Some(Self::PlayTvEffect),
            "ACTION_TYPE_SET_INTERACT_POINT" => Some(Self::SetInteractPoint),
            "ACTION_TYPE_HIDE_MAIN_CITY_UI" => Some(Self::HideMainCityUi),
            "ACTION_TYPE_CHAT_CAMERA" => Some(Self::ChatCamera),
            "ACTION_TYPE_CREATE_CLIENT_ENTITY" => Some(Self::CreateClientEntity),
            "ACTION_TYPE_SET_NPC_VISIBLE_CLIENT" => Some(Self::SetNpcVisibleClient),
            "ACTION_TYPE_GACHA_ITEM_PERFORM" => Some(Self::GachaItemPerform),
            "ACTION_TYPE_SET_MESSAGE_CLIENT" => Some(Self::SetMessageClient),
            "ACTION_TYPE_MOD_MAIN_CITY_TIME_CLIENT" => Some(Self::ModMainCityTimeClient),
            "ACTION_TYPE_MODIFY_LIGHT_LEVEL_PERFORM" => {
                Some(Self::ModifyLightLevelPerform)
            }
            "ACTION_TYPE_SET_POSITION" => Some(Self::SetPosition),
            "ACTION_TYPE_SET_CHESSBOARD_PERFORM_MODE" => {
                Some(Self::SetChessboardPerformMode)
            }
            "ACTION_TYPE_TRANSITION" => Some(Self::Transition),
            "ACTION_TYPE_WAIT_UNTIL_UI_CLOSE" => Some(Self::WaitUntilUiClose),
            "ACTION_TYPE_WAIT_TRANSITION_END" => Some(Self::WaitTransitionEnd),
            "ACTION_TYPE_CLOSE_UI" => Some(Self::CloseUi),
            "ACTION_TYPE_QUIT_PHOTO" => Some(Self::QuitPhoto),
            "ACTION_TYPE_SHOW_TELEPORT_UI" => Some(Self::ShowTeleportUi),
            "ACTION_TYPE_MODIFY_CAMERA_TARGET_SECTION" => {
                Some(Self::ModifyCameraTargetSection)
            }
            "ACTION_TYPE_CAMERA_BACK_TO_PLAYER" => Some(Self::CameraBackToPlayer),
            "ACTION_TYPE_RESET_SCENE_OBJ" => Some(Self::ResetSceneObj),
            "ACTION_TYPE_MANUAL_ACCELERATE" => Some(Self::ManualAccelerate),
            "ACTION_TYPE_BREAK_NAVIGATE" => Some(Self::BreakNavigate),
            "ACTION_TYPE_SHOW_EXIT_BUTTON_NEW" => Some(Self::ShowExitButtonNew),
            "ACTION_TYPE_SHOW_BOTTOM_TIP_HOLLOW" => Some(Self::ShowBottomTipHollow),
            "ACTION_TYPE_SHOW_CHAPTER_TIP" => Some(Self::ShowChapterTip),
            "ACTION_TYPE_ENTER_DUNGEON_QUEST" => Some(Self::EnterDungeonQuest),
            "ACTION_TYPE_DOWNLOAD_FULL_RESOURCE" => Some(Self::DownloadFullResource),
            "ACTION_TYPE_AREA_TIPS" => Some(Self::AreaTips),
            "ACTION_TYPE_CLIENT_PERFORM" => Some(Self::ClientPerform),
            "ACTION_TYPE_SHOW_ITEM" => Some(Self::ShowItem),
            "ACTION_TYPE_SWITCH_OVA" => Some(Self::SwitchOva),
            "ACTION_TYPE_SET_LIFT_STATUS" => Some(Self::SetLiftStatus),
            "ACTION_TYPE_AREA_CAMERA_MODIFY" => Some(Self::AreaCameraModify),
            "ACTION_TYPE_TRIGGER_PERFORM_BEHAVIOR" => Some(Self::TriggerPerformBehavior),
            "ACTION_TYPE_SWITCH_ATMOSPHERE" => Some(Self::SwitchAtmosphere),
            "ACTION_TYPE_MODIFY_LIGHT_DIFFUSION_POINTS" => {
                Some(Self::ModifyLightDiffusionPoints)
            }
            "ACTION_TYPE_MOD_CAT_NAME" => Some(Self::ModCatName),
            "ACTION_TYPE_OPEN_UI_GAME" => Some(Self::OpenUiGame),
            "ACTION_TYPE_OPEN_DIALOG_HOLLOW_V2" => Some(Self::OpenDialogHollowV2),
            "ACTION_TYPE_PLAY_DIALOG_ANIM_V2" => Some(Self::PlayDialogAnimV2),
            "ACTION_TYPE_CLOSE_DIALOG_HOLLOW_V2" => Some(Self::CloseDialogHollowV2),
            "ACTION_TYPE_BREAK_DIALOG_ANIM_V2" => Some(Self::BreakDialogAnimV2),
            "ACTION_TYPE_WAIT_ANIM_END" => Some(Self::WaitAnimEnd),
            "ACTION_TYPE_PLAY_ANIM_SEQUENCE" => Some(Self::PlayAnimSequence),
            "ACTION_TYPE_END_OVERLORDFEAST_GAME" => Some(Self::EndOverlordfeastGame),
            "ACTION_TYPE_PLAY_AIM_SEQUENCE" => Some(Self::PlayAimSequence),
            "ACTION_TYPE_CLIENT_SWITCH_DELAY" => Some(Self::ClientSwitchDelay),
            "ACTION_TYPE_BEGIN_PHOTO" => Some(Self::BeginPhoto),
            "ACTION_TYPE_CHESSBOARD_GAME_HENSHIN" => Some(Self::ChessboardGameHenshin),
            "ACTION_TYPE_SWITCH_GUISE_AVATAR" => Some(Self::SwitchGuiseAvatar),
            "ACTION_TYPE_IF" => Some(Self::If),
            "ACTION_TYPE_START_LOOP" => Some(Self::StartLoop),
            "ACTION_TYPE_END_LOOP" => Some(Self::EndLoop),
            "ACTION_TYPE_CALL_FUNCTION" => Some(Self::CallFunction),
            "ACTION_TYPE_RETURN" => Some(Self::Return),
            "ACTION_TYPE_RESET_EVENT" => Some(Self::ResetEvent),
            "ACTION_TYPE_ADD_ITEM" => Some(Self::AddItem),
            "ACTION_TYPE_SET_VARIABLE" => Some(Self::SetVariable),
            "ACTION_TYPE_SET_CONDITION_PROGRESS" => Some(Self::SetConditionProgress),
            "ACTION_TYPE_RANDOM_VARIABLE_VALUE" => Some(Self::RandomVariableValue),
            "ACTION_TYPE_LIST_SPECIAL_OPT" => Some(Self::ListSpecialOpt),
            "ACTION_TYPE_FINISH_QUEST" => Some(Self::FinishQuest),
            "ACTION_TYPE_RANDOM_WITH_WEIGHT" => Some(Self::RandomWithWeight),
            "ACTION_TYPE_PERFORM" => Some(Self::Perform),
            "ACTION_TYPE_REWARD" => Some(Self::Reward),
            "ACTION_TYPE_SET_LIST" => Some(Self::SetList),
            "ACTION_TYPE_GET_LIST" => Some(Self::GetList),
            "ACTION_TYPE_START_ACTION" => Some(Self::StartAction),
            "ACTION_TYPE_SET_STRING" => Some(Self::SetString),
            "ACTION_TYPE_SEND_CUSTOM_EVENT_TRACKING" => {
                Some(Self::SendCustomEventTracking)
            }
            "ACTION_TYPE_EMPTY_ACTION" => Some(Self::EmptyAction),
            "ACTION_TYPE_SET_VECTOR2" => Some(Self::SetVector2),
            "ACTION_TYPE_SWITCH" => Some(Self::Switch),
            "ACTION_TYPE_SWITCH_COMPARE_INT" => Some(Self::SwitchCompareInt),
            "ACTION_TYPE_DRAW" => Some(Self::Draw),
            "ACTION_TYPE_SET_VEC2_LIST" => Some(Self::SetVec2List),
            "ACTION_TYPE_GET_VEC2_LIST" => Some(Self::GetVec2List),
            "ACTION_TYPE_CALL_FUNCTION_V2" => Some(Self::CallFunctionV2),
            "ACTION_TYPE_ENTER_HOLLOW_SHOP" => Some(Self::EnterHollowShop),
            "ACTION_TYPE_MAKE_CHOICE" => Some(Self::MakeChoice),
            "ACTION_TYPE_MODIFY_SCENE_PROPERTY" => Some(Self::ModifySceneProperty),
            "ACTION_TYPE_FINISH_EVENT" => Some(Self::FinishEvent),
            "ACTION_TYPE_TRIGGER_BATTLE" => Some(Self::TriggerBattle),
            "ACTION_TYPE_AVERAGE_AVATAR_HP" => Some(Self::AverageAvatarHp),
            "ACTION_TYPE_REMOVE_CARD" => Some(Self::RemoveCard),
            "ACTION_TYPE_DROP_POOL" => Some(Self::DropPool),
            "ACTION_TYPE_TRANSFER" => Some(Self::Transfer),
            "ACTION_TYPE_FINISH_HOLLOW" => Some(Self::FinishHollow),
            "ACTION_TYPE_RANDOM_ITEM_CARD" => Some(Self::RandomItemCard),
            "ACTION_TYPE_EVENT_MODIFICATION" => Some(Self::EventModification),
            "ACTION_TYPE_CHANGE_AVATAR_STATE" => Some(Self::ChangeAvatarState),
            "ACTION_TYPE_DROP_PACK" => Some(Self::DropPack),
            "ACTION_TYPE_SET_MAP_STATE" => Some(Self::SetMapState),
            "ACTION_TYPE_DROP_CURSE" => Some(Self::DropCurse),
            "ACTION_TYPE_LOG_HOLLOW" => Some(Self::LogHollow),
            "ACTION_TYPE_DROP_CARD" => Some(Self::DropCard),
            "ACTION_TYPE_CHANGE_HOLLOW_EVENT_WEIGHT" => {
                Some(Self::ChangeHollowEventWeight)
            }
            "ACTION_TYPE_REMOVE_CURSE" => Some(Self::RemoveCurse),
            "ACTION_TYPE_HIDE_NODE" => Some(Self::HideNode),
            "ACTION_TYPE_SET_CHALLENGE" => Some(Self::SetChallenge),
            "ACTION_TYPE_DROP_CHALLENGE_ID" => Some(Self::DropChallengeId),
            "ACTION_TYPE_GET_AVATAR_INFO" => Some(Self::GetAvatarInfo),
            "ACTION_TYPE_SET_HOLLOW_ITEM" => Some(Self::SetHollowItem),
            "ACTION_TYPE_CHANGE_CHARACTER" => Some(Self::ChangeCharacter),
            "ACTION_TYPE_NEW_HOLLOW" => Some(Self::NewHollow),
            "ACTION_TYPE_SLOT_MACHINE" => Some(Self::SlotMachine),
            "ACTION_TYPE_SET_HOLLOW_BLACK_OUT" => Some(Self::SetHollowBlackOut),
            "ACTION_TYPE_FINISH_BLACK_OUT" => Some(Self::FinishBlackOut),
            "ACTION_TYPE_SET_HOLLOW_SYSTEM_STATE" => Some(Self::SetHollowSystemState),
            "ACTION_TYPE_ADD_CHARACTER" => Some(Self::AddCharacter),
            "ACTION_TYPE_LOCK_CURSE" => Some(Self::LockCurse),
            "ACTION_TYPE_HOLLOW_DISTANCE" => Some(Self::HollowDistance),
            "ACTION_TYPE_PUSH_BACK" => Some(Self::PushBack),
            "ACTION_TYPE_APPLY_ABILITY" => Some(Self::ApplyAbility),
            "ACTION_TYPE_REMOVE_ABILITY" => Some(Self::RemoveAbility),
            "ACTION_TYPE_RANDOM_BATTLE_ID" => Some(Self::RandomBattleId),
            "ACTION_TYPE_GET_INDEX_BY_FILTER" => Some(Self::GetIndexByFilter),
            "ACTION_TYPE_SET_BATTLE_TYPE" => Some(Self::SetBattleType),
            "ACTION_TYPE_GET_POSITION" => Some(Self::GetPosition),
            "ACTION_TYPE_START_MINI_GAME" => Some(Self::StartMiniGame),
            "ACTION_TYPE_SET_HOLLOW_ITEM_SLOT" => Some(Self::SetHollowItemSlot),
            "ACTION_TYPE_GET_HOLLOW_ITEM" => Some(Self::GetHollowItem),
            "ACTION_TYPE_SEARCH_GRID" => Some(Self::SearchGrid),
            "ACTION_TYPE_SET_NPC_STATE" => Some(Self::SetNpcState),
            "ACTION_TYPE_GET_NPC_INSTANCE_ID" => Some(Self::GetNpcInstanceId),
            "ACTION_TYPE_DESTORY_NPC" => Some(Self::DestoryNpc),
            "ACTION_TYPE_ADD_CHARACTER_ABYSS" => Some(Self::AddCharacterAbyss),
            "ACTION_TYPE_CHANGE_CHARACTER_ABYSS" => Some(Self::ChangeCharacterAbyss),
            "ACTION_TYPE_GET_CHARACTER_POOL_ABYSS" => Some(Self::GetCharacterPoolAbyss),
            "ACTION_TYPE_ABYSS_DROP_CHARACTER_POOL" => Some(Self::AbyssDropCharacterPool),
            "ACTION_TYPE_GET_LEADER_OF_HOLLOW_NPC" => Some(Self::GetLeaderOfHollowNpc),
            "ACTION_TYPE_SET_LEADER_OF_HOLLOW_NPC" => Some(Self::SetLeaderOfHollowNpc),
            "ACTION_TYPE_UPDATE_SAVE_NPC_NUM" => Some(Self::UpdateSaveNpcNum),
            "ACTION_TYPE_PUSH_WITH_DIRECTION" => Some(Self::PushWithDirection),
            "ACTION_TYPE_HOLLOW_NPC_FIND_PATH" => Some(Self::HollowNpcFindPath),
            "ACTION_TYPE_HOLLOW_NPC_MOVE" => Some(Self::HollowNpcMove),
            "ACTION_TYPE_NEW_CHESSBOARD" => Some(Self::NewChessboard),
            "ACTION_TYPE_GO_TO_NEXT_LAYER" => Some(Self::GoToNextLayer),
            "ACTION_TYPE_GO_TO_CHESSBOARD" => Some(Self::GoToChessboard),
            "ACTION_TYPE_GET_PRE_CHESSBOARD" => Some(Self::GetPreChessboard),
            "ACTION_TYPE_TRIGGER_HOLLOW_NPC_BEHAVIOR" => {
                Some(Self::TriggerHollowNpcBehavior)
            }
            "ACTION_TYPE_SHOW_LAYER_RESULT" => Some(Self::ShowLayerResult),
            "ACTION_TYPE_HENSHIN" => Some(Self::Henshin),
            "ACTION_TYPE_CREATE_HOLLOW_NPC" => Some(Self::CreateHollowNpc),
            "ACTION_TYPE_DROP_CHESSBOARD_ID" => Some(Self::DropChessboardId),
            "ACTION_TYPE_MAKE_DIALOG_CHOICE" => Some(Self::MakeDialogChoice),
            "ACTION_TYPE_GET_EVENT_ID" => Some(Self::GetEventId),
            "ACTION_TYPE_COUNT_DROP_POOL" => Some(Self::CountDropPool),
            "ACTION_TYPE_MAKE_ITEM_CHOICE" => Some(Self::MakeItemChoice),
            "ACTION_TYPE_HP_ACT_HOLLOW" => Some(Self::HpActHollow),
            "ACTION_TYPE_BAN_HOLLOW_EVENT" => Some(Self::BanHollowEvent),
            "ACTION_TYPE_COORDINATE_TRANSFORM" => Some(Self::CoordinateTransform),
            "ACTION_TYPE_REGISTER_VARIABLE_CONDITION" => {
                Some(Self::RegisterVariableCondition)
            }
            "ACTION_TYPE_ON_OFF_CATEGORY" => Some(Self::OnOffCategory),
            "ACTION_TYPE_RESET_BIG_TV_SNAPSHOT" => Some(Self::ResetBigTvSnapshot),
            "ACTION_TYPE_BIG_TV_SUPPORT_SNAPSHOT" => Some(Self::BigTvSupportSnapshot),
            "ACTION_TYPE_SET_EVENT_ICON" => Some(Self::SetEventIcon),
            "ACTION_TYPE_GET_ANIM_SHEET_ID" => Some(Self::GetAnimSheetId),
            "ACTION_TYPE_HOLLOW_NPC_HENSHIN" => Some(Self::HollowNpcHenshin),
            "ACTION_TYPE_HOLLOW_NPC_TRANSFER" => Some(Self::HollowNpcTransfer),
            "ACTION_TYPE_BIND_BIG_TV" => Some(Self::BindBigTv),
            "ACTION_TYPE_MOVE_NPC_TO_SECTION" => Some(Self::MoveNpcToSection),
            "ACTION_TYPE_GET_NPC_ID" => Some(Self::GetNpcId),
            "ACTION_TYPE_SEARCH_HOLLOW_NPC" => Some(Self::SearchHollowNpc),
            "ACTION_TYPE_BOOM" => Some(Self::Boom),
            "ACTION_TYPE_TRIGGER_HOLLOW_EVENT" => Some(Self::TriggerHollowEvent),
            "ACTION_TYPE_BREAK_DIALOG_ANIM" => Some(Self::BreakDialogAnim),
            "ACTION_TYPE_MOVE_BIG_TV" => Some(Self::MoveBigTv),
            "ACTION_TYPE_SET_NEXT_LAYER_CHESSBOARD_ID" => {
                Some(Self::SetNextLayerChessboardId)
            }
            "ACTION_TYPE_GET_BOSS_BATTLE_EVENT" => Some(Self::GetBossBattleEvent),
            "ACTION_TYPE_CREATE_HOLLOW_SNAKE" => Some(Self::CreateHollowSnake),
            "ACTION_TYPE_SET_GRID_STAMINA_STATE" => Some(Self::SetGridStaminaState),
            "ACTION_TYPE_DISPLAY_BIG_TV_CHESSBOARD" => Some(Self::DisplayBigTvChessboard),
            "ACTION_TYPE_SPLIT_HOLLOW_SNAKE" => Some(Self::SplitHollowSnake),
            "ACTION_TYPE_GET_HOLLOW_SNAKE_INFO" => Some(Self::GetHollowSnakeInfo),
            "ACTION_TYPE_MODIFY_HOLLOW_SNAKE" => Some(Self::ModifyHollowSnake),
            "ACTION_TYPE_CHANGE_HOLLOW_NPC_APPERANCE" => {
                Some(Self::ChangeHollowNpcApperance)
            }
            "ACTION_TYPE_OPEN_BIG_TV_SOKOBAN_GAME" => Some(Self::OpenBigTvSokobanGame),
            "ACTION_TYPE_SET_INTERCONNECTED_STORY_EVENT" => {
                Some(Self::SetInterconnectedStoryEvent)
            }
            "ACTION_TYPE_HOLLOW_NPC_IMITATE" => Some(Self::HollowNpcImitate),
            "ACTION_TYPE_TRIGGER_HOLLOW_NPC_EARLY_ACT" => {
                Some(Self::TriggerHollowNpcEarlyAct)
            }
            "ACTION_TYPE_GET_AVATAR_BY_TAG" => Some(Self::GetAvatarByTag),
            "ACTION_TYPE_SET_BATTLE_TYPE_ABYSS" => Some(Self::SetBattleTypeAbyss),
            "ACTION_TYPE_REMOVE_EVENT_ID_FROM_RANDOM_POOL" => {
                Some(Self::RemoveEventIdFromRandomPool)
            }
            "ACTION_TYPE_RECYCLE_HOLLOW_ITEM" => Some(Self::RecycleHollowItem),
            "ACTION_TYPE_COPY_EVENT" => Some(Self::CopyEvent),
            "ACTION_TYPE_BAN_CHARACTER" => Some(Self::BanCharacter),
            "ACTION_TYPE_REMOVE_CHARACTER" => Some(Self::RemoveCharacter),
            "ACTION_TYPE_SET_NPC_ATTR" => Some(Self::SetNpcAttr),
            "ACTION_TYPE_GET_NPC_ATTR" => Some(Self::GetNpcAttr),
            "ACTION_TYPE_HIT_NPC" => Some(Self::HitNpc),
            "ACTION_TYPE_GET_PLAYER_HOLLOW_MOVE_PATH" => {
                Some(Self::GetPlayerHollowMovePath)
            }
            "ACTION_TYPE_GET_BIG_TV_INDEX" => Some(Self::GetBigTvIndex),
            "ACTION_TYPE_CLEAR_NPC" => Some(Self::ClearNpc),
            "ACTION_TYPE_SAVE_MINI_SNAPSHOT" => Some(Self::SaveMiniSnapshot),
            "ACTION_TYPE_GET_POSSESS_HOLLOW_ITEM" => Some(Self::GetPossessHollowItem),
            "ACTION_TYPE_RESET_HOLLOW_LINE_QUEST" => Some(Self::ResetHollowLineQuest),
            "ACTION_TYPE_MODIFY_LIGHT_LEVEL" => Some(Self::ModifyLightLevel),
            "ACTION_TYPE_GET_LIGHT_LEVEL" => Some(Self::GetLightLevel),
            "ACTION_TYPE_ADD_HOLLOW_LIGHT" => Some(Self::AddHollowLight),
            "ACTION_TYPE_REMOVE_HOLLOW_LIGHT" => Some(Self::RemoveHollowLight),
            "ACTION_TYPE_MODIFY_CHESSBOARD_PLUGIN" => Some(Self::ModifyChessboardPlugin),
            "ACTION_TYPE_GET_LIGHT_UID" => Some(Self::GetLightUid),
            "ACTION_TYPE_NEW_BOOM" => Some(Self::NewBoom),
            "ACTION_TYPE_SET_ENTITY_PARAM" => Some(Self::SetEntityParam),
            "ACTION_TYPE_GET_ENTITY_PARAM" => Some(Self::GetEntityParam),
            "ACTION_TYPE_REPAIR_ZONE" => Some(Self::RepairZone),
            "ACTION_TYPE_PUSH_REPAIR_ZONE" => Some(Self::PushRepairZone),
            "ACTION_TYPE_SET_INNER_WORLD_MAP_STATE" => Some(Self::SetInnerWorldMapState),
            "ACTION_TYPE_LIST_CONVERT" => Some(Self::ListConvert),
            "ACTION_TYPE_ABYSS_GET_BATTLE_EVENT" => Some(Self::AbyssGetBattleEvent),
            "ACTION_TYPE_TRIGGER_ENTITY_BASIC_BEHAVIOR" => {
                Some(Self::TriggerEntityBasicBehavior)
            }
            "ACTION_TYPE_TRIGGER_ENTITY_MOVE" => Some(Self::TriggerEntityMove),
            "ACTION_TYPE_TRIGGER_ENTITY_TRANSFER" => Some(Self::TriggerEntityTransfer),
            "ACTION_TYPE_TRIGGER_ENTITY_INTERACT" => Some(Self::TriggerEntityInteract),
            "ACTION_TYPE_UPGRADE_CARD" => Some(Self::UpgradeCard),
            "ACTION_TYPE_NEW_TIME_REWIND" => Some(Self::NewTimeRewind),
            "ACTION_TYPE_ENTER_TIME_REWIND" => Some(Self::EnterTimeRewind),
            "ACTION_TYPE_INIT_TIME_SEGMENT" => Some(Self::InitTimeSegment),
            "ACTION_TYPE_MODIFY_TIME_SEGMENT" => Some(Self::ModifyTimeSegment),
            "ACTION_TYPE_MODIFY_TIME_REWIND" => Some(Self::ModifyTimeRewind),
            "ACTION_TYPE_GET_TIME_REWIND_INFO" => Some(Self::GetTimeRewindInfo),
            "ACTION_TYPE_FINISH_KEY_SEGMENT" => Some(Self::FinishKeySegment),
            "ACTION_TYPE_ACTIVATE_GRID_IN_SEGMENT" => Some(Self::ActivateGridInSegment),
            "ACTION_TYPE_COUNT_CARD_POOL" => Some(Self::CountCardPool),
            "ACTION_TYPE_MAKE_BANGBOO_CHOICE" => Some(Self::MakeBangbooChoice),
            "ACTION_TYPE_CHANGE_BANGBOO_CHOICE" => Some(Self::ChangeBangbooChoice),
            "ACTION_TYPE_TRIGGER_ENTITY_SCRIPT" => Some(Self::TriggerEntityScript),
            "ACTION_TYPE_ADD_STRESS_PUNISH_CURSE" => Some(Self::AddStressPunishCurse),
            "ACTION_TYPE_POPUP_TIP" => Some(Self::PopupTip),
            "ACTION_TYPE_HIDE_HOLLOW_ENTITY" => Some(Self::HideHollowEntity),
            "ACTION_TYPE_GET_ENTITY_PRIORITY_LIST" => Some(Self::GetEntityPriorityList),
            "ACTION_TYPE_CHESS_UI_CONTROLLER" => Some(Self::ChessUiController),
            "ACTION_TYPE_GET_ENTITY_PRIORITY" => Some(Self::GetEntityPriority),
            "ACTION_TYPE_CREATE_ENTITY" => Some(Self::CreateEntity),
            "ACTION_TYPE_DESTROY_ENTITY_BY_UID" => Some(Self::DestroyEntityByUid),
            "ACTION_TYPE_INTERACT_WITH_ENTITY" => Some(Self::InteractWithEntity),
            "ACTION_TYPE_SEARCH_POSITION" => Some(Self::SearchPosition),
            "ACTION_TYPE_FILTER_HOLLOW_ENTITY" => Some(Self::FilterHollowEntity),
            "ACTION_TYPE_MODIFY_STACKING_ORDER" => Some(Self::ModifyStackingOrder),
            "ACTION_TYPE_INIT_CONWAY_LIFE_GAME" => Some(Self::InitConwayLifeGame),
            "ACTION_TYPE_ITERATE_CONWAY_LIFE_GAME" => Some(Self::IterateConwayLifeGame),
            "ACTION_TYPE_CHANGE_CONWAY_LIFE_GAME_GRID_STATE" => {
                Some(Self::ChangeConwayLifeGameGridState)
            }
            "ACTION_TYPE_BIG_TV_CHESS_UI_CONTROLLER" => {
                Some(Self::BigTvChessUiController)
            }
            "ACTION_TYPE_SET_ENTITY_STATE" => Some(Self::SetEntityState),
            "ACTION_TYPE_REMOVE_ENTITY_STATE" => Some(Self::RemoveEntityState),
            "ACTION_TYPE_GET_EVENT_TEXTURE" => Some(Self::GetEventTexture),
            "ACTION_TYPE_MODIFY_COMPONENT" => Some(Self::ModifyComponent),
            "ACTION_TYPE_CHANGE_HOLLOW_SOUND_STATE" => Some(Self::ChangeHollowSoundState),
            "ACTION_TYPE_SET_ENTITY_SCRIPT_VARIABLE" => {
                Some(Self::SetEntityScriptVariable)
            }
            "ACTION_TYPE_CREATE_SIGNAL" => Some(Self::CreateSignal),
            "ACTION_TYPE_SUBSCRIBE_SIGNAL" => Some(Self::SubscribeSignal),
            "ACTION_TYPE_UNSUBSCRIBE_SIGNAL" => Some(Self::UnsubscribeSignal),
            "ACTION_TYPE_SEND_SIGNAL" => Some(Self::SendSignal),
            "ACTION_TYPE_DESTROY_SIGNAL" => Some(Self::DestroySignal),
            "ACTION_TYPE_SET_MULTI_HOLLOW_OUT_SECTION" => {
                Some(Self::SetMultiHollowOutSection)
            }
            "ACTION_TYPE_GET_ENTITY_SCRIPT_VARIABLE" => {
                Some(Self::GetEntityScriptVariable)
            }
            "ACTION_TYPE_REMOVE_CHESSBOARD" => Some(Self::RemoveChessboard),
            "ACTION_TYPE_BEGIN_TUTORIAL_GUIDE_INTERACT" => {
                Some(Self::BeginTutorialGuideInteract)
            }
            "ACTION_TYPE_TIME_REWIND_INTERACT" => Some(Self::TimeRewindInteract),
            "ACTION_TYPE_LIMBO_AVATAR_CARD" => Some(Self::LimboAvatarCard),
            "ACTION_TYPE_LIMBO_CAMP_EVENT" => Some(Self::LimboCampEvent),
            "ACTION_TYPE_MODIFY_AIM_RECT_COMPONENT" => Some(Self::ModifyAimRectComponent),
            "ACTION_TYPE_REMOVE_FROM_POOL" => Some(Self::RemoveFromPool),
            "ACTION_TYPE_ACTIVATE_SEGMENT_INTERACT" => {
                Some(Self::ActivateSegmentInteract)
            }
            "ACTION_TYPE_RECORD_USE_INITIATIVE_ITEM" => {
                Some(Self::RecordUseInitiativeItem)
            }
            "ACTION_TYPE_MODIFY_MULTI_HOLLOW_OUT_FLOOR" => {
                Some(Self::ModifyMultiHollowOutFloor)
            }
            "ACTION_TYPE_SET_MULTI_HOLLOW_OUT_VIEW" => Some(Self::SetMultiHollowOutView),
            "ACTION_TYPE_MARK_GRID_AS_ELEVATOR" => Some(Self::MarkGridAsElevator),
            "ACTION_TYPE_MOVE_ELEVATOR_TO_SECTION" => Some(Self::MoveElevatorToSection),
            "ACTION_TYPE_NEXT_DROP_CLUE_EVENT" => Some(Self::NextDropClueEvent),
            "ACTION_TYPE_MOVE_HOLLOW_EVENT" => Some(Self::MoveHollowEvent),
            "ACTION_TYPE_GET_FOCUS_CAMERA_PARAM" => Some(Self::GetFocusCameraParam),
            "ACTION_TYPE_REPLACE_CARD" => Some(Self::ReplaceCard),
            "ACTION_TYPE_LOAD_EVENT_PARAM" => Some(Self::LoadEventParam),
            "ACTION_TYPE_SET_LAYER_TYPE" => Some(Self::SetLayerType),
            "ACTION_TYPE_CREATE_HOLLOW_SPAWNER" => Some(Self::CreateHollowSpawner),
            "ACTION_TYPE_SET_HOLLOW_SPAWNER" => Some(Self::SetHollowSpawner),
            "ACTION_TYPE_GET_HOLLOW_SPAWNER" => Some(Self::GetHollowSpawner),
            "ACTION_TYPE_RUN_HOLLOW_SPAWNER" => Some(Self::RunHollowSpawner),
            "ACTION_TYPE_PLAY_HOLLOW_QTE_GAME" => Some(Self::PlayHollowQteGame),
            "ACTION_TYPE_SET_HOLLOW_PLAY_UI" => Some(Self::SetHollowPlayUi),
            "ACTION_TYPE_SET_HOLLOW_ACTIVITY_PARAM" => Some(Self::SetHollowActivityParam),
            "ACTION_TYPE_GET_HOLLOW_ACTIVITY_PARAM" => Some(Self::GetHollowActivityParam),
            "ACTION_TYPE_REWARD_WITH_PERFORM" => Some(Self::RewardWithPerform),
            "ACTION_TYPE_INIT_HACKER_GAME" => Some(Self::InitHackerGame),
            "ACTION_TYPE_MODIFY_HACKER_GAME_PARAM" => Some(Self::ModifyHackerGameParam),
            "ACTION_TYPE_MODIFY_POP_INTERACT_COMPONENT" => {
                Some(Self::ModifyPopInteractComponent)
            }
            "ACTION_TYPE_SET_LEVEL_GLOBAL_VARIABLE" => Some(Self::SetLevelGlobalVariable),
            "ACTION_TYPE_EVENT_MODIFICATION_BY_FALLING" => {
                Some(Self::EventModificationByFalling)
            }
            "ACTION_TYPE_TRY_MOVE_ELEVATOR" => Some(Self::TryMoveElevator),
            "ACTION_TYPE_GET_EVENT_POOL_EVENT" => Some(Self::GetEventPoolEvent),
            "ACTION_TYPE_CHESS_UI_3D_CONTROLLER" => Some(Self::ChessUi3dController),
            "ACTION_TYPE_HOLLOW_GAME_FINISH_TO_LEVEL" => {
                Some(Self::HollowGameFinishToLevel)
            }
            "ACTION_TYPE_CHESSBOARD_SOKOBAN_UI_INFO" => {
                Some(Self::ChessboardSokobanUiInfo)
            }
            "ACTION_TYPE_CREATE_NPC" => Some(Self::CreateNpc),
            "ACTION_TYPE_SET_QUEST_PHASE" => Some(Self::SetQuestPhase),
            "ACTION_TYPE_CHANGE_INTERACT" => Some(Self::ChangeInteract),
            "ACTION_TYPE_INTERACT_FINISH" => Some(Self::InteractFinish),
            "ACTION_TYPE_REMOVE_MAIN_CITY_QUEST_NPC" => {
                Some(Self::RemoveMainCityQuestNpc)
            }
            "ACTION_TYPE_REMOVE_MAIN_CITY_QUEST_INTERACT" => {
                Some(Self::RemoveMainCityQuestInteract)
            }
            "ACTION_TYPE_CHANGE_BACK_SCENE_INFO" => Some(Self::ChangeBackSceneInfo),
            "ACTION_TYPE_RESET_MAIN_CITY_QUEST_GROUP" => {
                Some(Self::ResetMainCityQuestGroup)
            }
            "ACTION_TYPE_UNLOCK_HOLLOW_QUEST" => Some(Self::UnlockHollowQuest),
            "ACTION_TYPE_SET_NPC_VISIBLE" => Some(Self::SetNpcVisible),
            "ACTION_TYPE_REMOVE_INTERACT" => Some(Self::RemoveInteract),
            "ACTION_TYPE_REMOVE_NPC" => Some(Self::RemoveNpc),
            "ACTION_TYPE_SET_VHS_STORE_LEVEL" => Some(Self::SetVhsStoreLevel),
            "ACTION_TYPE_SET_VHS_STORE_TREND_STATE" => Some(Self::SetVhsStoreTrendState),
            "ACTION_TYPE_SWITCH_MAIN_CITY_TIME" => Some(Self::SwitchMainCityTime),
            "ACTION_TYPE_THE_WORLD" => Some(Self::TheWorld),
            "ACTION_TYPE_FORCE_REFRESH" => Some(Self::ForceRefresh),
            "ACTION_TYPE_FORBID_AFK" => Some(Self::ForbidAfk),
            "ACTION_TYPE_SWITCH_MAIN_CHARACTER" => Some(Self::SwitchMainCharacter),
            "ACTION_TYPE_SET_LAND_EVENT_FINISH" => Some(Self::SetLandEventFinish),
            "ACTION_TYPE_SET_BGM" => Some(Self::SetBgm),
            "ACTION_TYPE_SET_MAIN_CITY_OBJECT_STATE" => {
                Some(Self::SetMainCityObjectState)
            }
            "ACTION_TYPE_EVENT_CHOICE" => Some(Self::EventChoice),
            "ACTION_TYPE_CREATE_MOVE_NPC" => Some(Self::CreateMoveNpc),
            "ACTION_TYPE_CHANGE_GUIDE_POINT" => Some(Self::ChangeGuidePoint),
            "ACTION_TYPE_ADD_DAILY_QUEST" => Some(Self::AddDailyQuest),
            "ACTION_TYPE_ADD_MICRO_TASK" => Some(Self::AddMicroTask),
            "ACTION_TYPE_SET_FIRST_MEET" => Some(Self::SetFirstMeet),
            "ACTION_TYPE_CREATE_CAMERA_ZONE" => Some(Self::CreateCameraZone),
            "ACTION_TYPE_SET_MAIN_CITY_TIME" => Some(Self::SetMainCityTime),
            "ACTION_TYPE_NEXT_MAIN_CITY_TIME_PERIOD" => {
                Some(Self::NextMainCityTimePeriod)
            }
            "ACTION_TYPE_PLAYER_SWITCH_MAIN_CHARACTER" => {
                Some(Self::PlayerSwitchMainCharacter)
            }
            "ACTION_TYPE_END_TRANSITION" => Some(Self::EndTransition),
            "ACTION_TYPE_ADD_VHS_FLOW_BUFF" => Some(Self::AddVhsFlowBuff),
            "ACTION_TYPE_ACTIVATE_PHOTO_ID" => Some(Self::ActivatePhotoId),
            "ACTION_TYPE_ACCELERATE_MAIN_CITY_TIME" => Some(Self::AccelerateMainCityTime),
            "ACTION_TYPE_SET_TRASH_NEW_FLAG" => Some(Self::SetTrashNewFlag),
            "ACTION_TYPE_USE_LAST_TIME" => Some(Self::UseLastTime),
            "ACTION_TYPE_OCCUPY_OVERNIGHT" => Some(Self::OccupyOvernight),
            "ACTION_TYPE_SHOW_PHOTO_QUEST_FINISH_TIP" => {
                Some(Self::ShowPhotoQuestFinishTip)
            }
            "ACTION_TYPE_ADD_SOUND_AMB" => Some(Self::AddSoundAmb),
            "ACTION_TYPE_SUBMIT_ITEM" => Some(Self::SubmitItem),
            "ACTION_TYPE_MOD_TRUST" => Some(Self::ModTrust),
            "ACTION_TYPE_SET_PARTNER_EVENT_STATE" => Some(Self::SetPartnerEventState),
            "ACTION_TYPE_SEND_MESSAGE" => Some(Self::SendMessage),
            "ACTION_TYPE_SWITCH_TRACK_QUEST" => Some(Self::SwitchTrackQuest),
            "ACTION_TYPE_MOD_NPC" => Some(Self::ModNpc),
            "ACTION_TYPE_ACCEPT_OVERNIGHT" => Some(Self::AcceptOvernight),
            "ACTION_TYPE_ACTIVE_TRIGGER" => Some(Self::ActiveTrigger),
            "ACTION_TYPE_MOD_OBJ_STATE" => Some(Self::ModObjState),
            "ACTION_TYPE_MOD_SCENE_OBJ" => Some(Self::ModSceneObj),
            "ACTION_TYPE_FANS_SETTLE" => Some(Self::FansSettle),
            "ACTION_TYPE_OPEN_HALL_GAME" => Some(Self::OpenHallGame),
            "ACTION_TYPE_ADD_PARTNER_EVENT" => Some(Self::AddPartnerEvent),
            "ACTION_TYPE_EXEC_OVERNIGHT_EVENT" => Some(Self::ExecOvernightEvent),
            "ACTION_TYPE_SOFA_REST_NEXT_TIME_PERIOD" => {
                Some(Self::SofaRestNextTimePeriod)
            }
            "ACTION_TYPE_BEGIN_UI_GAME" => Some(Self::BeginUiGame),
            "ACTION_TYPE_PREPARE_DATA" => Some(Self::PrepareData),
            "ACTION_TYPE_CLEAR_RP_RECOMMEND_RESULT" => Some(Self::ClearRpRecommendResult),
            "ACTION_TYPE_DO_MAIN_CITY_GAME" => Some(Self::DoMainCityGame),
            "ACTION_TYPE_SHOW_IN_TODO" => Some(Self::ShowInTodo),
            "ACTION_TYPE_CHANGE_NPC_NAME" => Some(Self::ChangeNpcName),
            "ACTION_TYPE_CREATE_OVA" => Some(Self::CreateOva),
            "ACTION_TYPE_SET_OVA_STATE" => Some(Self::SetOvaState),
            "ACTION_TYPE_SWITCH_MAIN_CHARACTER_GUISE" => {
                Some(Self::SwitchMainCharacterGuise)
            }
            "ACTION_TYPE_COMPLETE_HALL_GAME" => Some(Self::CompleteHallGame),
            "ACTION_TYPE_HIDE_MAIN_CONTROL_AVATAR" => Some(Self::HideMainControlAvatar),
            "ACTION_TYPE_EAT_RAMEN" => Some(Self::EatRamen),
            "ACTION_TYPE_ONGOING_TIPS" => Some(Self::OngoingTips),
            "ACTION_TYPE_SET_SOUND" => Some(Self::SetSound),
            "ACTION_TYPE_GEN_CAMP_IDLE_DYNAMIC_TEXT_ITEM" => {
                Some(Self::GenCampIdleDynamicTextItem)
            }
            "ACTION_TYPE_MAP_CHOOSE_BY_EVENT" => Some(Self::MapChooseByEvent),
            "ACTION_TYPE_MAP_CHOOSE_BY_LAYER" => Some(Self::MapChooseByLayer),
            "ACTION_TYPE_MAP_CHOOSE_BY_NUM" => Some(Self::MapChooseByNum),
            "ACTION_TYPE_MAP_CHOOSE_BY_RANGE" => Some(Self::MapChooseByRange),
            "ACTION_TYPE_MAP_CLEAR_POOL" => Some(Self::MapClearPool),
            "ACTION_TYPE_MAP_SET_EVENT" => Some(Self::MapSetEvent),
            "ACTION_TYPE_MAP_SET_LAYER" => Some(Self::MapSetLayer),
            "ACTION_TYPE_MAP_SET_TAG" => Some(Self::MapSetTag),
            _ => None,
        }
    }
}

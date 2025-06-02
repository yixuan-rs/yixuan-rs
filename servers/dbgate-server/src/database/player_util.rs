use std::collections::HashMap;

use vivian_proto::server_only::{
    AbyssData, ArchiveData, AvatarData, BattleData, BuddyData, BusinessCardData, GachaData,
    HollowData, ItemData, MainCityData, MapData, MiscData, NewbieData, NewsStandData,
    PlayerAccessoryData, PostGirlData, QuestData, SceneData, SwitchData, TeleportUnlockData,
    UnlockData,
};

pub trait ModelData {
    const TABLE: &str;
    fn create_default(uid: i32) -> Self;
}

impl ModelData for ItemData {
    const TABLE: &str = "t_item_data";

    fn create_default(_uid: i32) -> Self {
        ItemData {
            item_list: Vec::new(),
            weapon_list: Vec::new(),
            equip_list: Vec::new(),
            auto_recovery_item_map: HashMap::new(),
            item_uid_counter: 0,
        }
    }
}

impl ModelData for AvatarData {
    const TABLE: &str = "t_avatar_data";

    fn create_default(_uid: i32) -> Self {
        Self {
            avatar_list: Vec::new(),
        }
    }
}

impl ModelData for QuestData {
    const TABLE: &str = "t_quest_data";

    fn create_default(_uid: i32) -> Self {
        Self {
            quest_collection_list: Vec::new(),
            battle_data: Some(BattleData::default()),
        }
    }
}

impl ModelData for ArchiveData {
    const TABLE: &str = "t_archive_data";

    fn create_default(_uid: i32) -> Self {
        Self {
            hollow_archive_id_list: Vec::new(),
            videotape_list: Vec::new(),
        }
    }
}

impl ModelData for HollowData {
    const TABLE: &str = "t_hollow_data";

    fn create_default(_uid: i32) -> Self {
        Self {
            hollow_group_list: Vec::new(),
            unlock_hollow_group_list: Vec::new(),
            hollow_id_list: Vec::new(),
            unlock_hollow_id_list: Vec::new(),
            hollow_list: Vec::new(),
            executed_hollow_event_id_list: Vec::new(),
        }
    }
}

impl ModelData for AbyssData {
    const TABLE: &str = "t_abyss_data";

    fn create_default(_uid: i32) -> Self {
        Self {}
    }
}

impl ModelData for BuddyData {
    const TABLE: &str = "t_buddy_data";

    fn create_default(_uid: i32) -> Self {
        Self {
            buddy_list: Vec::new(),
        }
    }
}

impl ModelData for MiscData {
    const TABLE: &str = "t_misc_data";

    fn create_default(_uid: i32) -> Self {
        Self {
            switch: Some(SwitchData::default()),
            unlock: Some(UnlockData::default()),
            newbie: Some(NewbieData::default()),
            news_stand: Some(NewsStandData::default()),
            post_girl: Some(PostGirlData::default()),
            teleport: Some(TeleportUnlockData::default()),
            business_card: Some(BusinessCardData::default()),
            player_accessory: Some(PlayerAccessoryData::default()),
        }
    }
}

impl ModelData for MainCityData {
    const TABLE: &str = "t_main_city_data";

    fn create_default(_uid: i32) -> Self {
        MainCityData {
            bgm_id: 0,
            day_of_week: 0,
            time_of_day: 0,
            pos: None,
            time_period: 0,
        }
    }
}

impl ModelData for SceneData {
    const TABLE: &str = "t_scene_data";

    fn create_default(_uid: i32) -> Self {
        SceneData {
            cur_scene_uid: 0,
            default_scene_uid: 0,
            scenes: HashMap::new(),
            dungeons: HashMap::new(),
        }
    }
}

impl ModelData for GachaData {
    const TABLE: &str = "t_gacha_data";

    fn create_default(_uid: i32) -> Self {
        GachaData {
            gacha_statistics_list: Vec::new(),
            gacha_random: 0,
        }
    }
}

impl ModelData for MapData {
    const TABLE: &str = "t_map_data";

    fn create_default(_uid: i32) -> Self {
        MapData {
            area_group_list: Vec::new(),
            area_street_list: Vec::new(),
        }
    }
}

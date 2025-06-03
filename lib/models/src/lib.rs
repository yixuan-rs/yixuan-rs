pub mod property;

mod abyss;
mod archive;
mod avatar;
mod basic;
mod buddy;
mod gacha;
mod hollow;
mod item;
mod main_city;
mod map;
mod misc;
mod quest;
mod scene;

pub use abyss::AbyssModel;
pub use archive::{ArchiveFile, ArchiveModel};
pub use avatar::AvatarModel;
pub use basic::PlayerBasicModel;
pub use buddy::{BuddyItem, BuddyModel};
pub use gacha::{GachaModel, GachaStatistics};
pub use hollow::{Hollow, HollowModel};
pub use item::ItemModel;
pub use main_city::MainCityModel;
pub use map::{AreaGroupInfo, AreaStreetInfo, MapModel};
pub use misc::{
    InputSetting, MiscModel, PlayerAccessory, PlayerSkin, PropertyNewbieData,
    PropertyNewsStandData, PropertyPlayerAccessoryData, PropertyPostGirlData, PropertySwitchData,
    PropertyUnlockData, QuickAccess,
};
pub use quest::{MainCityQuestExt, Quest, QuestCollection, QuestModel};
pub use scene::*;

use yixuan_codegen::Model;
use yixuan_proto::server_only::*;

pub trait Model: Saveable {
    fn is_any_field_changed(&self) -> bool;
    fn reset_changed_fields(&mut self);
}

pub trait Saveable {
    fn save_to_pb(&self, root: &mut PlayerData);
}

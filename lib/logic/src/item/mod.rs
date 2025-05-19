mod avatar;
mod equip;
mod weapon;

pub use avatar::{AvatarItem, EAvatarSkillType};
pub use equip::EquipItem;
use num_enum::{IntoPrimitive, TryFromPrimitive};
pub use weapon::WeaponItem;

#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum EItemType {
    Currency = 1,
    Resource = 2,
    Avatar = 3,
    AvatarPiece = 4,
    Weapon = 5,
    Equip = 7,
    Buddy = 8,
    AvatarLevelUpMaterial = 12,
}

#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum EItemRarity {
    N = 1,
    R = 2,
    SR = 3,
    SSR = 4,
}

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, PartialEq, Eq, Hash, IntoPrimitive)]
#[repr(u32)]
pub enum EQuestStatisticsType {
    EventCount = 2,
    CostTime = 3,
    KilledEnemyCount = 4,
    ArcanaCount = 5,
    TarotCardCount = 6,
    StaminaOverLevelTimes = 7,
    RebornTimes = 8,
    FinishedEventTypeCount = 9,
    FinishedEventIDCount = 10,
    TotalCoinUse = 14,
    BangBooDreamLayer = 99,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum EQuestType {
    ArchiveFile = 1,
    DungeonInner = 2,
    Hollow = 3,
    Manual = 4,
    MainCity = 5,
    HollowChallenge = 6,
    ArchiveBattle = 7,
    Knowledge = 8,
    Daily = 9,
}

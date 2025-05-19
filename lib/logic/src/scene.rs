use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum SceneType {
    Hall = 1,
    Hollow = 2,
    Fight = 3,
    LongFight = 7,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum ELocalPlayType {
    #[default]
    Unknown = 0,
    BigBossBattleLongfight = 217,
    ActivityCombatPause = 230,
    ChessBoardBattle = 202,
    CoinBrushingBattle = 231,
    MiniScapeShortBattle = 229,
    MapChallengeBattle = 291,
    MiniScapeBattle = 228,
    TargetShootingBattle = 294,
    RallyLongFight = 207,
    HadalZoneAlivecount = 222,
    OperationBetaDemo = 216,
    S2RogueBattle = 226,
    BossRushBattle = 218,
    PureHollowBattle = 280,
    ChessBoardLongfihgtBattle = 204,
    TrainingRoom = 290,
    BangbooRoyale = 240,
    PureHollowBattleLonghfight = 281,
    BangbooAutobattle = 295,
    DualElite = 208,
    BossBattle = 210,
    HadalZone = 209,
    BigBossBattle = 211,
    BossNestHardBattle = 220,
    LevelZero = 205,
    HadalZoneBosschallenge = 224,
    PureHollowBattleHardmode = 282,
    ArchiveLongFight = 212,
    GuideSpecial = 203,
    ArchiveBattle = 201,
    BabelTower = 223,
    MpBigBossBattle = 214,
    AvatarDemoTrial = 213,
    BuddyTowerdefenseBattle = 227,
    MechbooBattle = 296,
    DailyChallenge = 206,
    BangbooDreamRogueBattle = 293,
    OperationTeamCoop = 219,
    BossLittleBattleLongfight = 215,
    SideScrollingThegunBattle = 221,
    TrainingRootTactics = 292,
}

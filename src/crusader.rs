use std::cmp;

use dps::*;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CrusaderName {
    // Slot 1
    // TheBushWhacker,
    // RoboRabbit,
    // GrahamTheDriver,
    // WarwickTheWarlock,

    // Slot 2
    JimTheLumberjack,
    // PilotPam,
    VeronicaTheAndroidArcher,
    Arachnobuddy,

    // Slot 3
    EmoWerewolf,
    SallyTheSuccubus,
    // KarenTheCatTeenager,

    // Slot 4
    SashaTheFierceWarrior,
    GroklokTheOrc,
    // MindyTheMime,

    // Slot 5
    TheWashedUpHermit,
    KyleThePartyBro,
    // SerpentKingDraco,
    // HenryTheScaredyGhoul,
    Grandmora,

    // Slot 6
    DetectiveKaine,
    // MisterTheMonkey,
    LarryTheLeprechaun,
    // BernardTheBartender,

    // Slot 7
    ThePrincess,
    // RoboTurkey,
    // RangerRayna,
    BaenarallAngelOfHope,

    // Slot 8
    NatalieDragon,
    // JackOLantern,
    PresidentBillySmithsonian,
    // KarlTheKicker,

    // Slot 9
    JasonMasterOfShadows,
    // PeteTheCarney,
    Broot,
    // PaulThePilgrim,

    // Slot 10
    ArtaxesTheLion,
    DrizzleTheDarkElf,
    // BubbaTheSwimmingOrc,
    SisaronTheDragonSorceress,

    // Slot 11
    // KhouriTheWitchDoctor,
    // MommaKaine,
    // BrogonPrinceOfDragons,
    // TheHalfBloodElf,
    // Foresight,

    // Slot 12
    // DarkGryphon,
    // RockyTheRockstar,
    // MontanaJames,
    // TheDarkHelper,

    // Slot 13
    // SarahTheCollector,
    // TheMetalSoldierette,
    // SnicketteTheSneaky,

    // Slot 14
    // GoldPanda,
    // RoboSanta,
    // LeerionTheRoyalDwarf,
    // KatieTheCupid,

    // Slot 15
    // PrinceSalTheMerman,
    // WendyTheWitch,
    // RobbieRaccoon,
    // PrincessValTheMermaid,

    // Slot 16
    // FirePhoenix,
    // AlanTheArchAngel,
    // FrightOTron4000,
    // Spaceking,

    // Slot 17
    // KingReginaldIV,
    // QueenSiri,
    // MrBogginsTheSubstitute,
    // SquigglesTheClown,

    // Slot 18
    // ThaliaTheThunderKing,
    // FrostyTheSnowman,
    // Littlefoot,
    // CindyTheCheerOrc,

    // Slot 19
    // MerciTheMadWizard,
    // TheBatBillionaire,
    // PetraThePilgrim,

    // Slot 20
    // NateDragon,
    // KizlblypTheAlienTraitor,
    // RoboRudolph,

    // Slot 21
    // TheExterminator,
    // GloriaTheGoodWitch,

    // Slot 22
    // TheShadowQueen,
    // IlsaTheInsaneWizard,
}

impl CrusaderName {
    pub fn slot(&self) -> Slot {
        use self::CrusaderName::*;
        match *self {
            // TheBushWhacker |
            // RoboRabbit |
            // GrahamTheDriver |
            // WarwickTheWarlock => SLOT_1,
            JimTheLumberjack |
            // PilotPam |
            VeronicaTheAndroidArcher |
            Arachnobuddy => SLOT_2,
            EmoWerewolf |
            SallyTheSuccubus => SLOT_3,
            // KarenTheCatTeenager => SLOT_3,
            SashaTheFierceWarrior |
            GroklokTheOrc => SLOT_4,
            // MindyTheMime => SLOT_4,
            TheWashedUpHermit |
            KyleThePartyBro |
            // SerpentKingDraco |
            // HenryTheScaredyGhoul |
            Grandmora => SLOT_5,
            DetectiveKaine |
            // MisterTheMonkey |
            LarryTheLeprechaun => SLOT_6,
            // BernardTheBartender => SLOT_6,
            ThePrincess |
            // RoboTurkey |
            // RangerRayna |
            BaenarallAngelOfHope => SLOT_7,
            NatalieDragon |
            // JackOLantern |
            PresidentBillySmithsonian => SLOT_8,
            // KarlTheKicker => SLOT_8,
            JasonMasterOfShadows |
            // PeteTheCarney |
            Broot => SLOT_9,
            // PaulThePilgrim => SLOT_9,
            ArtaxesTheLion |
            DrizzleTheDarkElf |
            // BubbaTheSwimmingOrc |
            SisaronTheDragonSorceress => SLOT_10,
            // KhouriTheWitchDoctor |
            // MommaKaine |
            // BrogonPrinceOfDragons |
            // TheHalfBloodElf |
            // Foresight => SLOT_11,
            // DarkGryphon |
            // RockyTheRockstar |
            // MontanaJames |
            // TheDarkHelper => SLOT_12,
            // SarahTheCollector |
            // TheMetalSoldierette |
            // SnicketteTheSneaky => SLOT_13,
            // GoldPanda |
            // RoboSanta |
            // LeerionTheRoyalDwarf |
            // KatieTheCupid => SLOT_14,
            // PrinceSalTheMerman |
            // WendyTheWitch |
            // RobbieRaccoon |
            // PrincessValTheMermaid => SLOT_15,
            // FirePhoenix |
            // AlanTheArchAngel |
            // FrightOTronSLOT_4000 |
            // Spaceking => SLOT_16,
            // KingReginaldIV |
            // QueenSiri |
            // MrBogginsTheSubstitute |
            // SquigglesTheClown => SLOT_17,
            // ThaliaTheThunderKing |
            // FrostyTheSnowman |
            // Littlefoot |
            // CindyTheCheerOrc => SLOT_18,
            // MerciTheMadWizard |
            // TheBatBillionaire |
            // PetraThePilgrim => SLOT_19,
            // NateDragon |
            // KizlblypTheAlienTraitor |
            // RoboRudolph => SLOT_20,
            // TheExterminator |
            // GloriaTheGoodWitch => SLOT_21,
            // TheShadowQueen |
            // IlsaTheInsaneWizard => SLOT_22,
        }
    }

    fn base_dps(&self) -> f64 {
        use self::CrusaderName::*;
        match *self {
            // Slot 1
            // TheBushWhacker |
            // RoboRabbit |
            // GrahamTheDriver |
            // WarwickTheWarlock |

            // Slot 2
            JimTheLumberjack |
            // PilotPam |
            VeronicaTheAndroidArcher |
            Arachnobuddy => 6.0,

            // Slot 3
            EmoWerewolf |
            SallyTheSuccubus => 26.4,
            // KarenTheCatTeenager |

            // Slot 4
            SashaTheFierceWarrior |
            GroklokTheOrc => 88.8,
            // MindyTheMime |

            // Slot 5
            TheWashedUpHermit |
            KyleThePartyBro |
            // SerpentKingDraco |
            // HenryTheScaredyGhoul |
            Grandmora => 733.0,

            // Slot 6
            DetectiveKaine |
            // MisterTheMonkey |
            LarryTheLeprechaun => 1464.0,
            // BernardTheBartender |

            // Slot 7
            ThePrincess |
            // RoboTurkey |
            // RangerRayna |
            BaenarallAngelOfHope => 4470.0,

            // Slot 8
            NatalieDragon |
            // JackOLantern |
            PresidentBillySmithsonian => 19545.6,
            // KarlTheKicker |

            // Slot 9
            JasonMasterOfShadows |
            // PeteTheCarney |
            Broot => 56571.6,
            // PaulThePilgrim |

            // Slot 10
            ArtaxesTheLion |
            DrizzleTheDarkElf |
            // BubbaTheSwimmingOrc |
            SisaronTheDragonSorceress => 224302.8,

            // Slot 11
            // KhouriTheWitchDoctor |
            // MommaKaine |
            // BrogonPrinceOfDragons |
            // TheHalfBloodElf |
            // Foresight |

            // Slot 12
            // DarkGryphon |
            // RockyTheRockstar |
            // MontanaJames |
            // TheDarkHelper |

            // Slot 13
            // SarahTheCollector |
            // TheMetalSoldierette |
            // SnicketteTheSneaky |

            // Slot 14
            // GoldPanda |
            // RoboSanta |
            // LeerionTheRoyalDwarf |
            // KatieTheCupid |

            // Slot 15
            // PrinceSalTheMerman |
            // WendyTheWitch |
            // RobbieRaccoon |
            // PrincessValTheMermaid |

            // Slot 16
            // FirePhoenix |
            // AlanTheArchAngel |
            // FrightOTron4000 |
            // Spaceking |

            // Slot 17
            // KingReginaldIV |
            // QueenSiri |
            // MrBogginsTheSubstitute |
            // SquigglesTheClown |

            // Slot 18
            // ThaliaTheThunderKing |
            // FrostyTheSnowman |
            // Littlefoot |
            // CindyTheCheerOrc |

            // Slot 19
            // MerciTheMadWizard |
            // TheBatBillionaire |
            // PetraThePilgrim |

            // Slot 20
            // NateDragon |
            // KizlblypTheAlienTraitor |
            // RoboRudolph |

            // Slot 21
            // TheExterminator |
            // GloriaTheGoodWitch |

            // Slot 22
            // TheShadowQueen |
            // IlsaTheInsaneWizard |
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Crusader {
    pub name: CrusaderName,
    base_dps: Dps,
    level: Level,
}

impl Crusader {
    pub fn new(name: CrusaderName, level: Level) -> Self {
        Crusader {
            name,
            base_dps: Dps(name.base_dps()),
            level,
        }
    }

    pub fn base_dps(&self) -> Dps {
        self.base_dps * self.level
    }

    pub fn slot(&self) -> Slot {
        self.name.slot()
    }
}

bitflags! {
    pub flags Slot: u32 {
        const SLOT_1   = 1,
        const SLOT_2   = 1 << 1,
        const SLOT_3   = 1 << 2,
        const SLOT_4   = 1 << 3,
        const SLOT_5   = 1 << 4,
        const SLOT_6   = 1 << 5,
        const SLOT_7   = 1 << 6,
        const SLOT_8   = 1 << 7,
        const SLOT_9   = 1 << 8,
        const SLOT_10  = 1 << 9,
        const SLOT_11  = 1 << 10,
        const SLOT_12  = 1 << 11,
        const SLOT_13  = 1 << 12,
        const SLOT_14  = 1 << 13,
        const SLOT_15  = 1 << 14,
        const SLOT_16  = 1 << 15,
        const SLOT_17  = 1 << 16,
        const SLOT_18  = 1 << 17,
        const SLOT_19  = 1 << 18,
        const SLOT_20  = 1 << 19,
        const SLOT_21  = 1 << 20,
        const SLOT_22  = 1 << 21,
    }
}

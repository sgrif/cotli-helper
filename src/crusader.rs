use dps::Dps;

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
    // Broot,
    // PaulThePilgrim,

    // Slot 10
    // ArtaxesTheLion,
    // DrizzleTheDarkElf,
    // BubbaTheSwimmingOrc,
    // SisaronTheDragonSorceress,

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
    pub fn slot(&self) -> u8 {
        use self::CrusaderName::*;
        match *self {
            // TheBushWhacker |
            // RoboRabbit |
            // GrahamTheDriver |
            // WarwickTheWarlock => 1,
            JimTheLumberjack |
            // PilotPam |
            VeronicaTheAndroidArcher |
            Arachnobuddy => 2,
            EmoWerewolf |
            SallyTheSuccubus => 3,
            // KarenTheCatTeenager => 3,
            SashaTheFierceWarrior |
            GroklokTheOrc => 4,
            // MindyTheMime => 4,
            TheWashedUpHermit |
            KyleThePartyBro |
            // SerpentKingDraco |
            // HenryTheScaredyGhoul |
            Grandmora => 5,
            DetectiveKaine |
            // MisterTheMonkey |
            LarryTheLeprechaun => 6,
            // BernardTheBartender => 6,
            ThePrincess |
            // RoboTurkey |
            // RangerRayna |
            BaenarallAngelOfHope => 7,
            NatalieDragon |
            // JackOLantern |
            PresidentBillySmithsonian => 8,
            // KarlTheKicker => 8,
            JasonMasterOfShadows => 9,
            // PeteTheCarney |
            // Broot |
            // PaulThePilgrim => 9,
            // ArtaxesTheLion |
            // DrizzleTheDarkElf |
            // BubbaTheSwimmingOrc |
            // SisaronTheDragonSorceress => 10,
            // KhouriTheWitchDoctor |
            // MommaKaine |
            // BrogonPrinceOfDragons |
            // TheHalfBloodElf |
            // Foresight => 11,
            // DarkGryphon |
            // RockyTheRockstar |
            // MontanaJames |
            // TheDarkHelper => 12,
            // SarahTheCollector |
            // TheMetalSoldierette |
            // SnicketteTheSneaky => 13,
            // GoldPanda |
            // RoboSanta |
            // LeerionTheRoyalDwarf |
            // KatieTheCupid => 14,
            // PrinceSalTheMerman |
            // WendyTheWitch |
            // RobbieRaccoon |
            // PrincessValTheMermaid => 15,
            // FirePhoenix |
            // AlanTheArchAngel |
            // FrightOTron4000 |
            // Spaceking => 16,
            // KingReginaldIV |
            // QueenSiri |
            // MrBogginsTheSubstitute |
            // SquigglesTheClown => 17,
            // ThaliaTheThunderKing |
            // FrostyTheSnowman |
            // Littlefoot |
            // CindyTheCheerOrc => 18,
            // MerciTheMadWizard |
            // TheBatBillionaire |
            // PetraThePilgrim => 19,
            // NateDragon |
            // KizlblypTheAlienTraitor |
            // RoboRudolph => 20,
            // TheExterminator |
            // GloriaTheGoodWitch => 21,
            // TheShadowQueen |
            // IlsaTheInsaneWizard => 22,
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
            JasonMasterOfShadows => 56571.6,
            // PeteTheCarney |
            // Broot |
            // PaulThePilgrim |

            // Slot 10
            // ArtaxesTheLion |
            // DrizzleTheDarkElf |
            // BubbaTheSwimmingOrc |
            // SisaronTheDragonSorceress |

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

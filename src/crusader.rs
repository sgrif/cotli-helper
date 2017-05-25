use std::cmp::{Ordering, min};

use aura::*;
use aura::Target::*;
use dps::*;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CrusaderName {
    // Testing only
    #[cfg(any(test, debug_assertions))]
    Dummy(Tags),

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
            #[cfg(any(test, debug_assertions))]
            Dummy(..) => Slot::empty(),
            // TheBushWhacker => SLOT_1,
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
            // Testing only
            #[cfg(any(test, debug_assertions))]
            Dummy(..) => 100.0,

            // Slot 1
            // TheBushWhacker => 0.0,
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

    pub fn tags(&self) -> Tags {
        use self::CrusaderName::*;
        match *self {
            // Testing only
            #[cfg(any(test, debug_assertions))]
            Dummy(tags) => tags,

            // Slot 1
            // TheBushWhacker => MALE | HUMAN | CLICKER,
            // RoboRabbit => MALE | EVENT | ROBOT | SUPPORT | CLICKER,
            // GrahamTheDriver => MALE | HUMAN | DPS | SUPPORT | CLICKER,
            // WarwickTheWarlock => MALE | MAGICAL | EVENT | LEPRECHAUN | CLICKER,

            // Slot 2
            JimTheLumberjack => MALE | HUMAN | DPS | SUPPORT,
            // PilotPam => FEMALE | HUMAN | EVENT | DPS | SUPPORT,
            VeronicaTheAndroidArcher => FEMALE | EVENT | ROBOT | ELF | SUPPORT,
            Arachnobuddy => MALE | HUMAN | EVENT | SUPPORT,

            // Slot 3
            EmoWerewolf => MALE | ANIMAL | SUPERNATURAL | DPS,
            SallyTheSuccubus => FEMALE | SUPERNATURAL | EVENT | DEMON | DPS,
            // KarenTheCatTeenager => FEMALE | ANIMAL | SUPERNATURAL | EVENT | DPS,

            // Slot 4
            SashaTheFierceWarrior => FEMALE | HUMAN | SUPPORT,
            GroklokTheOrc => MALE | EVENT | ORC | TANK | DPS | SUPPORT,
            // MindyTheMime => FEMALE | HUMAN | SUPERNATURAL | EVENT | DPS | SUPPORT,

            // Slot 5
            TheWashedUpHermit => MALE | HUMAN | DPS,
            KyleThePartyBro => MALE | HUMAN | EVENT | DPS,
            // SerpentKingDraco => MALE | ANIMAL | ROYAL | EVENT | DPS,
            // HenryTheScaredyGhoul => MALE | SUPERNATURAL | EVENT | DPS,
            Grandmora => FEMALE | EVENT | SUPPORT | ALIEN,

            // Slot 6
            DetectiveKaine => MALE | HUMAN | GOLD_FINDER,
            // MisterTheMonkey => MALE | ANIMAL | EVENT | GOLD_FINDER,
            LarryTheLeprechaun => MALE | MAGICAL | EVENT | LEPRECHAUN | SUPPORT | GOLD_FINDER,
            // BernardTheBartender => MALE | HUMAN | EVENT | SUPPORT | GOLD_FINDER,

            // Slot 7
            ThePrincess => FEMALE | HUMAN | ROYAL | SUPPORT,
            // RoboTurkey => MALE | EVENT | ROBOT | SUPPORT,
            // RangerRayna => FEMALE | HUMAN | EVENT | DPS | HEALER,
            BaenarallAngelOfHope => FEMALE | SUPERNATURAL | EVENT | ANGEL | SUPPORT,

            // Slot 8
            NatalieDragon => FEMALE | HUMAN | DPS | GOLD_FINDER,
            // JackOLantern => MALE | ANIMAL | SUPERNATURAL | EVENT | TANK | GOLD_FINDER,
            PresidentBillySmithsonian => MALE | HUMAN | EVENT | SUPPORT | GOLD_FINDER,
            // KarlTheKicker => MALE | HUMAN | EVENT | SUPPORT | GOLD_FINDER,

            // Slot 9
            JasonMasterOfShadows => MALE | HUMAN | DPS | GOLD_FINDER,
            // PeteTheCarney => MALE | HUMAN | EVENT | SUPPORT | GOLD_FINDER,
            Broot => MALE | FEMALE | SUPERNATURAL | EVENT | TANK | SUPPORT | GOLD_FINDER,
            // PaulThePilgrim => MALE | EVENT | ELF | TANK | SUPPORT | GOLD_FINDER,

            // Slot 10
            ArtaxesTheLion => MALE | ANIMAL | SUPERNATURAL | SUPPORT,
            DrizzleTheDarkElf => FEMALE | EVENT | ELF | SUPPORT,
            // BubbaTheSwimmingOrc => MALE | EVENT | ORC | SUPPORT | GOLD_FINDER,
            SisaronTheDragonSorceress => FEMALE | ANIMAL | MAGICAL | EVENT | SUPPORT | DRAGON,

            // Slot 11
            // KhouriTheWitchDoctor => MALE | HUMAN | MAGICAL | SUPPORT | HEALER,
            // MommaKaine => FEMALE | HUMAN | EVENT | SUPPORT | HEALER,
            // BrogonPrinceOfDragons => MALE | ANIMAL | ROYAL | EVENT | SUPPORT | HEALER | DRAGON,
            // TheHalfBloodElf => FEMALE | EVENT | ORC | ELF | SUPPORT | HEALER,
            // Foresight => SUPERNATURAL | EVENT | ROBOT | SUPPORT | HEALER,

            // Slot 12
            // DarkGryphon => FEMALE | ANIMAL | SUPERNATURAL | SUPPORT,
            // RockyTheRockstar => MALE | HUMAN | EVENT | DPS,
            // MontanaJames => MALE | HUMAN | EVENT | SUPPORT,
            // TheDarkHelper => FEMALE | EVENT | ELF | SUPPORT | GOLD_FINDER,

            // Slot 13
            // SarahTheCollector => FEMALE | HUMAN | DPS,
            // TheMetalSoldierette => FEMALE | HUMAN | EVENT | ROBOT | TANK | DPS,
            // SnicketteTheSneaky => FEMALE | SUPERNATURAL | EVENT | LEPRECHAUN | SUPPORT,

            // Slot 14
            // GoldPanda => FEMALE | ANIMAL | SUPERNATURAL | GOLD_FINDER,
            // RoboSanta => MALE | EVENT | ROBOT | GOLD_FINDER,
            // LeerionTheRoyalDwarf => MALE | HUMAN | ROYAL | EVENT | GOLD_FINDER | DWARF,
            // KatieTheCupid => FEMALE | SUPERNATURAL | EVENT | SUPPORT | GOLD_FINDER,

            // Slot 15
            // PrinceSalTheMerman => MALE | ANIMAL | ROYAL | DPS,
            // WendyTheWitch => FEMALE | HUMAN | MAGICAL | EVENT | DPS,
            // RobbieRaccoon => MALE | ANIMAL | EVENT | DPS | SUPPORT,
            // PrincessValTheMermaid => FEMALE | ANIMAL | ROYAL | EVENT | SUPPORT | HEALER,

            // Slot 16
            // FirePhoenix => FEMALE | ANIMAL | SUPERNATURAL | SUPPORT,
            // AlanTheArchAngel => MALE | SUPERNATURAL | EVENT | ANGEL | SUPPORT,
            // FrightOTron4000 => FEMALE | EVENT | ROBOT | SUPPORT,
            // Spaceking => MALE | HUMAN | ROYAL | EVENT | DPS,

            // Slot 17
            // KingReginaldIV => MALE | HUMAN | ROYAL | SUPPORT,
            // QueenSiri => FEMALE | HUMAN | ROYAL | EVENT | SUPPORT | GOLD_FINDER,
            // MrBogginsTheSubstitute => MALE | EVENT | LEPRECHAUN | SUPPORT | GOLD_FINDER,
            // SquigglesTheClown => MALE | HUMAN | EVENT | DPS,

            // Slot 18
            // ThaliaTheThunderKing => MALE | HUMAN | ROYAL | MAGICAL | TANK | SUPPORT,
            // FrostyTheSnowman => MALE | ANIMAL | SUPERNATURAL | EVENT | DPS,
            // Littlefoot => FEMALE | ANIMAL | SUPERNATURAL | EVENT | TANK | SUPPORT,
            // CindyTheCheerOrc => FEMALE | EVENT | ORC | SUPPORT,

            // Slot 19
            // MerciTheMadWizard => MALE | HUMAN | MAGICAL | SUPPORT,
            // TheBatBillionaire => MALE | HUMAN | EVENT | SUPPORT,
            // PetraThePilgrim => FEMALE | EVENT | ELF | DPS,

            // Slot 20
            // NateDragon => MALE | HUMAN | DPS | SUPPORT,
            // KizlblypTheAlienTraitor => FEMALE | MAGICAL | EVENT | DPS | SUPPORT | ALIEN,
            // RoboRudolph => MALE | EVENT | ROBOT | DPS,

            // Slot 21
            // TheExterminator => MALE | ROBOT | DPS | GOLD_FINDER,
            // GloriaTheGoodWitch => FEMALE | ANIMAL | MAGICAL | SUPPORT | HEALER,

            // Slot 22
            // TheShadowQueen => FEMALE | HUMAN | ROYAL | SUPERNATURAL | SUPPORT,
            // IlsaTheInsaneWizard => FEMALE | HUMAN | MAGICAL | DPS,
        }
    }

    fn dps_auras(&self) -> Vec<Aura> {
        use self::CrusaderName::*;
        match *self {
            // Testing only
            #[cfg(any(test, debug_assertions))]
            Dummy(..) => vec![],

            // Slot 1
            // TheBushWhacker => vec![],
            // RoboRabbit => vec![],
            // GrahamTheDriver => vec![],
            // WarwickTheWarlock => vec![],

            // Slot 2
            JimTheLumberjack => vec![
                Aura::dps_increase(100.0).for_crusader(*self) // Buddy System
                    .when_exists(AdjacentTo(*self)),
                Aura::dps_increase(100.0).for_crusader(*self), // Chainsaw Kickback
                Aura::dps_increase(50.0) // Sharpen Party
                    .affecting(InSameColumn(*self)),
                Aura::dps_increase(100.0).for_crusader(*self), // Slick Shave
                Aura::dps_increase(150.0).for_crusader(*self), // Institute of Lumberjackology
                // FIXME: Uber Axing
            ],
            // PilotPam => vec![],
            VeronicaTheAndroidArcher => vec![
                Aura::dps_increase(150.0).for_crusader(*self), // Turing Complete
                Aura::dps_increase(50.0) // Precise Aim
                    .affecting(InSameColumn(*self).or(AdjacentTo(*self))) // Line of Sight
                    .plus(Aura::dps_global(25.0).times( // Multicore Processing
                        WithTag(ROBOT).and(!SpecificCrusader(*self))
                    )),
                // FIXME: Fire!
                Aura::dps_global(15.0),
            ],
            Arachnobuddy => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Skitter
                // FIXME: Web Blast
                Aura::dps_increase(150.0).for_crusader(*self), // Scuttle
            ],

            // Slot 3
            EmoWerewolf => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Sad Story
                Aura::dps_increase(200.0).for_crusader(*self) // Lone Wolf
                    .when_none(WithTag(HUMAN).and(AdjacentTo(*self))),
                Aura::dps_increase(100.0).for_crusader(*self), // Fashion Sense
                Aura::dps_increase(100.0).for_crusader(*self), // Teenage Agnst
                Aura::dps_increase(150.0).for_crusader(*self), // Parental Shame
                // FIXME: Whimper at the Moon
            ],
            SallyTheSuccubus => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Enchantress
                Aura::dps_increase(100.0).for_crusader(*self), // Charm
                Aura::dps_increase(300.0).for_crusader(*self) // Seduction
                    .minus(Aura::dps_global(25.0).times(
                        AdjacentTo(*self).and(WithTag(FEMALE))
                    )),
                Aura::dps_increase(100.0).for_crusader(*self), // Crack The Whip
                Aura::dps_increase(100.0).for_crusader(*self), // Absorb Lifeblood
                Aura::dps_increase(20.0).affecting(WithTag(FEMALE)), // Jealousy
            ],
            // KarenTheCatTeenager => vec![],

            // Slot 4
            SashaTheFierceWarrior => vec![
                Aura::dps_increase(30.0).affecting(InColumnBehind(*self)), // Bulwark
                Aura::dps_increase(100.0).for_crusader(*self), // Bad Guy Butt Kicking
                Aura::dps_increase(100.0).for_crusader(*self), // Power of the Warrior
                Aura::dps_increase(100.0).for_crusader(*self), // Soul of the Warrior
                Aura::dps_increase(150.0).for_crusader(*self), // Joan's Jiu-Jitsu
            ],
            GroklokTheOrc => vec![
                Aura::dps_increase(150.0).for_crusader(*self), // Dual-Threat
                Aura::dps_increase(150.0).for_crusader(*self), // Checkdown
                // FIXME: Defensive Team
                Aura::dps_increase(150.0).for_crusader(*self) // Gunslinger
                    .when_exists(EmptySlot),
                Aura::dps_increase(150.0).affecting(InColumnAhead(*self)) // Eligible Receivers
                    .divided_by(InColumnAhead(*self)),
                Aura::dps_increase(150.0).for_crusader(*self), // Fumblerooski
            ],
            // MindyTheMime => vec![],

            // Slot 5
            TheWashedUpHermit => vec![
                Aura::dps_increase(200.0).for_crusader(*self) // Craziness
                    .when_none(InColumnAhead(*self).and(AdjacentTo(*self))),
                Aura::dps_increase(100.0).for_crusader(*self), // Friendly Fisticuff
                Aura::dps_increase(100.0).for_crusader(*self), // Alien Attack
                Aura::dps_increase(100.0).for_crusader(*self), // Attorney Attack
                Aura::dps_increase(100.0).for_crusader(*self), // Dark Warstories
                // FIXME: Arrow Attack
            ],
            KyleThePartyBro => vec![
                Aura::dps_increase(25.0).affecting(AdjacentTo(*self)) // Get Smashed
                    .randomly_affecting(3),
                Aura::dps_increase(50.0).for_crusader(*self) // Mosh Pit
                    .times(AdjacentTo(*self).min(3)),
                Aura::dps_increase(100.0).for_crusader(*self) // Party Animal
                    .when_exists(AdjacentTo(*self).and(WithTag(ANIMAL))),
                Aura::dps_increase(100.0).for_crusader(*self) // Lady's Man
                    .when_exists(AdjacentTo(*self).and(WithTag(FEMALE))),
                Aura::dps_increase(100.0).for_crusader(*self) // Get Lucky
                    .when_exists(AdjacentTo(*self).and(WithTag(LEPRECHAUN))),
                Aura::dps_global(20.0), // Hangover Cure
            ],
            // SerpentKingDraco => vec![],
            // HenryTheScaredyGhoul => vec![],
            Grandmora => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Seen Better Days
                Aura::dps_increase(300.0).affecting(InColumnAhead(*self)) // Still Suspicious
                    .divided_by(InColumnBehind(*self)),
                Aura::dps_global(10.0), // Elder Tech
                Aura::dps_increase(75.0).affecting(InColumnBehind(*self)) // Untrusting
                    .times(InColumnAhead(*self)),
                Aura::dps_global(10.0), // Team Player
            ],

            // Slot 6
            DetectiveKaine => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Detective School
                Aura::dps_increase(100.0).for_crusader(*self), // Abductive Reasoning
                // FIXME: A-Hah!
                Aura::dps_increase(100.0).for_crusader(*self), // Detective Kaine: A P.I.
                Aura::dps_increase(150.0).for_crusader(*self), // Monster Magazine
            ],
            // MisterTheMonkey => vec![],
            LarryTheLeprechaun => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Luck of the Irish
                // FIXME: Little Pockets
                // FIXME: Hiding Spot
                Aura::dps_global(100.0) // Subtle Magics
                    .when(Condition::Lt(AdjacentTo(*self), 4)),
            ],
            // BernardTheBartender => vec![],

            // Slot 7
            ThePrincess => vec![
                Aura::dps_global(10.0), // Ignite
                Aura::dps_global(10.0), // Char
                Aura::dps_global(10.0), // Conflagrate
                Aura::dps_global(10.0), // Incinerate
                // FIXME: Burn Baby Burn!
            ],
            // RoboTurkey => vec![],
            // RangerRayna => vec![],
            BaenarallAngelOfHope => vec![
                Aura::dps_global(10.0), // Warmth
                Aura::dps_global(10.0), // Embolden
                Aura::dps_global(20.0).with_modifier(Modifier::Diversity), // Diversity
                Aura::dps_global(5.0).times(!WithTag(EVENT)), // The Old Guard
            ],

            // Slot 8
            NatalieDragon => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Cloak and Dagger
                Aura::dps_increase(100.0).for_crusader(*self), // Sisterly Love
                Aura::dps_increase(100.0).for_crusader(*self), // The Julius Caesar
                Aura::dps_global(15.0), // Daggerfall
                // FIXME: Double Dragon
                Aura::dps_increase(150.0).for_crusader(*self), // Trophy Hunter
            ],
            // JackOLantern => vec![],
            PresidentBillySmithsonian => vec![
                Aura::dps_global(10.0), // Election Year
                Aura::dps_increase(150.0).for_crusader(*self), // Secret Service
                Aura::dps_global(15.0), // Rousing Speech
                // FIXME: Peace Treaty
                Aura::dps_increase(50.0).affecting(WithTag(HUMAN)), // Us Vs. Them
            ],
            // KarlTheKicker => vec![],

            // Slot 9
            JasonMasterOfShadows => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Night Vision
                Aura::dps_increase(100.0).for_crusader(*self), // Crouching Jason, Hidden Jason
                Aura::dps_increase(100.0).for_crusader(*self), // X-Ray Vision
                Aura::dps_increase(150.0).for_crusader(*self), // Use the Force
                // FIXME: Ambush
            ],
            // PeteTheCarney => vec![],
            Broot => vec![
                // They're all called "I am Broot"
                Aura::dps_increase(100.0).for_crusader(*self),
                Aura::dps_global(10.0),
                Aura::dps_increase(25.0).for_crusader(*self)
                    .when_exists(SpecificCrusader(*self).and(InFrontColumn)),
                // FIXME: Test if this is immediately behind or any column behind
                // Aura::dps_increase(100.0).for_crusader(RobbieRaccoon)
                //     .when_exists(SpecificCrusader(RobbieRaccoon).and(InColumnBehind(*self))),
                // Aura::dps_increase(100.0).for_crusader(RobbieRaccoon)
                //     .when_exists(SpecificCrusader(RobbieRaccoon).and(AdjacentTo(*self))),
            ],
            // PaulThePilgrim => vec![],

            // Slot 10
            ArtaxesTheLion => vec![],
            DrizzleTheDarkElf => vec![],
            // BubbaTheSwimmingOrc => vec![],
            SisaronTheDragonSorceress => vec![],

            // Slot 11
            // KhouriTheWitchDoctor => vec![],
            // MommaKaine => vec![],
            // BrogonPrinceOfDragons => vec![],
            // TheHalfBloodElf => vec![],
            // Foresight => vec![],

            // Slot 12
            // DarkGryphon => vec![],
            // RockyTheRockstar => vec![],
            // MontanaJames => vec![],
            // TheDarkHelper => vec![],

            // Slot 13
            // SarahTheCollector => vec![],
            // TheMetalSoldierette => vec![],
            // SnicketteTheSneaky => vec![],

            // Slot 14
            // GoldPanda => vec![],
            // RoboSanta => vec![],
            // LeerionTheRoyalDwarf => vec![],
            // KatieTheCupid => vec![],

            // Slot 15
            // PrinceSalTheMerman => vec![],
            // WendyTheWitch => vec![],
            // RobbieRaccoon => vec![],
            // PrincessValTheMermaid => vec![],

            // Slot 16
            // FirePhoenix => vec![],
            // AlanTheArchAngel => vec![],
            // FrightOTron4000 => vec![],
            // Spaceking => vec![],

            // Slot 17
            // KingReginaldIV => vec![],
            // QueenSiri => vec![],
            // MrBogginsTheSubstitute => vec![],
            // SquigglesTheClown => vec![],

            // Slot 18
            // ThaliaTheThunderKing => vec![],
            // FrostyTheSnowman => vec![],
            // Littlefoot => vec![],
            // CindyTheCheerOrc => vec![],

            // Slot 19
            // MerciTheMadWizard => vec![],
            // TheBatBillionaire => vec![],
            // PetraThePilgrim => vec![],

            // Slot 20
            // NateDragon => vec![],
            // KizlblypTheAlienTraitor => vec![],
            // RoboRudolph => vec![],

            // Slot 21
            // TheExterminator => vec![],
            // GloriaTheGoodWitch => vec![],

            // Slot 22
            // TheShadowQueen => vec![],
            // IlsaTheInsaneWizard => vec![],
        }
    }

    fn level_at_cost(&self, cost: f64) -> Level {
        use self::CrusaderName::*;
        let base_cost = match *self {
            // Testing only
            #[cfg(any(test, debug_assertions))]
            Dummy(..) => 0.0,

            // Slot 1
            // TheBushWhacker |
            // RoboRabbit |
            // GrahamTheDriver |
            // WarwickTheWarlock |

            // Slot 2
            JimTheLumberjack |
            // PilotPam |
            VeronicaTheAndroidArcher |
            Arachnobuddy => 50.0,

            // Slot 3
            EmoWerewolf |
            SallyTheSuccubus => 250.0,
            // KarenTheCatTeenager |

            // Slot 4
            SashaTheFierceWarrior |
            GroklokTheOrc => 1000.0,
            // MindyTheMime |

            // Slot 5
            TheWashedUpHermit |
            KyleThePartyBro |
            // SerpentKingDraco |
            // HenryTheScaredyGhoul |
            Grandmora => 10_000.0,

            // Slot 6
            DetectiveKaine |
            // MisterTheMonkey |
            LarryTheLeprechaun => 25_000.0,
            // BernardTheBartender |

            // Slot 7
            ThePrincess |
            // RoboTurkey |
            // RangerRayna |
            BaenarallAngelOfHope => 100_000.0,

            // Slot 8
            NatalieDragon |
            // JackOLantern |
            PresidentBillySmithsonian => 600_000.0,
            // KarlTheKicker |

            // Slot 9
            JasonMasterOfShadows |
            // PeteTheCarney |
            Broot => 2.5e6,
            // PaulThePilgrim |

            // Slot 10
            ArtaxesTheLion |
            DrizzleTheDarkElf |
            // BubbaTheSwimmingOrc |
            SisaronTheDragonSorceress => 1.5e7,

            // Slot 11
            // KhouriTheWitchDoctor => 0.0,
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
        };
        let lvl = ((cost * -0.07 / base_cost - 1.0) / -1.0).ln() / 1.07f64.ln();
        Level(lvl as u16)
    }
}

pub struct Crusader {
    pub name: CrusaderName,
    dps_auras: Vec<Aura>,
    base_dps: Dps,
    level: Level,
}

impl PartialEq for Crusader {
    fn eq(&self, other: &Crusader) -> bool {
        self.name == other.name
    }
}

impl Eq for Crusader {}

impl PartialOrd for Crusader {
    fn partial_cmp(&self, other: &Crusader) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Crusader {
    fn cmp(&self, other: &Crusader) -> Ordering {
        self.name.cmp(&other.name)
    }
}

use std::hash::*;
impl Hash for Crusader {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.name.hash(h)
    }
}

use std::fmt;
impl fmt::Debug for Crusader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.name.fmt(f)
    }
}

impl Crusader {
    pub fn new(name: CrusaderName, level: Level, max_cost: Option<f64>) -> Self {
        let level = max_cost
            .map(|max_cost| min(level, name.level_at_cost(max_cost)))
            .unwrap_or(level);
        Crusader {
            name,
            base_dps: Dps(name.base_dps()),
            level,
            dps_auras: name.dps_auras(),
        }
    }

    #[cfg(any(test, debug_assertions))]
    pub fn dummy(tags: Tags) -> Self {
        Crusader::new(CrusaderName::Dummy(tags), Level(1), None)
    }

    pub fn at_level(self, level: u16) -> Self {
        Crusader { level: Level(level), ..self }
    }

    pub fn base_dps(&self) -> Dps {
        self.base_dps * self.level
    }

    pub fn slot(&self) -> Slot {
        self.name.slot()
    }

    pub fn dps_auras(&self) -> &[Aura] {
        &self.dps_auras
    }

    pub fn tags(&self) -> Tags {
        self.name.tags()
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

bitflags! {
    pub flags Tags: u32 {
        const MALE            = 0b000000000000000000000000000000001,
        const FEMALE          = 0b000000000000000000000000000000010,
        const HUMAN           = 0b000000000000000000000000000000100,
        const ANIMAL          = 0b000000000000000000000000000001000,
        const ROYAL           = 0b000000000000000000000000000010000,
        const MAGICAL         = 0b000000000000000000000000000100000,
        const SUPERNATURAL    = 0b000000000000000000000000001000000,
        const EVENT           = 0b000000000000000000000000010000000,
        const ROBOT           = 0b000000000000000000000000100000000,
        const ORC             = 0b000000000000000000000001000000000,
        const ELF             = 0b000000000000000000000010000000000,
        const LEPRECHAUN      = 0b000000000000000000000100000000000,
        const DEMON           = 0b000000000000000000001000000000000,
        const ANGEL           = 0b000000000000000000010000000000000,
        const TANK            = 0b000000000000000000100000000000000,
        const DPS             = 0b000000000000000001000000000000000,
        const SUPPORT         = 0b000000000000000010000000000000000,
        const HEALER          = 0b000000000000000100000000000000000,
        const GOLD_FINDER     = 0b000000000000001000000000000000000,
        const CLICKER         = 0b000000000000010000000000000000000,
        const DWARF           = 0b000000000000100000000000000000000,
        const DRAGON          = 0b000000000001000000000000000000000,
        const ALIEN           = 0b000000000010000000000000000000000,
    }
}

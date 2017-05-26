use std::cmp::{Ordering, min};

use aura::*;
use aura::Target::*;
use dps::*;
use gear::GearQuality;
use user_data::UserData;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CrusaderName {
    // Testing only
    #[cfg(any(test, debug_assertions))]
    Dummy(Tags),

    // Slot 1
    TheBushWhacker,
    RoboRabbit,
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
    KhouriTheWitchDoctor,
    // MommaKaine,
    // BrogonPrinceOfDragons,
    // TheHalfBloodElf,
    Foresight,

    // Slot 12
    DarkGryphon,
    RockyTheRockstar,
    MontanaJames,
    // TheDarkHelper,

    // Slot 13
    SarahTheCollector,
    TheMetalSoldierette,
    SnicketteTheSneaky,

    // Slot 14
    GoldPanda,
    // RoboSanta,
    // LeerionTheRoyalDwarf,
    // KatieTheCupid,

    // Slot 15
    PrinceSalTheMerman,
    // WendyTheWitch,
    RobbieRaccoon,
    // PrincessValTheMermaid,

    // Slot 16
    FirePhoenix,
    AlanTheArchAngel,
    // FrightOTron4000,
    Spaceking,

    // Slot 17
    KingReginaldIV,
    // QueenSiri,
    // MrBogginsTheSubstitute,
    SquigglesTheClown,

    // Slot 18
    ThaliaTheThunderKing,
    // FrostyTheSnowman,
    // Littlefoot,
    // CindyTheCheerOrc,

    // Slot 19
    MerciTheMadWizard,
    TheBatBillionaire,
    // PetraThePilgrim,
    PollyTheParrot,

    // Slot 20
    NateDragon,
    // KizlblypTheAlienTraitor,
    // RoboRudolph,

    // Slot 21
    TheExterminator,
    // GloriaTheGoodWitch,

    // Slot 22
    TheShadowQueen,
    // IlsaTheInsaneWizard,

    // Slot 23
    GreyskullThePirate,
}

impl CrusaderName {
    pub fn slot(&self) -> Slot {
        use self::CrusaderName::*;
        match *self {
            #[cfg(any(test, debug_assertions))]
            Dummy(..) => Slot::empty(),
            TheBushWhacker |
            RoboRabbit => SLOT_1,
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
            KhouriTheWitchDoctor |
            // MommaKaine |
            // BrogonPrinceOfDragons |
            // TheHalfBloodElf |
            Foresight => SLOT_11,
            DarkGryphon |
            RockyTheRockstar |
            MontanaJames => SLOT_12,
            // TheDarkHelper => SLOT_12,
            SarahTheCollector |
            TheMetalSoldierette |
            SnicketteTheSneaky => SLOT_13,
            GoldPanda => SLOT_14,
            // RoboSanta |
            // LeerionTheRoyalDwarf |
            // KatieTheCupid => SLOT_14,
            PrinceSalTheMerman |
            // WendyTheWitch |
            RobbieRaccoon => SLOT_15,
            // PrincessValTheMermaid => SLOT_15,
            FirePhoenix |
            AlanTheArchAngel |
            // FrightOTronSLOT_4000 |
            Spaceking => SLOT_16,
            KingReginaldIV |
            // QueenSiri |
            // MrBogginsTheSubstitute |
            SquigglesTheClown => SLOT_17,
            ThaliaTheThunderKing => SLOT_18,
            // FrostyTheSnowman |
            // Littlefoot |
            // CindyTheCheerOrc => SLOT_18,
            MerciTheMadWizard |
            TheBatBillionaire |
            // PetraThePilgrim |
            PollyTheParrot => SLOT_19,
            NateDragon => SLOT_20,
            // KizlblypTheAlienTraitor |
            // RoboRudolph => SLOT_20,
            TheExterminator => SLOT_21,
            // GloriaTheGoodWitch => SLOT_21,
            TheShadowQueen => SLOT_22,
            // IlsaTheInsaneWizard => SLOT_22,
            GreyskullThePirate => SLOT_23,
        }
    }

    pub fn base_dps(&self) -> f64 {
        use self::CrusaderName::*;
        match *self {
            // Testing only
            #[cfg(any(test, debug_assertions))]
            Dummy(..) => 100.0,

            // Slot 1
            TheBushWhacker |
            RoboRabbit => 0.0,
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
            KhouriTheWitchDoctor |
            // MommaKaine |
            // BrogonPrinceOfDragons |
            // TheHalfBloodElf |
            Foresight => 938000.0,

            // Slot 12
            DarkGryphon |
            RockyTheRockstar |
            MontanaJames => 4.46e6,
            // TheDarkHelper |

            // Slot 13
            SarahTheCollector |
            TheMetalSoldierette |
            SnicketteTheSneaky => 2.04e6,

            // Slot 14
            GoldPanda => 8.33e7,
            // RoboSanta |
            // LeerionTheRoyalDwarf |
            // KatieTheCupid |

            // Slot 15
            PrinceSalTheMerman |
            // WendyTheWitch |
            RobbieRaccoon => 5.52e8,
            // PrincessValTheMermaid |

            // Slot 16
            FirePhoenix |
            AlanTheArchAngel |
            // FrightOTron4000 |
            Spaceking => 3.62e9,

            // Slot 17
            KingReginaldIV |
            // QueenSiri |
            // MrBogginsTheSubstitute |
            SquigglesTheClown => 2.4e10,

            // Slot 18
            ThaliaTheThunderKing => 1.57e11,
            // FrostyTheSnowman |
            // Littlefoot |
            // CindyTheCheerOrc |

            // Slot 19
            MerciTheMadWizard |
            TheBatBillionaire |
            // PetraThePilgrim |
            PollyTheParrot => 9.77e11,

            // Slot 20
            NateDragon => 6.4e12,
            // KizlblypTheAlienTraitor |
            // RoboRudolph |

            // Slot 21
            TheExterminator => 1.89e14,
            // GloriaTheGoodWitch |

            // Slot 22
            TheShadowQueen => 5.56e15,
            // IlsaTheInsaneWizard |

            // Slot 23
            GreyskullThePirate => 1.63e17, // FIXME: This is probably wrong
        }
    }

    pub fn tags(&self) -> Tags {
        use self::CrusaderName::*;
        match *self {
            // Testing only
            #[cfg(any(test, debug_assertions))]
            Dummy(tags) => tags,

            // Slot 1
            TheBushWhacker => MALE | HUMAN | CLICKER,
            RoboRabbit => MALE | EVENT | ROBOT | SUPPORT | CLICKER,
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
            KhouriTheWitchDoctor => MALE | HUMAN | MAGICAL | SUPPORT | HEALER,
            // MommaKaine => FEMALE | HUMAN | EVENT | SUPPORT | HEALER,
            // BrogonPrinceOfDragons => MALE | ANIMAL | ROYAL | EVENT | SUPPORT | HEALER | DRAGON,
            // TheHalfBloodElf => FEMALE | EVENT | ORC | ELF | SUPPORT | HEALER,
            Foresight => SUPERNATURAL | EVENT | ROBOT | SUPPORT | HEALER,

            // Slot 12
            DarkGryphon => FEMALE | ANIMAL | SUPERNATURAL | SUPPORT,
            RockyTheRockstar => MALE | HUMAN | EVENT | DPS,
            MontanaJames => MALE | HUMAN | EVENT | SUPPORT,
            // TheDarkHelper => FEMALE | EVENT | ELF | SUPPORT | GOLD_FINDER,

            // Slot 13
            SarahTheCollector => FEMALE | HUMAN | DPS,
            TheMetalSoldierette => FEMALE | HUMAN | EVENT | ROBOT | TANK | DPS,
            SnicketteTheSneaky => FEMALE | SUPERNATURAL | EVENT | LEPRECHAUN | SUPPORT,

            // Slot 14
            GoldPanda => FEMALE | ANIMAL | SUPERNATURAL | GOLD_FINDER,
            // RoboSanta => MALE | EVENT | ROBOT | GOLD_FINDER,
            // LeerionTheRoyalDwarf => MALE | HUMAN | ROYAL | EVENT | GOLD_FINDER | DWARF,
            // KatieTheCupid => FEMALE | SUPERNATURAL | EVENT | SUPPORT | GOLD_FINDER,

            // Slot 15
            PrinceSalTheMerman => MALE | ANIMAL | ROYAL | DPS,
            // WendyTheWitch => FEMALE | HUMAN | MAGICAL | EVENT | DPS,
            RobbieRaccoon => MALE | ANIMAL | EVENT | DPS | SUPPORT,
            // PrincessValTheMermaid => FEMALE | ANIMAL | ROYAL | EVENT | SUPPORT | HEALER,

            // Slot 16
            FirePhoenix => FEMALE | ANIMAL | SUPERNATURAL | SUPPORT,
            AlanTheArchAngel => MALE | SUPERNATURAL | EVENT | ANGEL | SUPPORT,
            // FrightOTron4000 => FEMALE | EVENT | ROBOT | SUPPORT,
            Spaceking => MALE | HUMAN | ROYAL | EVENT | DPS,

            // Slot 17
            KingReginaldIV => MALE | HUMAN | ROYAL | SUPPORT,
            // QueenSiri => FEMALE | HUMAN | ROYAL | EVENT | SUPPORT | GOLD_FINDER,
            // MrBogginsTheSubstitute => MALE | EVENT | LEPRECHAUN | SUPPORT | GOLD_FINDER,
            SquigglesTheClown => MALE | HUMAN | EVENT | DPS,

            // Slot 18
            ThaliaTheThunderKing => MALE | HUMAN | ROYAL | MAGICAL | TANK | SUPPORT,
            // FrostyTheSnowman => MALE | ANIMAL | SUPERNATURAL | EVENT | DPS,
            // Littlefoot => FEMALE | ANIMAL | SUPERNATURAL | EVENT | TANK | SUPPORT,
            // CindyTheCheerOrc => FEMALE | EVENT | ORC | SUPPORT,

            // Slot 19
            MerciTheMadWizard => MALE | HUMAN | MAGICAL | SUPPORT,
            TheBatBillionaire => MALE | HUMAN | EVENT | SUPPORT,
            // PetraThePilgrim => FEMALE | EVENT | ELF | DPS,
            PollyTheParrot => MALE | ANIMAL | EVENT | SUPPORT,

            // Slot 20
            NateDragon => MALE | HUMAN | DPS | SUPPORT,
            // KizlblypTheAlienTraitor => FEMALE | MAGICAL | EVENT | DPS | SUPPORT | ALIEN,
            // RoboRudolph => MALE | EVENT | ROBOT | DPS,

            // Slot 21
            TheExterminator => MALE | ROBOT | DPS | GOLD_FINDER,
            // GloriaTheGoodWitch => FEMALE | ANIMAL | MAGICAL | SUPPORT | HEALER,

            // Slot 22
            TheShadowQueen => FEMALE | HUMAN | ROYAL | SUPERNATURAL | SUPPORT,
            // IlsaTheInsaneWizard => FEMALE | HUMAN | MAGICAL | DPS,

            // Slot 23
            GreyskullThePirate => MALE | HUMAN | TANK | GOLD_FINDER,
        }
    }

    fn dps_auras(&self) -> Vec<Aura> {
        use self::CrusaderName::*;
        match *self {
            // Testing only
            #[cfg(any(test, debug_assertions))]
            Dummy(..) => vec![],

            // Slot 1
            TheBushWhacker => vec![
                // FIXME: Click damage abilities
            ],
            RoboRabbit => vec![
                // FIXME: Click damage abilities
                Aura::dps_global(25.0), // Wind-up-Bunny
            ],
            // GrahamTheDriver => vec![],
            // WarwickTheWarlock => vec![],

            // Slot 2
            JimTheLumberjack => vec![
                Aura::dps_increase(100.0).for_crusader(*self) // Buddy System
                    .when_exists(AdjacentTo(*self))
                    .with_tag(AuraTag::BuddySystem),
                Aura::dps_increase(100.0).for_crusader(*self), // Chainsaw Kickback
                Aura::dps_increase(50.0) // Sharpen Party
                    .affecting(InSameColumn(*self))
                    .with_tag(AuraTag::SharpenParty),
                Aura::dps_increase(100.0).for_crusader(*self), // Slick Shave
                Aura::dps_increase(150.0).for_crusader(*self), // Institute of Lumberjackology
            ],
            // PilotPam => vec![],
            VeronicaTheAndroidArcher => vec![
                Aura::dps_increase(150.0).for_crusader(*self), // Turing Complete
                Aura::dps_increase(50.0) // Precise Aim
                    .affecting(InSameColumn(*self).or(AdjacentTo(*self))) // Line of Sight
                    .with_tag(AuraTag::PreciseAim)
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
                    .when_none(WithTag(HUMAN).and(AdjacentTo(*self)))
                    .with_tag(AuraTag::LoneWolf),
                Aura::dps_increase(100.0).for_crusader(*self), // Fashion Sense
                Aura::dps_increase(100.0).for_crusader(*self), // Teenage Agnst
                Aura::dps_increase(150.0).for_crusader(*self), // Parental Shame
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
                Aura::dps_increase(30.0).affecting(InColumnBehind(*self)) // Bulwark
                    .with_tag(AuraTag::Bulwark),
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
                    .when_exists(EmptySlot)
                    .with_tag(AuraTag::Gunslinger),
                Aura::dps_increase(150.0).affecting(InColumnAhead(*self)) // Eligible Receivers
                    .divided_by(InColumnAhead(*self))
                    .with_tag(AuraTag::EligibleReceivers),
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
            ],
            KyleThePartyBro => vec![
                Aura::dps_increase(25.0).affecting(AdjacentTo(*self)) // Get Smashed
                    .randomly_affecting(3),
                Aura::dps_increase(50.0).for_crusader(*self) // Mosh Pit
                    .times(AdjacentTo(*self).min(3))
                    .with_tag(AuraTag::MoshPit),
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
                    .divided_by(InColumnBehind(*self))
                    .with_tag(AuraTag::StillSuspicious),
                Aura::dps_global(10.0), // Elder Tech
                Aura::dps_increase(75.0).affecting(InColumnBehind(*self)) // Untrusting
                    .times(InColumnAhead(*self))
                    .with_tag(AuraTag::Untrusting),
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
                Aura::dps_global(10.0).with_tag(AuraTag::Ignite), // Ignite
                Aura::dps_global(10.0).with_tag(AuraTag::Ignite), // Char
                Aura::dps_global(10.0).with_tag(AuraTag::Ignite), // Conflagrate
                Aura::dps_global(10.0).with_tag(AuraTag::Ignite), // Incinerate
            ],
            // RoboTurkey => vec![],
            // RangerRayna => vec![],
            BaenarallAngelOfHope => vec![
                Aura::dps_global(10.0), // Warmth
                Aura::dps_global(10.0), // Embolden
                Aura::dps_global(20.0).with_modifier(Modifier::Diversity) // Diversity
                    .with_tag(AuraTag::Diversity),
                Aura::dps_global(5.0).times(!WithTag(EVENT)) // The Old Guard
                    .with_tag(AuraTag::TheOldGuard),
            ],

            // Slot 8
            NatalieDragon => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Cloak and Dagger
                Aura::dps_increase(100.0).for_crusader(*self), // Sisterly Love
                Aura::dps_increase(100.0).for_crusader(*self), // The Julius Caesar
                Aura::dps_global(15.0), // Daggerfall
                Aura::dps_increase(200.0).for_crusader(*self) // Double Dragon
                    .when_exists(SpecificCrusader(NateDragon)),
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
                Aura::dps_increase(100.0)
                    .affecting(SpecificCrusader(RobbieRaccoon).and(InColumnBehind(*self))),
                Aura::dps_increase(100.0)
                    .affecting(SpecificCrusader(RobbieRaccoon).and(AdjacentTo(*self))),
            ],
            // PaulThePilgrim => vec![],

            // Slot 10
            ArtaxesTheLion => vec![
                Aura::dps_increase(125.0).for_crusader(*self), // Claw Your Way Up
                Aura::dps_increase(50.0).affecting(InColumnAhead(*self)), // Roar!
                Aura::dps_increase(125.0).for_crusader(*self), // Lion's Mane
            ],
            DrizzleTheDarkElf => vec![
                Aura::dps_global(20.0), // All Star
                Aura::dps_increase(20.0).affecting(AdjacentTo(*self)), // Inspiring Presence
                Aura::dps_increase(400.0) // Lateral
                    .affecting(SpecificCrusader(GroklokTheOrc).and(InSameColumn(*self))),
            ],
            // BubbaTheSwimmingOrc => vec![],
            SisaronTheDragonSorceress => vec![
                Aura::dps_increase(150.0).for_crusader(*self), // Swoop
                Aura::dps_increase(100.0).affecting(AdjacentTo(*self)) // Loose Magic
                    .divided_by(AdjacentTo(*self))
                    .with_tag(AuraTag::LooseMagic),
                Aura::dps_global(10.0), // Recovered Magic
            ],

            // Slot 11
            KhouriTheWitchDoctor => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Zombie Kittens
                Aura::dps_increase(30.0).affecting(AdjacentTo(*self)), // Koffee Potion
                Aura::dps_increase(100.0).for_crusader(*self), // Shrunken Heads
                Aura::dps_increase(100.0).for_crusader(*self), // Playing with Dolls
                Aura::dps_increase(100.0).for_crusader(*self), // MLG
            ],
            // MommaKaine => vec![],
            // BrogonPrinceOfDragons => vec![],
            // TheHalfBloodElf => vec![],
            Foresight => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Powering Up
                Aura::dps_increase(150.0).for_crusader(*self), // Full Potential
                Aura::dps_global(20.0), // Global Influence
                Aura::dps_increase(50.0).affecting(WithTag(SUPERNATURAL)), // Supernatural
            ],

            // Slot 12
            DarkGryphon => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Brave Bonk
                Aura::dps_increase(100.0).for_crusader(*self), // Chivalrous Cuff
                Aura::dps_increase(100.0).for_crusader(*self), // Courageous Chop
                Aura::dps_global(15.0), // Heart Brimming Bravery
            ],
            RockyTheRockstar => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Rock 'n Roll
                Aura::dps_increase(50.0).for_crusader(*self) // Groupies + Pick and Choose
                    .times(WithTag(FEMALE)),
                Aura::dps_increase(100.0).for_crusader(*self), // Amp up
                Aura::dps_increase(100.0).for_crusader(*self), // Sold Out Show
            ],
            MontanaJames => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // CRACK
                Aura::dps_increase(50.0).affecting(WithTag(ANIMAL)), // Just In Time
                // FIXME: Turn the tides
                // FIXME: He's Got a Gun, Too
                Aura::dps_global(40.0) // Damsel In Distress
                    .when_exists(SpecificCrusader(ThePrincess)),
            ],
            // TheDarkHelper => vec![],

            // Slot 13
            SarahTheCollector => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Ooh Shiny!
                Aura::dps_increase(150.0).for_crusader(*self) // Full Set
                    .when_none(EmptySlot),
                Aura::dps_increase(100.0).for_crusader(*self), // Collect Them All!
                Aura::dps_increase(100.0).for_crusader(*self), // My Precioussss
                Aura::dps_increase(150.0).for_crusader(*self), // Mine! Mine! Mine!
                // FIXME: Lucky Set
            ],
            TheMetalSoldierette => vec![
                Aura::dps_increase(200.0).for_crusader(*self), // New Paint Job
                Aura::dps_increase(400.0) // Cocky Leader
                    .affecting(SpecificCrusader(*self).and(InFrontColumn)),
                Aura::dps_increase(200.0).for_crusader(*self), // Arms Race
                // FIXME: Cool Under Pressure
            ],
            SnicketteTheSneaky => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Quick Fingers
                Aura::dps_increase(50.0).affecting(AdjacentTo(*self)) // Focused Support
                    .when(Condition::Lt(AdjacentTo(*self), 4)),
                Aura::dps_increase(50.0).affecting(WithTag(HUMAN)) // Favorite Prey
                    .plus(Aura::dps_global(10.0).times(WithTag(HUMAN))),
                // FIXME: Critical Thinking
                Aura::dps_global(50.0) // The Blame Game
                    .when(Condition::Gt(WithTag(LEPRECHAUN), 1)),
            ],

            // Slot 14
            GoldPanda => vec![
                // FIXME: Gold buffs
                // FIXME: Lucky Panda
            ],
            // RoboSanta => vec![],
            // LeerionTheRoyalDwarf => vec![],
            // KatieTheCupid => vec![],

            // Slot 15
            PrinceSalTheMerman => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Mad Trident Skillz
                Aura::dps_global(10.0), // Salt Water Taffy
                Aura::dps_increase(100.0).for_crusader(*self), // Shark Attack
                Aura::dps_increase(150.0).for_crusader(*self), // Triton's Blessing
                Aura::dps_increase(150.0).for_crusader(*self), // 20,000 Leagues
            ],
            // WendyTheWitch => vec![],
            RobbieRaccoon => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Soloist
                Aura::dps_increase(100.0).for_crusader(*self), // Tinkerer
                Aura::dps_increase(150.0).for_crusader(*self), // Acoustics
                Aura::dps_increase(200.0).for_crusader(*self), // Plan B
            ],
            // PrincessValTheMermaid => vec![],

            // Slot 16
            FirePhoenix => vec![
                // FIXME: Vengeful Fury
                Aura::dps_increase(100.0).for_crusader(*self), // Song of the Phoenix
                Aura::dps_increase(100.0).for_crusader(*self), // Evil Eye
                Aura::dps_increase(100.0).for_crusader(*self), // Power of the Fiery Nest
                Aura::dps_increase(150.0).for_crusader(*self), // Egg Laying
            ],
            AlanTheArchAngel => vec![
                Aura::dps_increase(150.0).for_crusader(*self), // Alan, Alan, Alan
                Aura::dps_global(10.0), // Glowing Presence
                // FIXME: Resurrection
                Aura::dps_global(25.0), // Strength of Will
            ],
            // FrightOTron4000 => vec![],
            Spaceking => vec![
                Aura::dps_increase(125.0).for_crusader(*self), // Have You Heard Of Me?
                Aura::dps_increase(25.0).for_crusader(*self) // Ladies' Space-Man
                    .times(WithTag(FEMALE)),
                Aura::dps_increase(125.0).for_crusader(*self), // Pew Pew Pew
                Aura::dps_increase(100.0).for_crusader(*self) // Kirkin' It Up
                    .times(WithTag(ALIEN)),
                Aura::dps_increase(125.0).for_crusader(*self), // Bad-ass Laser Guns
                // FIXME: Avenger
            ],

            // Slot 17
            KingReginaldIV => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Unicorn Academy
                Aura::dps_increase(100.0).for_crusader(*self), // Hunting Trophies
                Aura::dps_global(10.0), // The Royal Army
                Aura::dps_increase(200.0).affecting(WithTag(ROYAL)), // Royal Grail
            ],
            // QueenSiri => vec![],
            // MrBogginsTheSubstitute => vec![],
            SquigglesTheClown => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Hilarious
                Aura::dps_increase(100.0).for_crusader(*self), // Face Pain
                Aura::dps_increase(150.0).for_crusader(*self) // Royal Past
                    .minus(Aura::dps_global(25.0).times(WithTag(ROYAL))),
                Aura::dps_increase(150.0).for_crusader(*self), // Tummy Heart
                Aura::dps_increase(10.0).for_crusader(*self) // Warming Up
                    .times(!WithTag(ROYAL)),
                Aura::dps_increase(200.0).for_crusader(*self) // Receptive Audience
                    .when(Condition::Gt(
                        WithTag(HUMAN).and(AdjacentTo(*self)),
                        1,
                    )),
            ],

            // Slot 18
            ThaliaTheThunderKing => vec![
                Aura::dps_increase(125.0).for_crusader(*self), // Lightning Bolted
                Aura::dps_increase(125.0).for_crusader(*self), // Forked Lightning
                Aura::dps_global(15.0), // Lightning Ore
                // FIXME: Storm Rider
            ],
            // FrostyTheSnowman => vec![],
            // Littlefoot => vec![],
            // CindyTheCheerOrc => vec![],

            // Slot 19
            MerciTheMadWizard => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Sorry!
                Aura::dps_global(15.0), // Zombie Friends?
                // FIXME: Deflect Evil
                Aura::dps_increase(100.0).for_crusader(*self), // Lotus Land
                Aura::dps_increase(150.0).for_crusader(*self), // All Apologies
            ],
            TheBatBillionaire => vec![
                Aura::dps_global(20.0), // The Bat Signal
                Aura::dps_global(20.0), // Bat-o-Level
                Aura::dps_increase(50.0).affecting(AdjacentTo(*self)), // Sidekicks
                // FIXME: Smart Investing
            ],
            // PetraThePilgrim => vec![],
            PollyTheParrot => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Peck
                // FIXME: Got a Cracker! (maybe don't care)
                // FIXME: Instant Regret
                Aura::dps_global(20.0), // Flit and Flutter
                Aura::dps_global(50.0).times(WithTag(TANK)), // Tough Nut to Crack
            ],

            // Slot 20
            NateDragon => vec![
                Aura::dps_increase(200.0).for_crusader(*self) // Double Dragon
                    .when_exists(SpecificCrusader(NatalieDragon)),
                Aura::dps_increase(100.0).for_crusader(*self), // Forest Fire
                Aura::dps_global(10.0), // Dynamite!!!
                Aura::dps_global(10.0), // Desecration
                Aura::dps_increase(150.0).for_crusader(*self), // Unexpected Explosion
            ],
            // KizlblypTheAlienTraitor => vec![],
            // RoboRudolph => vec![],

            // Slot 21
            TheExterminator => vec![
                Aura::dps_increase(100.0).for_crusader(*self), // Give Me Your Jacket
                Aura::dps_increase(50.0).for_crusader(*self) // Spare Parts
                    .times(WithTag(ROBOT).and(!SpecificCrusader(*self))),
                Aura::dps_increase(100.0).for_crusader(*self) // Slaved Systems
                    .times(WithTag(ROBOT).and(AdjacentTo(*self))),
                // FIXME: Triangulation
                Aura::dps_increase(200.0).for_crusader(*self), // Time Travelled
                // FIXME: Thumb's Up
            ],
            // GloriaTheGoodWitch => vec![],

            // Slot 22
            TheShadowQueen => vec![
                Aura::dps_increase(150.0).for_crusader(*self), // The Shadow's Strike
                Aura::dps_increase(300.0).affecting(AdjacentTo(*self)) // The Shadow's Cowl
                    .divided_by(AdjacentTo(*self))
                    .with_tag(AuraTag::TheShadowsCowl),
                Aura::dps_increase(250.0).for_crusader(*self), // The Shadow's Grasp
                Aura::dps_global(50.0), // All In Shadow
            ],
            // IlsaTheInsaneWizard => vec![],

            // Slot 23
            GreyskullThePirate => vec![
                Aura::dps_increase(200.0).for_crusader(*self), // Pirate's Cunning
                // FIXME: Plunder
                Aura::dps_global(10.0), // Pay Me Crew
                // FIXME: Greyskull's Handcannon
            ],
        }
    }

    fn dps_auras_from_gear(&self, gear: &[GearQuality; 3]) -> Vec<Aura> {
        use self::CrusaderName::*;

        let dps_self = |gear| {
            let multiplier = match gear {
                GearQuality::None => 0.0,
                GearQuality::Common => 25.0,
                GearQuality::Uncommon => 50.0,
                GearQuality::Rare => 100.0,
                GearQuality::Epic => 400.0,
                GearQuality::GoldenEpic => 600.0,
                GearQuality::Legendary(_) => 800.0,
                GearQuality::GoldenLegendary(_) => 1200.0,
            };
            Aura::dps_increase(multiplier).for_crusader(*self)
        };
        let dps_all = |gear| {
            let multiplier = match gear {
                GearQuality::None => 0.0,
                GearQuality::Common => 5.0,
                GearQuality::Uncommon => 10.0,
                GearQuality::Rare => 15.0,
                GearQuality::Epic => 40.0,
                GearQuality::GoldenEpic => 60.0,
                GearQuality::Legendary(_) => 80.0,
                GearQuality::GoldenLegendary(_) => 120.0,
            };
            Aura::dps_global(multiplier)
        };
        let legendary_effect = |base: f64, gear: GearQuality| {
            let multiplier = gear.legendary_level()
                .map(|lvl| 2u16.pow(lvl as u32 - 1) as f64)
                .unwrap_or(0.0);
            Aura::dps_increase(base * multiplier)
        };

        match *self {
            // Testing only
            #[cfg(any(test, debug_assertions))]
            Dummy(..) => vec![],

            // Slot 1
            TheBushWhacker => vec![],
            RoboRabbit => vec![],
            // GrahamTheDriver => vec![],
            // WarwickTheWarlock => vec![],

            // Slot 2
            JimTheLumberjack => vec![
                // Axe
                dps_self(gear[0]),
                // Cap
                dps_all(gear[1]),
                // Gloves
                dps_self(gear[2]),
                legendary_effect(100.0, gear[2])
                    .affecting(AllCrusaders)
                    .when_exists(SpecificCrusader(SashaTheFierceWarrior)),
            ],
            // PilotPam => vec![],
            VeronicaTheAndroidArcher => vec![
                // Bow
                dps_all(gear[1]),
                legendary_effect(25.0, gear[1])
                    .affecting(AllCrusaders)
                    .times(WithTag(ROBOT)),
                // Quiver
                legendary_effect(50.0, gear[2])
                    .affecting(AllCrusaders)
                    .times(WithTag(ELF)),
            ],
            Arachnobuddy => vec![
                // Suit
                legendary_effect(100.0, gear[1])
                    .affecting(AllCrusaders)
                    .when_exists(SpecificCrusader(TheMetalSoldierette)),
                // Mask
                dps_all(gear[2]),
                legendary_effect(25.0, gear[2])
                    .affecting(AllCrusaders)
                    .times(WithTag(SUPERNATURAL)),
            ],

            // Slot 3
            EmoWerewolf => vec![
                // Crystal
                dps_self(gear[0]),
                // Cape
                dps_self(gear[1]),
                legendary_effect(25.0, gear[1]).for_crusader(*self)
                    .times(AdjacentTo(*self)),
                // Ham
                dps_all(gear[2]),
                legendary_effect(100.0, gear[2])
                    .affecting(WithTag(ANIMAL)),
            ],
            SallyTheSuccubus => vec![
                // Whip
                dps_self(gear[0]),
                legendary_effect(20.0, gear[0]).for_crusader(*self)
                    .times(WithTag(MALE)),
                // Corset
                dps_self(gear[1]),
                legendary_effect(20.0, gear[1]).for_crusader(*self)
                    .times(!WithTag(SUPERNATURAL)),
                // Perfume
                dps_all(gear[2]),
                legendary_effect(100.0, gear[2]).for_crusader(*self)
                    .when(Condition::Gt(AdjacentTo(*self), 3)),
            ],
            // KarenTheCatTeenager => vec![],

            // Slot 4
            SashaTheFierceWarrior => vec![
                // Gloves
                dps_all(gear[0]),
                // Shield
                dps_self(gear[1]),
                legendary_effect(50.0, gear[1])
                    .affecting(AllCrusaders)
                    .times(WithTag(TANK)),
                // Helmet
                legendary_effect(33.0, gear[2])
                    .affecting(AllCrusaders)
                    .times(InColumnBehind(*self)),
            ],
            GroklokTheOrc => vec![
                // Football
                dps_self(gear[2]),
            ],
            // MindyTheMime => vec![],

            // Slot 5
            TheWashedUpHermit => vec![
                // Backpack
                dps_self(gear[0]),
                legendary_effect(100.0, gear[0]).for_crusader(*self)
                    .times(EmptySlot),
                // Hat
                dps_self(gear[1]),
                legendary_effect(100.0, gear[1]).affecting(WithTag(HUMAN)),
                // Ring
                dps_all(gear[2]),
                legendary_effect(25.0, gear[2]).for_crusader(*self)
                    .times(WithTag(SUPERNATURAL)),
            ],
            KyleThePartyBro => vec![
                // Hat
                dps_all(gear[0]),
                // Shirt
                dps_self(gear[1]),
                legendary_effect(100.0, gear[1]).for_crusader(*self)
                    .when(Condition::Gt(AdjacentTo(*self), 3)),
                // Keg
                dps_self(gear[2]),
                legendary_effect(100.0, gear[2]).for_crusader(*self)
                    .when(Condition::GtComplex(Behind(*self), AheadOf(*self))),
            ],
            // SerpentKingDraco => vec![],
            // HenryTheScaredyGhoul => vec![],
            Grandmora => vec![
                // Glasses
                legendary_effect(100.0, gear[0])
                    .affecting(AllCrusaders)
                    .when(Condition::Gt(WithTag(ALIEN), 1)),
                // Knitting
                legendary_effect(100.0, gear[1]).affecting(WithTag(HUMAN)),
                // Chair
                dps_all(gear[2]),
                legendary_effect(100.0, gear[2])
                    .affecting(AdjacentTo(*self)),
            ],

            // Slot 6
            DetectiveKaine => vec![
                // Hat
                dps_all(gear[0]),
                // FIXME: Hat legendary (gold)
                // FIXME: Magnifier legendary (gold + xp points)
                legendary_effect(100.0, gear[2])
                    .affecting(AllCrusaders)
                    .when_exists(SpecificCrusader(NateDragon)),
            ],
            // MisterTheMonkey => vec![],
            LarryTheLeprechaun => vec![
                // Cane
                dps_all(gear[1]),
                legendary_effect(50.0, gear[1])
                    .affecting(AllCrusaders)
                    .times(WithTag(LEPRECHAUN)),
                // Pot
                legendary_effect(25.0, gear[2])
                    .affecting(AllCrusaders)
                    .times(WithTag(MAGICAL)),
            ],
            // BernardTheBartender => vec![],

            // Slot 7
            ThePrincess => vec![
                // Gloves
                legendary_effect(25.0, gear[0])
                    .affecting(AllCrusaders)
                    .times(WithTag(ROYAL)),
                // Cape
                legendary_effect(100.0, gear[1])
                    .affecting(AllCrusaders)
                    .when_exists(SpecificCrusader(KingReginaldIV)),
                // Necklace
                dps_all(gear[2]),
                legendary_effect(100.0, gear[2])
                    .affecting(!WithTag(EVENT)),
            ],
            // RoboTurkey => vec![],
            // RangerRayna => vec![],
            BaenarallAngelOfHope => vec![
                // FIXME: Mace legendary (gold)
                // Shield
                legendary_effect(100.0, gear[1])
                    .affecting(AllCrusaders)
                    .when_exists(SpecificCrusader(AlanTheArchAngel)),
                // Breastplate
                dps_all(gear[2]),
            ],

            // Slot 8
            NatalieDragon => vec![],
            // JackOLantern => vec![],
            PresidentBillySmithsonian => vec![],
            // KarlTheKicker => vec![],

            // Slot 9
            JasonMasterOfShadows => vec![],
            // PeteTheCarney => vec![],
            Broot => vec![],
            // PaulThePilgrim => vec![],

            // Slot 10
            ArtaxesTheLion => vec![],
            DrizzleTheDarkElf => vec![],
            // BubbaTheSwimmingOrc => vec![],
            SisaronTheDragonSorceress => vec![],

            // Slot 11
            KhouriTheWitchDoctor => vec![],
            // MommaKaine => vec![],
            // BrogonPrinceOfDragons => vec![],
            // TheHalfBloodElf => vec![],
            Foresight => vec![],

            // Slot 12
            DarkGryphon => vec![],
            RockyTheRockstar => vec![],
            MontanaJames => vec![],
            // TheDarkHelper => vec![],

            // Slot 13
            SarahTheCollector => vec![],
            TheMetalSoldierette => vec![],
            SnicketteTheSneaky => vec![],

            // Slot 14
            GoldPanda => vec![],
            // RoboSanta => vec![],
            // LeerionTheRoyalDwarf => vec![],
            // KatieTheCupid => vec![],

            // Slot 15
            PrinceSalTheMerman => vec![],
            // WendyTheWitch => vec![],
            RobbieRaccoon => vec![],
            // PrincessValTheMermaid => vec![],

            // Slot 16
            FirePhoenix => vec![],
            AlanTheArchAngel => vec![],
            // FrightOTron4000 => vec![],
            Spaceking => vec![],

            // Slot 17
            KingReginaldIV => vec![],
            // QueenSiri => vec![],
            // MrBogginsTheSubstitute => vec![],
            SquigglesTheClown => vec![],

            // Slot 18
            ThaliaTheThunderKing => vec![],
            // FrostyTheSnowman => vec![],
            // Littlefoot => vec![],
            // CindyTheCheerOrc => vec![],

            // Slot 19
            MerciTheMadWizard => vec![],
            TheBatBillionaire => vec![],
            // PetraThePilgrim => vec![],
            PollyTheParrot => vec![],

            // Slot 20
            NateDragon => vec![],
            // KizlblypTheAlienTraitor => vec![],
            // RoboRudolph => vec![],

            // Slot 21
            TheExterminator => vec![],
            // GloriaTheGoodWitch => vec![],

            // Slot 22
            TheShadowQueen => vec![],
            // IlsaTheInsaneWizard => vec![],

            // Slot 23
            GreyskullThePirate => vec![],
        }
    }

    fn ability_buffs(&self) -> Vec<AbilityBuff> {
        use self::CrusaderName::*;
        match *self {
            JimTheLumberjack => vec![
                AbilityBuff::new(20.0, AuraTag::Swordplay), // Uber Axing
            ],
            EmoWerewolf => vec![
                AbilityBuff::new(20.0, AuraTag::Swordplay), // Whimper at the Moon
            ],
            TheWashedUpHermit => vec![
                AbilityBuff::new(20.0, AuraTag::Swordplay), // Arrow Attack
            ],
            ThePrincess => vec![
                AbilityBuff::new(20.0, AuraTag::Swordplay), // Burn Baby Burn!
            ],
            ArtaxesTheLion => vec![
                AbilityBuff::new(20.0, AuraTag::Swordplay), // Jungle Speed
            ],
            DrizzleTheDarkElf => vec![
                AbilityBuff::new(100.0, AuraTag::EligibleReceivers) // Running Play
                    .when_exists(SpecificCrusader(GroklokTheOrc).and(Behind(*self))),
            ],
            SisaronTheDragonSorceress => vec![
                AbilityBuff::new(300.0, AuraTag::LooseMagic) // Focused Magic
                    .when(Condition::Gt(AdjacentTo(*self), 3)),
            ],
            DarkGryphon => vec![
                AbilityBuff::new(20.0, AuraTag::Swordplay), // Lion Swipe
            ],
            PrinceSalTheMerman => vec![
                AbilityBuff::new(20.0, AuraTag::Swordplay), // Neptune's Wrath
            ],
            TheShadowQueen => vec![
                AbilityBuff::new(100.0, AuraTag::TheShadowsCowl) // The Shadow Mastered
                    .when_exists(SpecificCrusader(JasonMasterOfShadows).and(AdjacentTo(*self))),
            ],
            _ => vec![],
        }
    }

    fn ability_buffs_from_gear(&self, gear: &[GearQuality; 3]) -> Vec<AbilityBuff> {
        use self::CrusaderName::*;
        use self::AuraTag::*;

        let ability_mod = |tag, gear| {
            let multiplier = match gear {
                GearQuality::None => 0.0,
                GearQuality::Common => 10.0,
                GearQuality::Uncommon => 25.0,
                GearQuality::Rare => 50.0,
                GearQuality::Epic => 100.0,
                GearQuality::GoldenEpic => 150.0,
                GearQuality::Legendary(_) => 200.0,
                GearQuality::GoldenLegendary(_) => 300.0,
            };
            AbilityBuff::new(multiplier, tag)
        };
        let legendary_ability_mod = |tag, gear: GearQuality| {
            let multiplier = gear.legendary_level()
                .map(|lvl| 100.0 * 2u16.pow(lvl as u32 - 1) as f64)
                .unwrap_or(0.0);
            AbilityBuff::new(multiplier, tag)
        };

        match *self {
            // Testing only
            #[cfg(any(test, debug_assertions))]
            Dummy(..) => vec![],

            // Slot 1
            TheBushWhacker => vec![],
            RoboRabbit => vec![],
            // GrahamTheDriver => vec![],
            // WarwickTheWarlock => vec![],

            // Slot 2
            JimTheLumberjack => vec![
                // Axe
                legendary_ability_mod(SharpenParty, gear[0]),
                // Cap
                legendary_ability_mod(BuddySystem, gear[1]),
            ],
            // PilotPam => vec![],
            VeronicaTheAndroidArcher => vec![
                // Armguard
                ability_mod(PreciseAim, gear[0]),
                legendary_ability_mod(PreciseAim, gear[0]),
                // Quiver
                ability_mod(Fire, gear[2]),
            ],
            Arachnobuddy => vec![
                // FIXME: Web
                legendary_ability_mod(WebBlast, gear[0]),
                // FIXME: Suit
            ],

            // Slot 3
            EmoWerewolf => vec![
                legendary_ability_mod(LoneWolf, gear[0]),
            ],
            SallyTheSuccubus => vec![],
            // KarenTheCatTeenager => vec![],

            // Slot 4
            SashaTheFierceWarrior => vec![
                // Gloves
                legendary_ability_mod(Bulwark, gear[0]),
                // Helm
                ability_mod(Bulwark, gear[2]),
            ],
            GroklokTheOrc => vec![
                // Tooth
                ability_mod(EligibleReceivers, gear[0]),
                // Bracer
                legendary_ability_mod(Gunslinger, gear[1]),
                // Football
                legendary_ability_mod(EligibleReceivers, gear[2]),
            ],
            // MindyTheMime => vec![],

            // Slot 5
            TheWashedUpHermit => vec![],
            KyleThePartyBro => vec![
                // Hat
                legendary_ability_mod(MoshPit, gear[0]),
            ],
            // SerpentKingDraco => vec![],
            // HenryTheScaredyGhoul => vec![],
            Grandmora => vec![
                ability_mod(StillSuspicious, gear[0]),
                ability_mod(Untrusting, gear[1]),
            ],

            // Slot 6
            DetectiveKaine => vec![
                ability_mod(AHah, gear[2]),
            ],
            // MisterTheMonkey => vec![],
            LarryTheLeprechaun => vec![
                // Hat
                legendary_ability_mod(LittlePockets, gear[0]),
                // Pot
                ability_mod(LittlePockets, gear[2]),
            ],
            // BernardTheBartender => vec![],

            // Slot 7
            ThePrincess => vec![
                // Gloves
                ability_mod(Ignite, gear[0]),
            ],
            // RoboTurkey => vec![],
            // RangerRayna => vec![],
            BaenarallAngelOfHope => vec![
                // Mace
                ability_mod(AncientHatred, gear[0]),
                // Shield
                ability_mod(Diversity, gear[1]),
                // Breastplate
                legendary_ability_mod(TheOldGuard, gear[2]),
            ],

            // Slot 8
            NatalieDragon => vec![],
            // JackOLantern => vec![],
            PresidentBillySmithsonian => vec![],
            // KarlTheKicker => vec![],

            // Slot 9
            JasonMasterOfShadows => vec![],
            // PeteTheCarney => vec![],
            Broot => vec![],
            // PaulThePilgrim => vec![],

            // Slot 10
            ArtaxesTheLion => vec![],
            DrizzleTheDarkElf => vec![],
            // BubbaTheSwimmingOrc => vec![],
            SisaronTheDragonSorceress => vec![],

            // Slot 11
            KhouriTheWitchDoctor => vec![],
            // MommaKaine => vec![],
            // BrogonPrinceOfDragons => vec![],
            // TheHalfBloodElf => vec![],
            Foresight => vec![],

            // Slot 12
            DarkGryphon => vec![],
            RockyTheRockstar => vec![],
            MontanaJames => vec![],
            // TheDarkHelper => vec![],

            // Slot 13
            SarahTheCollector => vec![],
            TheMetalSoldierette => vec![],
            SnicketteTheSneaky => vec![],

            // Slot 14
            GoldPanda => vec![],
            // RoboSanta => vec![],
            // LeerionTheRoyalDwarf => vec![],
            // KatieTheCupid => vec![],

            // Slot 15
            PrinceSalTheMerman => vec![],
            // WendyTheWitch => vec![],
            RobbieRaccoon => vec![],
            // PrincessValTheMermaid => vec![],

            // Slot 16
            FirePhoenix => vec![],
            AlanTheArchAngel => vec![],
            // FrightOTron4000 => vec![],
            Spaceking => vec![],

            // Slot 17
            KingReginaldIV => vec![],
            // QueenSiri => vec![],
            // MrBogginsTheSubstitute => vec![],
            SquigglesTheClown => vec![],

            // Slot 18
            ThaliaTheThunderKing => vec![],
            // FrostyTheSnowman => vec![],
            // Littlefoot => vec![],
            // CindyTheCheerOrc => vec![],

            // Slot 19
            MerciTheMadWizard => vec![],
            TheBatBillionaire => vec![],
            // PetraThePilgrim => vec![],
            PollyTheParrot => vec![],

            // Slot 20
            NateDragon => vec![],
            // KizlblypTheAlienTraitor => vec![],
            // RoboRudolph => vec![],

            // Slot 21
            TheExterminator => vec![],
            // GloriaTheGoodWitch => vec![],

            // Slot 22
            TheShadowQueen => vec![],
            // IlsaTheInsaneWizard => vec![],

            // Slot 23
            GreyskullThePirate => vec![],
        }

    }

    fn level_at_cost(&self, cost: f64) -> Level {
        use self::CrusaderName::*;
        let base_cost = match *self {
            // Testing only
            #[cfg(any(test, debug_assertions))]
            Dummy(..) => 0.0,

            // Slot 1
            TheBushWhacker |
            RoboRabbit => 5.0,
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
            KhouriTheWitchDoctor |
            // MommaKaine |
            // BrogonPrinceOfDragons |
            // TheHalfBloodElf |
            Foresight => 1e8,

            // Slot 12
            DarkGryphon |
            RockyTheRockstar |
            MontanaJames => 8e8,
            // TheDarkHelper |

            // Slot 13
            SarahTheCollector |
            TheMetalSoldierette |
            SnicketteTheSneaky => 6.5e9,

            // Slot 14
            GoldPanda => 5e10,
            // RoboSanta |
            // LeerionTheRoyalDwarf |
            // KatieTheCupid |

            // Slot 15
            PrinceSalTheMerman |
            // WendyTheWitch |
            RobbieRaccoon => 4.50e11,
            // PrincessValTheMermaid |

            // Slot 16
            FirePhoenix |
            AlanTheArchAngel |
            // FrightOTron4000 |
            Spaceking => 4e12,

            // Slot 17
            KingReginaldIV |
            // QueenSiri |
            // MrBogginsTheSubstitute |
            SquigglesTheClown => 3.6e13,

            // Slot 18
            ThaliaTheThunderKing => 3.2e14,
            // FrostyTheSnowman |
            // Littlefoot |
            // CindyTheCheerOrc |

            // Slot 19
            MerciTheMadWizard |
            TheBatBillionaire |
            // PetraThePilgrim |
            PollyTheParrot => 2.7e15,

            // Slot 20
            NateDragon => 2.4e16,
            // KizlblypTheAlienTraitor |
            // RoboRudolph |

            // Slot 21
            TheExterminator => 9.6e17,
            // GloriaTheGoodWitch |

            // Slot 22
            TheShadowQueen => 3.84e19,
            // IlsaTheInsaneWizard |

            // Slot 23
            GreyskullThePirate => 1.53e21,
        };
        let lvl = ((cost * -0.07 / base_cost - 1.0) / -1.0).ln() / 1.07f64.ln();
        Level(lvl as u16)
    }
}

pub struct Crusader {
    pub name: CrusaderName,
    dps_auras: Vec<Aura>,
    ability_buffs: Vec<AbilityBuff>,
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
    pub fn new(name: CrusaderName, user_data: &UserData, max_cost: Option<f64>) -> Self {
        let max_level = user_data.max_level();
        let level = max_cost
            .map(|max_cost| min(max_level, name.level_at_cost(max_cost)))
            .unwrap_or(max_level);
        let gear = user_data.gear_for(name);
        let mut dps_auras = name.dps_auras();
        let mut ability_buffs = name.ability_buffs();
        if let Some(gear) = gear {
            dps_auras.extend(name.dps_auras_from_gear(gear));
            ability_buffs.extend(name.ability_buffs_from_gear(gear));
        }
        Crusader {
            name,
            base_dps: user_data.base_dps_for_crusader(name),
            level,
            dps_auras,
            ability_buffs,
        }
    }

    #[cfg(any(test, debug_assertions))]
    pub fn dummy(tags: Tags) -> Self {
        Crusader::new(CrusaderName::Dummy(tags), &Default::default(), None)
            .at_level(1)
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

    pub fn ability_buffs(&self) -> &[AbilityBuff] {
        &self.ability_buffs
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
        const SLOT_23  = 1 << 22,
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

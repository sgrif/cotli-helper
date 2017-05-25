use std::cmp::Ordering;

use aura::*;
use aura::Target::*;
use dps::*;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CrusaderName {
    // Slot 1
    TheBushWhacker,
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
            TheBushWhacker => SLOT_1,
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
            TheBushWhacker => 0.0,
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
            // Slot 1
            TheBushWhacker => MALE | HUMAN | CLICKER,
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
            // Slot 1
            TheBushWhacker => vec![],
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
            VeronicaTheAndroidArcher => vec![],
            Arachnobuddy => vec![],

            // Slot 3
            EmoWerewolf => vec![],
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
            SashaTheFierceWarrior => vec![],
            GroklokTheOrc => vec![],
            // MindyTheMime => vec![],

            // Slot 5
            TheWashedUpHermit => vec![],
            KyleThePartyBro => vec![],
            // SerpentKingDraco => vec![],
            // HenryTheScaredyGhoul => vec![],
            Grandmora => vec![],

            // Slot 6
            DetectiveKaine => vec![],
            // MisterTheMonkey => vec![],
            LarryTheLeprechaun => vec![],
            // BernardTheBartender => vec![],

            // Slot 7
            ThePrincess => vec![],
            // RoboTurkey => vec![],
            // RangerRayna => vec![],
            BaenarallAngelOfHope => vec![],

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
    pub fn new(name: CrusaderName, level: Level) -> Self {
        Crusader {
            name,
            base_dps: Dps(name.base_dps()),
            level,
            dps_auras: name.dps_auras(),
        }
    }

    pub fn at_level(self, level: Level) -> Self {
        Crusader { level, ..self }
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

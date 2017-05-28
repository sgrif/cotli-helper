use toml;

use crusader::CrusaderName;
use gear::*;
use super::{UserData, CrusaderData};
use talent::Talent;

pub fn from_toml(data: &str) -> Result<UserData, toml::de::Error> {
    let data = toml::from_str::<UserDataToml>(data)?;
    let mut result = UserData::default()
        .with_dps_from_rings(data.dps_from_rings)
        .with_cooldown_percent(data.cooldown_percent)
        .with_dps_from_achievements(data.dps_from_achievements)
        .with_unspent_idols(data.unspent_idols);
    for talent in &data.talents {
        result = result.level_talent(talent.talent(), talent.level);
    }
    for data in &data.crusaders {
        if let Some(name) = data.crusader() {
            result = result.add_crusader(name, data.data());
        }
    }
    Ok(result)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct UserDataToml {
    #[serde(default)]
    dps_from_rings: f64,
    #[serde(default)]
    cooldown_percent: f64,
    #[serde(default)]
    dps_from_achievements: f64,
    #[serde(default)]
    unspent_idols: u64,
    #[serde(default)]
    talents: Vec<TalentFromToml>,
    #[serde(default)]
    crusaders: Vec<CrusaderFromToml>
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TalentFromToml {
    name: String,
    level: u16,
}

impl TalentFromToml {
    fn talent(&self) -> Talent {
        use self::Talent::*;
        match &*self.name {
            // Active Tree
            // Tier 1
            "Time-o-rama" => TimeORama,
            "Massive Criticals" => MassiveCriticals,
            "Speed Runner" => SpeedRunner,

            // Tier 2
            "Endurance Training" => EnduranceTraining,
            "Ride the Storm" => RideTheStorm,
            "Storm's Building" => StormsBuilding,

            // Tier 3
            "Gold-o-splosion" => GoldOSplosion,
            "Sniper" => Sniper,

            // Passive Tree
            // Tier 1
            "Passive Criticals" => PassiveCriticals,
            "Surplus Cooldown" => SurplusCooldown,
            "Every Last Cent" => EveryLastCent,

            // Tier 2
            "Overenchanted" => Overenchanted,
            "Set Bonus" => SetBonus,
            "Sharing is Caring" => SharingIsCaring,

            // Tier 3
            "Fast Learners" => FastLearners,
            "Well Equipped" => WellEquipped,
            "Swap Day" => SwapDay,

            // Utility Tree
            // Tier 1
            "Spend It All" => SpendItAll,
            "Upgrade Them All" => UpgradeThemAll,
            "Scavenger" => Scavenger,

            // Tier 2
            "Speed Looter" => SpeedLooter,
            "Efficient Crusading" => EfficientCrusading,
            "Doing It Again" => DoingItAgain,

            // Tier 3
            "Deep Idol Scavenger" => DeepIdolScavenger,
            "Extra Training" => ExtraTraining,
            "Triple Tier Trouble" => TripleTierTrouble,

            _ => panic!("Unrecognized talent {}", self.name),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CrusaderFromToml {
    name: String,
    #[serde(default)]
    enchantment_points: u32,
    #[serde(default)]
    gear: [GearQuality; 3],
}

impl CrusaderFromToml {
    fn crusader(&self) -> Option<CrusaderName> {
        use self::CrusaderName::*;
        match &*self.name {
            // Slot 1
            "The Bush Whacker" => Some(TheBushWhacker),
            "RoboRabbit" => Some(RoboRabbit),
            // Some(GrahamTheDriver),
            // Some(WarwickTheWarlock),

            // Slot 2
            "Jim the Lumberjack" => Some(JimTheLumberjack),
            // Some(PilotPam),
            "Veronica, the Android Archer" => Some(VeronicaTheAndroidArcher),
            "Arachnobuddy" => Some(Arachnobuddy),

            // Slot 3
            "Emo Werewolf" => Some(EmoWerewolf),
            "Sally the Succubus" => Some(SallyTheSuccubus),
            // Some(KarenTheCatTeenager),

            // Slot 4
            "Sasha the Fierce Warrior" => Some(SashaTheFierceWarrior),
            "Groklok the Orc" => Some(GroklokTheOrc),
            // Some(MindyTheMime),

            // Slot 5
            "The Washed Up Hermit" => Some(TheWashedUpHermit),
            "Kyle the Party Bro" => Some(KyleThePartyBro),
            // "Serpent King Draco" => Some(SerpentKingDraco),
            // Some(HenryTheScaredyGhoul),
            "Grandmora" => Some(Grandmora),

            // Slot 6
            "Detective Kaine" => Some(DetectiveKaine),
            // Some(MisterTheMonkey),
            "Larry the Leprechaun" => Some(LarryTheLeprechaun),
            // Some(BernardTheBartender),

            // Slot 7
            "The Princess" => Some(ThePrincess),
            // Some(RoboTurkey),
            // Some(RangerRayna),
            "Baenarall, Angel of Hope" => Some(BaenarallAngelOfHope),

            // Slot 8
            "Natalie Dragon" => Some(NatalieDragon),
            // Some(JackOLantern),
            "President Billy Smithsonian" => Some(PresidentBillySmithsonian),
            // Some(KarlTheKicker),

            // Slot 9
            "Jason, Master of Shadows" => Some(JasonMasterOfShadows),
            // Some(PeteTheCarney),
            "Broot" => Some(Broot),
            // Some(PaulThePilgrim),

            // Slot 10
            "Artaxes, the Lion" => Some(ArtaxesTheLion),
            "Drizzle the Dark Elf" => Some(DrizzleTheDarkElf),
            // Some(BubbaTheSwimmingOrc),
            "Sisaron the Dragon Sorceress" => Some(SisaronTheDragonSorceress),

            // Slot 11
            "Khouri, the Witch Doctor" => Some(KhouriTheWitchDoctor),
            // Some(MommaKaine),
            // Some(BrogonPrinceOfDragons),
            // Some(TheHalfBloodElf),
            "Foresight" => Some(Foresight),

            // Slot 12
            "Dark Gryphon" => Some(DarkGryphon),
            "Rocky the Rockstar" => Some(RockyTheRockstar),
            "Montana James" => Some(MontanaJames),
            // Some(TheDarkHelper),

            // Slot 13
            "Sarah, the Collector" => Some(SarahTheCollector),
            "The Metal Soldierette" => Some(TheMetalSoldierette),
            "Snickette the Sneaky" => Some(SnicketteTheSneaky),

            // Slot 14
            "Gold Panda" => Some(GoldPanda),
            // Some(RoboSanta),
            // Some(LeerionTheRoyalDwarf),
            // Some(KatieTheCupid),

            // Slot 15
            "Prince Sal, the Merman" => Some(PrinceSalTheMerman),
            // Some(WendyTheWitch),
            "Robbie Raccoon" => Some(RobbieRaccoon),
            // Some(PrincessValTheMermaid),

            // Slot 16
            "Fire Phoenix" => Some(FirePhoenix),
            "Alan the ArchAngel" => Some(AlanTheArchAngel),
            // Some(FrightOTron4000),
            "Spaceking" => Some(Spaceking),

            // Slot 17
            "King Reginald IV" => Some(KingReginaldIV),
            // Some(QueenSiri),
            // Some(MrBogginsTheSubstitute),
            "Squiggles the Clown" => Some(SquigglesTheClown),

            // Slot 18
            "Thalia, the Thunder King" => Some(ThaliaTheThunderKing),
            // Some(FrostyTheSnowman),
            // Some(Littlefoot),
            // Some(CindyTheCheerOrc),

            // Slot 19
            "Merci, the Mad Wizard" => Some(MerciTheMadWizard),
            "The Bat Billionaire" => Some(TheBatBillionaire),
            // Some(PetraThePilgrim),
            "Polly the Parrot" => Some(PollyTheParrot),

            // Slot 20
            "Nate Dragon" => Some(NateDragon),
            // Some(KizlblypTheAlienTraitor),
            // Some(RoboRudolph),

            // Slot 21
            "The Exterminator" => Some(TheExterminator),
            // Some(GloriaTheGoodWitch),

            // Slot 22
            "The Shadow Queen" => Some(TheShadowQueen),
            // Some(IlsaTheInsaneWizard),

            // Slot 23
            "Greyskull the Pirate" => Some(GreyskullThePirate),
            // FIXME: stderr?
            _ => {
                println!("Unrecognized crusader name: {}", self.name);
                println!("Either you have mistyped the name, or that crusader \
                    is not yet supported. They will not be considered for placement");
                None
            }
        }
    }

    fn data(&self) -> CrusaderData {
        CrusaderData {
            enchantment_points: self.enchantment_points,
            gear: self.gear,
        }
    }
}

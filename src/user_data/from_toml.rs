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
        CrusaderName::from_str(&self.name).or_else(|| {
            println!("Unrecognized crusader name: {}", self.name);
            println!("Either you have mistyped the name, or that crusader \
                is not yet supported. They will not be considered for placement");
            None
        })
    }

    fn data(&self) -> CrusaderData {
        CrusaderData {
            enchantment_points: self.enchantment_points,
            gear: self.gear,
        }
    }
}

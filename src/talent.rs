use dps::Level;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Talent {
    // Active Tree
    // Tier 1
    TimeORama,
    MassiveCriticals,
    SpeedRunner,

    // Tier 2
    EnduranceTraining,
    RideTheStorm,
    StormsBuilding,

    // Tier 3
    GoldOSplosion,
    Sniper,

    // Passive Tree
    // Tier 1
    PassiveCriticals,
    SurplusCooldown,
    EveryLastCent,

    // Tier 2
    Overenchanted,
    SetBonus,
    SharingIsCaring,

    // Tier 3
    FastLearners,
    WellEquipped,
    SwapDay,

    // Utility Tree
    // Tier 1
    SpendItAll,
    UpgradeThemAll,
    Scavenger,

    // Tier 2
    SpeedLooter,
    EfficientCrusading,
    DoingItAgain,

    // Tier 3
    DeepIdolScavenger,
    ExtraTraining,
    TripleTierTrouble,
}

impl Talent {
    pub fn total_cost_at_level(&self, Level(lvl): Level) -> u64 {
        // We do this in a loop rather than the normal
        // (cost * (1-coef^lvl) / (1-coef)
        // because we need to round up at each step
        let mut total = 0;
        let base = self.base_cost();
        let coef = self.cost_multiplier();
        let mut lvl = lvl as i32;
        while lvl != 0 {
            total += (base * coef.powi(lvl-1)).ceil() as u64;
            lvl -= 1
        }
        total
    }

    fn base_cost(&self) -> f64 {
        use self::Talent::*;

        match *self {
            // Active Tree
            // Tier 1
            TimeORama => 25.0,
            MassiveCriticals => 50.0,
            SpeedRunner => 25.0,

            // Tier 2
            EnduranceTraining => 50.0,
            RideTheStorm => 100.0,
            StormsBuilding => 100.0,

            // Tier 3
            GoldOSplosion => 500.0,
            Sniper => 200.0,

            // Passive Tree
            // Tier 1
            PassiveCriticals => 10.0,
            SurplusCooldown => 25.0,
            EveryLastCent => 50.0,

            // Tier 2
            Overenchanted |
            SetBonus => 100.0,
            SharingIsCaring => 500.0,

            // Tier 3
            FastLearners => 250.0,
            WellEquipped |
            SwapDay => 500.0,

            // Utility Tree
            // Tier 1
            SpendItAll => 25.0,
            UpgradeThemAll => 50.0,
            Scavenger => 25.0,

            // Tier 2
            SpeedLooter => 100.0,
            EfficientCrusading => 50.0,
            DoingItAgain => 1000.0,

            // Tier 3
            DeepIdolScavenger => 500.0,
            ExtraTraining => 1000.0,
            TripleTierTrouble => 5000.0,
        }
    }

    fn cost_multiplier(&self) -> f64 {
        use self::Talent::*;

        match *self {
            // Active Tree
            // Tier 1
            TimeORama |
            MassiveCriticals |
            SpeedRunner => 1.25,

            // Tier 2
            EnduranceTraining => 1.25,
            RideTheStorm => 1.15,
            StormsBuilding => 1.33,

            // Tier 3
            GoldOSplosion => 1.15,
            Sniper => 1.1,

            // Passive Tree
            // Tier 1
            PassiveCriticals |
            SurplusCooldown => 1.1,
            EveryLastCent => 1.25,

            // Tier 2
            Overenchanted |
            SetBonus => 1.1,
            SharingIsCaring => 1.25,

            // Tier 3
            FastLearners => 1.2,
            WellEquipped |
            SwapDay => 1.075,

            // Utility Tree
            // Tier 1
            SpendItAll => 1.0,
            UpgradeThemAll => 1.0,
            Scavenger => 1.1,

            // Tier 2
            SpeedLooter => 1.0,
            EfficientCrusading => 1.1,
            DoingItAgain => 1.0,

            // Tier 3
            DeepIdolScavenger => 1.15,
            ExtraTraining => 1.075,
            TripleTierTrouble => 1.0,
        }
    }
}

pub const BONUS_PER_PASSIVE_CRITICALS_LEVEL: f64 = 1.0;
pub const BONUS_PER_SURPLUS_COOLDOWN_LEVEL: f64 = 0.25;
pub const BONUS_PER_OVERENCHANTED_LEVEL: f64 = 5.0;
pub const BONUS_PER_SET_BONUS_LEVEL: f64 = 20.0;
pub const BONUS_PER_SHARING_IS_CARING_LEVEL: f64 = 5.0;
pub const BONUS_PER_WELL_EQUIPPED_LEVEL: f64 = 20.0;
pub const BONUS_PER_SWAP_DAY_LEVEL: f64 = 20.0;


use std::collections::HashMap;
use std::hash::*;

use dps::Level;
use talent::*;
use talent::Talent::*;

#[derive(Default)]
pub struct TalentData {
    data: HashMap<Talent, Level>,
}

impl Hash for TalentData {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        let mut data = self.data.iter().collect::<Vec<_>>();
        data.sort_by_key(|&(k, _)| k);
        data.hash(hasher);
    }
}

const BASE_EP_FRACTION_FROM_BENCH: f64 = 0.3;
const BASE_DPS_PERCENT_PER_EP: f64 = 25.0;
const BASE_MAX_LEVEL: u16 = 5000;

impl TalentData {
    pub fn level_talent(&mut self, talent: Talent, level: u16) {
        self.data.insert(talent, Level(level));
    }

    pub fn get_level(&self, talent: &Talent) -> u16 {
        self.data.get(talent)
            .map(|&Level(lvl)| lvl)
            .unwrap_or(0)
    }

    pub fn spent_idols(&self) -> u64 {
        self.data.iter()
            .fold(0, |init, (talent, level)| {
                init + talent.total_cost_at_level(*level)
            })
    }

    pub fn ep_fraction_transfered_from_bench(&self) -> f64 {
        let ep_fraction_bonus = BONUS_PER_SHARING_IS_CARING_LEVEL / 100.0;
        let bonus_from_talents = ep_fraction_bonus * self.get_level(&SharingIsCaring) as f64;
        BASE_EP_FRACTION_FROM_BENCH + bonus_from_talents
    }

    pub fn dps_percent_per_ep(&self) -> f64 {
        let bonus_from_talents = BONUS_PER_OVERENCHANTED_LEVEL
            * self.get_level(&Overenchanted) as f64;
        BASE_DPS_PERCENT_PER_EP + bonus_from_talents
    }

    pub fn dps_percent_per_crit_chance(&self) -> f64 {
        BONUS_PER_PASSIVE_CRITICALS_LEVEL * self.get_level(&PassiveCriticals) as f64
    }

    pub fn dps_perent_per_cooldown_percent(&self) -> f64 {
        BONUS_PER_SURPLUS_COOLDOWN_LEVEL * self.get_level(&SurplusCooldown) as f64
    }

    pub fn dps_percent_from_set_bonus(&self) -> f64 {
        BONUS_PER_SET_BONUS_LEVEL * self.get_level(&SetBonus) as f64
    }

    pub fn dps_percent_from_well_equipped(&self) -> f64 {
        BONUS_PER_WELL_EQUIPPED_LEVEL * self.get_level(&WellEquipped) as f64
    }

    pub fn dps_percent_from_swap_day(&self) -> f64 {
        BONUS_PER_SWAP_DAY_LEVEL * self.get_level(&SwapDay) as f64
    }

    pub fn max_level(&self) -> Level {
        let bonus_levels = BONUS_PER_EXTRA_TRAINING_LEVEL
            * self.get_level(&ExtraTraining);
        Level(BASE_MAX_LEVEL + bonus_levels)
    }
}

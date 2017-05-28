#![allow(dead_code)]

mod crusader_data;
mod from_toml;
mod talent_data;

pub use self::crusader_data::CrusaderData;
pub use self::from_toml::from_toml;

use crusader::*;
use dps::*;
use gear::GearQuality;
use self::crusader_data::AllCrusaderData;
use self::talent_data::TalentData;
use talent::*;

#[derive(Default)]
pub struct UserData {
    dps_from_rings: f64,
    cooldown_percent: f64,
    dps_from_achievements: f64,
    unspent_idols: u64,
    talents: TalentData,
    crusader_data: AllCrusaderData,
}

const DPS_PERCENT_PER_IDOL: u64 = 3;
const _GOLD_PERCENT_PER_IDOL: u64 = 1;
const MAX_COOLDOWN: f64 = 50.0;

impl UserData {
    pub fn with_dps_from_rings(self, dps_from_rings: f64) -> Self {
        UserData { dps_from_rings, ..self }
    }

    pub fn with_cooldown_percent(self, cooldown_percent: f64) -> Self {
        UserData { cooldown_percent, ..self }
    }

    pub fn with_dps_from_achievements(self, dps_from_achievements: f64) -> Self {
        UserData { dps_from_achievements, ..self }
    }

    pub fn with_unspent_idols(self, unspent_idols: u64) -> Self {
        UserData { unspent_idols, ..self }
    }

    pub fn level_talent(mut self, talent: Talent, level: u16) -> Self {
        self.talents.level_talent(talent, level);
        self
    }

    pub fn add_crusader(mut self, crusader: CrusaderName, data: CrusaderData) -> Self {
        self.crusader_data.add_crusader(crusader, data);
        self
    }

    pub fn unlocked_crusaders(&self, gold: Option<f64>) -> Vec<Crusader> {
        self.crusader_data.unlocked_crusaders()
            .cloned()
            .map(|name| Crusader::new(name, self, gold))
            .collect()
    }

    pub fn max_level(&self) -> Level {
        self.talents.max_level()
    }

    pub fn base_dps_for_crusader(&self, crusader: CrusaderName) -> Dps {
        let mut dps = Dps(crusader.base_dps())
            .percent_increase(self.dps_percent_from_idols())
            .percent_increase(self.dps_from_rings)
            .percent_increase(self.dps_from_achievements)
            .percent_increase(self.dps_percent_from_cooldown())
            .percent_increase(self.ep_for_crusader(&crusader) as f64
                * self.talents.dps_percent_per_ep());
        if let Some(data) = self.crusader_data.get(&crusader) {
            if data.has_full_set() {
                dps = dps.percent_increase(self.talents.dps_percent_from_set_bonus());
            }
            let multiplier = self.talents.dps_percent_from_well_equipped();
            dps = dps.percent_increase(data.num_epics() as f64 * multiplier);
            let num_epics = self.crusader_data.epics_in_same_slot(&crusader);
            let multiplier = self.talents.dps_percent_from_swap_day();
            dps = dps.percent_increase(num_epics as f64 * multiplier);
        }
        dps
    }

    pub fn gear_for(&self, crusader: CrusaderName) -> Option<&[GearQuality; 3]> {
        self.crusader_data.get(&crusader).map(|d| &d.gear)
    }

    fn dps_percent_from_idols(&self) -> f64 {
        let total_idols = self.unspent_idols + self.talents.spent_idols();
        (DPS_PERCENT_PER_IDOL * total_idols) as f64
    }

    fn dps_percent_from_cooldown(&self) -> f64 {
        (self.cooldown_percent - MAX_COOLDOWN)
            * self.talents.dps_perent_per_cooldown_percent()
    }

    fn ep_for_crusader(&self, crusader: &CrusaderName) -> u32 {
        let ep_on_main = self.crusader_data.ep_for_crusader(crusader);
        let modifier = self.talents.ep_fraction_transfered_from_bench();
        self.crusader_data.crusaders_in_same_slot(crusader)
            .fold(ep_on_main, |init, data| {
                init + (data.enchantment_points as f64 * modifier).round() as u32
            })
    }
}

mod crusader_data;
mod talent_data;

pub use self::crusader_data::CrusaderData;

use crusader::*;
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

    pub fn unlocked_crusaders(&self) -> Vec<Crusader> {
        self.crusader_data.unlocked_crusaders()
            .cloned()
            .map(|name| Crusader::new(name, self.talents.max_level()))
            .collect()
    }

    // pub fn auras(&self) -> Vec<Aura> {
    //     let mut auras = vec![
    //         Aura::dps_increase(self.dps_percent_from_idols()),
    //         Aura::dps_increase(self.dps_from_rings),
    //         Aura::dps_increase(self.dps_from_achievements),
    //         // FIXME: This needs to be multiplied times crit percent
    //         Aura::dps_increase(self.talents.dps_percent_per_crit_chance()),
    //         Aura::dps_increase(
    //             (self.cooldown_percent - MAX_COOLDOWN)
    //                 * self.talents.dps_perent_per_cooldown_percent()
    //         ),
    //     ];
    //     auras.extend(self.crusader_data.unlocked_crusaders()
    //         .map(move |name| {
    //             let dps_bonus = self.ep_for_crusader(name) as f64
    //                 * self.talents.dps_percent_per_ep();
    //             Aura::dps_increase(dps_bonus).for_crusader(*name)
    //         }));
    //     // FIXME: This needs to only include gear on crusaders in the formation
    //     auras.extend(self.crusader_data.iter()
    //         .flat_map(|(name, data)| {
    //             GearQuality::auras_for(*name, &data.gear)
    //         }));
    //     auras.extend(self.crusader_data.iter()
    //         .filter(|&(_, data)| data.has_full_set())
    //         .map(move |(name, _)| {
    //             Aura::dps_increase(self.talents.dps_percent_from_set_bonus())
    //                 .for_crusader(*name)
    //         }));
    //     auras.extend(self.crusader_data.iter()
    //         .map(move |(name, data)| {
    //             let multiplier = self.talents.dps_percent_from_well_equipped();
    //             Aura::dps_increase(data.num_epics() as f64 * multiplier).for_crusader(*name)
    //         }));
    //     auras.extend(self.crusader_data.unlocked_crusaders()
    //         .map(move |name| {
    //             let num_epics = self.crusader_data.epics_in_same_slot(name);
    //             let multiplier = self.talents.dps_percent_from_swap_day();
    //             Aura::dps_increase(num_epics as f64 * multiplier).for_crusader(*name)
    //         }));
    //     auras
    // }

    fn dps_percent_from_idols(&self) -> f64 {
        let total_idols = self.unspent_idols + self.talents.spent_idols();
        (DPS_PERCENT_PER_IDOL * total_idols) as f64
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

use std::collections::HashMap;

use crusader::CrusaderName;
use gear::GearQuality;

#[derive(Default)]
pub struct AllCrusaderData {
    data: HashMap<CrusaderName, CrusaderData>
}

impl AllCrusaderData {
    pub fn add_crusader(&mut self, name: CrusaderName, data: CrusaderData) {
        self.data.insert(name, data);
    }

    pub fn unlocked_crusaders<'a>(&'a self) -> impl Iterator<Item=&'a CrusaderName> {
        self.data.keys()
    }

    pub fn get(&self, name: &CrusaderName) -> Option<&CrusaderData> {
        self.data.get(name)
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item=(&'a CrusaderName, &'a CrusaderData)> {
        self.data.iter()
    }

    pub fn crusaders_in_same_slot<'a>(&'a self, name: &CrusaderName)
        -> impl Iterator<Item=&'a CrusaderData>
    {
        let name = *name;
        self.data.iter()
            .filter(move |&(c, _)| c.slot() == name.slot())
            .filter(move |&(c, _)| c != &name)
            .map(|(_, data)| data)
    }

    pub fn ep_for_crusader(&self, name: &CrusaderName) -> u32 {
        self.data.get(name)
            .map(|d| d.enchantment_points)
            .unwrap_or(0)
    }

    pub fn epics_in_same_slot(&self, name: &CrusaderName) -> u8 {
        self.crusaders_in_same_slot(name)
            .fold(0, |init, data| init + data.num_epics())
    }
}

#[derive(Default)]
pub struct CrusaderData {
    pub enchantment_points: u32,
    pub gear: [GearQuality; 3],
}

impl CrusaderData {
    pub fn num_epics(&self) -> u8 {
        self.gear.iter()
            .filter(|gear| **gear >= GearQuality::Epic)
            .count() as u8
    }

    pub fn has_full_set(&self) -> bool {
        self.gear.iter()
            .all(|gear| *gear != GearQuality::None)
    }
}

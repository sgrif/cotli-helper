mod coordinate;

pub use self::coordinate::Coordinate;

use std::cell::Cell;

use aura::{Aura, AbilityBuff, AuraTag};
use crusader::*;
use dps::*;
use formation_search::SearchPolicy;

#[derive(Debug, Clone)]
pub struct Formation<'a> {
    positions: Box<[FormationPosition<'a>]>,
    used_slots: Slot,
    dps: Cell<Option<Dps>>,
}

impl<'a> Formation<'a> {
    pub fn empty<I: IntoIterator<Item=Coordinate>>(positions: I) -> Self {
        let positions = positions.into_iter().map(FormationPosition::empty)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Formation {
            positions,
            used_slots: Slot::empty(),
            dps: Cell::new(None),
        }
    }

    pub fn place_crusader(&mut self, position: usize, crusader: &'a Crusader) -> &mut Self {
        self.positions[position].crusader = Some(crusader);
        self.used_slots |= crusader.slot();
        self.dps.set(None);
        self
    }

    pub fn remove_crusader(&mut self, position: usize) -> &mut Self {
        self.positions[position].crusader = None;
        let used_slots = self.crusaders().fold(Slot::empty(), |s, c| s | c.slot());
        self.used_slots = used_slots;
        self.dps.set(None);
        self
    }

    pub fn total_dps(&self, policy: &SearchPolicy) -> Dps {
        self.dps.get().unwrap_or_else(|| {
            let dps = self.positions.iter()
                .map(|p| p.total_dps(&self, self.auras(policy)))
                .sum();
            self.dps.set(Some(dps));
            dps
        })
    }

    pub fn empty_positions<'b>(&'b self) -> impl Iterator<Item=usize> + 'b {
        self.positions.iter()
            .enumerate()
            .filter(|&(_, p)| p.crusader.is_none())
            .map(|(i, _)| i)
    }

    pub fn used_slots(&self) -> Slot {
        self.used_slots
    }

    pub fn position_of(&self, crusader: CrusaderName) -> Option<&Coordinate> {
        self.positions.iter()
            .find(|p| p.crusader.as_ref().map(|c| c.name) == Some(crusader))
            .map(|p| &p.coordinate)
    }

    pub fn front_column(&self) -> Option<u8> {
        self.positions.iter()
            .map(|p| p.coordinate.x)
            .max()
    }

    pub fn auras<'b>(&'b self, policy: &'b SearchPolicy) -> impl Iterator<Item=&'a Aura> + 'b {
        self.crusaders()
            .flat_map(Crusader::dps_auras)
            .filter(move |aura| policy.allows_ability(aura))
    }

    pub fn ability_buffs<'b>(&'b self, tag: AuraTag)
        -> impl Iterator<Item=&'a AbilityBuff> + 'b
    {
        self.crusaders().flat_map(Crusader::ability_buffs)
            .filter(move |b| b.applies_to(tag, self))
    }

    pub fn print(&self, policy: &SearchPolicy) {
        let longest_crusader_name = self.crusaders()
            .map(|c| format!("{:?}", c.name).len())
            .max();
        let longest_crusader_name = match longest_crusader_name {
            Some(x) => x,
            None => return,
        };
        let front_column = self.front_column().unwrap();
        let num_rows = self.positions.iter()
            .map(|p| p.coordinate.y)
            .max().unwrap();
        println!("Total DPS: {}", self.total_dps(policy));
        for y in 0..(num_rows * 2 + 2) {
            for x in 0..(front_column + 1) {
                let crusader_name = self.positions.iter()
                    .find(|p| {
                        let c = p.coordinate;
                        c.x == x && c.y * 2 + x % 2 == y
                    }).map(|p| {
                        p.crusader.map(|c| format!("{:?}", c))
                            .unwrap_or_else(|| String::from("o"))
                    });
                let dps = self.positions.iter()
                    .filter(|p| p.crusader.is_some())
                    .find(|p| {
                        let c = p.coordinate;
                        y != 0 && c.x == x && c.y * 2 + x % 2 == y - 1
                    }).map(|p| {
                        p.total_dps(&self, self.crusaders().flat_map(Crusader::dps_auras))
                            .to_string()
                    });
                print!("{1:^0$}", longest_crusader_name, crusader_name.or(dps).unwrap_or(String::new()));
            }
            print!("\n");
        }
    }

    pub fn crusaders<'b>(&'b self) -> impl Iterator<Item=&'a Crusader> + 'b {
        self.positions.iter().filter_map(|p| p.crusader)
    }

    pub fn placements<'b>(&'b self) -> impl Iterator<Item=(usize, &'a Crusader)> + 'b {
        self.positions.iter()
            .enumerate()
            .filter_map(|(i, p)| p.crusader.map(|c| (i, c)))
    }
}

#[derive(Debug, Clone)]
struct FormationPosition<'a> {
    coordinate: Coordinate,
    crusader: Option<&'a Crusader>,
}

impl<'a> FormationPosition<'a> {
    fn empty(coordinate: Coordinate) -> Self {
        FormationPosition {
            coordinate,
            crusader: None,
        }
    }

    fn total_dps<I>(&self, formation: &Formation<'a>, auras: I) -> Dps where
        I: IntoIterator<Item=&'a Aura>,
    {
        let crusader = match self.crusader {
            Some(crusader) => crusader,
            None => return Dps(0.0),
        };
        auras.into_iter()
            .fold(crusader.base_dps(), |dps, aura| {
                dps.percent_increase(aura.amount_for_crusader(
                    crusader.name,
                    formation,
                ))
            })
    }
}

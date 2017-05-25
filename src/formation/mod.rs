mod coordinate;

pub use self::coordinate::Coordinate;

use aura::Aura;
use crusader::*;
use dps::*;

#[derive(Debug, Clone)]
pub struct Formation<'a> {
    positions: Box<[FormationPosition<'a>]>,
    used_slots: Slot,
}

impl<'a> Formation<'a> {
    pub fn empty<I: IntoIterator<Item=Coordinate>>(positions: I) -> Self {
        let positions = positions.into_iter().map(FormationPosition::empty)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Formation {
            positions,
            used_slots: Slot::empty(),
        }
    }

    pub fn place_crusader(&mut self, position: usize, crusader: &'a Crusader) -> &mut Self {
        self.positions[position].crusader = Some(crusader);
        self.used_slots |= crusader.slot();
        self
    }

    pub fn remove_crusader(&mut self, position: usize) -> &mut Self {
        self.positions[position].crusader = None;
        let used_slots = self.crusaders().fold(Slot::empty(), |s, c| s | c.slot());
        self.used_slots = used_slots;
        self
    }

    pub fn total_dps(&self) -> Dps {
        self.positions.iter()
            .map(|p| p.total_dps(&self, self.crusaders().flat_map(Crusader::dps_auras)))
            .sum()
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

    pub fn print(&self) {
        println!("Total DPS: {}", self.total_dps());
        for pos in self.positions.iter() {
            let coord = pos.coordinate;
            let crusader = pos.crusader;
            println!("({}, {}): {:?}", coord.x, coord.y, crusader.map(|c| c.name));
        }
    }

    pub fn crusaders<'b>(&'b self) -> impl Iterator<Item=&'a Crusader> + 'b {
        self.positions.iter().filter_map(|p| p.crusader)
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

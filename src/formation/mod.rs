mod coordinate;

pub use self::coordinate::Coordinate;

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

    pub fn total_dps(&self) -> Dps {
        self.crusaders().map(|c| {
            c.base_dps()
        }).sum()
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

    pub fn placements<'b>(&'b self)
        -> impl Iterator<Item=(Coordinate, Option<&'a Crusader>)> + 'b
    {
        self.positions.iter()
            .map(|pos| (pos.coordinate, pos.crusader))
    }

    fn crusaders<'b>(&'b self) -> impl Iterator<Item=&'a Crusader> + 'b {
        self.positions.iter().filter_map(|p| p.crusader)
    }
}

#[derive(Debug, Clone)]
pub struct FormationPosition<'a> {
    pub coordinate: Coordinate,
    pub crusader: Option<&'a Crusader>,
}

impl<'a> FormationPosition<'a> {
    fn empty(coordinate: Coordinate) -> Self {
        FormationPosition {
            coordinate,
            crusader: None,
        }
    }
}

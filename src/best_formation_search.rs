use itertools::*;
use rand::*;
use std::cmp::max;

use crusader::*;
use dps::*;
use formation::*;

pub struct BestFormationSearch<'a> {
    crusaders: &'a [Crusader],
    formation: Formation<'a>,
}

impl<'a> BestFormationSearch<'a> {
    pub fn new(crusaders: &'a [Crusader], formation: Formation<'a>) -> Self {
        BestFormationSearch { crusaders, formation }
    }

    pub fn valid_placements<'b>(&'b self) -> impl Iterator<Item=(usize, &'a Crusader)> + 'b {
        self.valid_positions().cartesian_product(self.valid_crusaders())
    }

    fn random_placement(&self) -> Option<(usize, &'a Crusader)> {
        let mut rng = thread_rng();
        let num_placements = self.valid_positions().count()
            * self.valid_crusaders().count();
        self.valid_placements().nth(rng.gen_range(0, max(1, num_placements)))
    }

    fn valid_positions<'b>(&'b self) -> impl Iterator<Item=usize> + 'b {
        self.formation.empty_positions()
    }

    fn valid_crusaders(&self) -> impl Iterator<Item=&'a Crusader> + Clone {
        UnusedSlots {
            crusaders: self.crusaders.iter(),
            used_slots: self.formation.used_slots(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct UnusedSlots<T> {
    crusaders: T,
    used_slots: Slot,
}

impl<'a, T: Iterator<Item=&'a Crusader>> Iterator for UnusedSlots<T> {
    type Item = &'a Crusader;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.crusaders.next() {
            if !self.used_slots.contains(item.slot()) {
                return Some(item);
            }
        }
        None
    }
}

#[test]
fn no_valid_placements_when_no_crusaders() {
    let search = BestFormationSearch::new(&[], test_formation());
    assert_eq!(0, search.valid_placements().count());
}

#[test]
fn no_valid_placements_when_no_empty_slots() {
    let crusaders = test_crusaders();
    let formation = Formation::empty(vec![]);
    let search = BestFormationSearch::new(&crusaders, formation);
    assert_eq!(0, search.valid_placements().count());
}

#[test]
fn valid_placements_is_product_of_positions_and_crusaders() {
    let crusaders = test_crusaders();
    let formation = test_formation();
    let search = BestFormationSearch::new(&crusaders, formation);
    let expected = vec![
        (0, &crusaders[0]),
        (0, &crusaders[1]),
        (0, &crusaders[2]),
        (1, &crusaders[0]),
        (1, &crusaders[1]),
        (1, &crusaders[2]),
    ];
    let actual = search.valid_placements().collect::<Vec<_>>();
    assert_eq!(actual, expected);
}

#[test]
fn valid_placements_excludes_placed_crusaders() {
    let crusaders = test_crusaders();
    let mut formation = test_formation();
    formation.place_crusader(0, &crusaders[2]);
    let search = BestFormationSearch::new(&crusaders, formation);
    let expected = vec![
        (1, &crusaders[0]),
        (1, &crusaders[1]),
    ];
    let actual = search.valid_placements().collect::<Vec<_>>();
    assert_eq!(actual, expected);
}

#[test]
fn valid_placements_excludes_placed_slots() {
    let crusaders = test_crusaders();
    let mut formation = test_formation();
    formation.place_crusader(1, &crusaders[0]);
    let search = BestFormationSearch::new(&crusaders, formation);
    let expected = vec![(0, &crusaders[2])];
    let actual = search.valid_placements().collect::<Vec<_>>();
    assert_eq!(actual, expected);
}

#[cfg(test)]
fn test_formation<'a>() -> Formation<'a> {
    let positions = vec![
        Coordinate::new(0, 0),
        Coordinate::new(0, 1),
    ];
    Formation::empty(positions)
}

#[cfg(test)]
fn test_crusaders() -> Vec<Crusader> {
    vec![
        Crusader::new(CrusaderName::JimTheLumberjack, Level(5000)),
        Crusader::new(CrusaderName::VeronicaTheAndroidArcher, Level(5000)),
        Crusader::new(CrusaderName::SallyTheSuccubus, Level(5000)),
    ]
}

#[cfg(test)]
mod benchmarks {
    extern crate test;
    use super::*;
    use super::super::create_user_data;
    use self::test::*;

    #[bench]
    fn test_random_placement(b: &mut Bencher) {
        let positions = vec![
            Coordinate::new(0, 0),
            Coordinate::new(0, 1),
            Coordinate::new(0, 2),
            Coordinate::new(0, 3),
            Coordinate::new(1, 0),
            Coordinate::new(1, 1),
            Coordinate::new(1, 2),
            Coordinate::new(2, 1),
            Coordinate::new(2, 2),
            Coordinate::new(3, 1),
        ];
        let formation = Formation::empty(positions);
        let crusaders = create_user_data().unlocked_crusaders();
        let search = BestFormationSearch::new(&crusaders, formation);
        b.iter(|| search.random_placement())
    }
}

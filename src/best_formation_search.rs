use itertools::*;

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
        let used_slots = self.formation.used_slots();
        let valid_positions = self.formation.empty_positions();
        let valid_crusaders = self.crusaders.iter()
            .filter(|c| !used_slots.contains(c.slot()))
            .collect::<Vec<_>>();
        valid_positions.cartesian_product(valid_crusaders)
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

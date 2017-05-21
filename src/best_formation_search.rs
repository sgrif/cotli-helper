extern crate ordermap;

use itertools::*;
use rand::*;
use std::cmp::max;
use std::time::*;
use self::ordermap::OrderMap;

use crusader::*;
use dps::*;
use formation::*;

pub struct BestFormationSearch<'a> {
    crusaders: &'a [Crusader],
    search: FormationSearch<'a>,
}

struct FormationSearch<'a> {
    formation: Formation<'a>,
    highest_dps_seen: Dps,
    tried_placements: OrderMap<(usize, &'a Crusader), FormationSearch<'a>>,
}

impl<'a> BestFormationSearch<'a> {
    pub fn new(crusaders: &'a [Crusader], formation: Formation<'a>) -> Self {
        let search = FormationSearch::new(formation);
        BestFormationSearch { crusaders, search }
    }

    pub fn calculate_best_formation(&mut self, max_time: Duration) {
        let loop_start = Instant::now();
        while loop_start.elapsed() < max_time {
            self.calculate_single_formation();
        }
    }

    pub fn best_formation(&self) -> &Formation<'a> {
        self.search.best_formation()
    }

    fn calculate_single_formation(&mut self) {
        self.search.calculate_single_formation(self.crusaders)
    }
}

impl<'a> FormationSearch<'a> {
    fn new(formation: Formation<'a>) -> Self {
        let formation_dps = formation.total_dps();
        FormationSearch {
            formation,
            highest_dps_seen: formation_dps,
            tried_placements: Default::default(),
        }
    }

    fn best_formation(&self) -> &Formation<'a> {
        self.tried_placements
            .values()
            .find(|p| p.highest_dps_seen() == self.highest_dps_seen())
            .map(Self::best_formation)
            .unwrap_or(&self.formation)
    }

    fn highest_dps_seen(&self) -> Dps {
        self.highest_dps_seen
    }

    fn calculate_single_formation(&mut self, crusaders: &'a [Crusader]) {
        if let Some((position, crusader)) = self.random_placement(crusaders) {
            let search_dps = {
                let placement_search = self.search_after_placement(position, crusader);
                placement_search.calculate_single_formation(crusaders);
                placement_search.highest_dps_seen()
            };
            if search_dps > self.highest_dps_seen() {
                self.highest_dps_seen = search_dps;
            }
        }
    }

    fn search_after_placement(
        &mut self,
        position: usize,
        crusader: &'a Crusader,
    ) -> &mut Self {
        let formation = &self.formation;

        self.tried_placements
            .entry((position, crusader))
            .or_insert_with(|| {
                let mut new_formation = formation.clone();
                new_formation.place_crusader(position, crusader);
                FormationSearch::new(new_formation)
            })
    }

    fn valid_placements<'b>(&'b self, crusaders: &'a [Crusader]) -> impl Iterator<Item=(usize, &'a Crusader)> + 'b {
        self.valid_positions().cartesian_product(self.valid_crusaders(crusaders))
    }

    fn random_placement(&self, crusaders: &'a [Crusader]) -> Option<(usize, &'a Crusader)> {
        let mut rng = thread_rng();
        let num_placements = self.valid_positions().count()
            * self.valid_crusaders(crusaders).count();
        self.valid_placements(crusaders).nth(rng.gen_range(0, max(1, num_placements)))
    }

    fn valid_positions<'b>(&'b self) -> impl Iterator<Item=usize> + 'b {
        self.formation.empty_positions()
    }

    fn valid_crusaders(&self, crusaders: &'a [Crusader]) -> impl Iterator<Item=&'a Crusader> + Clone {
        UnusedSlots {
            crusaders: crusaders.iter(),
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
    let search = FormationSearch::new(test_formation());
    assert_eq!(0, search.valid_placements(&[]).count());
}

#[test]
fn no_valid_placements_when_no_empty_slots() {
    let crusaders = test_crusaders();
    let formation = Formation::empty(vec![]);
    let search = FormationSearch::new(formation);
    assert_eq!(0, search.valid_placements(&crusaders).count());
}

#[test]
fn valid_placements_is_product_of_positions_and_crusaders() {
    let crusaders = test_crusaders();
    let formation = test_formation();
    let search = FormationSearch::new(formation);
    let expected = vec![
        (0, &crusaders[0]),
        (0, &crusaders[1]),
        (0, &crusaders[2]),
        (1, &crusaders[0]),
        (1, &crusaders[1]),
        (1, &crusaders[2]),
    ];
    let actual = search.valid_placements(&crusaders).collect::<Vec<_>>();
    assert_eq!(actual, expected);
}

#[test]
fn valid_placements_excludes_placed_crusaders() {
    let crusaders = test_crusaders();
    let mut formation = test_formation();
    formation.place_crusader(0, &crusaders[2]);
    let search = FormationSearch::new(formation);
    let expected = vec![
        (1, &crusaders[0]),
        (1, &crusaders[1]),
    ];
    let actual = search.valid_placements(&crusaders).collect::<Vec<_>>();
    assert_eq!(actual, expected);
}

#[test]
fn valid_placements_excludes_placed_slots() {
    let crusaders = test_crusaders();
    let mut formation = test_formation();
    formation.place_crusader(1, &crusaders[0]);
    let search = FormationSearch::new(formation);
    let expected = vec![(0, &crusaders[2])];
    let actual = search.valid_placements(&crusaders).collect::<Vec<_>>();
    assert_eq!(actual, expected);
}

#[test]
fn test_calculate_formation_changes_best_placement_seen() {
    let crusaders = test_crusaders();
    let mut search = FormationSearch::new(test_formation());
    search.calculate_single_formation(&crusaders);
    assert_ne!(search.highest_dps_seen, search.formation.total_dps());
}

#[test]
fn test_calculate_formation_changes_best_formation() {
    let crusaders = test_crusaders();
    let mut search = BestFormationSearch::new(&crusaders, test_formation());
    search.calculate_single_formation();
    assert!(search.best_formation().placements().any(|(_, crusader)| crusader.is_some()));
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
    fn bench_random_placement(b: &mut Bencher) {
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
        let search = FormationSearch::new(formation);
        b.iter(|| search.random_placement(&crusaders))
    }

    #[bench]
    fn bench_calculate_single_formation(b: &mut Bencher) {
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
        let mut search = BestFormationSearch::new(&crusaders, formation);
        b.iter(|| search.calculate_single_formation())
    }
}

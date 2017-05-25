mod set_like;
mod unused_slots;

use itertools::*;
use ordermap::OrderMap;
use rand::*;
use std::cmp::max;
use std::time::*;

use crusader::*;
use dps::*;
use formation::*;
use self::unused_slots::unused_slots;
use self::set_like::*;

pub struct FormationSearch<'a> {
    state: State<'a>,
    search_root: Node<'a>,
}

impl<'a> FormationSearch<'a> {
    pub fn new(formation: Formation<'a>, crusaders: &'a [Crusader]) -> Self {
        FormationSearch {
            state: State { formation, crusaders },
            search_root: Node::new(),
        }
    }

    pub fn perform_search(&mut self, max_time: Duration) {
        let empty_positions = self.state.formation.empty_positions().count();
        for _ in 0..empty_positions {
            let loop_start = Instant::now();
            while loop_start.elapsed() < max_time && !self.search_root.is_complete() {
                self.search_root.expand(&mut self.state.clone())
            }
            {
                let mut children = self.search_root.children
                    .iter()
                    .collect::<Vec<_>>();
                children.sort_by_key(|&(_, ref c)| c.times_checked);
                for (placement, child) in children {
                    println!("{:?} checked {} times", placement, child.times_checked);
                }
            }
            let current_dps = self.state.formation.total_dps();
            let root_dps = self.search_root.highest_dps_seen;
            let best_option = self.search_root.children
                .drain(..)
                .filter(|&(_, ref c)| {
                    c.highest_dps_seen >= current_dps &&
                        c.highest_dps_seen >= root_dps
                }).nth(0); // FIXME: If multiple options have same, take most searched
            if let Some((placement, child)) = best_option {
                self.state.place(placement);
                self.state.formation.print();
                self.search_root = child;
            } else {
                break;
            }
        }
    }

    pub fn formation(self) -> Formation<'a> {
        self.state.formation
    }
}

type Placement<'a> = (usize, &'a Crusader);

#[derive(Debug, PartialEq, Eq)]
enum Progress {
    Expandable,
    FullyExpanded,
    Complete,
}

struct Node<'a> {
    highest_dps_seen: Dps,
    progress: Progress,
    children: OrderMap<Placement<'a>, Node<'a>>,
    times_checked: u32,
}

impl<'a> Node<'a> {
    fn new() -> Self {
        Node {
            progress: Progress::Expandable,
            highest_dps_seen: Default::default(),
            children: Default::default(),
            times_checked: 0,
        }
    }

    fn expand(&mut self, state: &mut State<'a>) {
        match self.progress {
            Progress::Expandable => {
                if let Some(placement) = state.random_placement(&self.children) {
                    state.place(placement);
                    let mut child = Node::new();
                    child.check_single_formation(state);
                    self.maybe_assign_dps(child.highest_dps_seen);
                    self.children.insert(placement, child);
                } else {
                    self.progress = Progress::FullyExpanded;
                    return self.expand(state);
                }
            }
            Progress::FullyExpanded => self.to_best_child(state, Self::expand),
            Progress::Complete => panic!("expand called on complete node"),
        }
        self.times_checked += 1;
    }

    fn best_child(&mut self) -> Option<(Placement<'a>, &mut Self)> {
        let mut rng = thread_rng();
        let possible_children = self.children.values().filter(|c| !c.is_complete()).count();
        let selection = rng.gen_range(0, max(1, possible_children));
        self.children.iter_mut()
            .filter(|&(_, ref v)| !v.is_complete())
            .map(|(&k, v)| (k, v))
            .nth(selection)
    }

    fn check_single_formation(&mut self, state: &mut State<'a>) {
        match self.progress {
            Progress::Expandable => {
                if let Some(placement) = state.random_placement(&EmptySet) {
                    state.place(placement);
                    let search_dps;
                    if let Some(child) = self.children.get_mut(&placement) {
                        child.check_single_formation(state);
                        search_dps = child.highest_dps_seen;
                    } else {
                        state.fill_formation_randomly();
                        search_dps = state.formation.total_dps()
                    }
                    self.maybe_assign_dps(search_dps)
                } else {
                    self.progress = Progress::Complete;
                    self.maybe_assign_dps(state.formation.total_dps());
                }
            }
            Progress::FullyExpanded => self.to_best_child(state, Self::check_single_formation),
            Progress::Complete => {}
        }
        self.times_checked += 1;
    }

    fn maybe_assign_dps(&mut self, dps: Dps) {
        if dps > self.highest_dps_seen {
            self.highest_dps_seen = dps;
        }
    }

    fn is_complete(&self) -> bool {
        self.progress == Progress::Complete
    }

    fn to_best_child(
        &mut self,
        state: &mut State<'a>,
        action: fn(&mut Self, &mut State<'a>),
    ) {
        let mut search_dps = None;
        if let Some((placement, child)) = self.best_child() {
            state.place(placement);
            action(child, state);
            search_dps = Some(child.highest_dps_seen);
        }
        if let Some(dps) = search_dps {
            self.maybe_assign_dps(dps);
        } else {
            self.progress = Progress::Complete;
        }
    }
}

#[derive(Clone)]
struct State<'a> {
    formation: Formation<'a>,
    crusaders: &'a [Crusader],
}

impl<'a> State<'a> {
    fn fill_formation_randomly(&mut self) {
        while let Some(placement) = self.random_placement(&EmptySet) {
            self.place(placement)
        }
    }

    fn random_placement<T>(&self, exclude: &T) -> Option<Placement<'a>> where
        T: SetLike<Placement<'a>>,
    {
        let mut rng = thread_rng();
        let num_placements = self.formation.empty_positions().count()
            * unused_slots(&self.formation, self.crusaders).count()
            - exclude.len();
        let selection = rng.gen_range(0, max(1, num_placements));
        self.formation.empty_positions()
            .cartesian_product(unused_slots(&self.formation, self.crusaders))
            .filter(|p| !exclude.contains(p))
            .nth(selection)
    }

    fn place(&mut self, (position, crusader): Placement<'a>) {
        self.formation.place_crusader(position, crusader);
    }
}

#[test]
fn fill_formation_randomly_fills_formation_until_formation_is_full() {
    let crusaders = test_crusaders();
    let mut state = search_state(&crusaders);
    state.fill_formation_randomly();
    assert_eq!(0, state.formation.empty_positions().count());
}

#[test]
fn fill_formation_randomly_stops_when_no_more_crusaders() {
    let crusaders = vec![test_crusaders().pop().unwrap()];
    let mut state = search_state(&crusaders);
    state.fill_formation_randomly();
    assert_eq!(1, state.formation.empty_positions().count());
}

#[test]
fn expanding_expandable_node_always_adds_new_child() {
    let crusaders = test_crusaders();
    let state = search_state(&crusaders);
    let mut search = Node::new();
    for _ in 0..6 {
        assert_eq!(Progress::Expandable, search.progress);
        search.expand(&mut state.clone());
    }
    // Expand one more time. Whether determining FullyExpanded is eager when 1
    // choice left, or lazy when attempting to expand and no choices is an impl
    // detail
    search.expand(&mut state.clone());
    assert_eq!(Progress::FullyExpanded, search.progress);
}

#[test]
fn expanding_updates_highest_dps_seen() {
    let crusaders = test_crusaders();
    let mut state = search_state(&crusaders);
    let mut search = Node::new();
    assert_eq!(Dps(0.0), search.highest_dps_seen);
    search.expand(&mut state);
    // Sanity check, make sure the formation was filled
    assert_eq!(0, state.formation.empty_positions().count());
    assert_eq!(state.formation.total_dps(), search.highest_dps_seen);
}

#[test]
fn expand_eventually_completes_node() {
    let crusaders = test_crusaders();
    let state = search_state(&crusaders);
    let mut search = Node::new();
    for _ in 0..1000 {
        if search.progress == Progress::Complete {
            break;
        }
        search.expand(&mut state.clone());
    }
    assert_eq!(Progress::Complete, search.progress);
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
fn search_state(crusaders: &[Crusader]) -> State {
    State {
        formation: test_formation(),
        crusaders,
    }
}

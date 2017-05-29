mod parameters;
mod search_policy;
mod set_like;
mod unused_slots;

pub use self::parameters::*;
pub use self::search_policy::*;

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

const EXPLORATION_COEF: f64 = 1.4;

pub struct FormationSearch<'a> {
    state: State<'a>,
    search_root: Node<'a>,
    parameters: Parameters,
}

impl<'a> FormationSearch<'a> {
    pub fn new(
        formation: Formation<'a>,
        crusaders: &'a [Crusader],
        parameters: Parameters,
    ) -> Self {
        FormationSearch {
            state: State { formation, crusaders, placements: Vec::new(), },
            search_root: Node::new(),
            parameters,
        }
    }

    pub fn perform_search(&mut self) {
        let empty_positions = self.state.formation.empty_positions().count();
        let max_time = self.parameters.max_time_per_step;
        for _ in 0..empty_positions {
            let loop_start = Instant::now();
            while loop_start.elapsed() < max_time && !self.search_root.is_complete() {
                let mut state = self.state.clone();
                self.search_root.expand(&mut state);
                self.search_root.track_score_changes(&state, &self.parameters.policy);
            }

            self.print_debug_output();

            let current_score = self.parameters.policy.score(&self.state.formation);
            let options = self.search_root.children
                .drain(..)
                .filter(|&(_, ref c)| c.highest_score_seen >= current_score)
                .collect::<Vec<_>>();
            let best_option = if options.iter().any(|&(_, ref c)| c.is_complete()) {
                options.into_iter()
                    .max_by_key(|&(_, ref c)| (c.highest_score_seen, c.times_checked))
            } else {
                options.into_iter()
                    .max_by_key(|&(_, ref c)| (c.times_checked, c.highest_score_seen))
            };
            if let Some((placement, child)) = best_option {
                self.state.place(placement);
                self.state.formation.print(&self.parameters.policy);
                self.search_root = child;
            } else {
                break;
            }
        }
    }

    pub fn formation(self) -> Formation<'a> {
        self.state.formation
    }

    fn print_debug_output(&self) {
        if self.parameters.verbosity.is_some() && !self.search_root.is_complete() {
            let mut children = self.search_root.children
                .iter()
                .collect::<Vec<_>>();
            children.sort_by_key(|&(_, ref c)| (c.times_checked, c.highest_score_seen));
            let num_skip = match self.parameters.verbosity {
                Verbosity::Debug => 0,
                _ => children.len() - 5,
            };
            for (placement, child) in children.into_iter().skip(num_skip) {
                println!("{:?} checked {} times (max {})", placement, child.times_checked, child.highest_score_seen);
            }
            println!("checked {} total", self.search_root.times_checked);
        }
    }
}

type Placement<'a> = (usize, &'a Crusader);

#[derive(Debug, PartialEq, Eq)]
enum Progress {
    Expandable,
    FullyExpanded,
    Complete,
}

// NOTE: We're storing/checking redundant information here due to how the
// tree is set up. e.g. Placing Grandmora and then Sally will not share data
// with placing Sally then Grandmora even if the formation is the same. The
// alternative would be to store everything in a flat map, where the key
// is the formation. What we have now is probably better than at least the
// naive implementation of the alternative, since that would require duplicating
// the vec of placements once per option, and would balloon memory usage.
// However, if we track the placements using some sort of persistent data
// structure, we may be able to see a decent boost from flattening the tree
struct Node<'a> {
    highest_score_seen: FormationScore,
    total_score_seen: FormationScore,
    progress: Progress,
    children: OrderMap<Placement<'a>, Node<'a>>,
    times_checked: u32,
}

impl<'a> Node<'a> {
    fn new() -> Self {
        Node {
            progress: Progress::Expandable,
            highest_score_seen: Default::default(),
            total_score_seen: Default::default(),
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
                    self.children.insert(placement, child);
                } else {
                    self.progress = Progress::FullyExpanded;
                    return self.expand(state);
                }
            }
            Progress::FullyExpanded => self.to_best_child(state, Self::expand),
            Progress::Complete => panic!("expand called on complete node"),
        }
    }

    fn best_child(&mut self) -> Option<(Placement<'a>, &mut Self)> {
        let log_total = (self.times_checked as f64).ln();
        let highest_score = self.highest_score_seen;
        let score = |c: &Self| {
            let normalized_score_seen = c.total_score_seen / highest_score / (c.times_checked as f64);
            let exploration_adjustment = EXPLORATION_COEF
                * (log_total / c.times_checked as f64).sqrt();
            Dps(normalized_score_seen + exploration_adjustment)
        };
        self.children.iter_mut()
            .filter(|&(_, ref v)| !v.is_complete())
            .map(|(&k, v)| (k, v))
            .max_by_key(|&(_, ref c1)| score(c1))
    }

    fn check_single_formation(&mut self, state: &mut State<'a>) {
        match self.progress {
            Progress::Expandable => {
                if let Some(placement) = state.random_placement(&EmptySet) {
                    state.place(placement);
                    if let Some(child) = self.children.get_mut(&placement) {
                        child.check_single_formation(state);
                    } else {
                        state.fill_formation_randomly();
                    }
                } else {
                    self.progress = Progress::Complete;
                }
            }
            Progress::FullyExpanded => self.to_best_child(state, Self::check_single_formation),
            Progress::Complete => {}
        }
    }

    fn track_score_changes(&mut self, state: &State<'a>, policy: &SearchPolicy) {
        let score = policy.score(&state.formation);
        self.total_score_seen += score;
        self.times_checked += 1;
        if score > self.highest_score_seen {
            self.highest_score_seen = score;
        }
        if state.placements.is_empty() { // Never randomly filled
            for placement in state.formation.placements() {
                if let Some(child) = self.children.get_mut(&placement) {
                    child.track_score_changes(state, policy)
                }
            }
        } else {
            for placement in &state.placements {
                if let Some(child) = self.children.get_mut(&placement) {
                    child.track_score_changes(state, policy)
                }
            }
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
        let mut found_child = false;
        if let Some((placement, child)) = self.best_child() {
            state.place(placement);
            action(child, state);
            found_child = true;
        }
        if !found_child {
            self.progress = Progress::Complete;
        }
    }
}

#[derive(Clone)]
struct State<'a> {
    formation: Formation<'a>,
    crusaders: &'a [Crusader],
    placements: Vec<Placement<'a>>,
}

impl<'a> State<'a> {
    fn fill_formation_randomly(&mut self) {
        self.placements = self.formation.placements().collect();
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
    assert_eq!(search.highest_score_seen, Default::default());
    search.expand(&mut state);
    search.track_score_changes(&state, &Default::default());
    // Sanity check, make sure the formation was filled
    assert_eq!(0, state.formation.empty_positions().count());
    assert_ne!(search.highest_score_seen, Default::default());
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
        Crusader::new(CrusaderName::JimTheLumberjack, &Default::default(), None),
        Crusader::new(CrusaderName::VeronicaTheAndroidArcher, &Default::default(), None),
        Crusader::new(CrusaderName::SallyTheSuccubus, &Default::default(), None),
    ]
}

#[cfg(test)]
fn search_state(crusaders: &[Crusader]) -> State {
    State {
        formation: test_formation(),
        crusaders,
        placements: Vec::new(),
    }
}

#[cfg(test)]
mod benchmarks {
    extern crate test;
    use create_user_data;
    use self::test::Bencher;
    use super::*;

    #[bench]
    fn bench_expand_root_node(b: &mut Bencher) {
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
        let crusaders = create_user_data().unlocked_crusaders(None);
        let mut search = Node::new();
        let state = State { formation, crusaders: &crusaders, placements: Vec::new(), };
        let policy = SearchPolicy { active_play: false, considers_gold: true };

        b.iter(|| {
            let mut new_state = state.clone();
            search.expand(&mut new_state);
            search.track_score_changes(&new_state, &policy)
        })
    }

    #[bench]
    fn bench_search_root_after_1k_expands(b: &mut Bencher) {
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
        let crusaders = create_user_data().unlocked_crusaders(None);
        let mut search = Node::new();
        let state = State { formation, crusaders: &crusaders, placements: Vec::new(), };
        let policy = SearchPolicy { active_play: false, considers_gold: true };
        for _ in 0..1_000 {
            let mut new_state = state.clone();
            search.expand(&mut new_state);
            search.track_score_changes(&new_state, &policy);
        }

        b.iter(|| {
            let mut new_state = state.clone();
            search.check_single_formation(&mut new_state);
            search.track_score_changes(&new_state, &policy);
        })
    }
}

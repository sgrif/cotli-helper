extern crate cotli_helper;

pub use cotli_helper::*;

use formation_search::*;
use formation::*;

fn main() {
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
    let mut search = FormationSearch::new(formation, &crusaders);
    search.perform_search(::std::time::Duration::from_secs(5));
    search.formation().print();
}

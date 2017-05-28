extern crate cotli_helper;

pub use cotli_helper::*;

use cli::*;
use formation_search::*;
use formation::*;

fn main() {
    let options = CliOptions::load();
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
    let crusaders = create_user_data().unlocked_crusaders(options.max_gold());
    let formation = Formation::empty(positions);
    let mut search = FormationSearch::new(formation, &crusaders, options.search_parameters());
    search.perform_search();
    search.formation().print();
}

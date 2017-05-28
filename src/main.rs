extern crate cotli_helper;

pub use cotli_helper::*;

use cli::*;
use formation_search::*;

fn main() {
    let options = CliOptions::load();
    let crusaders = create_user_data().unlocked_crusaders(options.max_gold());
    let formation = options.selected_formation();
    let mut search = FormationSearch::new(formation, &crusaders, options.search_parameters());
    search.perform_search();
    search.formation().print(&options.search_policy());
}

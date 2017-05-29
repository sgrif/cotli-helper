extern crate cotli_helper;

pub use cotli_helper::*;

use cli::*;
use formation_search::*;

fn main() {
    let options = CliOptions::load();
    let user_data = create_user_data();
    let crusaders = user_data.unlocked_crusaders(options.max_gold());
    let formation = options.selected_formation(&crusaders);
    let params = options.search_parameters(&user_data);
    let mut search = FormationSearch::new(formation, &crusaders, params);
    search.perform_search();
}

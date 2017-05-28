use clap::*;
use std::time::Duration;

use formation_search::{Parameters, Verbosity};

pub struct CliOptions<'a> {
    matches: ArgMatches<'a>,
}

impl<'a> CliOptions<'a> {
    pub fn load() -> Self {
        CliOptions {
            matches: app().get_matches(),
        }
    }

    pub fn max_gold(&self) -> Option<f64> {
        self.matches.value_of("max-gold")
            .map(|s| s.parse().expect("Invalid max-gold. The value must be a \
                number. (Note: formats like '100T' are not recognized. Use \
                scientific notation)"))
    }

    pub fn search_parameters(&self) -> Parameters {
        Parameters {
            verbosity: self.verbosity(),
            max_time_per_step: Duration::from_secs(self.search_time()),
        }
    }

    fn search_time(&self) -> u64 {
        self.matches.value_of("search-time")
            .expect("No search-time provided")
            .parse()
            .expect("Invalid search time -- The value must be a number in seconds")
    }

    fn verbosity(&self) -> Verbosity {
        match self.matches.occurrences_of("verbose") {
            0 => Verbosity::None,
            1 => Verbosity::Verbose,
            _ => Verbosity::Debug,
        }
    }
}

fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("COTLI Helper")
        .version("0.1.0")
        .author("Sean Griffin <sean@seantheprogrammer.com>")
        .about("Determines the best formation for Crusaders of the Lost Idols")
        .arg(Arg::with_name("search-time")
             .long("search-time")
             .takes_value(true)
             .value_name("TIME_IN_SECONDS")
             .default_value("30")
             .help("The maximum amount of time to spend deciding each placement. \
                    The time should be provided in seconds. This limits the time \
                    spent on each step, not the whole formation."))
        .arg(Arg::with_name("max-gold")
             .long("max-gold")
             .takes_value(true)
             .value_name("GOLD_PER_CRUSADER")
             .help("The amount of gold spent on leveling each crusader. For \
                    example, passing '1.34e168' here will level Nate to 5125, \
                    The Exterminator to around 5000, and so on. If no max gold \
                    is provided, all crusaders will be placed at the level cap. \
                    A value of '1e110' is a good mid-level value that will \
                    consider all crusaders for progression."))
        .arg(Arg::with_name("verbose")
             .short("v")
             .long("verbose")
             .multiple(true)
             .help("Include additional output about how many times the most \
                    likely placements were searched, and other debug output. \
                    Passing this argument mutliple times will increase the \
                    level of debug output"))
}

use std::time::Duration;

use super::SearchPolicy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Verbosity {
    None,
    Verbose,
    Debug,
}

impl Verbosity {
    pub fn is_some(&self) -> bool {
        *self != Verbosity::None
    }
}

#[derive(Debug, Clone)]
pub struct Parameters {
    pub verbosity: Verbosity,
    pub max_time_per_step: Duration,
    pub policy: SearchPolicy,
}

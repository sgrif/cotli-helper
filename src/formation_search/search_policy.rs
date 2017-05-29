use std::fmt;
use std::ops::{AddAssign, Div};

use aura::Aura;
use dps::Dps;
use formation::Formation;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
pub struct FormationScore(Dps);

impl AddAssign for FormationScore {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Div for FormationScore {
    type Output = f64;

    fn div(self, other: Self) -> Self::Output {
        (self.0).0 / (other.0).0
    }
}

impl fmt::Display for FormationScore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SearchPolicy {
    pub active_play: bool,
}

impl SearchPolicy {
    pub fn score(&self, formation: &Formation) -> FormationScore {
        FormationScore(formation.total_dps(self))
    }

    pub fn allows_ability(&self, aura: &Aura) -> bool {
        !aura.requires_active_play || self.active_play
    }
}

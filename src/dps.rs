use std::cmp::Ordering;
use std::fmt;
use std::iter::Sum;
use std::ops::{Mul, Add, AddAssign};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Dps(pub f64);
#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Deserialize)]
pub struct Level(pub u16);

impl Dps {
    pub fn percent_increase(self, percent: f64) -> Self {
        if percent == 0.0 {
            return self
        }

        Dps(self.0 * (1.0 + percent / 100.0))
    }
}

// We're super sure we don't have NaN
impl Eq for Dps {}
impl Ord for Dps {
    fn cmp(&self, rhs: &Dps) -> Ordering {
        self.partial_cmp(&rhs).unwrap()
    }
}

impl Mul<Level> for Dps {
    type Output = Dps;

    fn mul(self, Level(lvl): Level) -> Self::Output {
        let over_200_multiplier = 4f64.powi(lvl.saturating_sub(175) as i32 / 25);
        Dps(self.0 * lvl as f64 * over_200_multiplier)
    }
}

impl Add for Dps {
    type Output = Dps;

    fn add(self, other: Dps) -> Self::Output {
        Dps(self.0 + other.0)
    }
}

impl AddAssign for Dps {
    fn add_assign(&mut self, other: Dps) {
        *self = *self + other;
    }
}

impl Sum for Dps {
    fn sum<I>(iter: I) -> Self where
        I: Iterator<Item=Self>,
    {
        iter.fold(Dps(0.0), |total, dps| total + dps)
    }
}

impl fmt::Display for Dps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.0.trunc().to_string();
        if s.len() <= 3 {
            write!(f, "{}", s)
        } else {
            let scale = self.0.log10().trunc() as i32;
            let x = (self.0 / 10f64.powi(scale - 2)).floor();
            write!(f, "{:.2}e{}", x / 100.0, scale)
        }
    }
}

#[test]
fn multiply_by_level_is_straight_multiplier() {
    assert_eq!(Dps(5.0), Dps(1.0) * Level(5));
    assert_eq!(Dps(10.0), Dps(2.0) * Level(5));
    assert_eq!(Dps(15.0), Dps(2.5) * Level(6));
}

#[test]
fn multiply_by_level_adds_4x_for_every_25_past_200() {
    assert_eq!(Dps(800.0), Dps(1.0) * Level(200));
    assert_eq!(Dps(804.0), Dps(1.0) * Level(201));
    assert_eq!(Dps(7200.0), Dps(2.0) * Level(225));
    assert_eq!(Dps(7232.0), Dps(2.0) * Level(226));
}

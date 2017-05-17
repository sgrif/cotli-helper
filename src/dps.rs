use std::iter::Sum;
use std::ops::{Mul, Add};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Dps(pub f64);
#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Level(pub u16);

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

impl Sum for Dps {
    fn sum<I>(iter: I) -> Self where
        I: Iterator<Item=Self>,
    {
        iter.fold(Dps(0.0), |total, dps| total + dps)
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

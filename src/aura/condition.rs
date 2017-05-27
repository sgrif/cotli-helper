use super::*;

pub enum Condition {
    Eq(Target, usize),
    GtComplex(Target, Target),
    GtEq(Target, usize),
    LtEq(Target, usize),
}

impl Condition {
    pub fn is_met(&self, formation: &Formation) -> bool {
        use self::Condition::*;
        match *self {
            Eq(ref target, amount) => target.count_in_formation(formation) == amount,
            GtComplex(ref t1, ref t2) => t1.count_in_formation(formation) > t2.count_in_formation(formation),
            GtEq(ref target, amount) => target.count_in_formation(formation) >= amount,
            LtEq(ref target, amount) => target.count_in_formation(formation) <= amount,
        }
    }
}

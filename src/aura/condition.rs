use super::*;

pub enum Condition {
    Gt(Target, usize),
    GtComplex(Target, Target),
    Lt(Target, usize),
}

impl Condition {
    pub fn is_met(&self, formation: &Formation) -> bool {
        use self::Condition::*;
        match *self {
            Gt(ref target, amount) => target.count_in_formation(formation) > amount,
            GtComplex(ref t1, ref t2) => t1.count_in_formation(formation) > t2.count_in_formation(formation),
            Lt(ref target, amount) => target.count_in_formation(formation) < amount,
        }
    }
}

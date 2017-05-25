use super::*;

pub enum Condition {
    Gt(Target, usize),
    Lt(Target, usize),
}

impl Condition {
    pub fn is_met(&self, formation: &Formation) -> bool {
        use self::Condition::*;
        match *self {
            Gt(ref target, amount) => target.count_in_formation(formation) > amount,
            Lt(ref target, amount) => target.count_in_formation(formation) < amount,
        }
    }
}

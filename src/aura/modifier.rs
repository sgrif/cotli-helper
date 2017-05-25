use std::cmp::max;
use super::*;

pub enum Modifier {
    DividedBy(Target),
    Minus(Box<Aura>),
    Plus(Box<Aura>),
    Times(Target),
}

impl Modifier {
    pub fn apply(&self, base: f64, formation: &Formation) -> f64 {
        use self::Modifier::*;
        match *self {
            DividedBy(ref target) => base /
                max(1, target.count_in_formation(formation)) as f64,
            Minus(ref aura) => base - aura.modifier_amount(formation),
            Plus(ref aura) => base + aura.modifier_amount(formation),
            Times(ref target) => base * target.count_in_formation(formation) as f64,
        }
    }
}

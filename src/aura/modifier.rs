use super::*;

pub enum Modifier {
    Minus(Box<Aura>),
    Times(Target),
}

impl Modifier {
    pub fn apply(&self, base: f64, formation: &Formation) -> f64 {
        use self::Modifier::*;
        match *self {
            Minus(ref aura) => base - aura.modifier_amount(formation),
            Times(ref target) => base * target.count_in_formation(formation) as f64,
        }
    }
}

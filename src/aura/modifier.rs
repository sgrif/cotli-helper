use std::cmp::max;
use super::*;

pub enum Modifier {
    DividedBy(Target),
    Diversity,
    Minus(Box<Aura>),
    Plus(Box<Aura>),
    RandomlyAffecting(usize, Target),
    Times(Target),
}

impl Modifier {
    pub fn apply(&self, base: f64, formation: &Formation) -> f64 {
        use self::Modifier::*;
        match *self {
            DividedBy(ref target) => base /
                max(1, target.count_in_formation(formation)) as f64,
            Diversity => base * diversity_multiplier(formation),
            Minus(ref aura) => base - aura.modifier_amount(formation),
            Plus(ref aura) => base + aura.modifier_amount(formation),
            RandomlyAffecting(count, ref target) =>
                base / 1f64.max(target.count_in_formation(formation) as f64 / count as f64),
            Times(ref target) => base * target.count_in_formation(formation) as f64,
        }
    }
}

fn diversity_multiplier(formation: &Formation) -> f64 {
    let mut unique_tags = Tags::empty();
    let mut duplicate_tags = Tags::empty();

    for crusader in formation.crusaders() {
        let tags = crusader.tags();
        duplicate_tags |= unique_tags & tags;
        unique_tags ^= tags;
        unique_tags &= !duplicate_tags;
    }

    1f64.max(unique_tags.bits().count_ones() as f64
        - 0.25 * duplicate_tags.bits().count_ones() as f64)
}

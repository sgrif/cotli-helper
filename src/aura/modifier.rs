use std::cmp::max;
use super::*;

#[derive(Debug)]
pub enum Modifier {
    Composite(Box<Modifier>, Box<Modifier>),
    DividedBy(Target),
    Diversity,
    Minus(Box<Aura>),
    Plus(Box<Aura>),
    RandomlyAffecting(usize, Target),
    Times(Target),
    ToPowerOf(Target),
}

impl Modifier {
    pub fn apply(&self, base: f64, formation: &Formation) -> f64 {
        use self::Modifier::*;
        match *self {
            Composite(ref m1, ref m2) => m2.apply(m1.apply(base, formation), formation),
            DividedBy(ref target) => base /
                max(1, target.count_in_formation(formation)) as f64,
            Diversity => base * diversity_multiplier(formation),
            Minus(ref aura) => 0.0f64.max(base - aura.modifier_amount(formation)),
            Plus(ref aura) => base + aura.modifier_amount(formation),
            RandomlyAffecting(count, ref target) =>
                base / 1f64.max(target.count_in_formation(formation) as f64 / count as f64),
            Times(ref target) => base * target.count_in_formation(formation) as f64,
            ToPowerOf(ref target) => (1.0 + base / 100.0)
                .powi(target.count_in_formation(formation) as i32)
                * 100.0 - 100.0,
        }
    }

    pub fn with_tag(self, tag: AuraTag) -> Self {
        match self {
            Modifier::Composite(t1, t2) => Modifier::Composite(
                Box::new(t1.with_tag(tag)),
                Box::new(t2.with_tag(tag)),
            ),
            Modifier::Minus(aura) => Modifier::Minus(Box::new(aura.with_tag(tag))),
            Modifier::Plus(aura) => Modifier::Plus(Box::new(aura.with_tag(tag))),
            _ => self,
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

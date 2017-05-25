use std::ops;

use crusader::*;
use formation::*;

pub enum Target {
    AdjacentTo(CrusaderName),
    AllCrusaders,
    And(Box<Target>, Box<Target>),
    InSameColumn(CrusaderName),
    Not(Box<Target>),
    SpecificCrusader(CrusaderName),
    WithTag(Tags),
}

impl Target {
    pub fn and(self, other: Target) -> Self {
        Target::And(Box::new(self), Box::new(other))
    }

    pub fn matches(
        &self,
        crusader: CrusaderName,
        formation: &Formation,
    ) -> bool {
        use self::Target::*;
        match *self {
            AdjacentTo(source) => {
                match (formation.position_of(crusader), formation.position_of(source)) {
                    (Some(p1), Some(p2)) => p1.is_adjacent_to(p2),
                    _ => false,
                }
            }
            AllCrusaders => true,
            And(ref t1, ref t2) => t1.matches(crusader, formation) &&
                t2.matches(crusader, formation),
            InSameColumn(source) =>
                formation.position_of(crusader).map(|c| c.x) ==
                    formation.position_of(source).map(|c| c.x),
            Not(ref t1) => !t1.matches(crusader, formation),
            SpecificCrusader(name) => crusader == name,
            WithTag(tag) => crusader.tags().contains(tag),
        }
    }

    pub fn count_in_formation(&self, formation: &Formation) -> usize {
        formation.crusaders()
            .filter(|c| self.matches(c.name, formation))
            .count()
    }
}

impl ops::Not for Target {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Target::Not(t) => *t,
            _ => Target::Not(Box::new(self)),
        }
    }
}

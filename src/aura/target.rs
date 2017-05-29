use std::cmp::min;
use std::ops;

use crusader::*;
use formation::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    AdjacentTo(CrusaderName),
    AheadOf(CrusaderName),
    AllCrusaders,
    And(Box<Target>, Box<Target>),
    Behind(CrusaderName),
    EmptySlot,
    GoldFind,
    InColumnAhead(CrusaderName),
    InColumnBehind(CrusaderName),
    InFrontColumn,
    InSameColumn(CrusaderName),
    Min(Box<Target>, usize),
    Not(Box<Target>),
    Or(Box<Target>, Box<Target>),
    SpecificCrusader(CrusaderName),
    WithTag(Tags),
}

impl Target {
    pub fn and(self, other: Target) -> Self {
        Target::And(Box::new(self), Box::new(other))
    }

    pub fn or(self, other: Target) -> Self {
        Target::Or(Box::new(self), Box::new(other))
    }

    pub fn min(self, count: usize) -> Self {
        Target::Min(Box::new(self), count)
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
            AheadOf(source) => {
                match (formation.position_of(source), formation.position_of(crusader)) {
                    (Some(source), Some(target)) => target.x > source.x,
                    _ => false,
                }
            }
            AllCrusaders => true,
            And(ref t1, ref t2) => t1.matches(crusader, formation) &&
                t2.matches(crusader, formation),
            Behind(source) => {
                match (formation.position_of(source), formation.position_of(crusader)) {
                    (Some(source), Some(target)) => target.x < source.x,
                    _ => false,
                }
            }
            EmptySlot => false,
            GoldFind => false,
            InColumnAhead(source) => {
                let source_col = formation.position_of(source).map(|c| c.x);
                let target_col = formation.position_of(crusader).map(|c| c.x);
                target_col == source_col.map(|x| x+1)
            },
            InColumnBehind(source) => {
                let source_col = formation.position_of(source).map(|c| c.x);
                let target_col = formation.position_of(crusader).map(|c| c.x);
                source_col != Some(0) && target_col == source_col.map(|x| x-1)
            },
            InFrontColumn => {
                formation.position_of(crusader).map(|c| c.x) ==
                    formation.front_column()
            }
            InSameColumn(source) =>
                formation.position_of(crusader).map(|c| c.x) ==
                    formation.position_of(source).map(|c| c.x),
            Min(ref t1, _) => t1.matches(crusader, formation),
            Not(ref t1) => !t1.matches(crusader, formation),
            Or(ref t1, ref t2) => t1.matches(crusader, formation) ||
                t2.matches(crusader, formation),
            SpecificCrusader(name) => crusader == name,
            WithTag(tag) => crusader.tags().contains(tag),
        }
    }

    pub fn count_in_formation(&self, formation: &Formation) -> usize {
        use self::Target::*;
        match *self {
            EmptySlot => formation.empty_positions().count(),
            Min(ref t1, count) => min(t1.count_in_formation(formation), count),
            _ => formation.crusaders()
                .filter(|c| self.matches(c.name, formation))
                .count(),
        }
    }
}

impl ops::Not for Target {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Target::Not(t) => *t,
            Target::GoldFind |
            Target::AllCrusaders => panic!("Not {:?} doesn't make sense", self),
            _ => Target::Not(Box::new(self)),
        }
    }
}

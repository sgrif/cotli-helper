mod condition;
mod modifier;
mod target;

pub use self::condition::*;
pub use self::modifier::*;
pub use self::target::*;

use crusader::*;
use formation::*;

pub struct Aura {
    amount: f64,
    target: Target,
    modifier: Option<Modifier>,
    condition: Option<Condition>,
    tag: Option<AuraTag>,
}

impl Aura {
    pub fn dps_increase(amount: f64) -> AuraBuilder {
        AuraBuilder { amount }
    }

    pub fn dps_global(amount: f64) -> Self {
        Aura::dps_increase(amount).affecting(Target::AllCrusaders)
    }

    pub fn plus(self, aura: Aura) -> Self {
        self.with_modifier(Modifier::Plus(Box::new(aura)))
    }

    pub fn minus(self, aura: Aura) -> Self {
        self.with_modifier(Modifier::Minus(Box::new(aura)))
    }

    pub fn times(self, target: Target) -> Self {
        self.with_modifier(Modifier::Times(target))
    }

    pub fn divided_by(self, target: Target) -> Self {
        self.with_modifier(Modifier::DividedBy(target))
    }

    pub fn randomly_affecting(self, count: usize) -> Self {
        let target = self.target.clone();
        self.with_modifier(Modifier::RandomlyAffecting(count, target))
    }

    pub fn with_modifier(self, modifier: Modifier) -> Self {
        Aura { modifier: Some(modifier), ..self }
    }

    pub fn when_exists(self, target: Target) -> Self {
        self.when(Condition::Gt(target, 0))
    }

    pub fn when_none(self, target: Target) -> Self {
        self.when(Condition::Lt(target, 1))
    }

    pub fn when(self, condition: Condition) -> Self {
        Aura { condition: Some(condition), ..self }
    }

    pub fn with_tag(self, tag: AuraTag) -> Self {
        Aura { tag: Some(tag), ..self }
    }

    pub fn amount_for_crusader(
        &self,
        crusader: CrusaderName,
        formation: &Formation,
    ) -> f64 {
        if !self.applies_to(crusader, formation) {
            return 0.0;
        }
        self.modifier_amount(formation)
    }

    fn applies_to(
        &self,
        crusader: CrusaderName,
        formation: &Formation,
    ) -> bool {
        self.target.matches(crusader, formation)
            && self.condition.as_ref().map(|c| c.is_met(formation)).unwrap_or(true)
    }

    fn modifier_amount(&self, formation: &Formation) -> f64 {
        let res = self.modifier.as_ref().map(|m| m.apply(self.amount, formation))
            .unwrap_or(self.amount);
        self.tag.map(|t| formation.ability_buffs(t).fold(res, |r, m| m.modify(r)))
            .unwrap_or(res)
    }
}

pub struct AbilityBuff {
    amount: f64,
    target: AuraTag,
    condition: Option<Condition>,
}

impl AbilityBuff {
    pub fn new(amount: f64, target: AuraTag) -> Self {
        AbilityBuff {
            amount,
            target,
            condition: None,
        }
    }

    pub fn applies_to(&self, tag: AuraTag, formation: &Formation) -> bool {
        self.target == tag
            && self.condition.as_ref().map(|c| c.is_met(formation)).unwrap_or(true)
    }

    pub fn when_exists(self, target: Target) -> Self {
        self.when(Condition::Gt(target, 0))
    }

    pub fn when_none(self, target: Target) -> Self {
        self.when(Condition::Lt(target, 1))
    }

    pub fn when(self, condition: Condition) -> Self {
        Self { condition: Some(condition), ..self }
    }

    fn modify(&self, percent: f64) -> f64 {
        percent * (1.0 + self.amount / 100.0)
    }
}

pub struct AuraBuilder {
    amount: f64,
}

impl AuraBuilder {
    pub fn for_crusader(self, crusader: CrusaderName) -> Aura {
        self.affecting(Target::SpecificCrusader(crusader))
    }

    pub fn affecting(self, target: Target) -> Aura {
        Aura {
            amount: self.amount,
            target: target,
            modifier: None,
            condition: None,
            tag: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuraTag {
    // Slot 1
    Swordplay,

    // Slot 2
    SharpenParty,
    BuddySystem,
    PreciseAim,
    Fire,
    WebBlast,

    // Slot 4
    EligibleReceivers,

    // Slot 10
    LooseMagic,

    // Slot 22
    TheShadowsCowl,
}

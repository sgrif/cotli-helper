mod modifier;
mod target;

pub use self::target::*;

use crusader::*;
use formation::*;
use self::modifier::*;

pub struct Aura {
    amount: f64,
    target: Target,
    modifier: Option<Modifier>,
}

impl Aura {
    pub fn dps_increase(amount: f64) -> AuraBuilder {
        AuraBuilder { amount }
    }

    pub fn dps_global(amount: f64) -> Self {
        Aura::dps_increase(amount).affecting(Target::AllCrusaders)
    }

    pub fn minus(self, aura: Aura) -> Self {
        self.with_modifier(Modifier::Minus(Box::new(aura)))
    }

    pub fn times(self, target: Target) -> Self {
        self.with_modifier(Modifier::Times(target))
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

    fn with_modifier(self, modifier: Modifier) -> Self {
        Aura { modifier: Some(modifier), ..self }
    }

    fn applies_to(
        &self,
        crusader: CrusaderName,
        formation: &Formation,
    ) -> bool {
        self.target.matches(crusader, formation)
    }

    fn modifier_amount(&self, formation: &Formation) -> f64 {
        self.modifier.as_ref().map(|m| m.apply(self.amount, formation))
            .unwrap_or(self.amount)
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
        }
    }
}

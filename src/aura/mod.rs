mod condition;
mod modifier;
mod target;

pub use self::condition::*;
pub use self::modifier::*;
pub use self::target::*;

use crusader::*;
use formation::*;

#[derive(Debug)]
pub struct Aura {
    amount: f64,
    target: Target,
    pub requires_active_play: bool,
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

    pub fn gold_find_bonus(amount: f64) -> Self {
        Aura::dps_increase(amount).affecting(Target::GoldFind)
    }

    pub fn plus(self, mut aura: Aura) -> Self {
        if let Some(tag) = self.tag {
            aura = aura.with_tag(tag);
        }
        self.with_modifier(Modifier::Plus(Box::new(aura)))
    }

    pub fn minus(self, mut aura: Aura) -> Self {
        if let Some(tag) = self.tag {
            aura = aura.with_tag(tag);
        }
        self.with_modifier(Modifier::Minus(Box::new(aura)))
    }

    pub fn times(self, target: Target) -> Self {
        self.with_modifier(Modifier::Times(target))
    }

    pub fn divided_by(self, target: Target) -> Self {
        self.with_modifier(Modifier::DividedBy(target))
    }

    pub fn to_power_of(self, target: Target) -> Self {
        self.with_modifier(Modifier::ToPowerOf(target))
    }

    pub fn randomly_affecting(self, count: usize) -> Self {
        let target = self.target.clone();
        self.with_modifier(Modifier::RandomlyAffecting(count, target))
    }

    pub fn with_modifier(self, modifier: Modifier) -> Self {
        let modifier = match self.modifier {
            Some(old) => Modifier::Composite(Box::new(old), Box::new(modifier)),
            None => modifier,
        };
        Aura { modifier: Some(modifier), ..self }
    }

    pub fn when_exists(self, target: Target) -> Self {
        self.when(Condition::GtEq(target, 1))
    }

    pub fn when_none(self, target: Target) -> Self {
        self.when(Condition::Eq(target, 0))
    }

    pub fn when(self, condition: Condition) -> Self {
        Aura { condition: Some(condition), ..self }
    }

    pub fn with_tag(self, tag: AuraTag) -> Self {
        Aura {
            tag: Some(tag),
            modifier: self.modifier.map(|m| m.with_tag(tag)),
            ..self
        }
    }

    pub fn requires_active_play(self) -> Self {
        Aura { requires_active_play: true, ..self }
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

    pub fn gold_find(&self, formation: &Formation) -> f64 {
        if !self.is_active_gold_buff(formation) {
            return 0.0;
        }
        self.modifier_amount(formation)
    }

    pub fn is_noop(&self) -> bool {
        match (self.amount, self.modifier.as_ref(), self.tag) {
            (_, Some(&Modifier::Plus(..)), _) => false,
            (0.0, _, None) => true,
            _ => false
        }
    }

    fn applies_to(
        &self,
        crusader: CrusaderName,
        formation: &Formation,
    ) -> bool {
        self.target.matches(crusader, formation) && self.condition_is_met(formation)
    }

    fn is_active_gold_buff(&self, formation: &Formation) -> bool {
        self.target == Target::GoldFind && self.condition_is_met(formation)
    }

    fn modifier_amount(&self, formation: &Formation) -> f64 {
        let amount = self.tag
            .map(|t| formation.ability_buffs(t).fold(self.amount, |r, m| m.modify(r)))
            .unwrap_or(self.amount);
        self.modifier.as_ref().map(|m| m.apply(amount, formation))
            .unwrap_or(amount)
    }

    fn condition_is_met(&self, formation: &Formation) -> bool {
        self.condition.as_ref().map(|c| c.is_met(formation)).unwrap_or(true)
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

    pub fn is_noop(&self) -> bool {
        self.amount == 0.0
    }

    pub fn when_exists(self, target: Target) -> Self {
        self.when(Condition::GtEq(target, 1))
    }

    pub fn when_none(self, target: Target) -> Self {
        self.when(Condition::Eq(target, 0))
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
            requires_active_play: false,
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

    // Slot 3
    LoneWolf,

    // Slot 4
    Bulwark,
    EligibleReceivers,
    Gunslinger,
    EyeCandy,
    PennyInYourPocket,

    // Slot 5
    MoshPit,
    StillSuspicious,
    Untrusting,

    // Slot 6
    AHah,
    LittlePockets,

    // Slot 7
    Ignite,
    Diversity,
    AncientHatred,
    TheOldGuard,

    // Slot 8
    DoubleDragon,
    ActOfCongress,
    UsVsThem,

    // Slot 9
    Ambush,
    InspiringPresence,

    // Slot 10
    Roar,
    LooseMagic,

    // Slot 11
    KoffeePotion,
    FrogSoup,
    PlayingFavorites,

    // Slot 12
    UnderMyWing,
    Groupies,
    HesGotAGunToo,
    JustInTime,
    TurnTheTides,

    // Slot 13
    FavoritePrey,
    FocusedSupport,

    // Slot 14
    GoldORama,

    // Slot 16
    StormOfFlame,
    HeartOfThePhoenix,
    LadiesSpaceMan,
    KirkinItUp,

    // Slot 17
    RoyalGrail,
    RoyalPast,

    // Slot 18
    StormRider,

    // Slot 19
    DeflectEvil,
    Alchemy,
    SmartInvesting,
    BatOLevel,
    Sidekicks,
    ToughNutToCrack,
    InstantRegret,

    // Slot 21
    SpareParts,
    SlavedSystems,

    // Slot 22
    TheShadowsCowl,

    // Slot 23
    Plunder,
}

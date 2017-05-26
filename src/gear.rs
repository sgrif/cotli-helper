use dps::Level;

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum GearQuality {
    None,
    Common,
    Uncommon,
    Rare,
    Epic,
    GoldenEpic,
    Legendary(Level),
    GoldenLegendary(Level),
}

impl Default for GearQuality {
    fn default() -> Self {
        GearQuality::None
    }
}

impl GearQuality {
    pub fn legendary_level(&self) -> Option<u16> {
        match *self {
            GearQuality::Legendary(Level(lvl)) |
            GearQuality::GoldenLegendary(Level(lvl)) => Some(lvl),
            _ => None,
        }
    }
}

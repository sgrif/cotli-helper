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

use std::fmt;

use serde::de::*;
use dps::Level;

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug, Hash)]
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

impl<'de> Deserialize<'de> for GearQuality {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(GearQualityVisitor)
    }
}

struct GearQualityVisitor;

static KNOWN_VARIANTS: &'static [&'static str] = &[
    "None",
    "Uncommon",
    "Rare",
    "Epic",
    "Legendary",
    "Golden Legendary",
];

static VALID_FIELDS: &'static [&'static str] = &["kind", "level"];
static ONLY_KIND: &'static [&'static str] = &["kind"];

impl<'de> Visitor<'de> for GearQualityVisitor {
    type Value = GearQuality;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a gear variant such as \"Rare\"")
    }

    fn visit_str<E: Error>(self, s: &str) -> Result<Self::Value, E> where
        E: Error,
    {
        match s {
            "None" => Ok(GearQuality::None),
            "Uncommon" => Ok(GearQuality::Uncommon),
            "Rare" => Ok(GearQuality::Rare),
            "Epic" => Ok(GearQuality::Epic),
            "Golden Epic" => Ok(GearQuality::GoldenEpic),
            "Legendary" => Ok(GearQuality::Legendary(Level(1))),
            "Golden Legendary" =>
                Ok(GearQuality::GoldenLegendary(Level(1))),
            _ => Err(E::unknown_variant(s, KNOWN_VARIANTS)),
        }
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where
        A: MapAccess<'de>,
    {
        let mut kind = None::<&str>;
        let mut level = None::<u16>;
        // Grab the first key
        match (map.next_key()?, kind, level) {
            (Some("kind"), None, _) => kind = map.next_value()?,
            (Some("kind"), Some(_), _) => return Err(A::Error::duplicate_field("kind")),
            (Some("level"), _, None) => level = map.next_value()?,
            (Some("level"), _, Some(_)) => return Err(A::Error::duplicate_field("level")),
            (None, ..) => {},
            (Some(s), ..) => return Err(A::Error::unknown_field(s, VALID_FIELDS)),
        }
        // Grab the second key if present
        match (map.next_key()?, kind, level) {
            (Some("kind"), None, _) => kind = map.next_value()?,
            (Some("kind"), Some(_), _) => return Err(A::Error::duplicate_field("kind")),
            (Some("level"), _, None) => level = map.next_value()?,
            (Some("level"), _, Some(_)) => return Err(A::Error::duplicate_field("level")),
            (None, ..) => {},
            (Some(s), ..) => return Err(A::Error::unknown_field(s, VALID_FIELDS)),
        }
        // Ensure no remaining fields
        match map.next_key()? {
            Some("kind") => return Err(A::Error::duplicate_field("kind")),
            Some("level") => return Err(A::Error::duplicate_field("level")),
            Some(s) => return Err(A::Error::unknown_field(s, VALID_FIELDS)),
            None => {},
        }

        match (kind, level) {
            // Handle legendaries
            (Some("Golden Legendary"), level) =>
                Ok(GearQuality::GoldenLegendary(Level(level.unwrap_or(1)))),
            (Some("Legendary"), level) =>
                Ok(GearQuality::Legendary(Level(level.unwrap_or(1)))),

            // Error cases for non-legendary
            (_, Some(_)) => Err(A::Error::unknown_field("level", ONLY_KIND)),
            (None, _) => Err(A::Error::missing_field("kind")),

            // Non-legendary
            (Some("None"), None) => Ok(GearQuality::None),
            (Some("Uncommon"), None) => Ok(GearQuality::Uncommon),
            (Some("Rare"), None) => Ok(GearQuality::Rare),
            (Some("Epic"), None) => Ok(GearQuality::Epic),
            (Some("Golden Epic"), None) => Ok(GearQuality::GoldenEpic),

            (Some(s), None) => Err(A::Error::unknown_variant(s, KNOWN_VARIANTS)),
        }
    }
}

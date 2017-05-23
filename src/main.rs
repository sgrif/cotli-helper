#![allow(dead_code)]
#![cfg_attr(bench, feature(test))]
#![feature(conservative_impl_trait)]

#[macro_use] extern crate bitflags;
extern crate itertools;
extern crate ordermap;
extern crate rand;

pub mod crusader;
pub mod dps;
pub mod formation;
pub mod formation_search;
pub mod gear;
pub mod talent;
pub mod user_data;

use formation_search::*;
use crusader::*;
use dps::*;
use formation::*;
use gear::*;
use talent::*;
use user_data::*;

fn main() {
    let positions = vec![
        Coordinate::new(0, 0),
        Coordinate::new(0, 1),
        Coordinate::new(0, 2),
        Coordinate::new(0, 3),
        Coordinate::new(1, 0),
        Coordinate::new(1, 1),
        Coordinate::new(1, 2),
        Coordinate::new(2, 1),
        Coordinate::new(2, 2),
        Coordinate::new(3, 1),
    ];
    let formation = Formation::empty(positions);
    let crusaders = create_user_data().unlocked_crusaders();
    let mut search = FormationSearch::new(formation, &crusaders);
    search.perform_search(::std::time::Duration::from_secs(5));
    search.formation().print();
}

fn create_user_data() -> UserData {
    UserData::default()
        .with_dps_from_rings(455.0)
        .with_cooldown_percent(177.0)
        .with_dps_from_achievements(381.0)
        .with_unspent_idols(779)
        .level_talent(Talent::SpeedRunner, 10)
        .level_talent(Talent::RideTheStorm, 4)
        .level_talent(Talent::StormsBuilding, 6)
        .level_talent(Talent::PassiveCriticals, 22)
        .level_talent(Talent::SurplusCooldown, 19)
        .level_talent(Talent::Overenchanted, 9)
        .level_talent(Talent::SetBonus, 9)
        .level_talent(Talent::FastLearners, 2)
        .level_talent(Talent::WellEquipped, 4)
        .level_talent(Talent::SwapDay, 3)
        .level_talent(Talent::SpendItAll, 1)
        .level_talent(Talent::UpgradeThemAll, 1)
        .level_talent(Talent::Scavenger, 1)
        .level_talent(Talent::SpeedLooter, 1)
        .level_talent(Talent::EfficientCrusading, 25)
        .level_talent(Talent::DoingItAgain, 1)
        .level_talent(Talent::DeepIdolScavenger, 13)
        .level_talent(Talent::ExtraTraining, 2)
        .level_talent(Talent::TripleTierTrouble, 1)
        .add_crusader(CrusaderName::JimTheLumberjack, CrusaderData {
            enchantment_points: 77,
            gear: [GearQuality::Rare, GearQuality::Epic, GearQuality::Epic],
        }).add_crusader(CrusaderName::Arachnobuddy, CrusaderData {
            enchantment_points: 47,
            gear: [GearQuality::Rare, GearQuality::Rare, GearQuality::Rare],
        }).add_crusader(CrusaderName::VeronicaTheAndroidArcher, CrusaderData {
            enchantment_points: 4,
            gear: [GearQuality::Legendary(Level(3)), GearQuality::Rare, GearQuality::Rare],
        }).add_crusader(CrusaderName::SallyTheSuccubus, CrusaderData {
            enchantment_points: 153,
            gear: [
                GearQuality::GoldenLegendary(Level(3)),
                GearQuality::Legendary(Level(3)),
                GearQuality::Legendary(Level(2)),
            ],
        }).add_crusader(CrusaderName::EmoWerewolf, CrusaderData {
            enchantment_points: 176,
            gear: [GearQuality::Epic, GearQuality::Epic, GearQuality::Rare],
        }).add_crusader(CrusaderName::SashaTheFierceWarrior, CrusaderData {
            enchantment_points: 109,
            gear: [GearQuality::Rare, GearQuality::Epic, GearQuality::Legendary(Level(2))],
        }).add_crusader(CrusaderName::GroklokTheOrc, CrusaderData {
            enchantment_points: 41,
            gear: [GearQuality::GoldenEpic, GearQuality::Epic, GearQuality::Rare],
        }).add_crusader(CrusaderName::TheWashedUpHermit, CrusaderData {
            enchantment_points: 145,
            gear: [GearQuality::Rare, GearQuality::Rare, GearQuality::Rare],
        }).add_crusader(CrusaderName::KyleThePartyBro, CrusaderData {
            enchantment_points: 109,
            gear: [GearQuality::Rare, GearQuality::Rare, GearQuality::Uncommon],
        }).add_crusader(CrusaderName::Grandmora, CrusaderData {
            enchantment_points: 0,
            gear: [GearQuality::Epic, GearQuality::None, GearQuality::Rare],
        }).add_crusader(CrusaderName::DetectiveKaine, CrusaderData {
            enchantment_points: 82,
            gear: [GearQuality::Epic, GearQuality::Rare, GearQuality::Epic],
        }).add_crusader(CrusaderName::LarryTheLeprechaun, CrusaderData {
            enchantment_points: 13,
            gear: [GearQuality::Rare, GearQuality::Legendary(Level(2)), GearQuality::Rare],
        }).add_crusader(CrusaderName::BaenarallAngelOfHope, CrusaderData {
            enchantment_points: 0,
            gear: [GearQuality::Legendary(Level(1)), GearQuality::Uncommon, GearQuality::Rare],
        }).add_crusader(CrusaderName::ThePrincess, CrusaderData {
            enchantment_points: 85,
            gear: [GearQuality::Rare, GearQuality::Rare, GearQuality::Rare],
        }).add_crusader(CrusaderName::NatalieDragon, CrusaderData {
            enchantment_points: 69,
            gear: [
                GearQuality::Legendary(Level(2)),
                GearQuality::Legendary(Level(2)),
                GearQuality::Legendary(Level(1)),
            ],
        }).add_crusader(CrusaderName::PresidentBillySmithsonian, CrusaderData {
            enchantment_points: 0,
            gear: [GearQuality::Rare, GearQuality::Legendary(Level(2)), GearQuality::Epic],
        }).add_crusader(CrusaderName::JasonMasterOfShadows, CrusaderData {
            enchantment_points: 82,
            gear: [GearQuality::Epic, GearQuality::Rare, GearQuality::Epic],
        }).add_crusader(CrusaderName::Broot, CrusaderData {
            enchantment_points: 70,
            gear: [GearQuality::Uncommon, GearQuality::Uncommon, GearQuality::Uncommon],
        }).add_crusader(CrusaderName::ArtaxesTheLion, CrusaderData {
            enchantment_points: 94,
            gear: [
                GearQuality::Legendary(Level(3)),
                GearQuality::Legendary(Level(1)),
                GearQuality::Legendary(Level(3)),
            ],
        })
}

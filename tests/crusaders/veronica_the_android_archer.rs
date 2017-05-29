use cotli_helper::crusader::CrusaderName::VeronicaTheAndroidArcher;
use cotli_helper::crusader::{Crusader, Tags, ROBOT};
use cotli_helper::gear::GearQuality;
use cotli_helper::user_data::*;
use support::*;

#[test]
fn veronica_has_correct_base_dps() {
    let veronica = default_crusader(VeronicaTheAndroidArcher);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &veronica);
    assert_formation_dps!("2.07e4", formation);
}

#[test]
fn veronica_buffs_crusaders_in_same_column() {
    let dummy_crusader = Crusader::dummy(Tags::empty());
    let veronica = default_crusader(VeronicaTheAndroidArcher).at_level(0);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &dummy_crusader);
    assert_formation_dps!("100", formation);
    formation.place_crusader(3, &veronica);
    assert_formation_dps!("172", formation);
}

#[test]
fn precise_aim_affects_adjacent() {
    let dummy_crusader = Crusader::dummy(Tags::empty());
    let veronica = default_crusader(VeronicaTheAndroidArcher).at_level(0);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &dummy_crusader);
    assert_formation_dps!("100", formation);
    formation.place_crusader(4, &veronica);
    assert_formation_dps!("172", formation);
}

#[test]
fn precise_aim_is_not_global() {
    let dummy_crusader = Crusader::dummy(Tags::empty());
    let veronica = default_crusader(VeronicaTheAndroidArcher).at_level(0);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &dummy_crusader);
    assert_formation_dps!("100", formation);
    formation.place_crusader(9, &veronica);
    assert_formation_dps!("114", formation); // Floating point rounding error
}

#[test]
fn precise_aim_additively_affected_by_robots() {
    let dummy_dps = Crusader::dummy(Tags::empty());
    let dummy_robot = Crusader::dummy(ROBOT).at_level(0);
    let veronica = default_crusader(VeronicaTheAndroidArcher).at_level(0);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &dummy_dps);
    assert_formation_dps!("100", formation);
    formation.place_crusader(1, &veronica);
    assert_formation_dps!("172", formation);
    formation.place_crusader(9, &dummy_robot);
    assert_formation_dps!("201", formation);
    formation.place_crusader(8, &dummy_robot);
    assert_formation_dps!("229", formation); // Floating point rounding error
}

#[test]
fn precise_aim_scales_correctly_with_gear() {
    let dummy_dps = Crusader::dummy(Tags::empty());
    let dummy_robot = Crusader::dummy(ROBOT).at_level(0);
    let veronica = UserData::default()
        .add_crusader(VeronicaTheAndroidArcher, CrusaderData {
            gear: [GearQuality::Common, GearQuality::None, GearQuality::None],
            ..Default::default()
        }).unlocked_crusaders(None)
        .into_iter()
        .find(|c| c.name == VeronicaTheAndroidArcher)
        .unwrap()
        .at_level(0);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &dummy_dps);
    assert_formation_dps!("100", formation);
    formation.place_crusader(1, &veronica);
    assert_formation_dps!("178", formation);
    formation.place_crusader(9, &dummy_robot);
    assert_formation_dps!("209", formation);
    formation.place_crusader(8, &dummy_robot);
    assert_formation_dps!("241", formation);
}

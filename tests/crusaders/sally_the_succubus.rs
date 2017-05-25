use cotli_helper::crusader::{Crusader, MALE, FEMALE};
use cotli_helper::crusader::CrusaderName::SallyTheSuccubus;
use support::*;

#[test]
fn sally_loses_no_dps_from_adjacent_males() {
    let dummy_male = Crusader::dummy(MALE);
    let sally = default_crusader(SallyTheSuccubus);
    let mut formation = worlds_wake();

    formation.place_crusader(4, &sally);
    assert_formation_dps!("1.62e6", formation);
    formation.place_crusader(0, &dummy_male);
    assert_formation_dps!("1.62e6", formation);
}

#[test]
fn sally_loses_dps_from_adjacent_non_males() {
    let dummy_non_male = Crusader::dummy(FEMALE);
    let sally = default_crusader(SallyTheSuccubus);
    let mut formation = worlds_wake();

    formation.place_crusader(4, &sally);
    assert_formation_dps!("1.62e6", formation);
    formation.place_crusader(0, &dummy_non_male);
    assert_formation_dps!("1.52e6", formation);
}

#[test]
fn sally_buffs_all_females() {
    let dummy_female = Crusader::dummy(FEMALE);
    let sally = default_crusader(SallyTheSuccubus).at_level(0);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &dummy_female);
    assert_formation_dps!("100", formation);
    formation.place_crusader(9, &sally);
    assert_formation_dps!("120", formation);
}

#[test]
fn sally_does_not_buff_males() {
    let dummy_male = Crusader::dummy(MALE);
    let sally = default_crusader(SallyTheSuccubus).at_level(0);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &dummy_male);
    assert_formation_dps!("100", formation);
    formation.place_crusader(9, &sally);
    assert_formation_dps!("100", formation);
}

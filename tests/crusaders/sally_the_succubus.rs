use cotli_helper::dps::Level;
use cotli_helper::crusader::CrusaderName::*;
use support::*;

#[test]
fn sally_loses_no_dps_from_adjacent_males() {
    let dummy_male = default_crusader(TheBushWhacker);
    let sally = default_crusader(SallyTheSuccubus);
    let mut formation = worlds_wake();

    formation.place_crusader(4, &sally);
    assert_formation_dps!("1.62e6", formation);
    formation.place_crusader(0, &dummy_male);
    assert_formation_dps!("1.62e6", formation);
}

#[test]
fn sally_loses_dps_from_adjacent_females() {
    let dummy_female = default_crusader(SashaTheFierceWarrior).at_level(Level(0));
    let sally = default_crusader(SallyTheSuccubus);
    let mut formation = worlds_wake();

    formation.place_crusader(4, &sally);
    assert_formation_dps!("1.62e6", formation);
    formation.place_crusader(0, &dummy_female);
    assert_formation_dps!("1.52e6", formation);
}

#[test]
fn sally_buffs_all_females() {
    let dummy_female = default_crusader(SashaTheFierceWarrior);
    let sally = default_crusader(SallyTheSuccubus).at_level(Level(0));
    let mut formation = worlds_wake();

    formation.place_crusader(0, &dummy_female);
    // assert_formation_dps!("1.42e6", formation);
    assert_formation_dps!("7.10e4", formation);
    formation.place_crusader(9, &sally);
    // assert_formation_dps!("1.70e6", formation);
    assert_formation_dps!("8.52e4", formation);
}

#[test]
fn sally_does_not_buff_males() {
    let dummy_male = default_crusader(JimTheLumberjack);
    let sally = default_crusader(SallyTheSuccubus).at_level(Level(0));
    let mut formation = worlds_wake();

    formation.place_crusader(0, &dummy_male);
    assert_formation_dps!("7.20e4", formation);
    formation.place_crusader(9, &sally);
    assert_formation_dps!("7.20e4", formation);
}

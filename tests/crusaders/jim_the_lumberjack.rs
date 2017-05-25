use cotli_helper::dps::Level;
use cotli_helper::crusader::CrusaderName::*;
use support::*;

#[test]
fn jims_dps_increases_when_crusader_is_adjacent() {
    let dummy_crusader = default_crusader(TheBushWhacker);
    let jim = default_crusader(JimTheLumberjack);
    let mut formation = worlds_wake();

    formation.place_crusader(3, &jim);
    assert_formation_dps!("7.20e4", formation);
    formation.place_crusader(6, &dummy_crusader);
    assert_formation_dps!("1.44e5", formation);
}

#[test]
fn jims_dps_not_affected_by_non_adjacent_crusaders() {
    let dummy_crusader = default_crusader(TheBushWhacker);
    let jim = default_crusader(JimTheLumberjack);
    let mut formation = worlds_wake();

    formation.place_crusader(3, &jim);
    assert_formation_dps!("7.20e4", formation);
    formation.place_crusader(5, &dummy_crusader);
    assert_formation_dps!("7.20e4", formation);
}

#[test]
fn jim_increases_dps_of_crusaders_in_same_column() {
    let sally = default_crusader(SallyTheSuccubus);
    let jim = default_crusader(JimTheLumberjack).at_level(Level(0));
    let mut formation = worlds_wake();

    formation.place_crusader(0, &sally);
    // assert_formation_dps!("1.62e6", formation);
    assert_formation_dps!("2.11e4", formation);
    formation.place_crusader(1, &jim);
    // assert_formation_dps!("2.43e6", formation);
    assert_formation_dps!("3.16e4", formation);
    formation.remove_crusader(1);
    formation.place_crusader(4, &jim);
    assert_formation_dps!("2.11e4", formation);
}

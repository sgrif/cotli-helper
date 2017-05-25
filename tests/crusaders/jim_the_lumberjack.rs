use cotli_helper::crusader::{Crusader, Tags};
use cotli_helper::crusader::CrusaderName::JimTheLumberjack;
use support::*;

#[test]
fn jims_dps_increases_when_crusader_is_adjacent() {
    let dummy_crusader = Crusader::dummy(Tags::empty());
    let jim = default_crusader(JimTheLumberjack);
    let mut formation = worlds_wake();

    formation.place_crusader(3, &jim);
    assert_formation_dps!("7.20e4", formation);
    formation.place_crusader(6, &dummy_crusader);
    assert_formation_dps!("1.44e5", formation);
}

#[test]
fn jims_dps_not_affected_by_non_adjacent_crusaders() {
    let dummy_crusader = Crusader::dummy(Tags::empty()).at_level(0);
    let jim = default_crusader(JimTheLumberjack);
    let mut formation = worlds_wake();

    formation.place_crusader(3, &jim);
    assert_formation_dps!("7.20e4", formation);
    formation.place_crusader(5, &dummy_crusader);
    assert_formation_dps!("7.20e4", formation);
}

#[test]
fn jim_increases_dps_of_crusaders_in_same_column() {
    let dummy_dps = Crusader::dummy(Tags::empty());
    let jim = default_crusader(JimTheLumberjack).at_level(0);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &dummy_dps);
    assert_formation_dps!("100", formation);
    formation.place_crusader(2, &jim);
    assert_formation_dps!("150", formation);
    formation.remove_crusader(2);
    formation.place_crusader(4, &jim);
    assert_formation_dps!("100", formation);
}

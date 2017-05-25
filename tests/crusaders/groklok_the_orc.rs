use cotli_helper::crusader::{Crusader, Tags};
use cotli_helper::crusader::CrusaderName::GroklokTheOrc;
use cotli_helper::formation::*;
use support::*;

#[test]
fn groklok_has_correct_base_dps() {
    let groklok = default_crusader(GroklokTheOrc);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &groklok);
    assert_formation_dps!("2.77e6", formation);
}

#[test]
fn groklok_loses_gunslinger_when_formation_full() {
    let dummy_crusader = Crusader::dummy(Tags::empty());
    let groklok = default_crusader(GroklokTheOrc);
    let mut formation = Formation::empty(vec![
        Coordinate::new(0, 0),
        Coordinate::new(0, 1),
        Coordinate::new(1, 0),
    ]);

    formation.place_crusader(0, &groklok);
    assert_formation_dps!("2.77e6", formation);
    formation.place_crusader(1, &dummy_crusader);
    assert_formation_dps!("2.77e6", formation);
    formation.place_crusader(2, &dummy_crusader);
    assert_formation_dps!("1.11e6", formation);
    formation.remove_crusader(1);
    assert_formation_dps!("2.77e6", formation);
}

#[test]
fn groklok_buffs_crusaders_in_front_divided_by_number_affected() {
    let dummy_dps = Crusader::dummy(Tags::empty());
    let dummy_buffer = Crusader::dummy(Tags::all()).at_level(0);
    let groklok = default_crusader(GroklokTheOrc).at_level(0);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &groklok);
    formation.place_crusader(1, &dummy_dps);
    assert_formation_dps!("100", formation);
    formation.remove_crusader(1);
    formation.place_crusader(7, &dummy_dps);
    assert_formation_dps!("100", formation);
    formation.remove_crusader(7);
    formation.place_crusader(6, &dummy_dps);
    assert_formation_dps!("250", formation);
    formation.place_crusader(5, &dummy_buffer);
    assert_formation_dps!("175", formation);
    formation.remove_crusader(5);
    formation.place_crusader(7, &dummy_buffer);
    assert_formation_dps!("250", formation);
}

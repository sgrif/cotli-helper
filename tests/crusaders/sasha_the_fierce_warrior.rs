use cotli_helper::crusader::{Crusader, Tags};
use cotli_helper::crusader::CrusaderName::SashaTheFierceWarrior;
use support::*;

#[test]
fn sasha_has_correct_base_dps() {
    let sasha = default_crusader(SashaTheFierceWarrior);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &sasha);
    assert_formation_dps!("1.42e6", formation);
}

#[test]
fn sasha_buffs_crusaders_immediately_behind() {
    test_dps_with_sasha_placed_at_position("130", 7);
}

#[test]
fn sasha_buffs_non_adjacent() {
    test_dps_with_sasha_placed_at_position("130", 8);
}

#[test]
fn sasha_does_not_buff_same_column() {
    test_dps_with_sasha_placed_at_position("100", 5);
}

#[test]
fn sasha_does_not_buff_in_front() {
    test_dps_with_sasha_placed_at_position("100", 0);
}

#[test]
fn sasha_does_not_buff_two_columns_behind() {
    test_dps_with_sasha_placed_at_position("100", 9);
}

fn test_dps_with_sasha_placed_at_position(dps: &str, pos: usize) {
    let dummy_dps = Crusader::dummy(Tags::empty());
    let sasha = default_crusader(SashaTheFierceWarrior).at_level(0);
    let mut formation = worlds_wake();

    formation.place_crusader(4, &dummy_dps);
    assert_formation_dps!("100", formation);
    formation.place_crusader(pos, &sasha);
    assert_formation_dps!(dps, formation);
}

use cotli_helper::crusader::{Crusader, ANIMAL, FEMALE, LEPRECHAUN, HUMAN};
use cotli_helper::crusader::CrusaderName::KyleThePartyBro;
use support::*;

#[test]
fn kyle_is_buffed_by_adjacent_crusaders_with_special_tags() {
    let animal = Crusader::dummy(ANIMAL);
    let female = Crusader::dummy(FEMALE);
    let leprechaun = Crusader::dummy(LEPRECHAUN);
    let human = Crusader::dummy(HUMAN);
    let kyle = default_crusader(KyleThePartyBro);
    let mut formation = worlds_wake();

    formation.place_crusader(1, &kyle);
    assert_formation_dps!("7.03e5", formation);
    formation.place_crusader(0, &animal);
    assert_formation_dps!("2.11e6", formation);
    formation.place_crusader(2, &female);
    assert_formation_dps!("5.62e6", formation);
    formation.place_crusader(4, &leprechaun);
    assert_formation_dps!("1.40e7", formation);
    formation.remove_crusader(4);
    formation.place_crusader(4, &human);
    assert_formation_dps!("7.03e6", formation);
    formation.place_crusader(6, &leprechaun);
    assert_formation_dps!("7.03e6", formation);
}

#[test]
fn multiple_buffs_can_come_from_single_crusader() {
    let dummy_one = Crusader::dummy(ANIMAL | FEMALE);
    let dummy_two = Crusader::dummy(LEPRECHAUN | FEMALE);
    let kyle = default_crusader(KyleThePartyBro);
    let mut formation = worlds_wake();

    formation.place_crusader(1, &kyle);
    assert_formation_dps!("7.03e5", formation);
    formation.place_crusader(0, &dummy_one);
    assert_formation_dps!("4.22e6", formation);
    formation.place_crusader(2, &dummy_two);
    assert_formation_dps!("1.12e7", formation);
}

#[test]
fn kyle_causes_adjacent_crusaders_to_get_smashed() {
    let dummy_one = Crusader::dummy(ANIMAL);
    let dummy_two = Crusader::dummy(HUMAN);
    let kyle = default_crusader(KyleThePartyBro).at_level(0);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &dummy_one);
    assert_formation_dps!("100", formation);
    formation.place_crusader(1, &kyle);
    assert_formation_dps!("150", formation);
    formation.place_crusader(9, &dummy_two);
    assert_formation_dps!("270", formation);
}

#[test]
fn more_crusaders_than_can_be_smashed_averages_bonus() {
    let dummy_one = Crusader::dummy(ANIMAL);
    let dummy_two = Crusader::dummy(HUMAN);
    let dummy_three = Crusader::dummy(LEPRECHAUN).at_level(0);
    let dummy_four = Crusader::dummy(FEMALE).at_level(0);
    let kyle = default_crusader(KyleThePartyBro).at_level(0);
    let mut formation = worlds_wake();

    formation.place_crusader(1, &kyle);
    formation.place_crusader(0, &dummy_one);
    assert_formation_dps!("150", formation);
    formation.place_crusader(2, &dummy_two);
    assert_formation_dps!("300", formation);
    formation.place_crusader(4, &dummy_three);
    assert_formation_dps!("300", formation);
    formation.place_crusader(5, &dummy_four);
    assert_formation_dps!("285", formation);
}

#[test]
fn kyle_has_max_buff_of_number_smashed_crusaders() {
    let dummy_one = Crusader::dummy(ANIMAL).at_level(0);
    let dummy_two = Crusader::dummy(HUMAN).at_level(0);
    let dummy_three = Crusader::dummy(LEPRECHAUN).at_level(0);
    let dummy_four = Crusader::dummy(FEMALE).at_level(0);
    let kyle = default_crusader(KyleThePartyBro);
    let mut formation = worlds_wake();

    formation.place_crusader(1, &kyle);
    formation.place_crusader(0, &dummy_one);
    assert_formation_dps!("2.11e6", formation);
    formation.place_crusader(2, &dummy_two);
    assert_formation_dps!("2.81e6", formation);
    formation.place_crusader(4, &dummy_three);
    assert_formation_dps!("7.03e6", formation);
    formation.place_crusader(5, &dummy_four);
    assert_formation_dps!("1.40e7", formation);
}

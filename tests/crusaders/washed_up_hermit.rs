use cotli_helper::crusader::{Crusader, Tags, HUMAN, FEMALE};
use cotli_helper::crusader::CrusaderName::TheWashedUpHermit;
use support::*;

#[test]
fn hermit_has_correct_base_dps() {
    let hermit = default_crusader(TheWashedUpHermit);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &hermit);
    assert_formation_dps!("2.81e7", formation);
}

#[test]
fn hermit_loses_craziness_when_crusader_in_front() {
    let dummy = Crusader::dummy(Tags::empty());
    let hermit = default_crusader(TheWashedUpHermit);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &hermit);
    assert_formation_dps!("2.81e7", formation);
    formation.place_crusader(4, &dummy);
    assert_formation_dps!("9.38e6", formation);
}

#[test]
fn craziness_unaffected_by_non_adjecent_or_non_in_front() {
    let dummy = Crusader::dummy(Tags::empty());
    let dummy2 = Crusader::dummy(HUMAN);
    let dummy3 = Crusader::dummy(FEMALE);
    let hermit = default_crusader(TheWashedUpHermit);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &hermit);
    assert_formation_dps!("2.81e7", formation);
    formation.place_crusader(1, &dummy);
    assert_formation_dps!("2.81e7", formation);
    formation.place_crusader(5, &dummy2);
    assert_formation_dps!("2.81e7", formation);
    formation.place_crusader(7, &dummy3);
    assert_formation_dps!("2.81e7", formation);
}

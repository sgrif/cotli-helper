use cotli_helper::crusader::{Crusader, Tags, HUMAN, ANIMAL, MALE};
use cotli_helper::crusader::CrusaderName::SisaronTheDragonSorceress;
use support::*;

#[test]
fn loose_magic_is_buffed_when_at_least_4_crusaders() {
    let dummy_dps = Crusader::dummy(Tags::empty());
    let dummy_crusader1 = Crusader::dummy(HUMAN).at_level(0);
    let dummy_crusader2 = Crusader::dummy(ANIMAL).at_level(0);
    let dummy_crusader3 = Crusader::dummy(MALE).at_level(0);
    let dummy_crusader4 = Crusader::dummy(Tags::all()).at_level(0);
    let sisaron = default_crusader(SisaronTheDragonSorceress).at_level(0);
    let mut formation = worlds_wake();

    formation.place_crusader(5, &sisaron);
    formation.place_crusader(1, &dummy_dps);
    assert_formation_dps!("220", formation);
    formation.place_crusader(2, &dummy_crusader1);
    assert_formation_dps!("165", formation);
    formation.place_crusader(4, &dummy_crusader2);
    assert_formation_dps!("146", formation);
    formation.place_crusader(6, &dummy_crusader3);
    assert_formation_dps!("220", formation);
    formation.place_crusader(7, &dummy_crusader4);
    assert_formation_dps!("198", formation);
}

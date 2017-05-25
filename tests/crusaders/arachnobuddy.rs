use cotli_helper::crusader::CrusaderName::Arachnobuddy;
use support::*;

#[test]
fn arachnobuddy_has_correct_base_dps() {
    let arachnobuddy = default_crusader(Arachnobuddy);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &arachnobuddy);
    assert_formation_dps!("2.40e4", formation);
}

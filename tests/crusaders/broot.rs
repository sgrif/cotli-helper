use cotli_helper::crusader::CrusaderName::Broot;
use support::*;

#[test]
fn broot_has_correct_base_dps() {
    let broot = default_crusader(Broot);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &broot);
    assert_formation_dps!("9.95e7", formation);
}

#[test]
fn broot_is_buffed_in_front_row() {
    let broot = default_crusader(Broot);
    let mut formation = worlds_wake();

    formation.place_crusader(9, &broot);
    assert_formation_dps!("1.24e8", formation);
}

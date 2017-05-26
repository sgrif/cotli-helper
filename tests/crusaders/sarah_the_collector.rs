use cotli_helper::crusader::{Crusader, Tags};
use cotli_helper::crusader::CrusaderName::SarahTheCollector;
use cotli_helper::formation::*;
use support::*;

#[test]
fn sarah_has_correct_base_dps() {
    let sarah = default_crusader(SarahTheCollector);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &sarah);
    assert_formation_dps!("3.26e10", formation);
}

#[test]
fn sarah_is_buffed_by_full_formation() {
    let dummy = Crusader::dummy(Tags::empty());
    let sarah = default_crusader(SarahTheCollector);
    let mut formation = Formation::empty(vec![
        Coordinate::new(0, 0),
        Coordinate::new(0, 1),
        Coordinate::new(0, 2),
    ]);

    formation.place_crusader(0, &sarah);
    assert_formation_dps!("3.26e10", formation);
    formation.place_crusader(1, &dummy);
    assert_formation_dps!("3.26e10", formation);
    formation.place_crusader(2, &dummy);
    assert_formation_dps!("8.16e10", formation);
}

use cotli_helper::crusader::{Crusader, Tags, HUMAN};
use cotli_helper::crusader::CrusaderName::EmoWerewolf;
use support::*;

#[test]
fn emo_werewolf_has_correct_base_dps() {
    let emo_werewolf = default_crusader(EmoWerewolf);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &emo_werewolf);
    assert_formation_dps!("1.26e6", formation);
}

#[test]
fn emo_werewolf_loses_lone_wolf_when_human_adjacent() {
    let dummy_human = Crusader::dummy(HUMAN);
    let emo_werewolf = default_crusader(EmoWerewolf);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &emo_werewolf);
    assert_formation_dps!("1.26e6", formation);
    formation.place_crusader(1, &dummy_human);
    assert_formation_dps!("4.22e5", formation);
    formation.remove_crusader(1);
    formation.place_crusader(4, &dummy_human);
    assert_formation_dps!("4.22e5", formation);
}

#[test]
fn lone_wolf_not_affected_by_non_human_or_non_adjacent() {
    let dummy_crusader = Crusader::dummy(Tags::empty());
    let dummy_human = Crusader::dummy(HUMAN);
    let emo_werewolf = default_crusader(EmoWerewolf);
    let mut formation = worlds_wake();

    formation.place_crusader(0, &emo_werewolf);
    assert_formation_dps!("1.26e6", formation);
    formation.place_crusader(1, &dummy_crusader);
    formation.place_crusader(9, &dummy_human);
    assert_formation_dps!("1.26e6", formation);
}

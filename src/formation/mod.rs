mod coordinate;

pub use self::coordinate::Coordinate;

use crusader::Crusader;
use dps::Level;

pub struct Formation<'a> {
    slots: Vec<FormationSlot<'a>>,
}

impl<'a> Formation<'a> {
    pub fn empty<I: IntoIterator<Item=Coordinate>>(slots: I) -> Self {
        let slots = slots.into_iter().map(FormationSlot::empty).collect();
        Formation { slots }
    }
}

pub struct FormationSlot<'a> {
    coordinate: Coordinate,
    crusader: Option<&'a Crusader>,
}

impl<'a> FormationSlot<'a> {
    fn empty(coordinate: Coordinate) -> Self {
        FormationSlot {
            coordinate,
            crusader: None,
        }
    }
}

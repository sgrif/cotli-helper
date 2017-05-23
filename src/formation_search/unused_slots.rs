use formation::Formation;
use crusader::{Crusader, Slot};

pub fn unused_slots<'a>(
    formation: &Formation,
    crusaders: &'a [Crusader],
) -> impl Iterator<Item=&'a Crusader> + Clone {
    UnusedSlots {
        crusaders: crusaders.iter(),
        used_slots: formation.used_slots(),
    }
}

#[derive(Debug, Clone, Copy)]
struct UnusedSlots<T> {
    crusaders: T,
    used_slots: Slot,
}

impl<'a, T: Iterator<Item=&'a Crusader>> Iterator for UnusedSlots<T> {
    type Item = &'a Crusader;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.crusaders.next() {
            if !self.used_slots.contains(item.slot()) {
                return Some(item);
            }
        }
        None
    }
}

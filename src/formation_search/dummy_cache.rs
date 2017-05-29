use super::*;

pub struct Cache;

impl Cache {
    pub fn new(_: u64) -> Self {
        Cache
    }

    pub(super) fn write_to_cache(&self, _: &Formation, _: &Node) {
    }

    pub(super) fn load_from_cache<'a>(
        &self,
        _: &Formation<'a>,
        _: &'a [Crusader],
    ) -> Option<Node<'a>> {
        None
    }
}

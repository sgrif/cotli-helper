use ordermap::OrderMap;
use std::hash::Hash;

pub trait SetLike<T> {
    fn contains(&self, elem: &T) -> bool;
    fn len(&self) -> usize;
}

pub struct EmptySet;

impl<T> SetLike<T> for EmptySet {
    fn contains(&self, _: &T) -> bool {
        false
    }

    fn len(&self) -> usize {
        0
    }
}

impl<K, V> SetLike<K> for OrderMap<K, V> where
    K: Hash + Eq,
{
    fn contains(&self, elem: &K) -> bool {
        self.contains_key(elem)
    }

    fn len(&self) -> usize {
        self.len()
    }
}

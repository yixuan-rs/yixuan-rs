use std::{collections::HashSet, hash::Hash};

use super::Property;

#[derive(Default)]
pub struct PropertyHashSet<Key> {
    base: HashSet<Key>,
    added_keys: HashSet<Key>,
}

impl<Key> PropertyHashSet<Key>
where
    Key: PartialEq + Eq + Hash + Clone,
{
    pub fn insert(&mut self, key: Key) -> bool {
        if self.base.insert(key.clone()) {
            self.added_keys.insert(key);
            true
        } else {
            false
        }
    }

    pub fn contains(&self, key: &Key) -> bool {
        self.base.contains(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Key> {
        self.base.iter()
    }

    pub fn iter_added_keys(&self) -> impl Iterator<Item = &Key> {
        self.added_keys.iter()
    }

    pub fn clear(&mut self) {
        self.base.clear();
        self.added_keys.clear();
    }
}

impl<Key> Property for PropertyHashSet<Key> {
    fn is_changed(&self) -> bool {
        !self.added_keys.is_empty()
    }

    fn reset_changed_state(&mut self) {
        self.added_keys.clear();
    }
}

impl<Key> FromIterator<Key> for PropertyHashSet<Key>
where
    Key: PartialEq + Eq + Hash,
{
    fn from_iter<T: IntoIterator<Item = Key>>(iter: T) -> Self {
        Self {
            base: HashSet::from_iter(iter),
            added_keys: HashSet::new(),
        }
    }
}

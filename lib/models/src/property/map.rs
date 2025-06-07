use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use super::Property;

#[derive(Default)]
pub struct PropertyHashMap<Key, Value> {
    pub base: HashMap<Key, Value>,
    pub changed_keys: HashSet<Key>,
}

impl<Key, Value> PropertyHashMap<Key, Value>
where
    Key: PartialEq + Eq + Hash + Clone,
{
    pub fn insert(&mut self, key: Key, value: Value) {
        self.base.insert(key.clone(), value);
        self.changed_keys.insert(key);
    }

    pub fn get(&self, key: &Key) -> Option<&Value> {
        self.base.get(key)
    }

    pub fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
        self.changed_keys.insert(key.clone());
        self.base.get_mut(key)
    }

    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        self.changed_keys.insert(key.clone());
        self.base.remove(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Key, &Value)> {
        self.base.iter()
    }

    pub fn iter_changed_items(&self) -> impl Iterator<Item = (&Key, &Value)> {
        self.base
            .iter()
            .filter(|(k, _)| self.changed_keys.contains(k))
    }

    pub fn keys(&self) -> impl Iterator<Item = &Key> {
        self.base.keys()
    }

    pub fn removed_keys(&self) -> impl Iterator<Item = &Key> {
        self.changed_keys
            .iter()
            .filter(|key| !self.base.contains_key(key))
    }

    pub fn contains_key(&self, key: &Key) -> bool {
        self.base.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.base.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        self.base.clear();
        self.changed_keys.clear();
    }
}

impl<Key, Value> Property for PropertyHashMap<Key, Value> {
    fn is_changed(&self) -> bool {
        !self.changed_keys.is_empty()
    }

    fn reset_changed_state(&mut self) {
        self.changed_keys.clear();
    }
}

impl<Key, Value> FromIterator<(Key, Value)> for PropertyHashMap<Key, Value>
where
    Key: PartialEq + Eq + Hash,
{
    fn from_iter<T: IntoIterator<Item = (Key, Value)>>(iter: T) -> Self {
        Self {
            base: HashMap::from_iter(iter),
            changed_keys: HashSet::new(),
        }
    }
}

use std::{collections::HashMap, hash::Hash};

#[macro_export]
macro_rules! extract {
    ($value:expr, $target:pat) => {
        let $target = $value else {
            unreachable!();
        };
    };
}

#[derive(Debug, Clone)]
pub struct Storage<K: Hash + Eq, V> {
    pub map: HashMap<K, usize>,
    pub pool: Vec<V>,
}

impl<K: Hash + Eq, V> Storage<K, V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            pool: Vec::new(),
        }
    }

    pub fn get_by_id_key(&self, key: &K) -> Option<usize> {
        self.map.get(&key).copied()
    }

    pub fn put(&mut self, key: K, value: V) -> usize {
        match self.get_by_id_key(&key) {
            Some(id) => {
                self.pool.insert(id, value);
                return id;
            }
            None => {
                let new_id = self.pool.len();
                self.pool.push(value);
                return new_id;
            }
        }
    }

    pub fn map(&mut self, key: K, id: usize) -> Option<usize> {
        self.map.insert(key, id)
    }

    pub fn get_from_key(&self, key: &K) -> Option<&V> {
        let id = self.get_by_id_key(key)?;
        self.pool.get(id)
    }

    pub fn get_from_key_mut(&mut self, key: &K) -> Option<&mut V> {
        let id = self.get_by_id_key(key)?;
        self.pool.get_mut(id)
    }

    pub fn get_pool(&self) -> &Vec<V> {
        &self.pool
    }

    pub fn get_pool_mut(&mut self) -> &mut Vec<V> {
        &mut self.pool
    }
}

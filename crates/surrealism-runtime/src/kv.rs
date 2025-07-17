use std::collections::BTreeMap;
use std::sync::RwLock;

use anyhow::Result;
use surrealdb::sql;

use std::ops::Bound;

pub trait KVStore: Send {
    fn get(&self, key: String) -> Result<Option<sql::Value>>;
    fn set(&mut self, key: String, value: sql::Value) -> Result<()>;
    fn del(&mut self, key: String) -> Result<()>;
    fn exists(&self, key: String) -> Result<bool>;

    fn del_rng(&mut self, start: Bound<String>, end: Bound<String>) -> Result<()>;

    fn get_batch(&self, keys: Vec<String>) -> Result<Vec<Option<sql::Value>>>;
    fn set_batch(&mut self, entries: Vec<(String, sql::Value)>) -> Result<()>;
    fn del_batch(&mut self, keys: Vec<String>) -> Result<()>;

    fn keys(&self, start: Bound<String>, end: Bound<String>) -> Result<Vec<String>>;
    fn values(&self, start: Bound<String>, end: Bound<String>) -> Result<Vec<sql::Value>>;
    fn entries(
        &self,
        start: Bound<String>,
        end: Bound<String>,
    ) -> Result<Vec<(String, sql::Value)>>;
    fn count(&self, start: Bound<String>, end: Bound<String>) -> Result<u64>;
}

/// In-memory BTreeMap implementation of KVStore
pub struct BTreeMapStore {
    inner: RwLock<BTreeMap<String, sql::Value>>,
}

impl BTreeMapStore {
    /// Create a new empty BTreeMap store
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(BTreeMap::new()),
        }
    }

    /// Create a BTreeMap store with initial capacity
    pub fn with_capacity(_capacity: usize) -> Self {
        // BTreeMap doesn't have with_capacity, but we keep the method for API compatibility
        Self {
            inner: RwLock::new(BTreeMap::new()),
        }
    }

    /// Helper function to check if a key falls within a range
    fn in_range(&self, key: &str, start: &Bound<String>, end: &Bound<String>) -> bool {
        match start {
            Bound::Included(start_key) => {
                if key < start_key.as_str() {
                    return false;
                }
            }
            Bound::Excluded(start_key) => {
                if key <= start_key.as_str() {
                    return false;
                }
            }
            Bound::Unbounded => {}
        }

        match end {
            Bound::Included(end_key) => {
                if key > end_key.as_str() {
                    return false;
                }
            }
            Bound::Excluded(end_key) => {
                if key >= end_key.as_str() {
                    return false;
                }
            }
            Bound::Unbounded => {}
        }
        true
    }
}

impl Default for BTreeMapStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KVStore for BTreeMapStore {
    fn get(&self, key: String) -> Result<Option<sql::Value>> {
        let map = self.inner.read().unwrap();
        Ok(map.get(&key).cloned())
    }

    fn set(&mut self, key: String, value: sql::Value) -> Result<()> {
        let mut map = self.inner.write().unwrap();
        map.insert(key, value);
        Ok(())
    }

    fn del(&mut self, key: String) -> Result<()> {
        let mut map = self.inner.write().unwrap();
        map.remove(&key);
        Ok(())
    }

    fn exists(&self, key: String) -> Result<bool> {
        let map = self.inner.read().unwrap();
        Ok(map.contains_key(&key))
    }

    fn del_rng(&mut self, start: Bound<String>, end: Bound<String>) -> Result<()> {
        let mut map = self.inner.write().unwrap();
        let keys_to_remove: Vec<String> = map
            .keys()
            .filter(|key| self.in_range(key, &start, &end))
            .cloned()
            .collect();
        for key in keys_to_remove {
            map.remove(&key);
        }
        Ok(())
    }

    fn get_batch(&self, keys: Vec<String>) -> Result<Vec<Option<sql::Value>>> {
        let map = self.inner.read().unwrap();
        let mut results = Vec::with_capacity(keys.len());
        for key in keys {
            results.push(map.get(&key).cloned());
        }
        Ok(results)
    }

    fn set_batch(&mut self, entries: Vec<(String, sql::Value)>) -> Result<()> {
        let mut map = self.inner.write().unwrap();
        for (key, value) in entries {
            map.insert(key, value);
        }
        Ok(())
    }

    fn del_batch(&mut self, keys: Vec<String>) -> Result<()> {
        let mut map = self.inner.write().unwrap();
        for key in keys {
            map.remove(&key);
        }
        Ok(())
    }

    fn keys(&self, start: Bound<String>, end: Bound<String>) -> Result<Vec<String>> {
        let map = self.inner.read().unwrap();
        let keys: Vec<String> = map
            .keys()
            .filter(|key| self.in_range(key, &start, &end))
            .map(|key| key.clone())
            .collect();
        Ok(keys)
    }

    fn values(&self, start: Bound<String>, end: Bound<String>) -> Result<Vec<sql::Value>> {
        let map = self.inner.read().unwrap();
        let values: Vec<sql::Value> = map
            .iter()
            .filter(|(key, _)| self.in_range(key, &start, &end))
            .map(|(_, value)| value.clone())
            .collect();
        Ok(values)
    }

    fn entries(
        &self,
        start: Bound<String>,
        end: Bound<String>,
    ) -> Result<Vec<(String, sql::Value)>> {
        let map = self.inner.read().unwrap();
        let entries: Vec<(String, sql::Value)> = map
            .iter()
            .filter(|(key, _)| self.in_range(key, &start, &end))
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
        Ok(entries)
    }

    fn count(&self, start: Bound<String>, end: Bound<String>) -> Result<u64> {
        let map = self.inner.read().unwrap();
        let count = map
            .keys()
            .filter(|key| self.in_range(key, &start, &end))
            .count();
        Ok(count as u64)
    }
}

#![deny(missing_docs)]
//! A basic key value store
use std::collections::HashMap;

/// Main struct that powers the key value store
pub struct KvStore {
    values: HashMap<String, String>,
}

impl KvStore {
    /// Creates an emptry Key-Value store.
    /// # Examples
    /// ```
    /// # use kvs::KvStore;
    /// let kv = KvStore::new();
    /// ```
    pub fn new() -> Self {
        KvStore {
            values: HashMap::new(),
        }
    }

    /// Sets a given value for the given key
    /// # Examples
    /// ```
    /// # use kvs::KvStore;
    /// let mut kv = KvStore::new();
    ///
    /// kv.set("key1".to_owned(), "value1".to_owned());
    /// kv.set("key2".to_owned(), "value2".to_owned());
    ///
    /// # assert_eq!(kv.get("key1".to_owned()), Some("value1".to_owned()));
    /// # assert_eq!(kv.get("key2".to_owned()), Some("value2".to_owned()));
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.values.insert(key, value);
    }

    /// Gets the value for the given key
    /// # Examples
    /// ```
    /// # use kvs::KvStore;
    /// let mut kv = KvStore::new();
    ///
    /// kv.set("key1".to_owned(), "value1".to_owned());
    /// kv.set("key2".to_owned(), "value2".to_owned());
    ///
    /// assert_eq!(kv.get("key1".to_owned()), Some("value1".to_owned()));
    /// assert_eq!(kv.get("key2".to_owned()), Some("value2".to_owned()));
    /// assert_eq!(kv.get("key3".to_owned()), None);
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        self.values.get(&key).cloned()
    }

    /// Removes the value for the given key
    /// # Examples
    /// ```
    /// # use kvs::KvStore;
    /// let mut kv = KvStore::new();
    ///
    /// kv.set("key1".to_owned(), "value1".to_owned());
    /// assert_eq!(kv.get("key1".to_owned()), Some("value1".to_owned()));
    ///
    /// kv.remove("key1".to_owned());
    /// assert_eq!(kv.get("key1".to_owned()), None);
    /// ```
    pub fn remove(&mut self, key: String) {
        self.values.remove(&key);
    }
}

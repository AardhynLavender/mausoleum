use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

/**
 * Generic stores for both the stack and heap
 */

/// Generic unique identifier
pub type Key = usize;

static mut NEXT_KEY: Key = 0;

pub fn next_key() -> Key {
  unsafe {
    let key = NEXT_KEY;
    NEXT_KEY += 1;
    key
  }
}

/// Store T on the heap keyed by String
pub struct HeapStore<T: ?Sized> {
  pub store: HashMap<String, Rc<T>>,
}

impl<T: ?Sized> HeapStore<T> {
  /// Instantiate a new store
  pub fn new() -> Self {
    Self {
      store: HashMap::new(),
    }
  }

  /// Add a value to the store
  pub fn add(&mut self, key: String, value: Rc<T>) -> Rc<T> {
    Rc::clone(self.store.entry(key).or_insert(value))
  }

  /// Borrow an item from the store
  pub fn get(&self, key: &str) -> Result<Rc<T>, String> {
    return if let Some(value) = self.store.get(key) {
      Ok(Rc::clone(value))
    } else {
      Err(format!("Failed to get {} from store", key))
    };
  }
}

/// Store T on the stack keyed by V
pub struct Store<K, V> {
  pub store: HashMap<K, V>,
}

impl<K, V> Store<K, V> where
  K: Eq + std::hash::Hash + Display + Clone,
{
  pub fn new() -> Self {
    Self {
      store: HashMap::new(),
    }
  }
  /// Add a value to the store
  pub fn add(&mut self, key: K, value: V) -> &V {
    self.store.entry(key).or_insert(value)
  }

  /// Set a value in the store
  pub fn set(&mut self, key: K, value: V) -> &V {
    self.store.insert(key.clone(), value);
    self.store.get(&key).expect("Failed to set value in store")
  }

  /// Retrieve an immutable reference to item in the store
  pub fn get(&self, key: impl Into<K>) -> Result<&V, String> {
    let key = key.into();
    return if let Some(value) = self.store.get(&key) {
      Ok(value)
    } else {
      Err(format!("Failed to get {} from store", key))
    };
  }
}

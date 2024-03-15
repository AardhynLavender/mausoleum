use std::collections::HashMap;
use std::rc::Rc;

/**
 * Generic stores for both the stack and heap
 */

/// Store T on the heap keyed by String
pub struct HeapStore<T: ?Sized> {
  pub store: HashMap<String, Rc<T>>,
}

impl<T: ?Sized> HeapStore<T> {
  /// Instantiate a new store
  pub fn new() -> Self {
    Self { store: HashMap::new() }
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

/// Store T on the stack keyed by String
pub struct Store<T> {
  pub store: HashMap<String, T>,
}

impl<T> Store<T> {
  /// Instantiate a new store
  pub fn new() -> Self {
    Self { store: HashMap::new() }
  }

  /// Add a value to the store
  pub fn add(&mut self, key: String, value: T) -> &mut T {
    self.store.entry(key).or_insert(value)
  }

  /// Retrieve an immutable reference to item in the store
  pub fn get(&self, key: &str) -> Result<&T, String> {
    return if let Some(value) = self.store.get(key) {
      Ok(value)
    } else {
      Err(format!("Failed to get {} from store", key))
    };
  }
}


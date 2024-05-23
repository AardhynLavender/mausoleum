use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Default)]
pub struct State {
  states: HashMap<TypeId, Box<dyn Any>>,
}

impl State {
  /// Add a state of `T` to the store
  pub fn add<T: 'static>(&mut self, state: T) -> Result<(), String> {
    let id = TypeId::of::<T>();
    if self.states.contains_key(&id) {
      return Err(format!("State already exists in store: {:?}", id));
    }

    self.states.insert(id, Box::new(state));
    Ok(())
  }
  /// Retrieve an immutable state of `T` from the store
  pub fn get<T: 'static>(&self) -> Result<&T, String> {
    let id = TypeId::of::<T>();
    self.states.get(&id)
      .ok_or_else(|| format!("Failed to get immutable state. Not found in store: {:?}", id))
      .and_then(|state| {
        state.downcast_ref::<T>()
          .ok_or_else(|| format!("Failed to downcast state to {:?}", id))
      })
  }
  /// Retrieve a mutable state of `T` from the store
  pub fn get_mut<T: 'static>(&mut self) -> Result<&mut T, String> {
    let id = TypeId::of::<T>();
    self.states.get_mut(&id)
      .ok_or_else(|| format!("Failed to get mutable state. Not found in store: {:?}", id))
      .and_then(|state| {
        state.downcast_mut::<T>()
          .ok_or_else(|| format!("Failed to downcast state to {:?}", id))
      })
  }
  /// Remove the state of `T` from the store
  pub fn remove<T: 'static>(&mut self) -> Result<(), String> {
    let id = TypeId::of::<T>();
    self.states.remove(&TypeId::of::<T>())
      .map(|_| ())
      .ok_or_else(|| format!("Failed to remove state. Not found in store: {:?}", id))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug, PartialEq)]
  struct TestState {
    value: i32,
  }

  #[test]
  fn test_add_state() {
    let mut store = State::default();
    let state = TestState { value: 528491 };
    assert!(store.add(state).is_ok(), "State added successfully");
    let quote = "I recognise your footsteps, old man";
    assert!(store.add(quote).is_ok(), "State added successfully");
    let hash = Box::new(String::from("LV-426"));
    assert!(store.add(hash).is_ok(), "State added successfully");
  }

  #[test]
  fn test_get_state() {
    let mut store = State::default();
    let state = TestState { value: 42 };
    store.add(state).expect("Failed to add state");

    let state = store.get::<TestState>().expect("Failed to get state");
    assert_eq!(state.value, 42, "State is unchanged");
  }

  #[test]
  fn test_duplicated_state() {
    let mut store = State::default();
    let film = TestState { value: 237 };
    store.add(film).expect("Failed to add state");

    let book = TestState { value: 217 };
    assert!(store.add(book).is_err(), "State already exists in store");
  }

  #[test]
  fn test_remove_state() {
    let mut store = State::default();
    let state = TestState { value: 42 };
    assert!(store.add(state).is_ok(), "State added successfully");

    assert!(store.remove::<TestState>().is_ok(), "State removed successfully");
  }

  #[test]
  fn test_remove_missing_state() {
    let mut store = State::default();
    assert!(store.remove::<TestState>().is_err(), "State not found in store");
  }

  #[test]
  fn test_get_mut_state() {
    let mut store = State::default();
    let state = TestState { value: 6 };
    assert!(store.add(state).is_ok(), "State added successfully");

    let state = store.get_mut::<TestState>().expect("Failed to get state");
    assert_eq!(state.value, 6, "State is unchanged");
    state.value = 27;
    assert_eq!(state.value, 27, "State has been mutated");
  }
}
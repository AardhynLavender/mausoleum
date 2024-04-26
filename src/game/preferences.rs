use crate::engine::state::State;

/// Stores global preferences for the game
pub struct Preferences {
  pub debug: bool,
}

impl Default for Preferences {
  /// Apply default preferences to the game
  fn default() -> Self {
    Self {
      debug: false,
    }
  }
}

/// Mutably borrows the preference state from the engine
pub fn use_preferences(state: &mut State) -> &mut Preferences {
  state.get_mut::<Preferences>().expect("Failed to get preferences")
}

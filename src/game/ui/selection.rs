
/**
 * Arbitrate user selection over a collection of UI entities
 */

use std::fmt::Display;
use std::ops::{AddAssign, SubAssign};

use hecs::Entity;


/// Index of the current selection
pub type SelectionIndex = usize;

#[derive(Debug)]
/// A collection of entities that can be selected
pub struct Selection {
  index: SelectionIndex,
  items: Vec<Entity>,
  cursor: Entity,
}

impl Display for Selection {
  /// Display the current selection index and total items
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Selection {}/{}", self.index + 1, self.items.len())
  }
}

impl AddAssign<i32> for Selection {
  /// Increment the selection by the given value
  fn add_assign(&mut self, rhs: i32) {
    let next = self.index as i32 + rhs + self.items.len() as i32;
    self.index = (next % self.items.len() as i32) as SelectionIndex;
  }
}

impl SubAssign<i32> for Selection {
  /// Decrement the selection by the given value
  fn sub_assign(&mut self, rhs: i32) { *self += -rhs; }
}

impl Selection {
  /// Instantiate a new selection
  pub fn build(items: impl Into<Vec<Entity>>, cursor: Entity) -> Result<Self, String> {
    let items = items.into();
    if items.len() == 0 {
      return Err("Items cannot be empty".to_string());
    }

    Ok(Self { index: 0, items, cursor })
  }

  /// Set the default index of the selection
  /// If the index is negative, it will be calculated from the end of the list
  pub fn with_default(mut self, index: SelectionIndex) -> Self {
    self.index = index;
    self
  }
  /// Get the current selection index
  pub fn get_selection(&self) -> (SelectionIndex, Entity) {
    (
      self.index,
      *self.items.get(self.index).expect("Failed to get selection")
    )
  }
  /// Get the cursor entity
  pub fn get_cursor(&self) -> Entity { self.cursor }
}

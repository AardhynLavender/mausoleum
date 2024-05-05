/**
 * Save data and parsing structs
 */

use serde::{Deserialize, Serialize};

use crate::engine::geometry::shape::Vec2;
use crate::engine::utility::io::{delete_file, write_file};
use crate::game::persistence::assertion::{assert_inventory, assert_save_room};
use crate::game::persistence::constant::{DEFAULT_PLAYER_POSITION, DEFAULT_SAVE_ROOM};
use crate::game::persistence::parse::{deserialize_save_data, serialize_save_data};
use crate::game::scene::level::meta::Collectable;

/// Inventory type
type Inventory = Vec<Collectable>;

/// Save data without validation
#[derive(Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawSaveData {
  save_room: String,
  inventory: Inventory,
  x: f32,
  y: f32,
}

/// Valid save data used to build a game state from
#[derive(Debug, Serialize)]
#[derive(PartialEq)]
pub struct SaveData {
  save_room: String,
  inventory: Inventory,
  x: f32,
  y: f32,
}

impl Default for SaveData {
  fn default() -> Self {
    SaveData::build(
      String::from(DEFAULT_SAVE_ROOM),
      Inventory::default(),
      DEFAULT_PLAYER_POSITION,
    ).expect("Failed to build default save data")
  }
}

impl SaveData {
  /// Validate and build save data
  pub fn build(save_room: String, inventory: Inventory, position: Vec2<f32>) -> Result<Self, String> {
    assert_inventory(&inventory)?;
    assert_save_room(&save_room)?;
    Ok(Self { save_room, inventory, x: position.x, y: position.y })
  }
  /// Load save data from a file
  pub fn from_file(filepath: impl AsRef<std::path::Path>) -> Result<Self, String> {
    deserialize_save_data(filepath).and_then(SaveData::try_from)
  }
  /// Removes the save data file and returns default save data
  pub fn from_erased(filepath: impl AsRef<std::path::Path>) -> Result<Self, String> {
    delete_file(filepath)?;
    Ok(SaveData::default())
  }
  /// Save the save data to a file
  pub fn to_file(&self, filepath: impl AsRef<std::path::Path>) -> Result<(), String> {
    serialize_save_data(self).and_then(|data| write_file(filepath, data))
  }
  /// Get the save room
  pub fn save_room(&self) -> String { self.save_room.clone() }
  /// Get the inventory
  pub fn inventory(&self) -> Inventory { self.inventory.clone() }
  /// Get the player position
  pub fn position(&self) -> Vec2<f32> { Vec2::new(self.x, self.y) }
}

impl TryFrom<RawSaveData> for SaveData {
  type Error = String;
  /// Attempt to convert raw save data into save data
  fn try_from(data: RawSaveData) -> Result<Self, Self::Error> {
    SaveData::build(data.save_room, data.inventory, Vec2::new(data.x, data.y))
  }
}

#[cfg(test)]
mod tests {
  use std::convert::TryFrom;

  use crate::game::persistence::data::{RawSaveData, SaveData};
  use crate::game::scene::level::meta::Collectable;

  #[test]
  fn test_try_from_raw_save_data() {
    let raw_1 = RawSaveData {
      save_room: String::from("save_61"),
      inventory: vec![Collectable::IceBeam, Collectable::MissileTank, Collectable::HighJump, Collectable::Health, Collectable::Health, Collectable::Health],
      x: 15.67,
      y: 71.0,
    };

    let save_1 = SaveData::try_from(raw_1).expect("Failed to convert raw save data");
    assert_eq!(save_1.save_room, "save_61");
    assert_eq!(save_1.inventory, vec![Collectable::IceBeam, Collectable::MissileTank, Collectable::HighJump, Collectable::Health, Collectable::Health, Collectable::Health]);
    assert_eq!(save_1.x, 15.67);
    assert_eq!(save_1.y, 71.0);

    let raw_2 = RawSaveData {
      save_room: String::from("save_64"),
      inventory: vec![],
      x: 0.0,
      y: 0.0,
    };
    let save_2 = SaveData::try_from(raw_2);
    assert!(save_2.is_ok());
  }

  #[test]
  fn test_invalid_inventory() {
    let raw_1 = RawSaveData {
      save_room: String::from("save_61"),
      inventory: vec![Collectable::IceBeam, Collectable::IceBeam],
      x: 0.0,
      y: 0.0,
    };
    let save_1 = SaveData::try_from(raw_1);
    assert_eq!(save_1, Err("Too many ice beams".to_string()));

    let raw_2 = RawSaveData {
      save_room: String::from("save_61"),
      inventory: vec![Collectable::MissileTank, Collectable::MissileTank],
      x: 0.0,
      y: 0.0,
    };
    let save_2 = SaveData::try_from(raw_2);
    assert_eq!(save_2, Err("Too many missile tanks".to_string()));

    let raw_3 = RawSaveData {
      save_room: String::from("save_61"),
      inventory: vec![Collectable::IceBeam, Collectable::IceBeam],
      x: 0.0,
      y: 0.0,
    };
    let save_3 = SaveData::try_from(raw_3);
    assert_eq!(save_3, Err("Too many ice beams".to_string()));
  }

  #[test]
  fn test_invalid_save_room() {
    let raw_1 = RawSaveData {
      save_room: String::from("bad_save"),
      inventory: vec![Collectable::IceBeam, Collectable::Health, Collectable::Health],
      x: 0.0,
      y: 0.0,
    };
    let save_1 = SaveData::try_from(raw_1);
    assert_eq!(save_1, Err("Invalid save room".to_string()));

    let raw_2 = RawSaveData {
      save_room: String::from("save_256"),
      inventory: vec![Collectable::IceBeam, Collectable::HighJump, Collectable::Health],
      x: 0.0,
      y: 0.0,
    };
    let save_2 = SaveData::try_from(raw_2);
    assert_eq!(save_2, Err("Invalid save room".to_string()));

    let raw_3 = RawSaveData {
      save_room: String::from("save_NaN"),
      inventory: vec![Collectable::IceBeam, Collectable::MissileTank, Collectable::HighJump],
      x: 0.0,
      y: 0.0,
    };
    let save_3 = SaveData::try_from(raw_3);
    assert_eq!(save_3, Err("Invalid save room".to_string()));
  }
}
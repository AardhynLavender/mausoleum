use std::path::Path;

use crate::engine::utility::io::{deserialize_json, read_file, serialize_json};
use crate::game::persistence::data::{RawSaveData, SaveData};

/// Deserialize a JSON string into raw save data
pub fn deserialize_save_data(filepath: impl AsRef<Path>) -> Result<RawSaveData, String> {
  let serialized = read_file(filepath)?;
  deserialize_json(&serialized)
}

/// Serialize the save data to a JSON string
pub fn serialize_save_data(data: &SaveData) -> Result<String, String> {
  serialize_json(data)
}

/**
 * IO utility functions
 */
use std::path::Path;

/// Read string data from a file
pub fn read_file(filepath: impl AsRef<Path>) -> Result<String, String> {
  std::fs::read_to_string(filepath.as_ref()).map_err(|e| e.to_string())
}

/// Write string data to a file
pub fn write_file(filepath: impl AsRef<Path>, data: String) -> Result<(), String> {
  std::fs::write(filepath, data).map_err(|e| e.to_string())
}

/// Serialize T data to a JSON string
pub fn serialize_json<T: serde::Serialize>(data: &T) -> Result<String, String> {
  serde_json::to_string(data).map_err(|e| e.to_string())
}

/// Deserialize a JSON string into T data
pub fn deserialize_json<T: serde::de::DeserializeOwned>(data: &str) -> Result<T, String> {
  serde_json::from_str(data).map_err(|e| e.to_string())
}
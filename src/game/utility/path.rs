/**
 * Path to string utilities
 */

use std::path::Path;

/// Get the filename of `path` as an owning string
pub fn get_filename(path: impl AsRef<Path>) -> Result<String, String> {
  Ok(path.as_ref()
    .file_stem()
    .ok_or("Failed to get filename")?
    .to_str()
    .map(|s| s.to_string())
    .ok_or("Failed to convert filename to string")?)
}

/// Get the extension of `path` as an owning string
pub fn get_extension(path: impl AsRef<Path>) -> Result<String, String> {
  Ok(path.as_ref()
    .extension()
    .ok_or("Failed to get extension")?
    .to_str()
    .map(|s| s.to_string())
    .ok_or("Failed to convert extension to string")?)
}

/// Get the basename of `path` as an owning string
pub fn get_basename(path: impl AsRef<Path>) -> Result<String, String> {
  Ok(path.as_ref()
    .file_name()
    .ok_or("Failed to get basename")?
    .to_str()
    .map(|s| s.to_string())
    .ok_or("Failed to convert basename to string")?)
}

/// Get the parent directory of `path` as an owning string
pub fn get_parent(path: impl AsRef<Path>) -> Result<String, String> {
  Ok(path.as_ref()
    .parent()
    .ok_or("Failed to get parent")?
    .to_str()
    .map(|s| s.to_string())
    .ok_or("Failed to convert parent to string")?)
}

/// Get `path` as an owning string
pub fn get_path(path: impl AsRef<Path>) -> Result<String, String> {
  Ok(path.as_ref()
    .to_str()
    .map(|s| s.to_string())
    .ok_or("Failed to convert path to string")?)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_filename() {
    assert_eq!(get_filename("foo/bar.txt"), Ok("bar".to_string()));
    assert_eq!(get_filename("foo/bar"), Ok("bar".to_string()));
    assert_eq!(get_filename("foo/bar/"), Ok("bar".to_string()));
    assert_eq!(get_filename("bar.txt"), Ok("bar".to_string()));
    assert_eq!(get_filename("bar"), Ok("bar".to_string()));
    assert_eq!(get_filename("bar/"), Ok("bar".to_string()));
  }

  #[test]
  fn test_get_extension() {
    assert_eq!(get_extension("foo/bar.txt"), Ok("txt".to_string()));
    assert_eq!(get_extension("foo/bar"), Err("Failed to get extension".to_string()));
    assert_eq!(get_extension("foo/bar/"), Err("Failed to get extension".to_string()));
    assert_eq!(get_extension("bar.txt"), Ok("txt".to_string()));
    assert_eq!(get_extension("bar"), Err("Failed to get extension".to_string()));
    assert_eq!(get_extension("bar/"), Err("Failed to get extension".to_string()));
  }

  #[test]
  fn test_get_basename() {
    assert_eq!(get_basename("foo/bar.txt"), Ok("bar.txt".to_string()));
    assert_eq!(get_basename("foo/bar"), Ok("bar".to_string()));
    assert_eq!(get_basename("foo/bar/"), Ok("bar".to_string()));
    assert_eq!(get_basename("bar.txt"), Ok("bar.txt".to_string()));
    assert_eq!(get_basename("bar"), Ok("bar".to_string()));
    assert_eq!(get_basename("bar/"), Ok("bar".to_string()));
  }

  #[test]
  fn test_get_parent() {
    assert_eq!(get_parent("foo/bar.txt"), Ok(String::from("foo")));
    assert_eq!(get_parent("foo/bar"), Ok(String::from("foo")));
    assert_eq!(get_parent("foo/bar/"), Ok(String::from("foo")));
    assert_eq!(get_parent("bar.txt"), Ok(String::from("")));
    assert_eq!(get_parent("bar"), Ok(String::from("")));
    assert_eq!(get_parent("/bar"), Ok(String::from("/")));
    assert_eq!(get_parent("bar/"), Ok(String::from("")));
  }

  #[test]
  fn test_get_path() {
    assert_eq!(get_path("foo/bar.txt"), Ok("foo/bar.txt".to_string()));
    assert_eq!(get_path("foo/bar"), Ok("foo/bar".to_string()));
    assert_eq!(get_path("foo/bar/"), Ok("foo/bar/".to_string()));
    assert_eq!(get_path("bar.txt"), Ok("bar.txt".to_string()));
    assert_eq!(get_path("bar"), Ok("bar".to_string()));
    assert_eq!(get_path("bar/"), Ok("bar/".to_string()));
  }
}
use std::path::Path;

use sdl2::ttf::{Font, Sdl2TtfContext};

use crate::engine::store::Store;

/**
 * Typeface loading, storage, and retrieval
 */

/// store typefaces
pub type TypefaceStore<'ttf, 'f> = Store<String, Font<'ttf, 'f>>;

/// Load and store typefaces
pub struct TypefaceLoader<'ttf, 'b> {
  subsystem: &'ttf Sdl2TtfContext,
  store: TypefaceStore<'ttf, 'b>,
}

impl<'ttf, 'l> TypefaceLoader<'ttf, 'l> {
  /// Instantiate a new typeface loader
  pub fn new(subsystem: &'ttf Sdl2TtfContext) -> Self {
    Self {
      subsystem,
      store: TypefaceStore::new(),
    }
  }

  /// Loads a typeface from a file and adds it to the store
  pub fn load(&mut self, filepath: &Path, size: u16) -> Result<(), String> {
    let path_str = filepath
      .to_str()
      .ok_or("Failed to convert path to str")?;
    let basename = filepath
      .file_stem()
      .ok_or("Could not extract file stem")?
      .to_str()
      .ok_or("Could not convert OsStr to str")?;

    let font = self.subsystem.load_font(path_str, size)?;
    self.store.add(String::from(basename), font);

    Ok(())
  }

  /// Returns an immutable reference to the store
  pub fn use_store(&self) -> &TypefaceStore {
    &self.store
  }
}

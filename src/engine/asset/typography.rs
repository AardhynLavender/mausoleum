use std::path::Path;

use sdl2::ttf::{Font, Sdl2TtfContext};

use crate::engine::store::Store;
use crate::game::utility::path::{get_filename, get_path};

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
    let path_str = get_path(filepath)?;
    let filename = get_filename(filepath)?;

    let font = self.subsystem.load_font(path_str, size)?;
    self.store.add(String::from(filename), font);

    Ok(())
  }

  /// Returns an immutable reference to the store
  pub fn use_store(&self) -> &TypefaceStore {
    &self.store
  }
}

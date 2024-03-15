use std::rc::Rc;

use sdl2::ttf::{Font, Sdl2TtfContext};

use crate::engine::store::HeapStore;

/**
 * Typeface loading, storage, and retrieval
 */

/// store typefaces
pub type TypefaceStore<'ttf, 'f> = HeapStore<Font<'ttf, 'f>>;

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
  pub fn load(&mut self, filepath: String, size: u16) -> Result<(), String> {
    let font = self.subsystem.load_font(filepath.as_str(), size)?;

    let filename = filepath.split("/").last().ok_or("Failed to get filename")?;
    let basename = filename.split(".").next().ok_or("Failed to get basename")?;

    self.store.add(String::from(basename), Rc::new(font));

    Ok(())
  }

  /// Returns an immutable reference to the store
  pub fn use_store(&self) -> &TypefaceStore {
    &self.store
  }
}
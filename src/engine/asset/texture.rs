use std::path::Path;

use sdl2::image::LoadTexture;
use sdl2::render::{TextureCreator, TextureQuery};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

use crate::engine::geometry::{Rec2, Vec2};
use crate::engine::store::{HeapStore, next_key, Store};

/**
 * Texture loading, storage, and retrieval
 */

pub type TextureKey = usize;

/// store textures
pub type TextureStore = Store<TextureKey, Texture>;

/// Load and store textures
pub struct TextureLoader {
  store: TextureStore,
  subsystem: TextureCreator<WindowContext>,
}

impl TextureLoader {
  /// Instantiate a new texture loader
  pub fn new(creator: TextureCreator<WindowContext>) -> Self {
    Self {
      subsystem: creator,
      store: TextureStore::new(),
    }
  }

  /// Loads a texture from a file and adds it to the store
  pub fn load(&mut self, filepath: &Path) -> Result<TextureKey, &str> {
    let internal_texture = self
      .subsystem
      .load_texture(filepath.to_str().ok_or("Invalid filepath")?)
      .map_err(|_| "Failed to load texture")?;
    let texture = Texture::new(internal_texture);

    let key = next_key();
    self.store.add(key, texture);

    Ok(key)
  }

  /// Builds a texture from a surface
  pub fn build_from_surface(&self, surface: Surface) -> Result<Texture, &str> {
    let internal_texture = self
      .subsystem
      .create_texture_from_surface(surface)
      .map_err(|_| "Failed to load texture")?;
    let texture = Texture::new(internal_texture);
    Ok(texture)
  }

  /// Returns an immutable reference to the store
  pub fn use_store(&self) -> &TextureStore {
    &self.store
  }
}

/// A rectangle of pixels
pub type SrcRect = Rec2<u32, u32>;

/// A wrapper for a SDL2 texture
pub struct Texture {
  pub internal: sdl2::render::Texture,
  pub dimensions: Vec2<u32>,
}

impl Texture {
  /// Instantiate a new texture from an SDL2 texture
  pub fn new(texture: sdl2::render::Texture) -> Self {
    let TextureQuery { width, height, .. } = texture.query();
    let dimensions = Vec2::new(width, height);
    Self {
      internal: texture,
      dimensions,
    }
  }
}

use std::path::Path;

use sdl2::image::LoadTexture;
use sdl2::render::{TextureCreator, TextureQuery};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

use crate::engine::geometry::Rec2;
use crate::engine::store::{next_key, Store};
use crate::engine::utility::alias::{Size, Size2};

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
  pub fn load(&mut self, filepath: &Path) -> Result<TextureKey, String> {
    // load texture
    let parsed_filepath = filepath
      .to_str()
      .ok_or(String::from("Invalid filepath"))?;
    let internal_texture = self
      .subsystem
      .load_texture(parsed_filepath)
      .map_err(|_| String::from(format!("Failed to load texture: '{}'", parsed_filepath)))?;

    // store texture
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
  pub fn use_store(&mut self) -> &mut TextureStore {
    &mut self.store
  }
}

/// A rectangle of pixels
pub type SrcRect = Rec2<u32, u32>;

/// A wrapper for a SDL2 texture
pub struct Texture {
  pub internal: sdl2::render::Texture,
  pub dimensions: Size2,
}

impl Texture {
  /// Instantiate a new texture from an SDL2 texture
  pub fn new(texture: sdl2::render::Texture) -> Self {
    let TextureQuery { width, height, .. } = texture.query();
    let dimensions = Size2::new(width as Size, height as Size);
    Self {
      internal: texture,
      dimensions,
    }
  }
}

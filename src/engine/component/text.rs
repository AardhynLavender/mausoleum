use sdl2::ttf::Font;

use crate::engine::asset::texture::{TextureKey, TextureLoader};
use crate::engine::geometry::Vec2;
use crate::engine::render::color::RGBA;
use crate::engine::store::next_key;
use crate::engine::utility::alias::Size2;

pub struct Text {
  content: String,
  dirty: bool,
  color: RGBA,
  texture: Option<TextureKey>,
  dimensions: Size2,
}

impl Text {
  /// Instantiate a new text component of `color`
  pub fn new(color: RGBA) -> Self {
    Self {
      content: String::new(),
      dirty: false,
      texture: None,
      color,
      dimensions: Vec2::default(),
    }
  }
  /// call with `Text::new` to set `content` with `font` in `color`
  /// ## Panics
  /// Will panic if the texture cannot be built
  pub fn with_content<'fonts, 'app>(mut self, content: impl Into<String>, font: &Font<'fonts, 'app>, texture_loader: &mut TextureLoader) -> Self {
    self.set_content(content.into());
    self.build_texture(font, texture_loader).expect("Failed to rebuild texture"); // panic is fine, as failing to rebuild a texture is unexpected
    self
  }

  /// Builds the texture to render for content in the font and color
  fn build_texture<'fonts, 'app>(&mut self, font: &Font<'fonts, 'app>, texture_loader: &mut TextureLoader) -> Result<(), String> {
    // create texture of the content in the font and color
    let surface = font
      .render(&self.content)
      .blended(self.color)
      .map_err(|e| e.to_string())?;
    let texture = texture_loader
      .build_from_surface(surface)
      .map_err(|e| e.to_string())?;
    self.dimensions = texture.dimensions;

    // update the texture in the store this text component references
    if let Some(texture_key) = self.texture {
      // replace the texture in the store
      texture_loader
        .use_store()
        .set(texture_key, texture);
    } else {
      // add a new texture to the store
      let texture_key = next_key();
      self.texture = Some(texture_key);
      texture_loader
        .use_store()
        .add(texture_key, texture);
    }

    // texture and dimensions are valid for the content
    self.dirty = false;

    Ok(())
  }

  /// Updates the content of a text
  pub fn set_content(&mut self, content: impl Into<String>) {
    self.dirty = true;
    self.content = content.into();
  }
  /// Updates the content of a text and rebuilds the texture recalculating dimensions
  /// ## Panics
  /// Will panic if the texture cannot be built
  pub fn get_content<'ttf, 'a>(&mut self, typeface: &Font<'ttf, 'a>, texture_loader: &mut TextureLoader) -> Option<TextureKey> {
    // if the content has changed, the texture is stale
    if self.dirty {
      self.texture = None;
      self.dimensions = Vec2::default();
    }

    // check if texture is missing
    if self.texture.is_none() && !self.content.is_empty() {
      self
        .build_texture(typeface, texture_loader)
        .expect("Failed to rebuild texture"); // panic is fine, as failing to rebuild a texture is unexpected
    }

    self.texture
  }
  /// Get the dimensions of the text
  pub fn get_dimensions(&self) -> Size2 {
    self.dimensions
  }
}

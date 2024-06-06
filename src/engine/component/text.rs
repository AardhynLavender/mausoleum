use std::marker::PhantomData;

use hecs::{Component, DynamicBundle};
use sdl2::ttf::Font;

use crate::engine::asset::texture::{TextureKey, TextureLoader};
use crate::engine::geometry::shape::Vec2;
use crate::engine::rendering::camera::Sticky1;
use crate::engine::rendering::color::RGBA;
use crate::engine::store::next_key;
use crate::engine::utility::alias::Size2;
use crate::engine::utility::alignment::{Aligner, Alignment};
use crate::game::physics::position::Position;

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
    self.build_texture(font, texture_loader).expect("Failed to rebuild texture");
    self
  }

  /// Builds the texture to render for content in the font and color
  fn build_texture<'fonts, 'app>(&mut self, font: &Font<'fonts, 'app>, texture_loader: &mut TextureLoader) -> Result<(), String> {
    if self.content.is_empty() { return Ok(()); } // no need to build a texture for empty content

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
    let content = content.into();
    if self.content == content { return; } // no need to update if the content is the same
    self.dirty = true;
    self.content = content;
  }
  /// Get the raw, potentially dirty, text content
  pub fn get_text(&self) -> &String { &self.content }
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

// Helpers //

/// Split a string into lines of `line_length` characters
pub fn split_text(text: &String, line_length: usize) -> Vec<String> {
  text
    .split_whitespace()
    .enumerate()
    .fold(vec![String::new()], |mut lines, (index, word)| {
      let last_line = lines.last_mut().expect("Failed to get last line");
      let space = if index == 0 || index == text.len() - 1 { "" } else { " " };
      if word.len() > line_length { panic!("'{}' is too long to fit in a line", word) }
      let new_line_length = last_line.len() + word.len() + space.len();
      if new_line_length > line_length {
        lines.push(word.to_string());
      } else {
        last_line.push_str(space);
        last_line.push_str(word);
      }
      lines
    })
}

/// Helper function to assemble the components for a text entity
pub fn make_text<'font, 'app, Meta, Layer>(
  content: impl Into<String>,
  position: Alignment,
  aligner: &Aligner,
  color: RGBA,
  typeface: &Font<'font, 'app>,
  texture_loader: &mut TextureLoader,
) -> impl DynamicBundle where Meta: Component + Default, Layer: Component + Default {
  let text = Text::new(color).with_content(content, &typeface, texture_loader);
  let position = aligner.align(position, text.get_dimensions());

  (Position(position), text, Layer::default(), Meta::default(), )
}

/// Helper struct for creating multiple text entities
pub struct TextBuilder<'fonts, 'app, Layer = Sticky1> {
  typeface: &'app Font<'fonts, 'app>,
  texture_loader: &'app mut TextureLoader,
  color: RGBA,
  aligner: Aligner,
  layer: PhantomData<Layer>,
}

impl<'app, 'fonts, Layer> TextBuilder<'app, 'fonts, Layer> where Layer: Default + Component {
  /// Instantiate a new text builder
  pub fn new(typeface: &'app Font<'fonts, 'app>, texture_loader: &'app mut TextureLoader, color: RGBA, aligner: Aligner) -> Self {
    Self {
      typeface,
      texture_loader,
      color,
      aligner,
      layer: PhantomData,
    }
  }
  /// Assemble the components for a text entity
  pub fn make_text<Meta>(&mut self, content: impl Into<String>, position: Alignment) -> impl DynamicBundle
    where Meta: Component + Default + 'static
  {
    make_text::<Meta, Layer>(content, position, &self.aligner, self.color, self.typeface, self.texture_loader)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_split_text() {
    let text = String::from("The quick brown fox jumps over the lazy dog");
    let lines = split_text(&text, 10);
    assert_eq!(lines, vec![
      String::from("The quick"),
      String::from("brown fox"),
      String::from("jumps over"),
      String::from("the lazy"),
      String::from("dog"),
    ]);
  }

  #[test]
  #[should_panic(expected = "'Disestablishmentarianism' is too long to fit in a line")]
  fn text_split_long_word() {
    let text = String::from("Disestablishmentarianism");
    split_text(&text, 10);
  }

  #[test]
  fn text_split_empty_text() {
    let text = String::default();
    let lines = split_text(&text, 10);
    assert_eq!(lines, vec![String::default()]);
  }
}
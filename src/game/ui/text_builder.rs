/**
  * Helper functions for creating text entities
  */

use std::marker::PhantomData;

use hecs::{Component, DynamicBundle};
use sdl2::ttf::Font;

use crate::engine::asset::texture::TextureLoader;
use crate::engine::component::position::Position;
use crate::engine::component::text::Text;
use crate::engine::render::camera::Sticky1;
use crate::engine::utility::alignment::{Aligner, Alignment};
use crate::engine::utility::color::RGBA;

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

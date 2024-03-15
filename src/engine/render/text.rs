use std::rc::Rc;

use sdl2::ttf::Font;

use crate::engine::asset::texture::{Texture, TextureLoader};
use crate::engine::geometry::Vec2;
use crate::engine::render::color::RGBA;
use crate::engine::render::Renderer;

/**
 * Text structure for rendering text to the screen
 */

/// A text object that can be rendered to the screen
pub struct Text {
  content: String,
  dirty: bool,
  color: RGBA,
  position: Vec2<i32>,
  texture: Option<Rc::<Texture>>,
}

impl Text {
  pub fn new(content: String, color: RGBA, position: Vec2<i32>) -> Self {
    Self {
      content,
      dirty: true,
      position,
      texture: None,
      color,
    }
  }

  /// Builds a `Texture` from `content` in `font`
  fn rebuild_texture<'t>(&mut self, font: &Rc<Font<'t, 't>>, texture_loader: &'t TextureLoader) -> Result<(), String> {
    let surface = font
      .render(self.content.as_str())
      .blended(self.color)
      .map_err(|e| e.to_string())?;
    let texture = texture_loader.build_from_surface(surface)?;
    self.texture = Some(Rc::new(texture));
    self.dirty = false;
    Ok(())
  }

  /// Sets the text content of the text and
  pub fn set_content(&mut self, content: String) {
    self.content = content;
    self.dirty = true;
  }
  /// Clears the text content of the text
  pub fn clear_content(&mut self) {
    self.content.clear();
    self.dirty = true;
  }

  /// Render the text to the screen, regenerating the texture if the content has changed
  pub fn render<'t>(&mut self, font: &Rc<Font<'t, 't>>, texture_loader: &'t TextureLoader, renderer: &mut Renderer) {
    // content changed; reset texture
    if self.dirty {
      self.texture = None;
    }

    // check if texture is missing
    if self.texture.is_none() && !self.content.is_empty() {
      self.rebuild_texture(font, texture_loader)
        .map_err(|e| eprintln!("Failed to build texture: {}", e))
        .ok();
    }

    // render text texture
    if let Some(texture) = &self.texture {
      renderer.draw_texture(&texture, self.position);
    }
  }
}
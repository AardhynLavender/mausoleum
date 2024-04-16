use crate::engine::asset::texture::{SrcRect, TextureKey};

/**
 * Engine components for rendering
 */

/// Render a texture to the screen each frame
feat#[derive(Copy, Clone)]
pub struct Sprite {
  pub texture: TextureKey,
  pub src: SrcRect,
  pub rotation: f64,
}

impl Sprite {
  /// Instantiate a new Sprite component
  pub fn new(texture: TextureKey, src: SrcRect) -> Self {
    Self { texture, src, rotation: 0.0 }
  }
  /// Rotate the sprite
  pub fn rotate(&mut self, rotation: f64) { self.rotation = rotation % 360.0; }
}

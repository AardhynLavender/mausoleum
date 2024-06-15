/**
 * A handle for a texture used for rendering
 */

use crate::engine::asset::texture::{SrcRect, TextureKey};
use crate::engine::geometry::shape::Vec2;

/// Render a texture to the screen each frame
#[derive(Copy, Clone)]
pub struct Sprite {
  pub texture: TextureKey,
  pub src: SrcRect,
  pub rotation: f64,
  pub centroid: Option<Vec2<i32>>,
}

impl Sprite {
  /// Instantiate a new Sprite component
  pub fn new(texture: TextureKey, src: SrcRect) -> Self {
    Self { texture, src, rotation: 0.0, centroid: None }
  }
  /// Rotate the sprite around an optional centroid
  pub fn rotate(&mut self, rotation: f64, centroid: Option<Vec2<i32>>) {
    self.rotation = rotation % 360.0;
    self.centroid = centroid;
  }
}

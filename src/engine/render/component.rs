use crate::engine::asset::texture::{SrcRect, TextureKey};

/**
 * Engine components for rendering
 */

/// Render a texture to the screen each frame
#[derive(Clone)]
pub struct Sprite {
  pub texture: TextureKey,
  pub src: SrcRect,
}

impl Sprite {
  pub fn new(texture: TextureKey, src: SrcRect) -> Self {
    Self { texture, src }
  }
}

use crate::engine::asset::texture::TextureKey;

/**
 * Engine components for rendering
 */

/// Render a texture to the screen each frame
#[derive(Clone)]
pub struct Sprite {
  pub texture: TextureKey,
}

impl Sprite {
  pub fn new(texture: TextureKey) -> Self {
    Self { texture }
  }
}

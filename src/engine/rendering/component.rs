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

// renderable //

// impl Renderable for Sprite {
//   fn render(&self, position: Vec2<i32>, renderer: &mut Renderer, asset_manager: &AssetManager) {}
//
//   /// Get the z value of this renderable
//   #[inline]
//   fn get_z(&self) -> i32 { 0 }
// }

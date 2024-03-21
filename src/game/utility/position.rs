use crate::engine::geometry::Vec2;
use crate::game::constant::LOGICAL_SIZE;

/**
 * position utility functions (cant help using css terms)
 */

/// center something horizontally
pub fn center_horizontal(width: f32) -> f32 {
  (LOGICAL_SIZE.x as f32 - width) / 2.0
}

/// center something vertically
pub fn center_vertical(height: f32) -> f32 {
  (LOGICAL_SIZE.y as f32 - height) / 2.0
}

/// center something both horizontally and vertically
pub fn center(width: f32, height: f32) -> Vec2<f32> {
  Vec2::new(center_horizontal(width), center_vertical(height))
}

/// Align something to the bottom of the screen
pub fn align_end(height: f32) -> f32 {
  LOGICAL_SIZE.y as f32 - height
}

/// Justify something to the right of the screen
pub fn justify_end(width: f32) -> f32 {
  LOGICAL_SIZE.x as f32 - width
}

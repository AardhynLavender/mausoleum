/**
 * Player physics constraints and calculations
 */

use crate::engine::geometry::shape::Vec2;

// 5 tiles
pub const INITIAL_JUMP_HEIGHT: f32 = 80.0;
// 7 tiles
pub const HIGH_JUMP_BOOTS_JUMP_HEIGHT: f32 = INITIAL_JUMP_HEIGHT + 24.0;
// 6 tiles
pub const INITIAL_JUMP_WIDTH: f32 = 96.0;
// 3 tiles per second
pub const INITIAL_WALK_SPEED: f32 = 128.0;

/// Calculate the jump acceleration that ensures the player reaches the desired jump height and width
pub fn calculate_jump_velocity(jump_height: f32, walk_speed: f32, jump_width: f32) -> Vec2<f32> {
  Vec2::new(0.0, -(((2.0 * jump_height) * walk_speed) / (jump_width / 2.0)))
}

/// Calculate the gravity that ensures the player reaches the desired jump height and width
pub fn calculate_gravity(jump_height: f32, walk_speed: f32, jump_width: f32) -> Vec2<f32> {
  Vec2::new(0.0, -((-2.0 * jump_height * (walk_speed * walk_speed)) / ((jump_width / 2.0) * (jump_width / 2.0))))
}
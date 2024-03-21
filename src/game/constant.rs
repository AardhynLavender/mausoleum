use crate::engine::geometry::Vec2;

pub const WINDOW_TITLE: &str = "Metroidvania";
pub const LOGICAL_SIZE: Vec2<u32> = Vec2::new(640, 362);
pub const WINDOW_SCALE: f32 = 2.0;
pub const WINDOW_SIZE: Vec2<u32> = Vec2::new(
  (LOGICAL_SIZE.x as f32 * WINDOW_SCALE) as u32,
  (LOGICAL_SIZE.y as f32 * WINDOW_SCALE) as u32,
);

pub const JUMP_HEIGHT: f32 = 10.0;
pub const JUMP_LENGTH: f32 = 10.0;
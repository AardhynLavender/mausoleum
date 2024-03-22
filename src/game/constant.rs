use crate::engine::geometry::Vec2;

// Menu //

pub const TITLE_Y: f32 = 80.0;
pub const COPYRIGHT_MARGIN: f32 = 4.0;
pub const BUTTONS_BEGIN_Y: f32 = 160.0;
pub const BUTTONS_Y_GAP: f32 = 16.0;

// Typeface //

pub const TYPEFACE_PATH: &str = "asset/typeface.ttf";
pub const TYPEFACE_SIZE: u16 = 5;

// Window //

pub const WINDOW_TITLE: &str = "Metroidvania";
pub const LOGICAL_SIZE: Vec2<u32> = Vec2::new(640, 362);
pub const WINDOW_SCALE: f32 = 2.0;
pub const WINDOW_SIZE: Vec2<u32> = Vec2::new(
  (LOGICAL_SIZE.x as f32 * WINDOW_SCALE) as u32,
  (LOGICAL_SIZE.y as f32 * WINDOW_SCALE) as u32,
);

// Physics //

pub const JUMP_HEIGHT: f32 = 10.0;
pub const JUMP_LENGTH: f32 = 10.0;

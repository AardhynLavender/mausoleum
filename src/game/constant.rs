use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::utility::alignment::Aligner;
use crate::engine::utility::interpolation::CubicBezierCurve;

// Tile //

pub const TILE_SIZE: Vec2<u32> = Vec2::new(16, 16);

// Typeface //

pub const TYPEFACE_PATH: &str = "asset/typography/typeface.ttf";
pub const TYPEFACE_SIZE: u16 = 5;

// Window //

pub const WINDOW_TITLE: &str = "Metroidvania";
pub const LOGICAL_SIZE_TILES: Vec2<u32> = Vec2::new(40, 22);
pub const LOGICAL_SIZE: Vec2<u32> = Vec2::new(LOGICAL_SIZE_TILES.x * TILE_SIZE.x, LOGICAL_SIZE_TILES.y * TILE_SIZE.y);
pub const WINDOW_SCALE: f32 = 2.0;
pub const WINDOW_SIZE: Vec2<u32> = Vec2::new(
  (LOGICAL_SIZE.x as f32 * WINDOW_SCALE) as u32,
  (LOGICAL_SIZE.y as f32 * WINDOW_SCALE) as u32,
);

pub const WINDOW: Aligner = Aligner::new(Rec2::new(Vec2::const_default(), LOGICAL_SIZE));

// Persistence //

pub const DEV_SAVE_FILE: &str = "data/dev_save.json";
pub const USER_SAVE_FILE: &str = "user_save.json";

// curves //

/// A cubic Bézier curve that eases in and out
///
/// This can't be a constant because its generic type depends on traits without const variants
pub fn ease_in_out() -> CubicBezierCurve<f32> {
  CubicBezierCurve::new(
    Vec2::new(0.0, 0.0),
    Vec2::new(0.42, 0.0),
    Vec2::new(0.58, 1.0),
    Vec2::new(1.0, 1.0),
  )
}
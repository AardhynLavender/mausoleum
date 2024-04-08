use crate::engine::asset::texture::SrcRect;
use crate::engine::geometry::collision::CollisionBox;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::utility::alias::Size2;
use crate::engine::utility::alignment::Aligner;

// Tile //

pub const TILE_SIZE: Vec2<u32> = Vec2::new(16, 16);

// Menu //

pub const TITLE_Y: f32 = 80.0;
pub const COPYRIGHT_MARGIN: f32 = 10.0;
pub const BUTTONS_BEGIN_Y: f32 = 160.0;
pub const BUTTONS_Y_GAP: f32 = 16.0;

// Typeface //

pub const TYPEFACE_PATH: &str = "asset/typeface.ttf";
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

// Player //

// 6 tiles
pub const JUMP_HEIGHT: f32 = 96.0;
// 4 tiles
pub const JUMP_WIDTH: f32 = 96.0;
// 3 tiles per second
pub const WALK_SPEED: f32 = 128.0;

pub const JUMP_ACCELERATION: Vec2<f32> = Vec2::new(0.0, ((2.0 * JUMP_HEIGHT) * WALK_SPEED) / (JUMP_WIDTH / 2.0));
pub const GRAVITY: Vec2<f32> = Vec2::new(0.0, (-2.0 * JUMP_HEIGHT * (WALK_SPEED * WALK_SPEED)) / ((JUMP_WIDTH / 2.0) * (JUMP_WIDTH / 2.0)));
pub const MAX_GRAVITY: f32 = -400.0;

pub const PLAYER_START: Vec2<f32> = Vec2::new(40.0, 24.0);
pub const PLAYER_SIZE: Size2 = Size2::new(12, 28);
pub const PLAYER_SPRITE: SrcRect = SrcRect::new(Vec2::new(0, 0), PLAYER_SIZE);
pub const PLAYER_COLLIDER: CollisionBox = Rec2::new(Vec2::new(0.0, 0.0), PLAYER_SIZE);
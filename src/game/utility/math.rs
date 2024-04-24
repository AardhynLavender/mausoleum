use crate::engine::geometry::shape::Vec2;
use crate::game::constant::TILE_SIZE;

/**
 * Math utilities
 */

/// Convert pixels to tiles
#[allow(unused)]
pub fn pixels_to_tiles(pixels: f32) -> f32 {
  if TILE_SIZE.x != TILE_SIZE.y {
    panic!("TILE_SIZE.x and TILE_SIZE.y must be equal to use this function");
  }
  pixels / TILE_SIZE.x as f32
}

/// Convert tiles to pixels
#[allow(unused)]
pub fn tiles_to_pixels(tiles: f32) -> f32 {
  if TILE_SIZE.x != TILE_SIZE.y {
    panic!("TILE_SIZE.x and TILE_SIZE.y must be equal to use this function");
  }
  tiles * TILE_SIZE.x as f32
}

/// Convert pixel to tile
#[allow(unused)]
pub fn pixel_to_tile(pixel: Vec2<f32>) -> Vec2<f32> {
  pixel * Vec2::<f32>::from(TILE_SIZE)
}

/// Convert tile to pixel
#[allow(unused)]
pub fn tile_to_pixel(tile: Vec2<f32>) -> Vec2<f32> {
  tile * Vec2::<f32>::from(TILE_SIZE)
}

/// Round a position to the nearest tile.
///
/// Assumes the tiles aligned such that {0,0} is a tile boundary.
///
/// Probably add a room parameter to provide any needed offset (but realistically, this will always {0,0}).
pub fn round_to_tile(position: Vec2<f32>) -> Vec2<f32> {
  (position / Vec2::<f32>::from(TILE_SIZE)).round() * Vec2::<f32>::from(TILE_SIZE)
}

/// Floors a position to the nearest tile.
///
/// Assumes the tiles aligned such that {0,0} is a tile boundary.
///
/// Probably add a room parameter to provide any needed offset (but realistically, this will always {0,0}).
pub fn floor_to_tile(position: Vec2<f32>) -> Vec2<f32> {
  (position / Vec2::<f32>::from(TILE_SIZE)).floor() * Vec2::<f32>::from(TILE_SIZE)
}

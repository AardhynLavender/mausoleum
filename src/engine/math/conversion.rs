/**
 * convert between related types and units
 */

use crate::engine::geometry::shape::Vec2;
use crate::engine::utility::alias::{Coordinate, Size2};

/// Convert a coordinate to an index
pub fn coordinate_to_index(position: &Coordinate, dimensions: Size2) -> usize {
  (dimensions.x as i32 * position.y + position.x) as usize
}

/// Convert an index to a coordinate
pub fn index_to_coordinate(index: usize, dimensions: Size2) -> Coordinate {
  let x = index % dimensions.x as usize;
  let y = index / dimensions.x as usize;
  Coordinate::new(x as i32, y as i32)
}

/// convert a position to a coordinate
pub fn position_to_coordinate(position: Vec2<f32>, tile_size: Size2) -> Coordinate {
  Coordinate::from(position / Vec2::<f32>::from(tile_size))
}

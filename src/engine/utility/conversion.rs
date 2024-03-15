use crate::engine::utility::types::{Coordinate, Size2};

/**
 * convert between related types
 */

/// Convert a coordinate to an index
pub fn coordinate_to_index(position: &Coordinate, dimensions: Size2) -> usize {
  (dimensions.x as i32 * position.y + position.x) as usize
}

/// Convert an index to a coordinate
pub fn index_to_coordinate(index: usize, dimensions: &Coordinate) -> Coordinate {
  let x = index % dimensions.x as usize;
  let y = index / dimensions.x as usize;
  Coordinate::new(x as i32, y as i32)
}
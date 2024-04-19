use crate::engine::geometry::shape::Vec2;
use crate::engine::utility::alias::Coordinate;

/**
 * Direction and conversion utilities
 */

/// Axis' direction
pub enum AxisDirection {
  Positive,
  Zero,
  Negative,
}

impl AxisDirection {
  /// Create a new AxisDirection from a value
  pub fn new(value: f32) -> Self {
    if value > 0.0 {
      AxisDirection::Positive
    } else if value < 0.0 {
      AxisDirection::Negative
    } else {
      AxisDirection::Zero
    }
  }
}

/// A cardinal or ordinal direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
  Up,
  UpRight,
  Right,
  DownRight,
  Down,
  DownLeft,
  Left,
  UpLeft,
}

/// A snap type for directions
pub enum SnapType {
  Ordinal,
  Cardinal,
  Any,
}

/// Define direction ordering
pub const DIRECTION: [Direction; 8] = [
  Direction::Up,
  Direction::UpRight,
  Direction::Right,
  Direction::DownRight,
  Direction::Down,
  Direction::DownLeft,
  Direction::Left,
  Direction::UpLeft,
];
/// Number of possible cardinal+ordinal directions
pub const DIRECTIONS: usize = DIRECTION.len();

pub fn get_direction_index(direction: &Direction) -> usize {
  DIRECTION
    .iter()
    .position(|&d| d == *direction)
    .expect("Direction not found in DIRECTION array")
}

pub const HALF_ROTATION: usize = DIRECTIONS / 2;
pub const QUARTER_ROTATION: usize = DIRECTIONS / 4;

/// A 2D binary rotation direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotation {
  /// Counter-clockwise rotation
  Left,
  /// Clockwise rotation
  Right,
}

impl Rotation {
  /// Invert the rotation
  pub fn invert(&self) -> Self {
    match self {
      Rotation::Left => Rotation::Right,
      Rotation::Right => Rotation::Left,
    }
  }
}

impl Direction {
  /// Convert a Direction to a Coordinate
  pub const fn to_coordinate(&self) -> Coordinate {
    match self {
      Direction::Up => Coordinate::new(0, -1),
      Direction::UpRight => Coordinate::new(1, -1),
      Direction::Right => Coordinate::new(1, 0),
      Direction::DownRight => Coordinate::new(1, 1),
      Direction::Down => Coordinate::new(0, 1),
      Direction::DownLeft => Coordinate::new(-1, 1),
      Direction::Left => Coordinate::new(-1, 0),
      Direction::UpLeft => Coordinate::new(-1, -1),
    }
  }
  /// Rotate the direction by a given number of times
  pub fn rotate(self, direction: Rotation, times: usize) -> Self {
    let index = get_direction_index(&self);
    let rotations = times % DIRECTIONS;
    let new_index = match direction {
      // we require a cast to unsigned integer to avoid overflow when rotating left
      Rotation::Left => (index as i32 - rotations as i32 + DIRECTIONS as i32) as usize % DIRECTIONS,
      Rotation::Right => (index + rotations) % DIRECTIONS,
    };
    DIRECTION[new_index]
  }
  /// Check if a direction is cardinal
  pub fn is_cardinal(&self) -> bool { get_direction_index(self) % 2 == 0 }
  /// Check if a direction is ordinal
  pub fn is_ordinal(&self) -> bool { get_direction_index(self) % 2 != 0 }
}

impl From<Direction> for f32 {
  // Convert a direction to an angle where `Direction::Up` is 0.0
  fn from(direction: Direction) -> f32 {
    match direction {
      Direction::Up => 0.0,
      Direction::UpRight => 45.0,
      Direction::Right => 90.0,
      Direction::DownRight => 135.0,
      Direction::Down => 180.0,
      Direction::DownLeft => 225.0,
      Direction::Left => 270.0,
      Direction::UpLeft => 315.0,
    }
  }
}

impl TryFrom<Vec2<f32>> for Direction {
  type Error = String;
  fn try_from(v: Vec2<f32>) -> Result<Self, Self::Error> {
    let x = AxisDirection::new(v.x);
    let y = AxisDirection::new(v.y);
    match (x, y) {
      (AxisDirection::Positive, AxisDirection::Zero) => Ok(Direction::Right),
      (AxisDirection::Positive, AxisDirection::Positive) => Ok(Direction::DownRight),
      (AxisDirection::Zero, AxisDirection::Positive) => Ok(Direction::Down),
      (AxisDirection::Negative, AxisDirection::Positive) => Ok(Direction::DownLeft),
      (AxisDirection::Negative, AxisDirection::Zero) => Ok(Direction::Left),
      (AxisDirection::Negative, AxisDirection::Negative) => Ok(Direction::UpLeft),
      (AxisDirection::Zero, AxisDirection::Negative) => Ok(Direction::Up),
      (AxisDirection::Positive, AxisDirection::Negative) => Ok(Direction::UpRight),
      (AxisDirection::Zero, AxisDirection::Zero) => Err(String::from("Vector is not moving")),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_direction_to_coordinate() {
    assert_eq!(Direction::Up.to_coordinate(), Coordinate::new(0, -1));
    assert_eq!(Direction::UpRight.to_coordinate(), Coordinate::new(1, -1));
    assert_eq!(Direction::Right.to_coordinate(), Coordinate::new(1, 0));
    assert_eq!(Direction::DownRight.to_coordinate(), Coordinate::new(1, 1));
    assert_eq!(Direction::Down.to_coordinate(), Coordinate::new(0, 1));
    assert_eq!(Direction::DownLeft.to_coordinate(), Coordinate::new(-1, 1));
    assert_eq!(Direction::Left.to_coordinate(), Coordinate::new(-1, 0));
    assert_eq!(Direction::UpLeft.to_coordinate(), Coordinate::new(-1, -1));
  }

  #[test]
  fn test_direction_rotate() {
    assert_eq!(Direction::Up.rotate(Rotation::Left, 1), Direction::UpLeft);
    assert_eq!(Direction::Up.rotate(Rotation::Right, 1), Direction::UpRight);
    assert_eq!(Direction::Up.rotate(Rotation::Left, 2), Direction::Left);
    assert_eq!(Direction::Up.rotate(Rotation::Right, 2), Direction::Right);
    assert_eq!(Direction::Up.rotate(Rotation::Left, 3), Direction::DownLeft);
    assert_eq!(Direction::Up.rotate(Rotation::Right, 3), Direction::DownRight);
    assert_eq!(Direction::Up.rotate(Rotation::Left, 4), Direction::Down);
    assert_eq!(Direction::Up.rotate(Rotation::Right, 4), Direction::Down);
    assert_eq!(Direction::Up.rotate(Rotation::Left, 5), Direction::DownRight);
    assert_eq!(Direction::Up.rotate(Rotation::Right, 5), Direction::DownLeft);
    assert_eq!(Direction::Up.rotate(Rotation::Left, 6), Direction::Right);
    assert_eq!(Direction::Up.rotate(Rotation::Right, 6), Direction::Left);
    assert_eq!(Direction::Up.rotate(Rotation::Left, 7), Direction::UpRight);
    assert_eq!(Direction::Up.rotate(Rotation::Right, 7), Direction::UpLeft);
    assert_eq!(Direction::Up.rotate(Rotation::Left, 8), Direction::Up);
    assert_eq!(Direction::Up.rotate(Rotation::Right, 8), Direction::Up);
  }

  #[test]
  fn test_from_vec2() {
    assert_eq!(Direction::try_from(Vec2::new(0.0, -1.2)), Ok(Direction::Up), "Vector is moving Up");
    assert_eq!(Direction::try_from(Vec2::new(2.0, -1.2)), Ok(Direction::UpRight), "Vector is moving UpRight");
    assert_eq!(Direction::try_from(Vec2::new(10.0, 0.0)), Ok(Direction::Right), "Vector is moving Right");
    assert_eq!(Direction::try_from(Vec2::new(0.1, 7.2)), Ok(Direction::DownRight), "Vector is moving DownRight");
    assert_eq!(Direction::try_from(Vec2::new(0.0, 11.0)), Ok(Direction::Down), "Vector is moving Down");
    assert_eq!(Direction::try_from(Vec2::new(-0.02, 102.0)), Ok(Direction::DownLeft), "Vector is moving DownLeft");
    assert_eq!(Direction::try_from(Vec2::new(-37.0, 0.0)), Ok(Direction::Left), "Vector is moving Left");
    assert_eq!(Direction::try_from(Vec2::new(-1.0, -1.0)), Ok(Direction::UpLeft), "Vector is moving UpLeft");
    assert_eq!(Direction::try_from(Vec2::new(0.0, 0.0)), Err(String::from("Vector is not moving")), "Vector is not moving");
  }

  #[test]
  fn test_direction_to_angle() {
    assert_eq!(f32::from(Direction::Up), 0.0);
    assert_eq!(f32::from(Direction::UpRight), 45.0);
    assert_eq!(f32::from(Direction::Right), 90.0);
    assert_eq!(f32::from(Direction::DownRight), 135.0);
    assert_eq!(f32::from(Direction::Down), 180.0);
    assert_eq!(f32::from(Direction::DownLeft), 225.0);
    assert_eq!(f32::from(Direction::Left), 270.0);
    assert_eq!(f32::from(Direction::UpLeft), 315.0);
  }

  #[test]
  fn test_direction_is_cardinal() {
    assert_eq!(Direction::Up.is_cardinal(), true, "Up is cardinal");
    assert_eq!(Direction::UpRight.is_cardinal(), false, "UpRight is not cardinal");
    assert_eq!(Direction::Right.is_cardinal(), true, "Right is cardinal");
    assert_eq!(Direction::DownRight.is_cardinal(), false, "DownRight is not cardinal");
    assert_eq!(Direction::Down.is_cardinal(), true, "Down is cardinal");
    assert_eq!(Direction::DownLeft.is_cardinal(), false, "DownLeft is not cardinal");
    assert_eq!(Direction::Left.is_cardinal(), true, "Left is cardinal");
    assert_eq!(Direction::UpLeft.is_cardinal(), false, "UpLeft is not cardinal");
  }

  #[test]
  fn test_direction_is_ordinal() {
    assert_eq!(Direction::Up.is_ordinal(), false, "Up is not ordinal");
    assert_eq!(Direction::UpRight.is_ordinal(), true, "UpRight is ordinal");
    assert_eq!(Direction::Right.is_ordinal(), false, "Right is not ordinal");
    assert_eq!(Direction::DownRight.is_ordinal(), true, "DownRight is ordinal");
    assert_eq!(Direction::Down.is_ordinal(), false, "Down is not ordinal");
    assert_eq!(Direction::DownLeft.is_ordinal(), true, "DownLeft is ordinal");
    assert_eq!(Direction::Left.is_ordinal(), false, "Left is not ordinal");
    assert_eq!(Direction::UpLeft.is_ordinal(), true, "UpLeft is ordinal");
  }
}
use crate::engine::utility::alias::Coordinate;

/// Direction enum
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
}
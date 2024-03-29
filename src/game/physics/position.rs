use crate::engine::geometry::shape::Vec2;

#[derive(Default, Debug)]
pub struct Position(pub Vec2<f32>);

impl Position {
  pub fn new(x: f32, y: f32) -> Self {
    Self(Vec2::new(x, y))
  }
}

impl From<Position> for Vec2<f32> {
  /// convert from Position to Vec2<f32>
  fn from(position: Position) -> Self {
    position.0
  }
}

impl From<Vec2<f32>> for Position {
  /// convert from Vec2<f32> to Position
  fn from(vec: Vec2<f32>) -> Self {
    Self(vec)
  }
}
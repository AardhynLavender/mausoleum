use crate::engine::geometry::Vec2;

type PositionPrimitive = f32;

#[derive(Default, Debug)]
pub struct Position(pub Vec2<PositionPrimitive>);

impl Position {
  pub fn new(x: PositionPrimitive, y: PositionPrimitive) -> Self {
    Self(Vec2::new(x, y))
  }
}

impl From<Position> for Vec2<PositionPrimitive> {
  /// convert from Position to Vec2<f32>
  fn from(position: Position) -> Self {
    position.0
  }
}

impl From<Vec2<PositionPrimitive>> for Position {
  /// convert from Vec2<f32> to Position
  fn from(vec: Vec2<PositionPrimitive>) -> Self {
    Self(vec)
  }
}
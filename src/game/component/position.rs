use crate::engine::geometry::Vec2;

type PositionPrimitive = f32;

#[derive(Default, Debug)]
pub struct Position(pub Vec2<PositionPrimitive>);

impl Position {
  pub fn new(x: PositionPrimitive, y: PositionPrimitive) -> Self {
    Self(Vec2::new(x, y))
  }
}
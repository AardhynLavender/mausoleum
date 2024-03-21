use crate::engine::geometry::Vec2;

type GravityPrimitive = f32;

/// Add Gravity to an entity
#[derive(Default, Debug)]
pub struct Gravity(pub Vec2<GravityPrimitive>);

impl Gravity {
  pub fn new(x: GravityPrimitive, y: GravityPrimitive) -> Self {
    Self(Vec2::new(x, y))
  }
}

use crate::engine::geometry::shape::Vec2;
use crate::engine::system::SysArgs;
use crate::game::physics::position::Position;

/**
 * Velocity component
 */

/// Add Velocity to an entity
#[derive(Default, Debug, Clone, Copy)]
pub struct Velocity(pub Vec2<f32>);

impl Velocity {
  /// Instantiate a new Velocity component
  pub fn new(x: f32, y: f32) -> Self {
    Self(Vec2::new(x, y))
  }
}

impl From<Velocity> for Vec2<f32> {
  /// Convert from Velocity to Vec2<f32>
  fn from(position: Velocity) -> Self {
    position.0
  }
}

impl From<Vec2<f32>> for Velocity {
  /// Convert from Vec2<f32> to Velocity
  fn from(vec: Vec2<f32>) -> Self {
    Self(vec)
  }
}

/// Apply `Velocity` components to `Position` components
pub fn sys_velocity(SysArgs { delta, world, .. }: &mut SysArgs) {
  for (_, (position, velocity)) in world.query::<(&mut Position, &mut Velocity)>() {
    position.0 = position.0 - velocity.0 * *delta;
  }
}

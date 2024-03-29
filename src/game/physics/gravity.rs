use crate::engine::geometry::shape::Vec2;
use crate::engine::system::SysArgs;
use crate::game::constant::MAX_GRAVITY;
use crate::game::physics::velocity::Velocity;

/**
 * Systems and components relating to gravity physics
 */

/// Add Gravity to an entity
#[derive(Default, Debug)]
pub struct Gravity(pub Vec2<f32>);

impl Gravity {
  pub fn new(v: Vec2<f32>) -> Self {
    Self(v)
  }
}

/// Process gravity in a world
pub fn sys_gravity(SysArgs { delta, world, .. }: &mut SysArgs) {
  for (_, (gravity, velocity)) in world.query::<(&mut Gravity, &mut Velocity)>() {
    velocity.0 = velocity.0 + (gravity.0 * *delta);
    velocity.0.y = velocity.0.y.min(MAX_GRAVITY);
  }
}


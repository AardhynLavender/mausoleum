/**
 * Gravity component and system
 */

use crate::engine::geometry::shape::Vec2;
use crate::engine::system::{SysArgs, Systemize};
use crate::game::physics::frozen::Frozen;
use crate::game::physics::velocity::Velocity;

pub const MAX_GRAVITY: f32 = 400.0;

/// Marks an entity subject to gravity
#[derive(Debug)]
pub struct Gravity(pub Vec2<f32>);

impl Gravity {
  /// Instantiate a new Gravity component
  pub fn new(v: Vec2<f32>) -> Self { Self(v) }
}

/// Process gravity in a world
impl Systemize for Gravity {
  /// Process gravity each frame
  fn system(SysArgs { delta, world, .. }: &mut SysArgs) -> Result<(), String> {
    for (_, (gravity, velocity)) in world
      .query::<(&mut Gravity, &mut Velocity)>()
      .without::<&Frozen>()
    {
      let gravity = Vec2::new(gravity.0.x, gravity.0.y);
      velocity.0 = velocity.0 + gravity * *delta;
      velocity.0.y = velocity.0.y.min(MAX_GRAVITY);
    }

    Ok(())
  }
}
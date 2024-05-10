/**
 * Velocity component
 */

use crate::engine::geometry::shape::Vec2;
use crate::engine::system::{SysArgs, Systemize};
use crate::game::physics::frozen::Frozen;
use crate::game::physics::position::Position;

/// Add Velocity to an entity
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Velocity(pub Vec2<f32>);

impl Velocity {
  /// Instantiate a new Velocity component
  pub fn new(x: f32, y: f32) -> Self {
    Self(Vec2::new(x, y))
  }

  /// Check if the velocity moves horizontally in the positive direction
  pub fn is_going_left(&self) -> bool { self.0.x < 0.0 }
  /// Check if the velocity moves horizontally in the negative direction
  pub fn is_going_right(&self) -> bool { self.0.x > 0.0 }
  /// Check if the velocity moves vertically in the positive direction
  pub fn is_going_up(&self) -> bool { self.0.y < 0.0 }
  /// Check if the velocity moves vertically in the negative direction
  pub fn is_going_down(&self) -> bool { self.0.y > 0.0 }

  /// Reverse the x component of the velocity
  pub fn reverse_x(&mut self) { self.0.x = -self.0.x; }
  /// Reverse the y component of the velocity
  pub fn reverse_y(&mut self) { self.0.y = -self.0.y; }
  /// Reverse the velocity
  #[inline]
  pub fn reverse(&mut self) {
    self.reverse_x();
    self.reverse_y();
  }
  // Remove the x component of the velocity
  pub fn remove_x(&mut self) { self.0.x = 0.0; }
  // Remove the y component of the velocity
  pub fn remove_y(&mut self) { self.0.y = 0.0; }
  #[inline]
  // Remove the velocity
  pub fn remove(&mut self) {
    self.remove_x();
    self.remove_y();
  }
}

impl From<Velocity> for Vec2<f32> {
  /// Convert from Velocity to Vec2<f32>
  fn from(position: Velocity) -> Self { position.0 }
}

impl From<Vec2<f32>> for Velocity {
  /// Convert from Vec2<f32> to Velocity
  fn from(vec: Vec2<f32>) -> Self { Self(vec) }
}

impl Systemize for Velocity {
  /// Process velocity each frame
  fn system(SysArgs { delta, world, .. }: &mut SysArgs) -> Result<(), String> {
    for (_, (position, velocity)) in world
      .query::<(&mut Position, &mut Velocity)>()
      .without::<&Frozen>()
    {
      position.0 = position.0 + velocity.0 * *delta;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_velocity() {
    let velocity = Velocity::new(1.0, 1.0);
    assert_eq!(velocity.0, Vec2::new(1.0, 1.0), "Velocity was instantiated");
  }

  #[test]
  fn test_velocity_directions() {
    let right_up = Velocity::new(1.0, -1.0);
    assert!(right_up.is_going_up(), "Velocity is going up");
    assert!(right_up.is_going_right(), "Velocity is going right");
    assert!(!right_up.is_going_down(), "Velocity is not going down");
    assert!(!right_up.is_going_left(), "Velocity is not going left");

    let left_down = Velocity::new(-1.0, 1.0);
    assert!(!left_down.is_going_up(), "Velocity is not going up");
    assert!(!left_down.is_going_right(), "Velocity is not going right");
    assert!(left_down.is_going_down(), "Velocity is going down");
    assert!(left_down.is_going_left(), "Velocity is going left");
  }

  #[test]
  fn test_velocity_reverse() {
    let mut velocity = Velocity::new(1.0, 1.0);
    velocity.reverse();
    assert_eq!(velocity.0, Vec2::new(-1.0, -1.0), "Velocity is reversed");
  }

  #[test]
  fn test_velocity_remove() {
    let mut velocity = Velocity::new(1.0, 1.0);
    velocity.remove();
    assert_eq!(velocity.0, Vec2::new(0.0, 0.0), "Velocity has been removed");
  }
}
/**
 * Cooldown structure and system
 */

use std::collections::HashSet;

use crate::engine::system::SysArgs;

/// A cooldown that can be used to limit the rate of some action.
#[derive(Debug, Default)]
#[allow(unused_variables)]
pub struct Cooldown {
  remaining: f32,
  active: bool,
}

impl Cooldown {
  /// Instantiates a new cooldown returning an error if the duration is invalid.
  pub fn build(duration_ms: u32) -> Result<Self, String> {
    Ok(Self {
      remaining: duration_ms as f32,
      active: true,
    })
  }
  /// Assert the cooldown is active
  fn active_invariant(&self) { assert!(self.active, "Ticking an inactive cooldown"); }
  // Decrease the remaining time
  fn tick(&mut self, delta: f32) { self.remaining -= delta }
  /// Update the cooldown and return the remaining time
  fn update(&mut self, delta: f32) -> f32 {
    self.active_invariant(); // a cooldown must be active to tick
    self.tick(delta);
    self.remaining.max(0.0)
  }
}

/// update the cooldowns of the world, removing any that have expired
pub fn sys_cooldown(SysArgs { world, delta, .. }: &mut SysArgs) {
  let delta = *delta * 1000.0; // convert to milliseconds
  let mut removal_queue = HashSet::new();
  for (entity, cooldown) in world.query::<&mut Cooldown>() {
    if cooldown.update(delta) == 0.0 { removal_queue.insert(entity); }
  }

  for entity in removal_queue {
    world
      .remove_components::<(Cooldown, )>(entity)
      .expect("Failed to remove expired cooldown");
  }
}
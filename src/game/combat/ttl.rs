use std::time::Duration;

use crate::engine::system::SysArgs;
use crate::engine::time::Timer;

/// Define the lifetime of an entity in milliseconds
pub struct TimeToLive(pub Timer);

impl TimeToLive {
  /// Instance a new lifetime
  pub fn new(ttl_ms: u64) -> Self {
    Self(Timer::new(Duration::from_millis(ttl_ms), true))
  }
}

/// Handle the cleanup of timed lifetime entities
pub fn sys_ttl(SysArgs { world, .. }: &mut SysArgs) {
  let to_free: Vec<_> = world
    .query::<&TimeToLive>()
    .into_iter()
    .filter(|(_, ttl)| ttl.0.done())
    .map(|(entity, _)| entity)
    .collect();
  for entity in to_free {
    // ttl will ensure there is no entity after the timer is done
    // it doesn't care if no entity is found
    world.free_now(entity).ok();
  }
}

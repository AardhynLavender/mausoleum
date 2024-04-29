/**
 * Frozen component
 **/

use std::time::Duration;

use hecs::Entity;

use crate::engine::geometry::collision::{CollisionBox, CollisionMask};
use crate::engine::system::SysArgs;
use crate::engine::tile::tile::TileCollider;
use crate::engine::time::Timer;
use crate::engine::world::World;

/// Mark an entity as frozen.
///
/// Frozen entities _should_ not affected by physics.
#[derive(Copy, Clone)]
pub struct Frozen(pub Timer);

impl Frozen {
  /// Instantiate a new Frozen component
  pub fn new(thaw_ms: u64) -> Self {
    let duration = Duration::from_millis(thaw_ms);
    Self(Timer::new(duration, true))
  }
}

/// Add the frozen component to an entity
pub fn freeze_entity(entity: Entity, collision_box: CollisionBox, world: &mut World, thaw_ms: u64) -> Result<(), String> {
  world.add_components(entity, (
    Frozen::new(thaw_ms),
    TileCollider::new(collision_box, CollisionMask::full())
  ))
}

/// Remove the frozen component from an entity
pub fn thaw_entity(entity: Entity, world: &mut World) -> Result<(), String> {
  world.remove_components::<(
    Frozen,
    TileCollider
  )>(entity).map(|_| ())
}

/// Thaw entities marked as frozen when their timer is up
pub fn sys_thaw(SysArgs { world, .. }: &mut SysArgs) {
  let frozen_entities = world
    .query::<&Frozen>()
    .into_iter()
    .map(|(entity, frozen)| (entity, *frozen))
    .collect::<Vec<(Entity, Frozen)>>();
  for (entity, frozen) in frozen_entities {
    if frozen.0.done() { thaw_entity(entity, world).expect("Failed to thaw entity"); }
  }
}

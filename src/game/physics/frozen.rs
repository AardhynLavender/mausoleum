/**
 * Frozen component
 **/

use std::time::Duration;

use hecs::Entity;

use crate::engine::geometry::collision::{CollisionBox, CollisionMask};
use crate::engine::system::{SysArgs, Systemize};
use crate::engine::tile::tile::TileCollider;
use crate::engine::time::Timer;
use crate::engine::world::World;
use crate::game::scene::level::collision::RoomCollision;

/// Mark an entity as frozen.
///
/// Frozen entities are not effected by physics and other systems until they thaw.
#[derive(Copy, Clone)]
pub struct Frozen(pub Timer);

pub struct FreezeResistant;

impl Frozen {
  /// Instantiate a new Frozen component
  pub fn new(thaw_ms: u64) -> Self {
    let duration = Duration::from_millis(thaw_ms);
    Self(Timer::new(duration, true))
  }
}

impl Systemize for Frozen {
  /// Process thawing entities
  fn system(SysArgs { world, .. }: &mut SysArgs) -> Result<(), String> {
    let frozen_entities = world
      .query::<&Frozen>()
      .into_iter()
      .map(|(entity, frozen)| (entity, *frozen))
      .collect::<Vec<(Entity, Frozen)>>();

    for (entity, frozen) in frozen_entities {
      if frozen.0.done() { thaw_entity(entity, world).expect("Failed to thaw entity"); }
    }

    Ok(())
  }
}

/// Add the frozen component to an entity
pub fn freeze_entity(entity: Entity, collision_box: CollisionBox, world: &mut World, thaw_ms: u64) -> Result<bool, String> {
  if world.get_component::<FreezeResistant>(entity).is_ok() { return Ok(false); }

  world.add_components(entity, (
    Frozen::new(thaw_ms),
    TileCollider::new(collision_box, CollisionMask::full()),
    RoomCollision::All,
  ))?;

  Ok(true)
}

/// Remove the frozen component from an entity
pub fn thaw_entity(entity: Entity, world: &mut World) -> Result<(), String> {
  world.remove_components::<(
    Frozen,
    TileCollider,
    RoomCollision,
  )>(entity).map(|_| ())?;
  world.add_components(entity, (
    RoomCollision::Creature,
  ))
}


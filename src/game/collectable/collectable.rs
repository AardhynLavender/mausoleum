use crate::engine::geometry::collision::{CollisionBox, CollisionMask, rec2_collision};
use crate::engine::system::SysArgs;
use crate::engine::tile::query::{TileHandle, TileQuery};
use crate::engine::tile::tile::TileCollider;
use crate::game::physics::collision::{Collider, make_collision_box};
use crate::game::physics::position::Position;
use crate::game::scene::level::meta::{CollectableType, TileLayerType};
use crate::game::scene::level::room::use_room;

/// Marks a combo as collectable
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Collectable(pub CollectableType);

#[derive(Default, Debug)]
pub struct Collection(Vec<Collectable>);

impl Collection {
  // Check if the collection contains a collectable
  pub fn has(&self, collectable: &Collectable) -> bool { self.0.contains(&collectable) }
  /// Take a collectable from the collection
  pub fn take(&mut self, collectable: Collectable) -> Option<Collectable> {
    if let Some(index) = self
      .0
      .iter()
      .position(|c| *c == collectable)
    {
      Some(self.0.swap_remove(index))
    } else {
      None
    }
  }
  /// count the number of collectables in the collection
  pub fn count(&self, collectable: &Collectable) -> usize {
    self
      .0
      .iter()
      .filter(|c| **c == *collectable)
      .count()
  }
}

pub fn sys_collectable(SysArgs { world, state, .. }: &mut SysArgs) {
  let collectables = world
    .query::<(&Collectable, &Position, &TileCollider)>()
    .into_iter()
    .map(|(entity, (collectable, position, collider))| {
      let collision_box = CollisionBox::new(position.0 + collider.collision_box.origin, collider.collision_box.size);
      (entity, *collectable, collision_box)
    })
    .collect::<Vec<_>>();

  if collectables.is_empty() { return; }

  let to_free = collectables
    .into_iter()
    .filter_map(|(collectable, collected, collectable_box)| {
      for (_, (collector_position, collector_collider, collection)) in world
        .query::<(&Position, &Collider, &mut Collection)>()
      {
        let collector_box = make_collision_box(collector_position, collector_collider);
        if rec2_collision(&collector_box, &collectable_box, CollisionMask::full()).is_some() {
          (*collection).0.push(collected);
          return Some(collectable);
        }
      }
      None
    })
    .collect::<Vec<_>>();

  let room = use_room(state);
  for collectable in to_free {
    let tile_query = room.query_tile(TileLayerType::Collision, TileQuery::Entity(collectable));
    let tile_handle = TileHandle::try_from(tile_query).expect("Failed to create handle for tile");
    room.remove_tile(world, tile_handle);
  }
}
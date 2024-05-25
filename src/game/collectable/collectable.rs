use crate::engine::geometry::collision::{CollisionBox, CollisionMask, rec2_collision};
use crate::engine::system::{SysArgs, Systemize};
use crate::engine::tile::query::{TileHandle, TileQuery};
use crate::engine::tile::tile::TileCollider;
use crate::engine::tile::tilemap::TilemapMutation;
use crate::game::collectable::modal::make_collectable_modal;
use crate::game::physics::collision::{Collider, make_collision_box};
use crate::game::physics::position::Position;
use crate::game::scene::level::meta::{Collectable, TileLayerType};
use crate::game::scene::level::room::use_room;
use crate::game::scene::level::scene::LevelState;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Collection(Vec<Collectable>);

impl Collection {
  /// Instantiate a new collection
  pub fn new(items: impl Iterator<Item=Collectable>) -> Self { Self(items.collect()) }
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
  /// Iterate over the collection
  pub fn iter(&self) -> impl Iterator<Item=&Collectable> { self.0.iter() }
}

impl Systemize for Collection {
  fn system(SysArgs { world, state, event, asset, .. }: &mut SysArgs) -> Result<(), String> {
    let collectables = world
      .query::<(&Collectable, &Position, &TileCollider)>()
      .into_iter()
      .map(|(entity, (collectable, position, collider))| {
        let collision_box = CollisionBox::new(position.0 + collider.collision_box.origin, collider.collision_box.size);
        (entity, *collectable, collision_box)
      })
      .collect::<Vec<_>>();

    if collectables.is_empty() { return Ok(()); }

    let mut modal_data = None;

    let data = &state.get::<LevelState>()?.weapon_data;
    let to_free = collectables
      .into_iter()
      .filter_map(|(collectable, collected, collectable_box)| {
        for (_, (collector_position, collector_collider, collection)) in world
          .query::<(&Position, &Collider, &mut Collection)>()
        {
          let collector_box = make_collision_box(collector_position, collector_collider);
          if rec2_collision(&collector_box, &collectable_box, CollisionMask::full()).is_some() {
            if !collection.has(&collected) {
              if modal_data.is_some() {
                eprintln!("Collection gained more than one new collectable in the same frame!");
              } else {
                // show the data modal when it's a new item
                modal_data = Some(data.get_data(&collected).clone());
              }
            }
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
      room.remove_tile(world, tile_handle, TilemapMutation::Session); // todo: change to `TilemapMutation::Persistent` when implemented
    }

    if let Some(collected) = modal_data {
      make_collectable_modal(world, event, asset, state, &collected);
    }

    Ok(())
  }
}

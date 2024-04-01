use hecs::QueryMut;

use crate::engine::geometry::shape::Vec2;
use crate::engine::render::component::Sprite;
use crate::engine::tile::tile::Tile;
use crate::engine::tile::tilemap::Tilemap;
use crate::engine::world::{EntityManager, World};
use crate::game::physics::position::Position;

pub struct Room {
  tilemap: Tilemap,
}

impl Room {
  pub fn new(tilemap: Tilemap) -> Self {
    Self { tilemap }
  }
}

impl EntityManager for Room {
  type Manager = Tilemap;
  type ComponentQuery<'q> = (&'q Position, &'q Tile, &'q Sprite);

  /// add the tiles to the world
  fn add_to_world(&mut self, world: &mut World) {
    self.tilemap.add_to_world(world, Vec2::new(0.0, 0.0)).expect("Failed to add tilemap to world");
  }
  /// remove the tiles from the world
  fn remove_from_world(&mut self, world: &mut World) -> Result<(), String> {
    self.tilemap.remove_from_world(world)?;
    Ok(())
  }
  /// query the world for the entities of the manager
  fn query_entities<'q>(&'q mut self, world: &'q mut World) -> QueryMut<Self::ComponentQuery<'q>> {
    world.query::<Self::ComponentQuery<'q>>()
  }
}

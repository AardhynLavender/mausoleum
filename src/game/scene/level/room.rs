/**
 * Room creation and management
 */

use std::collections::HashSet;

use hecs::{DynamicBundle, Entity};

use crate::engine::geometry::collision::CollisionBox;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::rendering::camera::CameraBounds;
use crate::engine::rendering::color::{OPAQUE, RGBA};
use crate::engine::rendering::component::Sprite;
use crate::engine::rendering::renderer::layer;
use crate::engine::state::State;
use crate::engine::system::SysArgs;
use crate::engine::tile::tile::{Tile, TileCollider};
use crate::engine::tile::tilemap::{TileHandle, Tilemap, TileQuery, TileQueryResult};
use crate::engine::world::World;
use crate::game::constant::TILE_SIZE;
use crate::game::physics::position::Position;
use crate::game::scene::level::meta::{Soft, Strong, TileBreakability, TileMeta};
use crate::game::scene::level::registry::RoomRegistry;
use crate::game::utility::controls::{Behaviour, Control, is_control};

pub const ROOM_ENTER_MARGIN: i32 = TILE_SIZE.x as i32 / 2;

/// A key to identify a room
pub type RoomKey = String;

// Components //

/// Add room entry detection to an entity
#[derive(Debug, Clone)]
pub struct RoomCollider {
  pub collision_box: CollisionBox,
  pub room: RoomKey,
}

impl RoomCollider {
  pub fn new(collision_box: CollisionBox, room: RoomKey) -> Self {
    Self { collision_box, room }
  }
}

/// Mark an entity with a `RoomCollider` as active
#[derive(Debug, Clone, Default)]
pub struct ActiveRoom;

// Structures //

pub struct Room {
  position: Vec2<f32>,
  tilemap: Tilemap<TileMeta>,
  entities: HashSet<Entity>,
}

impl Room {
  /// Instantiate a new room
  pub fn build(tilemap: Tilemap<TileMeta>, position: Vec2<f32>) -> Self {
    Self { tilemap, position, entities: HashSet::new() }
  }

  // Tilemap //

  /// Create and add tiles associated with the tilemap to the world
  fn add_tilemap_to_world(&mut self, world: &mut World) -> Result<(), String> {
    let tilemap_position = self.position;
    self.tilemap.add_tiles(|tile, _, position| {
      let position = position + tilemap_position;
      let entity = world.add((
        Tile::new(tile.data.tile_key),
        Position::from(position),
        Sprite::new(tile.data.texture_key, tile.data.src),
        layer::Layer5
      ));

      // add a collider if the tile has a mask
      if !tile.mask.is_empty() {
        let collider = TileCollider::new(
          Rec2::new(Vec2::default(), tile.data.src.size),
          tile.mask,
        );
        world.add_components(entity, (collider, ))?;
      }

      if tile.data.meta.breakability == TileBreakability::Soft {
        world.add_components(entity, (Soft, ))?;
      } else if tile.data.meta.breakability == TileBreakability::Strong {
        world.add_components(entity, (Strong, ))?;
      }

      Ok(entity)
    })
  }
  /// Remove the tiles from the world
  fn remove_tilemap_from_world(&mut self, world: &mut World) {
    self.tilemap.remove_tiles(|entity| world.free_now(entity).unwrap_or(()));
  }
  pub fn remove_tile(&mut self, world: &mut World, query: TileHandle<TileMeta>) {
    self.tilemap.remove_tile(query, |entity| world.free_now(entity).unwrap_or(()));
  }

  // Entities //

  // Add a new entity associated with the room
  pub fn add_entity(&mut self, world: &mut World, components: impl DynamicBundle) {
    self.entities.insert(world.add(components));
  }
  /// Attempt to remove an entity from the world that is registered with this room
  pub fn remove_entity(&mut self, entity: Entity, world: &mut World) -> bool {
    if self.entities.remove(&entity) { return world.free_now(entity).is_ok(); };
    false
  }
  /// Remove all entities associated with the room
  fn remove_entities(&mut self, world: &mut World) {
    for entity in self.entities.drain() {
      // ignore errors as some entities may have already been removed
      world.free_now(entity).ok();
    }
  }

  // Room //

  /// Add the entities and tilemap associated with the room to the world
  pub fn add_to_world(&mut self, world: &mut World) -> Result<(), String> {
    self.add_tilemap_to_world(world)
  }
  // Remove the entities associated with the room from the world
  pub fn remove_from_world(&mut self, world: &mut World) {
    self.remove_tilemap_from_world(world);
    self.remove_entities(world);
  }

  // Query //

  /// Get information about a tile in the current room at a position in worldspace
  pub fn query_tile(&mut self, get: TileQuery) -> TileQueryResult<TileMeta> {
    let mut result = if let TileQuery::Position(position) = get {
      let position = position - self.position; // convert to local position
      self.tilemap.query_tile(TileQuery::Position(position))
    } else {
      self.tilemap.query_tile(get)
    };
    result.2 = result.2 + self.position; // convert to world position
    result
  }
  /// Get the bounds of the tilemap in worldspace
  pub fn get_bounds(&self) -> CameraBounds {
    let position = Vec2::from(self.position);
    let dimensions = self.tilemap.get_dimensions();
    CameraBounds::new(position, dimensions)
  }
}

/// Render rectangles around the colliders that start room transitions
pub fn sys_render_room_colliders(SysArgs { world, render, camera, event, .. }: &mut SysArgs) {
  if !is_control(Control::Debug, Behaviour::Held, event) { return; }
  for (_, room_collider) in world.query::<&RoomCollider>() {
    let pos = Vec2::<i32>::from(camera.translate(room_collider.collision_box.origin));
    render.draw_rect(Rec2::new(pos, room_collider.collision_box.size), RGBA::new(0, 0, 255, OPAQUE));
  }
}

/// Use the current room mutably
/// ## Panics
/// if the `RoomRegistry` not in state or the current room is `None`
pub fn use_room(state: &mut State) -> &mut Room {
  state.get_mut::<RoomRegistry>()
    .expect("Failed to get RoomRegistry")
    .get_current_mut()
    .ok_or("Failed to get current room")
    .unwrap()
}
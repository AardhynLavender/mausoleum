/**
 * Room creation and management
 */

use std::collections::HashSet;

use hecs::{DynamicBundle, Entity};

use crate::engine::asset::AssetManager;
use crate::engine::geometry::collision::{CollisionBox, CollisionMask};
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::rendering::camera::CameraBounds;
use crate::engine::rendering::color::{OPAQUE, RGBA};
use crate::engine::rendering::component::Sprite;
use crate::engine::rendering::renderer::layer;
use crate::engine::state::State;
use crate::engine::system::SysArgs;
use crate::engine::tile::query::{TileHandle, TileQuery, TileQueryResult};
use crate::engine::tile::tile::{Tile, TileCollider};
use crate::engine::tile::tilemap::{Tilemap, TilemapMutation};
use crate::engine::utility::direction::{HALF_DIRECTION_ROTATION, Rotation};
use crate::engine::world::World;
use crate::game::combat::damage::Damage;
use crate::game::constant::TILE_SIZE;
use crate::game::creature::angry_buzz::make_angry_buzz;
use crate::game::creature::bubbly::make_bubbly;
use crate::game::creature::buzz::make_buzz;
use crate::game::creature::grunt::make_grunt;
use crate::game::creature::ripper::make_ripper;
use crate::game::creature::rotund::make_rotund;
use crate::game::creature::spiky::make_spiky;
use crate::game::creature::spore::make_spore;
use crate::game::creature::zoomer::make_zoomer;
use crate::game::persistence::world::make_save_area;
use crate::game::physics::collision::Fragile;
use crate::game::physics::position::Position;
use crate::game::player::combat::PlayerHostile;
use crate::game::preferences::use_preferences;
use crate::game::scene::level::meta::{ObjMeta, Soft, Strong, TileBreakability, TileLayerType, TileMeta};
use crate::game::scene::level::registry::RoomRegistry;

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
  name: String,
  position: Vec2<f32>,
  tilemap: Tilemap<TileMeta, TileLayerType, ObjMeta>,
  entities: HashSet<Entity>,
}

impl Room {
  /// Instantiate a new room
  pub fn build(name: String, tilemap: Tilemap<TileMeta, TileLayerType, ObjMeta>, position: Vec2<f32>) -> Self {
    Self { name, tilemap, position, entities: HashSet::new() }
  }

  // Tilemap //

  /// Create and add tiles associated with the tilemap to the world
  fn add_tilemap_to_world(&mut self, world: &mut World) -> Result<(), String> {
    let tilemap_position = self.position;
    self.tilemap.add_tiles(|layer, tile, _, position| {
      let position = position + tilemap_position;
      let entity = world.add((
        Tile::new(tile.data.tile_key),
        Position::from(position),
        Sprite::new(tile.data.texture_key, tile.data.src),
      ));

      // add render layer
      match layer {
        TileLayerType::Background => world.add_components(entity, (layer::Layer4, ))?,
        TileLayerType::Foreground => world.add_components(entity, (layer::Layer7, ))?,
        TileLayerType::Collision => world.add_components(entity, (layer::Layer6, ))?,
      }

      // add a collider if the tile has a mask
      if layer == TileLayerType::Collision {
        if !tile.mask.is_empty() {
          let collider = TileCollider::new(
            Rec2::new(Vec2::default(), tile.data.src.size),
            tile.mask,
          );
          world.add_components(entity, (collider, ))?;
        }

        if let Some(collectable) = tile.data.meta.collectable {
          world.add_components(entity, (collectable, ))?;
        }

        match tile.data.meta.breakability {
          TileBreakability::Soft => world.add_components(entity, (Soft, ))?,
          TileBreakability::Strong => world.add_components(entity, (Strong, ))?,
          TileBreakability::Brittle => world.add_components(entity, (Fragile, ))?,
          _ => (),
        }

        let damage = tile.data.meta.damage;
        if damage > 0 {
          world.add_components(entity, (PlayerHostile, Damage::new(damage)))?;
        }
      }

      Ok(entity)
    })
  }
  /// Remove the tiles from the world
  fn remove_tilemap_from_world(&mut self, world: &mut World) {
    self.tilemap.remove_tiles(|entity| world.free_now(entity).unwrap_or(()));
  }
  /// Remove a tile from the tilemap
  pub fn remove_tile(&mut self, world: &mut World, handle: TileHandle<TileMeta, TileLayerType>, mutation: TilemapMutation) {
    self.tilemap.remove_tile(
      &handle,
      |entity| {
        world.free_now(entity).unwrap_or(())
      },
      mutation,
    );
    self.tilemap.for_neighbour(
      &handle,
      |handle, neighbour| {
        // Repair the neighbor tile by adding a collider or updating the mask
        if !world.has_component::<TileCollider>(handle.entity).expect("Failed to check tile tile collider") {
          let collision_box = CollisionBox::new(Vec2::default(), handle.concept.data.src.size);
          world.add_components(handle.entity, (TileCollider::new(collision_box, CollisionMask::default()), )).expect("Failed to add tile collider");
        }
        let mut tile_collider = world.get_component_mut::<TileCollider>(handle.entity).expect("Failed to retrieve tile collider");
        tile_collider.mask.set_side(neighbour.rotate(Rotation::Left, HALF_DIRECTION_ROTATION), true).expect("failed to set side");
        let new_mask = tile_collider.mask.clone();
        handle.concept.mask = new_mask;
      },
      mutation,
    );
  }

  // Entities //

  pub fn add_entities_to_world(&mut self, world: &mut World, assets: &mut AssetManager) -> Result<(), String> {
    self.tilemap.add_objects(|object| {
      let entity = match object {
        ObjMeta::AngryBuzzConcept { position } => world.add(make_angry_buzz(assets, self.position + *position)?),
        ObjMeta::BubblyConcept { position, direction, .. } => world.add(make_bubbly(assets, self.position + *position, *direction)?),
        ObjMeta::BuzzConcept { position } => world.add(make_buzz(assets, self.position + *position)?),
        ObjMeta::EventConcept { position } => world.add((Position::from(self.position + *position), )),
        ObjMeta::GruntConcept { position } => world.add(make_grunt(assets, self.position + *position)?),
        ObjMeta::SaveAreaConcept { position, collision_box } => world.add(make_save_area(self.name.clone(), CollisionBox::new(self.position + *position, collision_box.size))?),
        ObjMeta::SpikyConcept { direction, position } => world.add(make_spiky(assets, self.position + *position, *direction)?),
        ObjMeta::SporeConcept { direction, position } => world.add(make_spore(assets, self.position + *position, *direction)?),
        ObjMeta::RipperConcept { direction, position } => world.add(make_ripper(assets, self.position + *position, *direction)?),
        ObjMeta::RotundConcept { direction, position, spit_axis } => world.add(make_rotund(assets, self.position + *position, *direction, *spit_axis)?),
        ObjMeta::ZoomerConcept { direction, position } => world.add(make_zoomer(assets, self.position + *position, *direction)?),
      };
      self.entities.insert(entity);
      Ok(entity)
    })
  }
  // Add a new entity associated with the room
  pub fn add_entity(&mut self, world: &mut World, components: impl DynamicBundle) {
    self.entities.insert(world.add(components));
  }
  /// Attempt to remove an entity from the world that is registered with this room
  pub fn remove_entity(&mut self, entity: Entity, world: &mut World, mutation: TilemapMutation) -> Result<(), String> {
    if !self.entities.remove(&entity) { return Err(String::from("Entity not found in room")); }
    world.free_now(entity)?;
    if mutation == TilemapMutation::Session {
      self.tilemap.remove_object(entity, |entity| {
        world.free_now(entity).ok();
      }, mutation)?;
    }
    Ok(())
  }
  /// Remove all entities associated with the room
  fn remove_entities(&mut self, world: &mut World) {
    for entity in self.entities.drain() {
      world.free_now(entity).ok(); // ignore errors as some entities may have already been removed
    }
  }

  // Room //

  /// Add the entities and tilemap associated with the room to the world
  pub fn add_to_world(&mut self, world: &mut World, assets: &mut AssetManager) -> Result<(), String> {
    self.add_tilemap_to_world(world)?;
    self.add_entities_to_world(world, assets)
  }
  // Remove the entities associated with the room from the world
  pub fn remove_from_world(&mut self, world: &mut World) {
    self.remove_tilemap_from_world(world);
    self.remove_entities(world);
  }

  // Query //

  /// Get information about a tile in the current room at a position in worldspace
  pub fn query_tile(&mut self, layer: TileLayerType, query: TileQuery) -> TileQueryResult<TileMeta, TileLayerType> {
    let mut result = if let TileQuery::Position(position) = query {
      let position = position - self.position; // convert to local position
      self.tilemap.query_tile(layer, TileQuery::Position(position))
    } else {
      self.tilemap.query_tile(layer, query)
    };
    result.position = result.position + self.position; // convert to world position
    result
  }
  /// Get the bounds of the tilemap in worldspace
  pub fn get_bounds(&self) -> CameraBounds {
    let position = Vec2::from(self.position);
    let dimensions = self.tilemap.get_dimensions();
    CameraBounds::new(position, dimensions)
  }
  /// Get the name of the room
  pub fn get_name(&self) -> String { self.name.clone() }
}

/// Render rectangles around the colliders that start room transitions
pub fn sys_render_room_colliders(SysArgs { world, render, camera, state, .. }: &mut SysArgs) -> Result<(), String> {
  if !use_preferences(state).debug { return Ok(()); }
  for (_, room_collider) in world.query::<&RoomCollider>() {
    let pos = Vec2::<i32>::from(camera.translate(room_collider.collision_box.origin));
    render.draw_rect(Rec2::new(pos, room_collider.collision_box.size), RGBA::new(0, 0, 255, OPAQUE));
  }
  Ok(())
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
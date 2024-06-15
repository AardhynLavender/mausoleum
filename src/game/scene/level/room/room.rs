/**
 * Room creation and management
 */

use std::collections::HashSet;

use hecs::{DynamicBundle, Entity};

use crate::engine::asset::asset::AssetManager;
use crate::engine::component::position::Position;
use crate::engine::component::sprite::Sprite;
use crate::engine::ecs::system::SysArgs;
use crate::engine::ecs::world::World;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::math::conversion::coordinate_to_index;
use crate::engine::render::camera::CameraBounds;
use crate::engine::render::renderer::layer;
use crate::engine::utility::color::{OPAQUE, RGBA};
use crate::engine::utility::direction::{HALF_DIRECTION_ROTATION, Rotation};
use crate::engine::utility::state::State;
use crate::game::constant::TILE_SIZE;
use crate::game::persistence::world::make_save_area;
use crate::game::preferences::use_preferences;
use crate::game::scene::level::combat::damage::Damage;
use crate::game::scene::level::creature::angry_buzz::make_angry_buzz;
use crate::game::scene::level::creature::bubbly::make_bubbly;
use crate::game::scene::level::creature::buzz::make_buzz;
use crate::game::scene::level::creature::grunt::make_grunt;
use crate::game::scene::level::creature::ripper::make_ripper;
use crate::game::scene::level::creature::rotund::make_rotund;
use crate::game::scene::level::creature::spiky::make_spiky;
use crate::game::scene::level::creature::spore::make_spore;
use crate::game::scene::level::creature::zoomer::make_zoomer;
use crate::game::scene::level::physics::collision::Fragile;
use crate::game::scene::level::player::combat::PlayerHostile;
use crate::game::scene::level::room::collision::{CollisionBox, CollisionMask, RoomCollision};
use crate::game::scene::level::room::meta::{ObjMeta, Soft, Strong, TileBreakability, TileLayerType, TileMeta};
use crate::game::scene::level::scene::LevelState;
use crate::game::scene::level::story::data::Story;
use crate::game::scene::level::story::world::make_story_area;
use crate::game::scene::level::tile::query::{TileHandle, TileQuery, TileQueryResult};
use crate::game::scene::level::tile::tile::{Tile, TileCollider, TileKey};
use crate::game::scene::level::tile::tilemap::{MapIndex, Tilemap, TilemapMutation};
use crate::game::scene::level::tile::tileset::Tileset;

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

/// An exception to the tilemap that must be overridden
#[derive(Default, Debug, Clone)]
pub struct RoomTileException {
  layer: TileLayerType,
  index: MapIndex,
  key: Option<TileKey>,
}

impl RoomTileException {
  /// Instantiate a new tile exception
  pub fn new(index: MapIndex, layer: TileLayerType, key: Option<TileKey>) -> Self {
    Self { index, key, layer }
  }
}

/// A room in the game, with a tilemap and entities that interact with it
pub struct Room {
  name: String,
  position: Vec2<f32>,
  tilemap: Tilemap<TileMeta, TileLayerType, ObjMeta>,
  exceptions: Vec<RoomTileException>,
  entities: HashSet<Entity>,
}

impl Room {
  /// Instantiate a new room
  pub fn build(name: String, tilemap: Tilemap<TileMeta, TileLayerType, ObjMeta>, position: Vec2<f32>, exceptions: Vec<RoomTileException>) -> Self {
    Self { name, tilemap, position, exceptions, entities: HashSet::new() }
  }

  // Tilemap //

  /// Create and add tiles associated with the tilemap to the world
  fn add_tilemap_to_world(&mut self, world: &mut World) -> Result<(), String> {
    let tilemap_position = self.position;
    let dimensions = self.tilemap.get_size();
    self.tilemap.add_tiles(|layer, tile, _, position| {
      let index = coordinate_to_index(&tile.coordinate, dimensions);
      if let Some(exception) = self.exceptions.iter().find(|exception| exception.layer == layer && exception.index == index) {
        if exception.key.is_none() {
          return Ok(None);
        } else {
          unimplemented!("Tile exceptions with Some tile keys are not yet implemented");
        }
      }

      let position = position + tilemap_position;

      let collision_layer = tile.data.meta.collision_layer;
      let entity = world.add((
        Tile::new(tile.data.tile_key),
        Position::from(position),
        collision_layer
      ));

      // todo: add sprites if `meta.hidden` is true instead...
      if collision_layer == RoomCollision::All {
        world.add_components(entity, (
          Sprite::new(tile.data.texture_key, tile.data.src),
        )).expect("Failed to add active room component");

        if let Some(animation) = tile.data.meta.animation.clone() {
          world.add_components(entity, (
            animation.start(),
          )).expect("Failed to add animation component");
        }
      }

      // add render layer
      match layer {
        TileLayerType::Foreground => world.add_components(entity, (layer::Layer7, ))?,
        TileLayerType::Collision => world.add_components(entity, (layer::Layer6, ))?,
        TileLayerType::Background => world.add_components(entity, (layer::Layer4, ))?,
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

        if let Some(collectable) = tile.data.meta.collectable.clone() {
          world.add_components(entity, (collectable, ))?;
        }

        match tile.data.meta.breakability {
          TileBreakability::Solid => (),
          TileBreakability::Soft => world.add_components(entity, (Soft, ))?,
          TileBreakability::Strong => world.add_components(entity, (Strong, ))?,
          TileBreakability::Brittle => world.add_components(entity, (Fragile, ))?,
        }

        let damage = tile.data.meta.damage;
        if damage > 0 {
          world.add_components(entity, (PlayerHostile, Damage::new(damage)))?;
        }
      }

      Ok(Some(entity))
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

  pub fn add_entities_to_world(&mut self, world: &mut World, assets: &mut AssetManager, state: &Story) -> Result<(), String> {
    self.tilemap.add_objects(|object| {
      let entity = match object {
        ObjMeta::AngryBuzzConcept { position } => world.add(make_angry_buzz(assets, self.position + *position)?),
        ObjMeta::BubblyConcept { position, direction, .. } => world.add(make_bubbly(assets, self.position + *position, *direction)?),
        ObjMeta::BuzzConcept { position } => world.add(make_buzz(assets, self.position + *position)?),
        ObjMeta::GruntConcept { position } => world.add(make_grunt(assets, self.position + *position)?),
        ObjMeta::SaveAreaConcept { position, collision_box } => {
          let story = state.get_entry("save");
          world.add(make_save_area(self.name.clone(), CollisionBox::new(self.position + *position, collision_box.size), story)?)
        }
        ObjMeta::SpikyConcept { direction, position } => world.add(make_spiky(assets, self.position + *position, *direction)?),
        ObjMeta::SporeConcept { direction, position } => world.add(make_spore(assets, self.position + *position, *direction)?),
        ObjMeta::RipperConcept { direction, position } => world.add(make_ripper(assets, self.position + *position, *direction)?),
        ObjMeta::RotundConcept { direction, position, spit_axis } => world.add(make_rotund(assets, self.position + *position, *direction, *spit_axis)?),
        ObjMeta::ZoomerConcept { direction, position } => world.add(make_zoomer(assets, self.position + *position, *direction)?),
        ObjMeta::StoryConcept { position, collision_box, key } => {
          if let Some(entry) = state.get_entry(key) {
            let collision_box = CollisionBox::new(self.position + *position, collision_box.size);
            return Ok(Some(world.add(make_story_area(entry.clone(), collision_box))));
          }
          return Ok(None);
        }
      };
      self.entities.insert(entity);
      Ok(Some(entity))
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
  pub fn add_to_world(&mut self, world: &mut World, assets: &mut AssetManager, state: &Story) -> Result<(), String> {
    self.add_tilemap_to_world(world)?;
    self.add_entities_to_world(world, assets, state)
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
  state.get_mut::<LevelState>()
    .expect("Failed to get RoomRegistry")
    .room_registry
    .get_current_mut()
    .expect("Failed to get current room")
}

/// Use the Room registries tileset
/// ## Panics
/// if the `RoomRegistry` not in state
pub fn use_tileset(state: &mut State) -> &Tileset<TileMeta> {
  let registry = &mut state
    .get_mut::<LevelState>()
    .expect("Failed to get RoomRegistry")
    .room_registry;
  registry
    .get_tileset(String::from("tileset"))
    .expect("Failed to get tileset")
}
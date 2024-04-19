/**
 * Manage rooms and transitions
 */

use std::collections::HashMap;

use hecs::{Entity, Without};

use crate::engine::asset::AssetManager;
use crate::engine::geometry::collision::{CollisionBox, CollisionMask, rec2_collision};
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::rendering::camera::{Camera, CameraBounds};
use crate::engine::system::SysArgs;
use crate::engine::tile::parse::TiledParser;
use crate::engine::utility::alias::Size;
use crate::engine::world::World;
use crate::game::player::world::{PQ, use_player};
use crate::game::scene::level::parse::{tilemap_from_tiled, tileset_from_tiled};
use crate::game::scene::level::room::{ActiveRoom, Room, ROOM_ENTER_MARGIN, RoomCollider};
use crate::game::utility::path::{get_basename, get_filename};

/// Consume a Tiled parser, and build its tilesets and tilemaps
pub struct RoomRegistry {
  current: Option<String>,
  rooms: HashMap<String, Room>,
  colliders: HashMap<String, Entity>,
}

impl RoomRegistry {
  /// Instantiate a new `RoomRegistry` from a `TiledParser`
  /// - Builds engine `Tileset`s and `Tilemap`s from the `TiledParser`.
  /// - Loads referenced assets into the `AssetManager`
  /// - Adds `RoomCollider`s into the world for each `Room`
  pub fn build(parser: TiledParser, assets: &mut AssetManager, world: &mut World) -> Result<Self, String> {
    // Build the engine tilesets from Tiled tilesets
    let mut tilesets = HashMap::new();
    for (path, tiled_tileset) in parser.tilesets {
      let name = get_filename(&path)?;
      let tileset = tileset_from_tiled(assets, path, &tiled_tileset)?;
      tilesets.insert(name, tileset);
    }

    let mut rooms = HashMap::new();
    let mut colliders = HashMap::new();
    for (path, tiled_tilemap) in parser.tilemaps {
      let tileset_path = &tiled_tilemap.tileset
        .first() // we don't support multiple tilesets per tilemap
        .ok_or("No tileset found")?
        .source;
      let tileset_name = get_filename(tileset_path)?;
      let tileset = tilesets
        .get(&tileset_name)
        .ok_or("Tileset not found")?;
      let tilemap = tilemap_from_tiled(&tiled_tilemap, &tileset)?;

      let tilemap_file = get_basename(&path)?;
      let world_map_reference = parser.world.maps
        .iter()
        .find(|m| m.file_name == tilemap_file)
        .ok_or("Tilemap not found")?;
      let position = Vec2::new(world_map_reference.x as f32, world_map_reference.y as f32);

      let tilemap_name = get_filename(&path)?;

      let room_collision_box = CollisionBox::new(position, tilemap.get_dimensions());
      let collider = RoomCollider::new(room_collision_box, tilemap_name.clone());
      let collider_entity = world.add((collider, ));
      colliders.insert(tilemap_name.clone(), collider_entity);

      let room = Room::build(tilemap, position);
      rooms.insert(tilemap_name.clone(), room);
    }

    Ok(Self {
      current: None,
      rooms,
      colliders,
    })
  }
  /// clear the `ActiveRoom` entity from the current room
  fn deactivate_room(&self, name: impl Into<String>, world: &mut World) -> Result<(), String> {
    let name = name.into();
    let entity = self.colliders
      .get(&name)
      .ok_or("Room collider not found")?;

    world.remove_components::<(ActiveRoom, )>(*entity)?;

    Ok(())
  }
  /// Add the `ActiveRoom` component to an entity
  fn activate_room(&self, name: impl Into<String>, world: &mut World) -> Result<(), String> {
    let name = name.into();
    let entity = self.colliders.get(&name).ok_or("Room collider not found")?;
    world.add_components(*entity, (ActiveRoom::default(), ))
  }
  /// Remove a room and it's entities from the world
  fn remove_room_from_world(&mut self, name: &String, world: &mut World) -> Result<(), String> {
    let room = self.rooms.get_mut(name).ok_or("Current room not found")?;
    room.remove_from_world(world);

    Ok(())
  }
  /// Add the entities associated with a room to the world
  fn add_room_to_world(&mut self, name: impl Into<String>, world: &mut World) -> Result<(), String> {
    self.rooms
      .get_mut(&name.into())
      .ok_or("Room not found")?
      .add_to_world(world)
  }
  /// Transition to a new room
  pub fn transition_to_room(&mut self, world: &mut World, name: impl Into<String>) -> Result<(), String> {
    let name = name.into();
    if let Some(current) = self.current.clone() {
      if current == name { return Err(String::from("Room is already active")); }
      self.remove_room_from_world(&current, world)?;
      self.deactivate_room(&current, world)?;
    }
    self.add_room_to_world(name.clone(), world)?;
    self.activate_room(name.clone(), world)?;
    self.current = Some(name.clone());

    Ok(())
  }
  /// Get the current room if one is active
  pub fn get_current(&self) -> Option<&Room> {
    self.current
      .as_ref()
      .and_then(|name| self.rooms.get(name))
  }
  /// Get the current room if one is active
  pub fn get_current_mut(&mut self) -> Option<&mut Room> {
    self.current
      .as_mut()
      .and_then(|name| self.rooms.get_mut(name))
  }
  /// clamp the camera to the bounds of the current room
  pub fn clamp_camera(&self, camera: &mut Camera) {
    if let Some(room) = self.get_current() {
      camera.set_bounds(room.get_bounds());
    }
  }
  /// Get the entry bounds for the current room
  pub fn get_entry_bounds(&self) -> Result<CameraBounds, String> {
    let mut translation_bounds = self
      .get_current()
      .ok_or("No current room")?
      .get_bounds();
    translation_bounds.origin = translation_bounds.origin + ROOM_ENTER_MARGIN;
    translation_bounds.size = translation_bounds.size - (ROOM_ENTER_MARGIN * 2) as Size;
    Ok(translation_bounds)
  }
}

// Systems //

/// Check for room collisions and enact room transitions
pub fn sys_room_transition(SysArgs { world, camera, state, .. }: &mut SysArgs) {
  let PQ { position, collider: player_collider, .. } = use_player(world);
  let player_box = Rec2::new(position.0 + player_collider.0.origin, player_collider.0.size);
  let mut room_collisions = Vec::new();
  for (_, room_collider) in world.query::<Without<&RoomCollider, &ActiveRoom>>() {
    let collision = rec2_collision(&room_collider.collision_box, &player_box, CollisionMask::default());
    if collision.is_some() {
      room_collisions.push(room_collider.clone());
    }
  }

  if room_collisions.len() == 0 { return; }
  if room_collisions.len() > 1 { panic!("Player is colliding with multiple rooms"); }

  let room = room_collisions
    .first()
    .expect("Failed to find room to enter");

  let room_registry = state.get_mut::<RoomRegistry>().expect("Failed to get room registry");
  room_registry.transition_to_room(world, &room.room).expect("Failed to set current room");
  room_registry.clamp_camera(camera);

  let entry_bounds = room_registry.get_entry_bounds().expect("Failed to get entry bounds");
  let PQ { position, collider, .. } = use_player(world);
  let mut player_box = Rec2::new(position.0 + collider.0.origin, collider.0.size);
  player_box.clamp_position(&Rec2::new(Vec2::<f32>::from(entry_bounds.origin), entry_bounds.size));
  position.0 = player_box.origin;
}

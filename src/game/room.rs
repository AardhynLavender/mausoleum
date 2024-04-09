/**
 * Room creation and management
 */

use std::collections::HashMap;

use hecs::{Entity, Without};

use crate::engine::asset::AssetManager;
use crate::engine::geometry::collision::{CollisionBox, CollisionMask, rec2_collision};
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::rendering::camera::{Camera, CameraBounds};
use crate::engine::rendering::color::{OPAQUE, RGBA};
use crate::engine::system::SysArgs;
use crate::engine::tile::consume::{tilemap_from_tiled, tileset_from_tiled};
use crate::engine::tile::parse::TiledParser;
use crate::engine::tile::tilemap::Tilemap;
use crate::engine::tile::tileset::Tileset;
use crate::engine::utility::alias::Size;
use crate::engine::world::World;
use crate::game::constant::TILE_SIZE;
use crate::game::player::world::use_player;
use crate::game::utility::controls::{Behaviour, Control, is_control};
use crate::game::utility::path::{get_basename, get_filename};

const ROOM_ENTER_MARGIN: i32 = TILE_SIZE.x as i32 / 2;

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
  tilemap: Tilemap,
  #[allow(unused)]
  save: bool,
}

impl Room {
  pub fn build(tilemap: Tilemap, position: Vec2<f32>) -> Self {
    Self { tilemap, position, save: false }
  }
  /// Add the tilemap to the world
  fn add_to_world(&mut self, world: &mut World) -> Result<(), String> {
    self.tilemap.add_to_world(world, self.position)
  }
  /// Remove the tilemap from the world
  fn remove_from_world(&mut self, world: &mut World) -> Result<(), String> {
    self.tilemap.remove_from_world(world)
  }
  /// Get the bounds of the tilemap in worldspace
  pub fn get_bounds(&self) -> CameraBounds {
    let position = Vec2::from(self.position);
    let dimensions = self.tilemap.get_dimensions();
    CameraBounds::new(position, dimensions)
  }
}

/// Consume a Tiled parser, and build its tilesets and tilemaps
#[allow(unused)]
pub struct RoomRegistry {
  current: Option<String>,
  tilesets: HashMap<String, Tileset>,
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
      tilesets,
      rooms,
      colliders,
    })
  }
  /// clear the `ActiveRoom` entity to the current room
  fn clear_active_room(&self, name: impl Into<String>, world: &mut World) -> Result<(), String> {
    let name = name.into();
    let entity = self.colliders
      .get(&name)
      .ok_or("Room collider not found")?;
    world.remove_components::<(ActiveRoom, )>(*entity)?;
    Ok(())
  }
  /// Add the `ActiveRoom` component to an entity
  fn set_active_room(&self, name: impl Into<String>, world: &mut World) -> Result<(), String> {
    let name = name.into();
    let entity = self.colliders
      .get(&name)
      .ok_or("Room collider not found")?;
    world.add_components(*entity, (ActiveRoom::default(), ))
  }
  /// Remove a room from the world
  fn remove_room_from_world(&mut self, name: &String, world: &mut World) -> Result<(), String> {
    Ok(self.rooms
      .get_mut(name)
      .ok_or("Current room not found")?
      .remove_from_world(world)?)
  }
  /// Add a room to the world
  fn add_room_to_world(&mut self, name: impl Into<String>, world: &mut World) -> Result<(), String> {
    self.rooms
      .get_mut(&name.into())
      .ok_or("Room not found")?
      .add_to_world(world)
  }
  /// Change the current room
  pub fn set_current(&mut self, world: &mut World, name: impl Into<String>) -> Result<(), String> {
    let name = name.into();
    if let Some(current) = self.current.clone() {
      if current == name {
        return Err(String::from("Room is already active"));
      }
      self.remove_room_from_world(&current, world)?;
      self.clear_active_room(&current, world)?;
    }
    self.add_room_to_world(name.clone(), world)?;
    self.set_active_room(name.clone(), world)?;
    self.current = Some(name.clone());

    Ok(())
  }
  /// Get the current room if one is active
  pub fn get_current(&self) -> Option<&Room> {
    self.current
      .as_ref()
      .and_then(|name| self.rooms.get(name))
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

/// Check for room transitions
pub fn sys_room_transition(SysArgs { world, camera, state, .. }: &mut SysArgs) {
  let (_, position, .., player_collider, _) = use_player(world);
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
  room_registry.set_current(world, &room.room).expect("Failed to set current room");
  room_registry.clamp_camera(camera);

  let entry_bounds = room_registry.get_entry_bounds().expect("Failed to get entry bounds");
  let (_, position, .., collider, _) = use_player(world);
  let mut player_box = Rec2::new(position.0 + collider.0.origin, collider.0.size);
  player_box.clamp_position(&Rec2::new(Vec2::<f32>::from(entry_bounds.origin), entry_bounds.size));
  position.0 = player_box.origin;
}

pub fn sys_render_room_colliders(SysArgs { world, render, camera, event, .. }: &mut SysArgs) {
  if !is_control(Control::Debug, Behaviour::Held, event) { return; }
  for (_, room_collider) in world.query::<&RoomCollider>() {
    let pos = Vec2::<i32>::from(camera.translate(room_collider.collision_box.origin));
    render.draw_rect(Rec2::new(pos, room_collider.collision_box.size), RGBA::new(0, 0, 255, OPAQUE));
  }
}
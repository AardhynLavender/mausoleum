#![allow(unused)]

use std::collections::HashMap;

use crate::engine::asset::AssetManager;
use crate::engine::geometry::collision::CollisionBox;
use crate::engine::geometry::shape::Vec2;
use crate::engine::rendering::camera::CameraBounds;
use crate::engine::tile::consume::{tilemap_from_tiled, tileset_from_tiled};
use crate::engine::tile::parse::TiledParser;
use crate::engine::tile::tilemap::Tilemap;
use crate::engine::tile::tileset::Tileset;
use crate::engine::world::World;
use crate::game::utility::path::{get_basename, get_filename};

/**
 * Room creation and management
 */

// Components //

pub struct RoomCollider(CollisionBox);

pub struct ActiveRoom;

// Structures //

pub struct Room {
  position: Vec2<f32>,
  tilemap: Tilemap,
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
}

impl RoomRegistry {
  pub fn build(parser: TiledParser, assets: &mut AssetManager) -> Result<Self, String> {
    // Build the engine tilesets from Tiled tilesets
    let mut tilesets = HashMap::new();
    for (path, tiled_tileset) in parser.tilesets {
      let name = get_filename(&path)?;
      let tileset = tileset_from_tiled(assets, path, &tiled_tileset)?;
      tilesets.insert(name, tileset);
    }

    // Build the rooms from Tiled tilemaps
    let mut rooms = HashMap::new();
    for (path, tiled_tilemap) in parser.tilemaps {
      let tileset_path = &tiled_tilemap.tileset
        .first()
        .ok_or("No tileset found")?
        .source;
      let tileset_name = get_filename(tileset_path)?;
      let tileset = tilesets
        .get(&tileset_name)
        .ok_or("Tileset not found")?;

      let tilemap = tilemap_from_tiled(&tiled_tilemap, &tileset)?;

      // lookup the tilemap in the world to get its position
      let tilemap_file = get_basename(&path)?;
      let world_map_reference = parser.world.maps
        .iter()
        .find(|m| m.file_name == tilemap_file)
        .ok_or("Tilemap not found")?;
      let position = Vec2::new(world_map_reference.x as f32, world_map_reference.y as f32);

      let room = Room::build(tilemap, position);
      let tilemap_name = get_filename(&path)?;
      rooms.insert(tilemap_name, room);
    }

    // build the registry
    Ok(Self {
      current: None,
      tilesets,
      rooms,
    })
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
      self.remove_room_from_world(&current, world)?;
    }
    self.add_room_to_world(name.clone(), world)?;
    self.current = Some(name.clone());

    Ok(())
  }
  /// Get the current room if one is active
  pub fn get_current(&self) -> Option<&Room> {
    self.current
      .as_ref()
      .and_then(|name| self.rooms.get(name))
  }
}

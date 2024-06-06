/**
 * Manage rooms and transitions
 */

use std::collections::HashMap;

use hecs::{Entity, Without};

use crate::engine::asset::AssetManager;
use crate::engine::geometry::collision::{CollisionBox, CollisionMask, rec2_collision};
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::rendering::camera::CameraBounds;
use crate::engine::system::{SysArgs, Systemize};
use crate::engine::tile::parse::{TiledParser, TiledTilemapChildren};
use crate::engine::tile::tileset::Tileset;
use crate::engine::utility::alias::Size;
use crate::engine::utility::interpolation::lerp;
use crate::engine::world::World;
use crate::game::constant::ease_in_out;
use crate::game::player::world::{PlayerQuery, use_player};
use crate::game::scene::level::meta::TileMeta;
use crate::game::scene::level::parse::{tilemap_from_tiled, tileset_from_tiled};
use crate::game::scene::level::room::{ActiveRoom, Room, ROOM_ENTER_MARGIN, RoomCollider, RoomTileException};
use crate::game::scene::level::scene::LevelState;
use crate::game::scene::level::transition::{RoomTransition, RoomTransitionData, RoomTransitionState};
use crate::game::story::data::Story;
use crate::game::utility::path::{get_basename, get_filename};

pub const ROOM_TRANSITION_TIME_MS: u64 = 800;

pub type RoomKey = String;

type RoomRegistryTileExceptions = HashMap<RoomKey, Vec<RoomTileException>>;

/// Consume a Tiled parser, and build its tilesets and tilemaps
pub struct RoomRegistry {
  current: Option<RoomKey>,
  transition: RoomTransition,

  tilesets: HashMap<RoomKey, Tileset<TileMeta>>,
  story_data: Story,
  rooms: HashMap<RoomKey, Room>,
  colliders: HashMap<RoomKey, Entity>,
}

impl RoomRegistry {
  /// Instantiate a new `RoomRegistry` from a `TiledParser`
  /// - Builds engine `Tileset`s and `Tilemap`s from the `TiledParser`.
  /// - Loads referenced assets into the `AssetManager`
  /// - Adds `RoomCollider`s into the world for each `Room`
  pub fn build(parser: TiledParser, mut exceptions: RoomRegistryTileExceptions, story_data: Story, assets: &mut AssetManager, world: &mut World) -> Result<Self, String> {
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
      let tileset_path = &tiled_tilemap
        .children
        .iter()
        .filter_map(|child| match child {
          TiledTilemapChildren::TilesetReference(child) => Some(child),
          _ => None,
        })
        .next() // we don't support multiple tilesets per tilemap
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

      let room = Room::build(tilemap_name.clone(), tilemap, position, exceptions.remove(&tilemap_name).unwrap_or(Vec::new()));
      rooms.insert(tilemap_name, room);
    }

    if tilesets.len() > 1 {
      return Err(String::from("Multiple tilesets found; can't store a single tileset in the register"));
    }

    Ok(Self {
      current: None,
      transition: RoomTransition::default(),

      story_data,
      tilesets,
      rooms,
      colliders,
    })
  }
  /// Load a starting room and bypass room transitions
  pub fn load_room(&mut self, room: impl Into<RoomKey>, world: &mut World, assets: &mut AssetManager) -> Result<(), String> {
    let next = room.into();
    self.add_room_to_world(&next, world, assets).expect("Failed to load starting room");
    self.activate_room(&next, world).expect("Failed to activate starting room");
    self.current = Some(next);

    Ok(())
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
    let room = self
      .rooms
      .get_mut(name)
      .ok_or("Room not found")?;

    room.remove_from_world(world);

    Ok(())
  }
  /// Add the entities associated with a room to the world
  fn add_room_to_world(&mut self, name: impl Into<String>, world: &mut World, assets: &mut AssetManager) -> Result<(), String> {
    self
      .rooms
      .get_mut(&name.into())
      .ok_or("Room not found")?
      .add_to_world(world, assets, &self.story_data)
  }

  pub fn queue_transition(&mut self, name: impl Into<String>) -> Result<(), String> {
    self.transition.queue(name)
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

  /// Get a tileset by name
  pub fn get_tileset(&self, name: String) -> Option<&Tileset<TileMeta>> { self.tilesets.get(&name) }
}

// Systems //

/// Check for room collisions and enact room transitions
impl Systemize for RoomRegistry {
  fn system(SysArgs { world, asset, camera, event, state, .. }: &mut SysArgs) -> Result<(), String> {
    let room_registry = &mut state.get_mut::<LevelState>()?.room_registry;

    let PlayerQuery { position, collider: player_collider, .. } = use_player(world);
    let player_box = Rec2::new(position.0 + player_collider.0.origin, player_collider.0.size);

    match room_registry.transition.integrate() {
      // no transition in progress, look for room collision and queue a transition
      RoomTransitionState::Idle => {
        let mut room_collisions = Vec::new();
        for (_, room_collider) in world.query::<Without<&RoomCollider, &ActiveRoom>>() {
          let collision = rec2_collision(&room_collider.collision_box, &player_box, CollisionMask::default());
          if collision.is_some() {
            room_collisions.push(room_collider.clone());
          }
        }

        // invariant
        if room_collisions.len() == 0 { return Ok(()); }
        if room_collisions.len() > 1 { return Err(String::from("Multiple room collisions")); }

        // transition to the new room
        let room_collider = room_collisions.first().ok_or(String::from("No room collision"))?;
        room_registry.queue_transition(&room_collider.room)?;
      }
      // Transition is queued, start it
      RoomTransitionState::Queued(next) => {
        room_registry.deactivate_room(&room_registry.current.clone().unwrap(), world)?;
        room_registry.add_room_to_world(&next, world, asset)?;

        let new_bounds = room_registry.rooms.get(&next).expect("Failed to get new bounds").get_bounds();
        let entry_bounds = CameraBounds::new(
          new_bounds.origin + ROOM_ENTER_MARGIN,
          new_bounds.size - (ROOM_ENTER_MARGIN * 2) as Size,
        );
        let mut new_player_box = player_box.clone();
        new_player_box.clamp(&Rec2::new(Vec2::<f32>::from(entry_bounds.origin), entry_bounds.size));

        // create a new viewport centered on the player
        let mut new_viewport = camera.get_viewport().clone();
        new_viewport.origin = Vec2::<i32>::from(new_player_box.origin) - Vec2::<i32>::from(new_viewport.size) / 2 + Vec2::<i32>::from(new_player_box.size / 2);
        new_viewport.clamp(&new_bounds);

        room_registry.transition.start(RoomTransitionData {
          old_viewport: *camera.get_viewport(),
          new_viewport,
          old_player: player_box,
          new_player: new_player_box,
        })?;

        camera.release(camera.get_position());
        camera.remove_bounds();

        event.queue_pause();
      }
      // transition is in progress, update the progress
      RoomTransitionState::Progress(t, data) => {

        // reintegrate t on a bezier curve
        let t2 = t;
        let t = ease_in_out().lerp(t).y;
        println!("t: {t2}, t2: {t}");

        let camera_position = lerp(Vec2::<f32>::from(data.old_viewport.origin), Vec2::<f32>::from(data.new_viewport.origin), t);
        camera.set_position(Vec2::<i32>::from(camera_position));

        let PlayerQuery { position, .. } = use_player(world);
        let new_player_position = lerp(Vec2::<f32>::from(data.old_player.origin), Vec2::<f32>::from(data.new_player.origin), t);
        position.0 = new_player_position;
      }
      // the transition is complete. out with the old...
      RoomTransitionState::Complete(next) => {
        room_registry.remove_room_from_world(&room_registry.current.clone().unwrap(), world)?;

        room_registry.activate_room(&next, world)?;
        room_registry.current = Some(next);

        camera.set_bounds(room_registry.get_current().expect("Failed to get entry bounds").get_bounds());
        camera.tether();

        event.queue_resume();
      }
    }

    Ok(())
  }
}


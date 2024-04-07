use std::path::Path;

use crate::engine::lifecycle::LifecycleArgs;
use crate::engine::scene::Scene;
use crate::engine::system::{Schedule, SysArgs};
use crate::engine::tile::parse::TiledParser;
use crate::engine::tile::tile::sys_render_tile_colliders;
use crate::engine::world::push_state_with;
use crate::game::physics::collision::sys_render_colliders;
use crate::game::physics::gravity::sys_gravity;
use crate::game::physics::velocity::sys_velocity;
use crate::game::player::world::add_player;
use crate::game::room::RoomRegistry;
use crate::game::scene::level::collision::sys_tile_collision;
use crate::game::scene::menu::MenuScene;
use crate::game::utility::controls::{Behaviour, Control, is_control};

/**
 * The level scene
 */

const WORLD_PATH: &str = "asset/world.world";

#[allow(unused)]
struct LevelState {
  level_key: String,
  room_registry: RoomRegistry,
}

pub struct LevelScene {
  level_key: String,
}

impl LevelScene {
  pub fn build(level_key: impl Into<String>) -> Result<Self, String> {
    Ok(Self { level_key: level_key.into() })
  }
}

impl Scene for LevelScene {
  /// Set up the level scene
  fn setup(&self, LifecycleArgs { world, camera, system, asset, .. }: &mut LifecycleArgs) {
    let path = Path::new(WORLD_PATH);
    let parser = TiledParser::parse(path)
      .map_err(|e| println!("Failed to parse Tiled data: {}", e))
      .expect("Failed to parse Tiled data");

    let mut room_registry = RoomRegistry::build(parser, asset).expect("Failed to build room registry");
    room_registry.set_current(world, &String::from("room_0")).expect("Failed to add room to world");

    let bounds = room_registry
      .get_current()
      .expect("Failed to get current room")
      .get_bounds();
    camera.set_bounds(bounds);

    add_player(world, system, asset);
    camera.tether();

    push_state_with(world, LevelState { level_key: self.level_key.clone(), room_registry });
  }
  /// Add systems to the level scene
  fn add_systems(&self, LifecycleArgs { system, .. }: &mut LifecycleArgs) {
    system.add(Schedule::FrameUpdate, sys_gravity);
    system.add(Schedule::FrameUpdate, sys_velocity);
    system.add(Schedule::FrameUpdate, sys_tile_collision);

    system.add(Schedule::PostUpdate, sys_render_tile_colliders);
    system.add(Schedule::PostUpdate, sys_render_colliders);

    system.add(Schedule::PostUpdate, sys_level_listener);
  }
  /// Clean up the level scene
  fn destroy(&self, _: &mut LifecycleArgs) {}
}

/// Events to listen for during the level scene
pub fn sys_level_listener(SysArgs { event, scene, .. }: &mut SysArgs) {
  if is_control(Control::Escape, Behaviour::Pressed, event) {
    scene.queue_next(MenuScene)
  }
}


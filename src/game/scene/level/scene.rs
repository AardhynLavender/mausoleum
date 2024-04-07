use std::path::Path;

use crate::engine::lifecycle::LifecycleArgs;
use crate::engine::scene::Scene;
use crate::engine::system::{Schedule, SysArgs};
use crate::engine::tile::parse::TiledParser;
use crate::engine::tile::tile::sys_render_tile_colliders;
use crate::game::physics::collision::sys_render_colliders;
use crate::game::physics::gravity::sys_gravity;
use crate::game::physics::velocity::sys_velocity;
use crate::game::player::world::add_player;
use crate::game::room::{RoomRegistry, sys_render_room_colliders, sys_room_transition};
use crate::game::scene::level::collision::sys_tile_collision;
use crate::game::scene::menu::MenuScene;
use crate::game::utility::controls::{Behaviour, Control, is_control};

/**
 * The level scene
 */

const WORLD_PATH: &str = "asset/world.world";

#[allow(unused)]
pub struct LevelState {
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
  fn setup(&self, LifecycleArgs { world, camera, system, state, asset, .. }: &mut LifecycleArgs) {
    let path = Path::new(WORLD_PATH);
    let parser = TiledParser::parse(path)
      .map_err(|e| println!("Failed to parse Tiled data: {}", e))
      .expect("Failed to parse Tiled data");

    let mut room_registry = RoomRegistry::build(parser, asset, world).expect("Failed to build room registry");
    room_registry.set_current(world, &self.level_key).expect("Failed to add room to world");
    room_registry.clamp_camera(camera);

    add_player(world, system, asset);
    camera.tether();

    state.add(room_registry).expect("Failed to add level state")
  }
  /// Add systems to the level scene
  fn add_systems(&self, LifecycleArgs { system, .. }: &mut LifecycleArgs) {
    system.add(Schedule::FrameUpdate, sys_gravity);
    system.add(Schedule::FrameUpdate, sys_velocity);
    system.add(Schedule::FrameUpdate, sys_tile_collision);
    system.add(Schedule::FrameUpdate, sys_room_transition);

    system.add(Schedule::PostUpdate, sys_render_tile_colliders);
    system.add(Schedule::PostUpdate, sys_render_colliders);
    system.add(Schedule::PostUpdate, sys_render_room_colliders);

    system.add(Schedule::PostUpdate, sys_exit_level);
  }
  /// Clean up the level scene
  fn destroy(&self, LifecycleArgs { state, .. }: &mut LifecycleArgs) {
    state.remove::<RoomRegistry>().expect("Failed to remove level state");
  }
}

/// Exit the level scene
pub fn sys_exit_level(SysArgs { event, scene, .. }: &mut SysArgs) {
  if is_control(Control::Escape, Behaviour::Pressed, event) {
    scene.queue_next(MenuScene)
  }
}


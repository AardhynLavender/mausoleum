use std::path::Path;

use crate::engine::geometry::shape::Vec2;
use crate::engine::lifecycle::LifecycleArgs;
use crate::engine::scene::Scene;
use crate::engine::system::{Schedule, SysArgs};
use crate::engine::tile::parse::TiledParser;
use crate::engine::tile::tile::sys_render_tile_colliders;
use crate::engine::utility::direction::Direction;
use crate::game::combat::damage::sys_damage;
use crate::game::combat::health::LiveState;
use crate::game::creature::ripper::{make_ripper, sys_ripper};
use crate::game::creature::spiky::{make_spiky, sys_spiky};
use crate::game::interface::hud::{make_player_health_text, sys_render_player_health};
use crate::game::physics::collision::sys_render_colliders;
use crate::game::physics::gravity::sys_gravity;
use crate::game::physics::velocity::sys_velocity;
use crate::game::player::component::sys_render_cooldown;
use crate::game::player::world::{add_player, use_player};
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

    make_player_health_text(world, asset);

    world.add(make_ripper(asset, Vec2::new(96.0, 48.0), Direction::Left).expect("Failed to create ripper"));
    world.add(make_spiky(asset, Vec2::new(96.0, 128.0), Direction::Left).expect("Failed to create spiky"));

    state.add(room_registry).expect("Failed to add level state")
  }
  /// Add systems to the level scene
  fn add_systems(&self, LifecycleArgs { system, .. }: &mut LifecycleArgs) {
    // physics //

    system.add(Schedule::FrameUpdate, sys_gravity);
    system.add(Schedule::FrameUpdate, sys_velocity);
    system.add(Schedule::FrameUpdate, sys_tile_collision);
    system.add(Schedule::FrameUpdate, sys_room_transition);

    // rendering //

    system.add(Schedule::PostUpdate, sys_render_tile_colliders);
    system.add(Schedule::PostUpdate, sys_render_colliders);
    system.add(Schedule::PostUpdate, sys_render_room_colliders);
    system.add(Schedule::PostUpdate, sys_render_player_health);
    system.add(Schedule::PostUpdate, sys_render_cooldown);

    // combat //

    system.add(Schedule::FrameUpdate, sys_ripper);
    system.add(Schedule::FrameUpdate, sys_spiky);

    system.add(Schedule::FrameUpdate, sys_damage);

    // misc //

    system.add(Schedule::PostUpdate, sys_exit_level);
  }
  /// Clean up the level scene
  fn destroy(&self, LifecycleArgs { state, .. }: &mut LifecycleArgs) {
    state.remove::<RoomRegistry>().expect("Failed to remove level state");
  }
}

/// Exit the level scene
pub fn sys_exit_level(SysArgs { event, scene, world, .. }: &mut SysArgs) {
  let (.., health) = use_player(world);
  let dead = health.get_state() == LiveState::Dead;
  let exit = is_control(Control::Escape, Behaviour::Pressed, event) || dead;
  if dead || exit { scene.queue_next(MenuScene) }
}


/**
 * The level scene
 */

use std::path::Path;

use crate::engine::geometry::shape::Vec2;
use crate::engine::lifecycle::LifecycleArgs;
use crate::engine::scene::Scene;
use crate::engine::system::{Schedule, SysArgs};
use crate::engine::tile::parse::TiledParser;
use crate::game::collectable::collectable::sys_collectable;
use crate::game::combat::damage::sys_damage;
use crate::game::combat::health::LiveState;
use crate::game::combat::ttl::sys_ttl;
use crate::game::creature::buzz::sys_buzz;
use crate::game::creature::ripper::sys_ripper;
use crate::game::creature::spiky::sys_spiky;
use crate::game::creature::zoomer::sys_zoomer;
use crate::game::interface::hud::{make_player_health_text, sys_render_player_health};
use crate::game::persistence::data::SaveData;
use crate::game::persistence::world::sys_save;
use crate::game::physics::collision::sys_render_colliders;
use crate::game::physics::frozen::sys_thaw;
use crate::game::physics::gravity::sys_gravity;
use crate::game::physics::velocity::sys_velocity;
use crate::game::player::combat::sys_render_cooldown;
use crate::game::player::world::{make_player, PlayerQuery, use_player};
use crate::game::preferences::use_preferences;
use crate::game::scene::level::collision::{sys_render_tile_colliders, sys_tile_collision};
use crate::game::scene::level::registry::{RoomRegistry, sys_room_transition};
use crate::game::scene::level::room::sys_render_room_colliders;
use crate::game::scene::menu::MenuScene;
use crate::game::utility::controls::{Behaviour, Control, is_control};

const WORLD_PATH: &str = "asset/world.world";

pub const PHYSICS_SCHEDULE: Schedule = Schedule::FrameUpdate;
// pub const PHYSICS_SCHEDULE: Schedule = Schedule::FixedUpdate;

pub struct LevelScene {
  save_data: SaveData,
}

impl LevelScene {
  /// Build the level scene from the save data
  pub fn new(save_data: SaveData) -> Self { Self { save_data } }
}

impl Scene for LevelScene {
  /// Set up the level scene
  fn setup(&self, LifecycleArgs { world, camera, system, state, asset, .. }: &mut LifecycleArgs) {
    let path = Path::new(WORLD_PATH);
    let parser = TiledParser::parse(path)
      .map_err(|e| println!("Failed to parse Tiled data: {}", e))
      .expect("Failed to parse Tiled data");

    let save_room = self.save_data.save_room();
    let inventory = self.save_data.inventory();

    let mut room_registry = RoomRegistry::build(parser, asset, world).expect("Failed to build room registry");
    room_registry.transition_to_room(world, asset, save_room).expect("Failed to add room to world");
    room_registry.clamp_camera(camera);
    camera.tether();

    let room_position = room_registry
      .get_current()
      .expect("no current room")
      .get_bounds()
      .origin;
    let player_position = self.save_data.position() + Vec2::<f32>::from(room_position);
    make_player(world, system, asset, inventory.into_iter(), player_position);
    make_player_health_text(world, asset);

    state.add(room_registry).expect("Failed to add level state")
  }
  /// Add systems to the level scene
  fn add_systems(&self, LifecycleArgs { system, .. }: &mut LifecycleArgs) {
    // Creatures //

    system.add(PHYSICS_SCHEDULE, sys_ripper);
    system.add(PHYSICS_SCHEDULE, sys_spiky);
    system.add(PHYSICS_SCHEDULE, sys_zoomer);
    system.add(PHYSICS_SCHEDULE, sys_buzz);

    // physics //

    system.add(PHYSICS_SCHEDULE, sys_velocity);
    system.add(PHYSICS_SCHEDULE, sys_gravity);
    system.add(PHYSICS_SCHEDULE, sys_damage);
    system.add(PHYSICS_SCHEDULE, sys_thaw);
    system.add(PHYSICS_SCHEDULE, sys_collectable);
    system.add(PHYSICS_SCHEDULE, sys_tile_collision);
    system.add(PHYSICS_SCHEDULE, sys_save);
    system.add(PHYSICS_SCHEDULE, sys_room_transition);

    // rendering //

    system.add(Schedule::PostUpdate, sys_render_tile_colliders);
    system.add(Schedule::PostUpdate, sys_render_colliders);
    system.add(Schedule::PostUpdate, sys_render_room_colliders);
    system.add(Schedule::PostUpdate, sys_render_player_health);
    system.add(Schedule::PostUpdate, sys_render_cooldown);

    // misc //

    system.add(PHYSICS_SCHEDULE, sys_ttl);
    system.add(Schedule::PostUpdate, sys_level_events);
  }
  /// Clean up the level scene
  fn destroy(&self, LifecycleArgs { state, .. }: &mut LifecycleArgs) {
    state.remove::<RoomRegistry>().expect("Failed to remove level state");
  }
}

/// Listen for level events
pub fn sys_level_events(SysArgs { event, scene, world, state, .. }: &mut SysArgs) {
  let PlayerQuery { health, .. } = use_player(world);
  let dead = health.get_state() == LiveState::Dead;
  let exit = is_control(Control::Escape, Behaviour::Pressed, event) || dead;
  if dead || exit { scene.queue_next(MenuScene) }

  let preferences = use_preferences(state);
  if is_control(Control::Debug, Behaviour::Pressed, event) {
    preferences.debug = !preferences.debug;
  }
}


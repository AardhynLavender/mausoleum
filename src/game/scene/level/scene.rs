/**
 * The level scene
 */

use std::path::Path;

use crate::engine::lifecycle::LifecycleArgs;
use crate::engine::scene::Scene;
use crate::engine::system::{Schedule, SysArgs, Systemize};
use crate::engine::tile::parse::TiledParser;
use crate::game::collectable::collectable::Collection;
use crate::game::combat::damage::Damage;
use crate::game::combat::health::LiveState;
use crate::game::combat::ttl::TimeToLive;
use crate::game::creature::angry_buzz::AngryBuzz;
use crate::game::creature::bubbly::Bubbly;
use crate::game::creature::buzz::Buzz;
use crate::game::creature::grunt::Grunt;
use crate::game::creature::ripper::Ripper;
use crate::game::creature::rotund::Rotund;
use crate::game::creature::spiky::Spiky;
use crate::game::creature::spore::Spore;
use crate::game::creature::zoomer::Zoomer;
use crate::game::interface::hud::{make_player_health_text, PlayerHealth};
use crate::game::persistence::data::SaveData;
use crate::game::persistence::world::{SaveArea, use_save_area};
use crate::game::physics::collision::sys_render_colliders;
use crate::game::physics::frozen::Frozen;
use crate::game::physics::gravity::Gravity;
use crate::game::physics::velocity::Velocity;
use crate::game::player::combat::PlayerCombat;
use crate::game::player::world::{make_player, PlayerQuery, use_player};
use crate::game::preferences::use_preferences;
use crate::game::scene::level::collision::{RoomCollision, sys_render_tile_colliders};
use crate::game::scene::level::registry::RoomRegistry;
use crate::game::scene::level::room::sys_render_room_colliders;
use crate::game::scene::menu::MenuScene;
use crate::game::utility::controls::{Behaviour, Control, is_control};

const WORLD_PATH: &str = "asset/area_1/area_1.world";

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
      .map_err(|e| eprintln!("Failed to parse Tiled data: {}", e))
      .expect("Failed to parse Tiled data");

    let save_room = self.save_data.get_save_room();
    let inventory = self.save_data.get_inventory();

    let mut room_registry = RoomRegistry::build(parser, asset, world).expect("Failed to build room registry");
    room_registry.transition_to_room(world, asset, save_room).expect("Failed to add room to world");
    room_registry.clamp_camera(camera);
    camera.tether();

    let save_position = use_save_area(world).collider.origin;
    let player_position = save_position + self.save_data.get_offset();
    make_player(world, system, asset, inventory.into_iter(), player_position);

    make_player_health_text(world, asset);

    state.add(room_registry).expect("Failed to add level state")
  }
  /// Add systems to the level scene
  fn add_systems(&self, LifecycleArgs { system, .. }: &mut LifecycleArgs) {
    // Creatures //

    system.add(PHYSICS_SCHEDULE, AngryBuzz::system);
    system.add(PHYSICS_SCHEDULE, Bubbly::system);
    system.add(PHYSICS_SCHEDULE, Buzz::system);
    system.add(PHYSICS_SCHEDULE, Grunt::system);
    system.add(PHYSICS_SCHEDULE, Spiky::system);
    system.add(PHYSICS_SCHEDULE, Spore::system);
    system.add(PHYSICS_SCHEDULE, Ripper::system);
    system.add(PHYSICS_SCHEDULE, Rotund::system);
    system.add(PHYSICS_SCHEDULE, Zoomer::system);

    // physics //

    system.add(PHYSICS_SCHEDULE, Gravity::system);
    system.add(PHYSICS_SCHEDULE, Velocity::system);
    system.add(PHYSICS_SCHEDULE, Damage::system);
    system.add(PHYSICS_SCHEDULE, Frozen::system);
    system.add(PHYSICS_SCHEDULE, Collection::system);
    system.add(PHYSICS_SCHEDULE, SaveArea::system);
    system.add(PHYSICS_SCHEDULE, RoomCollision::system);
    system.add(PHYSICS_SCHEDULE, RoomRegistry::system);

    // rendering //

    system.add(Schedule::PostUpdate, PlayerHealth::system);
    system.add(Schedule::PostUpdate, PlayerCombat::system);
    system.add(Schedule::PostUpdate, sys_render_tile_colliders);
    system.add(Schedule::PostUpdate, sys_render_colliders);
    system.add(Schedule::PostUpdate, sys_render_room_colliders);

    // misc //

    system.add(PHYSICS_SCHEDULE, TimeToLive::system);
    system.add(Schedule::PostUpdate, LevelScene::system);
  }
  /// Clean up the level scene
  fn destroy(&self, LifecycleArgs { state, .. }: &mut LifecycleArgs) {
    state.remove::<RoomRegistry>().expect("Failed to remove level state");
  }
}

/// Listen for level events
impl Systemize for LevelScene {
  fn system(SysArgs { event, scene, world, state, .. }: &mut SysArgs) -> Result<(), String> {
    let PlayerQuery { health, .. } = use_player(world);
    let dead = health.get_state() == LiveState::Dead;
    let exit = is_control(Control::Escape, Behaviour::Pressed, event) || dead;
    if dead || exit { scene.queue_next(MenuScene) }

    let preferences = use_preferences(state);
    if is_control(Control::Debug, Behaviour::Pressed, event) {
      preferences.debug = !preferences.debug;
    }

    Ok(())
  }
}

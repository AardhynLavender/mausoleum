/**
 * The level scene
 */

use std::path::Path;

use crate::engine::geometry::shape::Vec2;
use crate::engine::lifecycle::LifecycleArgs;
use crate::engine::scene::Scene;
use crate::engine::system::{Schedule, SysArgs, Systemize, SystemTag};
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
use crate::game::interface::menu::{make_menu, MenuPane};
use crate::game::persistence::data::SaveData;
use crate::game::persistence::world::{SaveArea, use_save_area};
use crate::game::physics::collision::sys_render_colliders;
use crate::game::physics::frozen::Frozen;
use crate::game::physics::gravity::Gravity;
use crate::game::physics::velocity::Velocity;
use crate::game::player::combat::PlayerCombat;
use crate::game::player::controller::PlayerController;
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
  pub fn new(save_data: SaveData) -> Self {
    Self { save_data }
  }
}

impl Scene for LevelScene {
  /// Set up the level scene
  fn setup(&mut self, LifecycleArgs { world, camera, system, state, asset, .. }: &mut LifecycleArgs) {
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
    make_player(world, asset, inventory.into_iter(), player_position);

    make_player_health_text(world, asset);

    state.add(room_registry).expect("Failed to add level state");

    // Add systems to the level scene
    system.add_many(Schedule::FrameUpdate, SystemTag::Suspendable, vec![
      // Creatures //
      AngryBuzz::system,
      Bubbly::system,
      Buzz::system,
      Grunt::system,
      Spiky::system,
      Spore::system,
      Ripper::system,
      Rotund::system,
      Zoomer::system,
      Gravity::system,
      Velocity::system,
      Damage::system,
      Frozen::system,
      Collection::system,
      SaveArea::system,
      RoomCollision::system,
      RoomRegistry::system,
      TimeToLive::system,
    ].into_iter()).expect("Failed to add level systems");

    // Add player systems to the level scene
    system.add_many(Schedule::PostUpdate, SystemTag::Suspendable, vec![
      PlayerController::system,
      PlayerHealth::system,
      PlayerCombat::system,
    ].into_iter()).expect("Failed to add player systems");

    system.add_many(Schedule::PostUpdate, SystemTag::Scene, vec![
      LevelScene::system,
      MenuPane::system,
      sys_render_colliders,
      sys_render_room_colliders,
      sys_render_tile_colliders,
    ].into_iter()).expect("Failed to add level systems");
  }
  /// Clean up the level scene
  fn destroy(&mut self, LifecycleArgs { state, camera, .. }: &mut LifecycleArgs) {
    camera.release(Vec2::default());
    state.remove::<RoomRegistry>().expect("Failed to remove level state");
  }
}

/// Listen for level events
impl Systemize for LevelScene {
  fn system(SysArgs { event, scene, asset, world, state, .. }: &mut SysArgs) -> Result<(), String> {
    let PlayerQuery { health, .. } = use_player(world);

    let dead = health.get_state() == LiveState::Dead;
    let exit = is_control(Control::Escape, Behaviour::Pressed, event) || dead;

    if dead {
      scene.queue_next(MenuScene)
      // todo: death stuff... write the obituary, engrave the tombstone, you know the drill...
    } else if exit && !event.must_pause() {
      event.queue_pause();
      make_menu(world, asset, state);
    }

    let preferences = use_preferences(state);
    if is_control(Control::Debug, Behaviour::Pressed, event) {
      preferences.debug = !preferences.debug;
    }

    Ok(())
  }
}

use std::path::Path;

use crate::engine::geometry::shape::Vec2;
use crate::engine::lifecycle::LifecycleArgs;
use crate::engine::scene::Scene;
use crate::engine::system::{Schedule, SysArgs};
use crate::engine::tile::consume::{tilemap_from_tiled, tileset_from_tiled};
use crate::engine::tile::parse::TiledParser;
use crate::engine::tile::tile::sys_render_tile_colliders;
use crate::game::physics::collision::sys_render_colliders;
use crate::game::physics::gravity::sys_gravity;
use crate::game::physics::velocity::sys_velocity;
use crate::game::player::world::add_player;
use crate::game::scene::level::collision::sys_tile_collision;
use crate::game::scene::menu::MenuScene;
use crate::game::utility::controls::{Behaviour, Control, is_control};

/**
 * The level scene
 */

pub struct LevelScene {
  level_key: u32,
}

impl LevelScene {
  pub fn build(level_key: u32) -> Result<Self, String> {
    Ok(Self { level_key })
  }
}

impl Scene for LevelScene {
  /// Set up the level scene
  fn setup(&self, LifecycleArgs { world, system, asset, .. }: &mut LifecycleArgs) {
    let parser = TiledParser::parse(&Path::new("asset/world.world"))
      .map_err(|e| println!("Failed to parse Tiled data: {}", e))
      .expect("Failed to parse Tiled data");

    let tileset = tileset_from_tiled(asset, &parser.tilesets[0]).expect("Failed to build tileset");
    let mut tilemap = tilemap_from_tiled(parser.tilemaps.get(0).unwrap(), &tileset).expect("Failed to build tilemap");
    tilemap.add_to_world(world, Vec2::default()).expect("Failed to add tilemap to world");

    println!("Level {} Initialized.", self.level_key);

    add_player(world, system, asset);
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
    scene.queue_next(MenuScene);
  }
}

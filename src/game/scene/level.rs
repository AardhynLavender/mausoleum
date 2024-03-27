use std::path::Path;

use hecs::QueryMut;

use crate::engine::geometry::Vec2;
use crate::engine::lifecycle::LifecycleArgs;
use crate::engine::render::component::Sprite;
use crate::engine::scene::Scene;
use crate::engine::system::{Schedule, SysArgs};
use crate::engine::tile::consume::{tilemap_from_tiled, tileset_from_tiled};
use crate::engine::tile::parse::TiledParser;
use crate::engine::tile::tile::Tile;
use crate::engine::tile::tilemap::Tilemap;
use crate::engine::world::{EntityManager, World};
use crate::game::component::physics::Gravity;
use crate::game::component::position::Position;
use crate::game::scene::menu::MenuScene;
use crate::game::system::physics::sys_gravity;
use crate::game::utility::controls::{Behaviour, Control, is_control};

/**
 * The level scenes
 */

pub struct Room {
  tilemap: Tilemap,
}

impl Room {
  pub fn new(tilemap: Tilemap) -> Self {
    Self { tilemap }
  }
}

impl EntityManager for Room {
  type Manager = Tilemap;
  type ComponentQuery<'q> = (&'q Position, &'q Tile, &'q Sprite);

  /// add the tiles to the world
  fn add_to_world(&mut self, world: &mut World) {
    self.tilemap.add_to_world(world, Vec2::new(0.0, 0.0));
  }
  /// remove the tiles from the world
  fn remove_from_world(&mut self, world: &mut World) -> Result<(), String> {
    self.tilemap.remove_from_world(world)?;
    Ok(())
  }
  /// query the world for the entities of the manager
  fn query_entities<'q>(&'q mut self, world: &'q mut World) -> QueryMut<Self::ComponentQuery<'q>> {
    world.query::<Self::ComponentQuery<'q>>()
  }
}

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
    tilemap.add_to_world(world, Vec2::default());

    println!("Level {} Initialized.", self.level_key);
    system.add(Schedule::FrameUpdate, sys_gravity);
    system.add(Schedule::FrameUpdate, sys_level_listener);

    world.add((
      Position::new(80.0, 90.0),
      Gravity::new(0.0, 1.0),
      Sprite::new(asset.texture.load(Path::new("asset/test.png")).expect("Failed to load texture"), Vec2::new(8, 8).into())
    ));
  }
  /// Clean up the level scene
  fn destroy(&self, _: &mut LifecycleArgs) {}
}

/// Events to listen for during the level scene
pub fn sys_level_listener(SysArgs { event, scene, .. }: &mut SysArgs) {
  if is_control(Control::Escape, Behaviour::Pressed, event) {
    scene.queue_next(MenuScene);
  }
  if is_control(Control::Select, Behaviour::Pressed, event) {}
}

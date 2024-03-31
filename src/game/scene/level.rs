use std::path::Path;

use hecs::QueryMut;

use crate::engine::geometry::collision::{Collision, CollisionBox, rec2_collision};
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::lifecycle::LifecycleArgs;
use crate::engine::render::component::Sprite;
use crate::engine::scene::Scene;
use crate::engine::system::{Schedule, SysArgs};
use crate::engine::tile::consume::{tilemap_from_tiled, tileset_from_tiled};
use crate::engine::tile::parse::TiledParser;
use crate::engine::tile::tile::{sys_render_tile_colliders, Tile, TileCollider};
use crate::engine::tile::tilemap::Tilemap;
use crate::engine::world::{EntityManager, World};
use crate::game::physics::collision::sys_render_colliders;
use crate::game::physics::gravity::sys_gravity;
use crate::game::physics::position::Position;
use crate::game::physics::velocity::sys_velocity;
use crate::game::player::world::{add_player, use_player};
use crate::game::scene::menu::MenuScene;
use crate::game::utility::controls::{Behaviour, Control, is_control};

/**
 * The level scene
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
    self.tilemap.add_to_world(world, Vec2::new(0.0, 0.0)).expect("Failed to add tilemap to world");
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
    tilemap.add_to_world(world, Vec2::default()).expect("Failed to add tilemap to world");

    println!("Level {} Initialized.", self.level_key);
    system.add(Schedule::FrameUpdate, sys_gravity);
    system.add(Schedule::FrameUpdate, sys_velocity);
    system.add(Schedule::FrameUpdate, sys_tile_collision);

    system.add(Schedule::PostUpdate, sys_render_tile_colliders);
    system.add(Schedule::PostUpdate, sys_render_colliders);

    system.add(Schedule::PostUpdate, sys_level_listener);

    add_player(world, system, asset);
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

#[allow(dead_code)]
pub fn sys_tile_collision(SysArgs { world, .. }: &mut SysArgs) {
  let mut phase = 0;
  'resolving: loop {
    phase += 1;
    let (position, _, collider, ..) = use_player(world);
    let player_rect = Rec2::new(position.0 + collider.0.origin, collider.0.size);

    let collision = get_tile_collisions(world, &player_rect).next();
    if let Some(collision) = collision {
      if phase > 10 {
        panic!("Infinite collision resolution loop detected, what do?");
      }

      let (position, v, ..) = use_player(world);
      let resolution = collision.get_resolution();

      position.0 = position.0 - resolution;
      if resolution.y > 0.0 && v.0.y < 0.0 {
        // cut vertical acceleration if resolving up while falling
        // eg: landing on a platform
        v.0.y = 0.0;
      } else if resolution.y < 0.0 && v.0.y > 0.0 {
        // cut vertical acceleration if resolving down while jumping
        // eg: hitting head on a platform
        v.0.y = 0.0;
      } else if resolution.x != 0.0 {
        // cut horizontal acceleration if resolving left or right
        v.0.x = 0.0;
      }
    } else {
      break 'resolving;
    }
  };
}

fn get_tile_collisions<'a>(world: &'a mut World, collider_box: &'a CollisionBox) -> impl Iterator<Item=Collision> + 'a {
  world.query::<(&Position, &TileCollider, &Tile)>()
    .into_iter()
    .filter_map(|(_, (tile_position, tile_collider, ..))| {
      let tile_rect = &CollisionBox::new(tile_position.0 + tile_collider.collision_box.origin, tile_collider.collision_box.size);
      rec2_collision(collider_box, tile_rect, tile_collider.mask)
    })
}
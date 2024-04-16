use std::path::Path;

use crate::engine::asset::AssetManager;
use crate::engine::geometry::shape::Vec2;
use crate::engine::rendering::camera::CameraTether;
use crate::engine::rendering::component::Sprite;
use crate::engine::rendering::renderer::layer;
use crate::engine::system::{Schedule, SystemManager};
use crate::engine::world::World;
use crate::game::combat::health::Health;
use crate::game::constant::{GRAVITY, PLAYER_COLLIDER, PLAYER_SPRITE, PLAYER_START};
use crate::game::physics::collision::Collider;
use crate::game::physics::gravity::Gravity;
use crate::game::physics::position::Position;
use crate::game::physics::velocity::Velocity;
use crate::game::player::combat::{PLAYER_HEALTH, PlayerCombat};
use crate::game::player::controller::{PlayerController, sys_player_controller};
use crate::game::scene::level::collision::RoomCollision;

/**
 * Useful queries for the player entity
 */

/// Path to the player asset
const PLAYER_ASSET: &str = "asset/test.png";

pub type LayerPlayer = layer::Layer5;

/// Components of the player entity
pub type PlayerComponents<'p> = (&'p mut PlayerCombat, &'p mut Position, &'p mut Velocity, &'p mut PlayerController, &'p mut Collider, &'p mut Health);

/// Query structure for the player entity
pub struct PQ<'p> {
  pub combat: &'p mut PlayerCombat,
  pub position: &'p mut Position,
  pub velocity: &'p mut Velocity,
  pub controller: &'p mut PlayerController,
  pub collider: &'p mut Collider,
  pub health: &'p mut Health,
}

/// Query the world for the player return its components
///
/// This mutably borrows the `world`, so a user will probably invoke this multiple times within a *system*
/// ## Panics
/// This function will panic if the player entity is not found
pub fn use_player(world: &mut World) -> PQ {
  let (_, components) = world.query::<PlayerComponents>()
    .into_iter()
    .next()
    .expect("Failed to get player");
  PQ {
    combat: components.0,
    position: components.1,
    velocity: components.2,
    controller: components.3,
    collider: components.4,
    health: components.5,
  }
}

pub fn add_player(world: &mut World, system: &mut SystemManager, asset: &mut AssetManager) {
  let player_texture = asset.texture
    .load(Path::new(PLAYER_ASSET))
    .expect("Failed to load player texture");
  let projectile_texture = asset.texture
    .load(Path::new("asset/plasma_burst.png"))
    .expect("Failed to load projectile texture");

  world.add((
    PlayerCombat::new(projectile_texture),
    PlayerController::default(),
    Sprite::new(player_texture, PLAYER_SPRITE.into()),
    Position::from(PLAYER_START),
    CameraTether::new(Vec2::<i32>::from(PLAYER_SPRITE.size / 2)), // player center
    LayerPlayer::default(),
    Gravity::new(GRAVITY),
    Velocity::default(),
    RoomCollision::default(),
    Collider::new(PLAYER_COLLIDER),
    Health::build(PLAYER_HEALTH).expect("Failed to build player health"),
  ));

  system.add(Schedule::PostUpdate, sys_player_controller);
}


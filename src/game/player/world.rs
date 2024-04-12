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
use crate::game::player::component::{PlayerController, PlayerData, sys_player_controller};

/**
 * Useful queries for the player entity
 */

/// Path to the player asset
const PLAYER_ASSET: &str = "asset/test.png";
/// The player's starting health
const PLAYER_HEALTH: i32 = 100;

pub type LayerPlayer = layer::Layer5;

/// Components of the player entity
pub type PlayerComponents<'p> = (&'p mut PlayerData, &'p mut Position, &'p mut Velocity, &'p mut PlayerController, &'p mut Collider, &'p mut Health);

/// Query the world for the player return its components
///
/// This mutably borrows the `world`, so a user will probably invoke this multiple times within a *system*
/// ## Panics
/// This function will panic if the player entity is not found
pub fn use_player(world: &mut World) -> PlayerComponents {
  let (_, components) = world.query::<PlayerComponents>()
    .into_iter()
    .next()
    .expect("Failed to get player");
  components
}

pub fn add_player(world: &mut World, system: &mut SystemManager, asset: &mut AssetManager) {
  let player_texture = asset.texture
    .load(Path::new(PLAYER_ASSET))
    .expect("Failed to load player texture");

  world.add((
    PlayerData::new(),
    PlayerController::default(),
    Sprite::new(player_texture, PLAYER_SPRITE.into()),
    Position::from(PLAYER_START),
    CameraTether::new(Vec2::<i32>::from(PLAYER_SPRITE.size / 2)), // player center
    LayerPlayer::default(),
    Gravity::new(GRAVITY),
    Velocity::default(),
    Collider::new(PLAYER_COLLIDER),
    Health::build(PLAYER_HEALTH).expect("Failed to build player health"),
  ));

  system.add(Schedule::PostUpdate, sys_player_controller);
}

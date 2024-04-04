use std::path::Path;

use crate::engine::asset::AssetManager;
use crate::engine::rendering::component::Sprite;
use crate::engine::rendering::renderer::layer;
use crate::engine::system::{Schedule, SystemManager};
use crate::engine::world::World;
use crate::game::constant::{GRAVITY, PLAYER_COLLIDER, PLAYER_SPRITE, PLAYER_START};
use crate::game::physics::collision::Collider;
use crate::game::physics::gravity::Gravity;
use crate::game::physics::position::Position;
use crate::game::physics::velocity::Velocity;
use crate::game::player::controls::{PlayerController, sys_player_controller};

/**
 * Useful queries for the player entity
 */

type LayerPlayer = layer::Layer4;

/// Components of the player entity
pub type PlayerComponents<'p> = (&'p mut Position, &'p mut Velocity, &'p mut Collider, &'p mut PlayerController);

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
  world.add((
    Position::from(PLAYER_START),
    Gravity::new(GRAVITY),
    Sprite::new(asset.texture.load(Path::new("asset/test.png")).expect("Failed to load texture"), PLAYER_SPRITE.into()),
    Collider::new(PLAYER_COLLIDER),
    PlayerController::default(),
    Velocity::default(),
    LayerPlayer {}
  ));

  system.add(Schedule::PostUpdate, sys_player_controller);
}
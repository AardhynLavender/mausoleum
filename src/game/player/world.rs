use std::path::Path;

use crate::engine::asset::AssetManager;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::render::component::Sprite;
use crate::engine::system::{Schedule, SystemManager};
use crate::engine::utility::alias::Size2;
use crate::engine::world::World;
use crate::game::constant::GRAVITY;
use crate::game::physics::collision::Collider;
use crate::game::physics::gravity::Gravity;
use crate::game::physics::position::Position;
use crate::game::physics::velocity::Velocity;
use crate::game::player::controls::{PlayerController, sys_player_controller};

/**
 * Useful queries for the player entity
 */

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
    Position::new(80.0, 90.0),
    Gravity::new(GRAVITY),
    Sprite::new(asset.texture.load(Path::new("asset/test.png")).expect("Failed to load texture"), Size2::new(8, 8).into()),
    Collider::new(Rec2::new(Vec2::default(), Vec2::new(8u32, 8u32))),
    PlayerController::default(),
    Velocity::default(),
  ));

  system.add(Schedule::PostUpdate, sys_player_controller);
}
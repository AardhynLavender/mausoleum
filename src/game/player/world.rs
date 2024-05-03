use std::path::Path;

use crate::engine::asset::AssetManager;
use crate::engine::asset::texture::SrcRect;
use crate::engine::geometry::collision::CollisionBox;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::rendering::camera::CameraTether;
use crate::engine::rendering::component::Sprite;
use crate::engine::rendering::renderer::layer;
use crate::engine::system::{Schedule, SystemManager};
use crate::engine::utility::alias::Size2;
use crate::engine::world::World;
use crate::game::collectable::collectable::Collection;
use crate::game::combat::health::Health;
use crate::game::physics::collision::Collider;
use crate::game::physics::gravity::Gravity;
use crate::game::physics::position::Position;
use crate::game::physics::velocity::Velocity;
use crate::game::player::combat::{PLAYER_BASE_HEALTH, PlayerCombat};
use crate::game::player::controller::{PlayerController, sys_player_controller};
use crate::game::player::physics::{calculate_gravity, INITIAL_JUMP_HEIGHT, INITIAL_JUMP_WIDTH, INITIAL_WALK_SPEED};
use crate::game::scene::level::collision::RoomCollision;
use crate::game::scene::level::meta::Collectable;

/**
 * Useful queries for the player entity
 */

pub const PLAYER_SIZE: Size2 = Size2::new(12, 28);

const PLAYER_ASSET: &str = "asset/test.png";
const PLAYER_SPRITE: SrcRect = SrcRect::new(Vec2::new(0, 0), PLAYER_SIZE);
const PLAYER_COLLIDER: CollisionBox = Rec2::new(Vec2::new(0.0, 0.0), PLAYER_SIZE);

/// Alias for the player layer
pub type LayerPlayer = layer::Layer5;

/// Components of the player entity
pub type PlayerComponents<'p> = (&'p mut PlayerCombat, &'p mut Position, &'p mut Velocity, &'p mut PlayerController, &'p mut Gravity, &'p mut Collider, &'p mut Health, &'p mut Collection);

/// Query structure for the player entity
pub struct PlayerQuery<'p> {
  pub combat: &'p mut PlayerCombat,
  pub gravity: &'p mut Gravity,
  pub position: &'p mut Position,
  pub velocity: &'p mut Velocity,
  pub controller: &'p mut PlayerController,
  pub collider: &'p mut Collider,
  pub health: &'p mut Health,
  pub inventory: &'p mut Collection,
}

/// Query the world for the player return its components
///
/// This mutably borrows the `world`, so a user will probably invoke this multiple times within a *system*
/// ## Panics
/// This function will panic if the player entity is not found
pub fn use_player(world: &mut World) -> PlayerQuery {
  let (_, components) = world.query::<PlayerComponents>()
    .into_iter()
    .next()
    .expect("Failed to get player");
  PlayerQuery {
    combat: components.0,
    position: components.1,
    velocity: components.2,
    controller: components.3,
    gravity: components.4,
    collider: components.5,
    health: components.6,
    inventory: components.7,
  }
}

/// Set up the world for the player
pub fn make_player(world: &mut World, system: &mut SystemManager, asset: &mut AssetManager, inventory: impl Iterator<Item=Collectable>, position: Vec2<f32>) {
  let player_texture = asset.texture
    .load(Path::new(PLAYER_ASSET))
    .expect("Failed to load player texture");
  let bullet = asset.texture
    .load(Path::new("asset/plasma_burst.png"))
    .expect("Failed to load bullet texture");
  let rocket = asset.texture
    .load(Path::new("asset/desolation_pulse.png"))
    .expect("Failed to load rocket texture");
  let ice_beam = asset.texture
    .load(Path::new("asset/temporal_flare.png"))
    .expect("Failed to load ice beam texture");

  world.add((
    PlayerCombat::new(bullet, rocket, ice_beam),
    PlayerController::default(),
    Sprite::new(player_texture, PLAYER_SPRITE.into()),
    Position::from(position),
    CameraTether::new(Vec2::<i32>::from(PLAYER_SPRITE.size / 2)), // player center
    LayerPlayer::default(),
    Gravity::new(calculate_gravity(INITIAL_JUMP_HEIGHT, INITIAL_WALK_SPEED, INITIAL_JUMP_WIDTH)),
    Collection::new(inventory),
    Velocity::default(),
    RoomCollision::default(),
    Collider::new(PLAYER_COLLIDER),
    Health::build(PLAYER_BASE_HEALTH).expect("Failed to build player health"),
  ));

  system.add(Schedule::PostUpdate, sys_player_controller);
}


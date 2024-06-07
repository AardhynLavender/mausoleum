/**
  * Flat flying creature that moves horizontally and switches direction when colliding with walls
 */

use std::path::Path;

use hecs::DynamicBundle;

use crate::engine::asset::asset::AssetManager;
use crate::engine::component::position::Position;
use crate::engine::component::sprite::Sprite;
use crate::engine::ecs::system::{SysArgs, Systemize};
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::utility::alias::Size2;
use crate::engine::utility::direction::Direction;
use crate::game::scene::level::combat::damage::Damage;
use crate::game::scene::level::combat::health::Health;
use crate::game::scene::level::creature::CreatureLayer;
use crate::game::scene::level::physics::collision::Collider;
use crate::game::scene::level::physics::frozen::Frozen;
use crate::game::scene::level::physics::velocity::Velocity;
use crate::game::scene::level::player::combat::PlayerHostile;
use crate::game::scene::level::room::collision::{CollisionBox, RoomCollision};
use crate::game::utility::math::floor_to_tile;

const RIPPER_SPEED: f32 = 64.0;
const RIPPER_ASSET: &str = "asset/sprite/ripper.png";
const RIPPER_HEALTH: u32 = 30;
const RIPPER_DAMAGE: u32 = 10;
const DIMENSIONS: Size2 = Size2::new(16, 8);

pub struct Ripper {
  last_velocity: Vec2<f32>,
}

impl Systemize for Ripper {
  /// Process Ripper logic each frame
  fn system(SysArgs { world, .. }: &mut SysArgs) -> Result<(), String> {
    for (.., (data, velocity)) in world.query::<(&mut Ripper, &mut Velocity)>().without::<&Frozen>() {
      if velocity.is_none() {
        velocity.0 = data.last_velocity.invert();
        data.last_velocity = velocity.0;
      }
    }
    Ok(())
  }
}

/// Add a Ripper to the world
pub fn make_ripper(asset_manager: &mut AssetManager, position: Vec2<f32>, initial_direction: Direction) -> Result<impl DynamicBundle, String> {
  if initial_direction != Direction::Left && initial_direction != Direction::Right {
    return Err(String::from("Ripper must be initialized with a horizontal direction"));
  }

  let ripper = asset_manager.texture.load(Path::new(RIPPER_ASSET))?;
  let floored_position = floor_to_tile(position);

  let velocity = Vec2::from(initial_direction.to_coordinate()) * RIPPER_SPEED;

  Ok((
    PlayerHostile,
    Ripper { last_velocity: velocity },
    Sprite::new(ripper, Rec2::new(Vec2::default(), DIMENSIONS)),
    Position(floored_position),
    Velocity::from(Vec2::<f32>::from(initial_direction.to_coordinate()) * RIPPER_SPEED),
    Collider::new(CollisionBox::new(Vec2::default(), DIMENSIONS)),
    CreatureLayer::default(),
    Damage::new(RIPPER_DAMAGE),
    Health::build(RIPPER_HEALTH).expect("Failed to build health"),
    RoomCollision::Creature,
  ))
}

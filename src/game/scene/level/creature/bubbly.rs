/**
 * Passive creature that moves in a straight line and bounces off walls
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

const SPEED: f32 = 40.0;
const ASSET: &str = "asset/sprite/bubbly.png";
const HEALTH: u32 = 30;
const DAMAGE: u32 = 15;
const DIMENSIONS: Size2 = Size2::new(16, 16);

// bubbly component
pub struct Bubbly {
  previous_velocity: Velocity,
}

impl Bubbly {
  pub fn new(initial_velocity: Velocity) -> Self {
    Self { previous_velocity: initial_velocity }
  }

  /// Updates the bubbly's velocity and return the old velocity
  pub fn update(&mut self, next: Velocity) -> Velocity {
    let previous = self.previous_velocity;
    self.previous_velocity = next;
    previous
  }
}

/// Add a Bubbly to the world
pub fn make_bubbly(asset_manager: &mut AssetManager, position: Vec2<f32>, direction: Direction) -> Result<impl DynamicBundle, String> {
  let creature = asset_manager.texture.load(Path::new(ASSET))?;
  let velocity = Velocity::from(Vec2::from(direction.to_coordinate()) * SPEED);
  let floored_position = floor_to_tile(position);
  Ok((
    PlayerHostile,
    Bubbly::new(velocity),
    Sprite::new(creature, Rec2::new(Vec2::default(), DIMENSIONS)),
    Position::from(floored_position),
    velocity,
    Collider::new(CollisionBox::new(Vec2::default(), DIMENSIONS)),
    CreatureLayer::default(),
    Damage::new(DAMAGE),
    Health::build(HEALTH).expect("Failed to build health"),
    RoomCollision::Creature,
  ))
}

impl Systemize for Bubbly {
  /// Process bubbly logic each frame
  fn system(SysArgs { world, .. }: &mut SysArgs) -> Result<(), String> {
    for (_, (bubbly, bubbly_velocity)) in world
      .query::<(&mut Bubbly, &mut Velocity)>()
      .without::<&Frozen>()
    {
      let previous_velocity = bubbly.update(*bubbly_velocity);
      if bubbly_velocity.0.x == 0.0 { bubbly_velocity.0.x = -previous_velocity.0.x; }
      if bubbly_velocity.0.y == 0.0 { bubbly_velocity.0.y = -previous_velocity.0.y; }
    }

    Ok(())
  }
}

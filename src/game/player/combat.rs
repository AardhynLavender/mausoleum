/**
 * Player combat components and systems
 */

use std::time::Duration;

use crate::engine::asset::texture::{SrcRect, TextureKey};
use crate::engine::geometry::collision::CollisionBox;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::rendering::color::{OPAQUE, RGBA};
use crate::engine::rendering::component::Sprite;
use crate::engine::rendering::renderer::layer;
use crate::engine::system::{SysArgs, Systemize};
use crate::engine::time::Timer;
use crate::engine::utility::alias::Size2;
use crate::engine::utility::direction::{Direction, QUARTER_ROTATION_DEG};
use crate::engine::world::World;
use crate::game::combat::damage::Damage;
use crate::game::combat::ttl::TimeToLive;
use crate::game::physics::collision::{Collider, Fragile};
use crate::game::physics::position::Position;
use crate::game::physics::velocity::Velocity;
use crate::game::player::world::{PLAYER_SIZE, PlayerQuery, use_player};
use crate::game::scene::level::collision::RoomCollision;

pub const PLAYER_BASE_HEALTH: u32 = 50;
pub const HEALTH_PICKUP_INCREASE: u32 = 10;
pub const HIT_COOLDOWN: u64 = 500;

const PROJECTILE_COOLDOWN: u64 = 300;
const PROJECTILE_LIFETIME: u64 = 500;
const PROJECTILE_SPEED: f32 = 300.0;

const BULLET_DAMAGE: u32 = 10;
const BULLET_DIMENSIONS: Size2 = Size2::new(12, 3);

const ROCKET_DAMAGE: u32 = 20;
const ROCKET_DIMENSIONS: Size2 = Size2::new(12, 3);

const ICE_BEAM_DIMENSIONS: Size2 = Size2::new(12, 3);
pub const THAW_DURATION: u64 = 5_000;

/// Mark an entity as being hostile to the player
pub struct PlayerHostile;

/// Store player specific data
pub struct PlayerCombat {
  pub hit_cooldown: Timer,
  pub trigger_cooldown: Timer,
  pub bullet_texture: TextureKey,
  pub rocket_texture: TextureKey,
  pub ice_beam_texture: TextureKey,
}

impl PlayerCombat {
  // Instantiate a new player combat component
  pub fn new(bullet_texture: TextureKey, rocket_texture: TextureKey, ice_beam_texture: TextureKey) -> Self {
    Self {
      hit_cooldown: Timer::new(Duration::from_millis(HIT_COOLDOWN), true),
      bullet_texture,
      rocket_texture,
      ice_beam_texture,
      trigger_cooldown: Timer::new(Duration::from_millis(PROJECTILE_COOLDOWN), false),
    }
  }
}

impl Systemize for PlayerCombat {
  /// Render the player's hit cooldown
  fn system(SysArgs { world, render, camera, .. }: &mut SysArgs) -> Result<(), String> {
    let PlayerQuery { combat, position, .. } = use_player(world);
    if !combat.hit_cooldown.done() {
      let rect = Rec2::new(Vec2::<i32>::from(camera.translate(position.0)) - 2, Size2::new(16, 32));
      render.draw_rect(rect, RGBA::new(255, 0, 255, OPAQUE));
    }

    Ok(())
  }
}

/// Available weapon types for the player
#[derive(PartialEq)]
pub enum Weapon { Bullet, Rocket, IceBeam }

pub type ProjectileLayer = layer::Layer8;

/// Mark an entity as a bullet projectile
pub struct Bullet;

/// Mark an entity as a rocket projectile
pub struct Rocket;

/// Mark an entity as a projectile that freezes creatures
pub struct IceBeam;

/// Mark an entity as damaging to creatures but the player
pub struct CreatureHostile;

/// Fire a plasma projectile in the direction the player is aiming
pub fn fire_weapon(world: &mut World, aim: Direction, weapon: Weapon) {
  let PlayerQuery { combat, position, .. } = use_player(world);
  let (position, velocity, rotation) = compute_projectile_spawn(aim, position.0, PLAYER_SIZE);

  let (dimensions, texture) = match weapon {
    Weapon::Bullet => (BULLET_DIMENSIONS, combat.bullet_texture),
    Weapon::Rocket => (ROCKET_DIMENSIONS, combat.rocket_texture),
    Weapon::IceBeam => (ICE_BEAM_DIMENSIONS, combat.ice_beam_texture)
  };

  let collision_box = CollisionBox::new(Vec2::new(0.0, 0.0), dimensions);
  let mut sprite = Sprite::new(texture, SrcRect::new(Vec2::new(0, 0), dimensions));
  sprite.rotate(rotation.into());

  if !combat.trigger_cooldown.done() { return; }
  combat.trigger_cooldown.reset();

  let projectile = world.add((
    sprite,
    CreatureHostile,
    ProjectileLayer::default(),
    Position(position),
    Velocity(velocity),
    Collider::new(collision_box),
    RoomCollision,
    Fragile,
    TimeToLive::new(PROJECTILE_LIFETIME),
  ));

  match weapon {
    Weapon::Bullet => world.add_components(projectile, (Bullet, Damage::new(BULLET_DAMAGE), )),
    Weapon::Rocket => world.add_components(projectile, (Rocket, Damage::new(ROCKET_DAMAGE), )),
    Weapon::IceBeam => world.add_components(projectile, (IceBeam, Damage::new(0), )),
  }.expect("Failed to add weapon components to projectile")
}

/// Compute the starting position and velocity and rotation of the player projectile
pub fn compute_projectile_spawn(aim: Direction, player_position: Vec2<f32>, player_bounds: Size2) -> (Vec2<f32>, Vec2<f32>, f32) {
  let player_centroid = player_position + Vec2::<f32>::from(player_bounds / 2);
  let projectile_centroid = Vec2::<f32>::from(BULLET_DIMENSIONS / 2);
  let unit_vector = Vec2::from(aim.to_coordinate());
  let position = player_centroid - projectile_centroid + unit_vector * player_bounds.x as f32;
  let velocity = unit_vector * PROJECTILE_SPEED;
  let rotation = f32::from(aim) + QUARTER_ROTATION_DEG as f32;
  (position, velocity, rotation)
}




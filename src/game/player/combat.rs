/**
 * Player combat components and systems
 */

use std::time::Duration;

use crate::engine::asset::texture::{SrcRect, TextureKey};
use crate::engine::geometry::collision::CollisionBox;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::rendering::color::{OPAQUE, RGBA};
use crate::engine::rendering::component::Sprite;
use crate::engine::system::SysArgs;
use crate::engine::time::Timer;
use crate::engine::utility::alias::Size2;
use crate::engine::utility::direction::Direction;
use crate::engine::world::World;
use crate::game::combat::projectile::make_projectile;
use crate::game::constant::PLAYER_SIZE;
use crate::game::player::world::use_player;

/// The player's starting health
pub const PLAYER_HEALTH: i32 = 100;

const PLASMA_DAMAGE: u32 = 10;
const PLASMA_LIFETIME_MS: u64 = 1000;
const PLASMA_SPEED: f32 = 300.0;
const PLASMA_DIMENSIONS: Size2 = Size2::new(8, 2);

/// Store player specific data
pub struct PlayerCombat {
  pub hit_cooldown: Timer,
  pub trigger_cooldown: Timer,
  pub projectile_texture: TextureKey,
}

impl PlayerCombat {
  // Instantiate a new player combat component
  pub fn new(projectile_texture: TextureKey) -> Self {
    Self {
      hit_cooldown: Timer::new(Duration::from_millis(500), true),
      projectile_texture,
      trigger_cooldown: Timer::new(Duration::from_millis(100), false),
    }
  }
}

/// Render the player's hit cooldown
pub fn sys_render_cooldown(SysArgs { world, render, camera, .. }: &mut SysArgs) {
  let (player_data, position, ..) = use_player(world);
  if !player_data.hit_cooldown.done() {
    render.draw_rect(Rec2::new(Vec2::<i32>::from(camera.translate(position.0)) - 2, Size2::new(16, 32)), RGBA::new(255, 0, 255, OPAQUE));
  }
}

/// Available weapon types for the player
pub enum Weapon {
  Plasma,
  Rocket,
}

/// Mark an entity as a player projectile
#[derive(Default)]
pub struct PlayerProjectile;

/// Fire a plasma projectile in the direction the player is aiming
pub fn fire_weapon(world: &mut World, aim: Direction, _weapon: Weapon) {
  let (data, position, ..) = use_player(world);
  let (position, velocity, rotation) = compute_projectile_spawn(aim, position.0, PLAYER_SIZE);

  let collision_box = CollisionBox::new(Vec2::new(0.0, 0.0), PLASMA_DIMENSIONS);

  let mut sprite = Sprite::new(data.projectile_texture, SrcRect::new(Vec2::new(0, 0), PLASMA_DIMENSIONS));
  sprite.rotate(rotation.into());

  if data.trigger_cooldown.done() {
    data.trigger_cooldown.reset();
    world.add(make_projectile::<PlayerProjectile>(
      PLASMA_DAMAGE,
      collision_box,
      position,
      velocity,
      sprite,
      PLASMA_LIFETIME_MS,
    ));
  }
}

// Compute the starting position and velocity and rotation of the player projectile
pub fn compute_projectile_spawn(aim: Direction, player_position: Vec2<f32>, player_bounds: Size2) -> (Vec2<f32>, Vec2<f32>, f32) {
  let radius = Vec2::from(player_bounds) / 2.0;
  let centroid = player_position + radius;
  let position = centroid + Vec2::from(aim.to_coordinate()) * radius;
  let velocity = Vec2::from(aim.to_coordinate()) * PLASMA_SPEED;
  let rotation = f32::from(aim) + 90.0;
  (position, velocity, rotation)
}

/**
 * Player controls
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
use crate::game::constant::{JUMP_ACCELERATION, PLAYER_SIZE, WALK_SPEED};
use crate::game::player::world::use_player;
use crate::game::utility::controls::{Behaviour, Control, get_direction, is_control};

const PLASMA_DAMAGE: u32 = 10;
const PLASMA_LIFETIME_MS: u64 = 1000;
const PLASMA_SPEED: f32 = 300.0;
const PLASMA_DIMENSIONS: Size2 = Size2::new(8, 2);

const INITIAL_DIRECTION: Direction = Direction::Right;

// data //

/// Store player specific data
pub struct PlayerData {
  pub hit_cooldown: Timer,
  pub trigger_cooldown: Timer,
  pub projectile_texture: TextureKey,
}

impl PlayerData {
  pub fn new(projectile_texture: TextureKey) -> Self {
    Self {
      hit_cooldown: Timer::new(Duration::from_millis(500), true),
      projectile_texture,
      trigger_cooldown: Timer::new(Duration::from_millis(100), false),
    }
  }
}

pub fn sys_render_cooldown(SysArgs { world, render, camera, .. }: &mut SysArgs) {
  let (player_data, position, ..) = use_player(world);
  if !player_data.hit_cooldown.done() {
    render.draw_rect(Rec2::new(Vec2::<i32>::from(camera.translate(position.0)) - 2, Size2::new(16, 32)), RGBA::new(255, 0, 255, OPAQUE));
  }
}

#[derive(Default)]
pub struct PlayerProjectile;

// Controller //

pub struct PlayerController {
  #[allow(unused)]
  jumping: bool,
  #[allow(unused)]
  can_jump: bool,
  last_walk: Direction,
  last_aim: Direction,
  locked: bool,
}

impl Default for PlayerController {
  fn default() -> Self {
    Self {
      jumping: false,
      can_jump: true,
      last_walk: INITIAL_DIRECTION,
      last_aim: INITIAL_DIRECTION,
      locked: false,
    }
  }
}

impl PlayerController {
  /// Set the last direction the player walked
  fn set_walked(&mut self, direction: Direction) { self.last_walk = direction; }
  /// Set the last direction the player aimed
  fn set_aimed(&mut self, direction: Direction) { self.last_aim = direction; }
}

pub fn sys_player_controller(SysArgs { event, world, .. }: &mut SysArgs) {
  let (_, _, velocity, controller, ..) = use_player(world);
  let aim = get_direction(event, Behaviour::Held).unwrap_or(controller.last_aim);

  // Jump //

  if is_control(Control::Select, Behaviour::Pressed, event) {
    velocity.0.y = JUMP_ACCELERATION.y;
  }

  // Walk //

  let left = is_control(Control::Left, Behaviour::Held, event);
  let right = is_control(Control::Right, Behaviour::Held, event);

  if left && !right {
    controller.set_walked(Direction::Left);
    velocity.0.x = -WALK_SPEED;
  } else if right && !left {
    controller.set_walked(Direction::Right);
    velocity.0.x = WALK_SPEED;
  } else {
    velocity.remove_x();
  }

  // Lock //

  if is_control(Control::Lock, Behaviour::Held, event) {
    velocity.remove_x();
    controller.locked = true;
  } else {
    controller.locked = false;
  }

  // Aim //

  controller.set_aimed(aim);

  if is_control(Control::Trigger, Behaviour::Pressed, event) {
    fire_plasma(world, aim);
  }
}

pub fn fire_plasma(world: &mut World, aim: Direction) {
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
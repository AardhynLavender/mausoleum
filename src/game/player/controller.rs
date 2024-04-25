use std::time::Duration;

use crate::engine::asset::texture::SrcRect;
use crate::engine::geometry::collision::CollisionBox;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::system::SysArgs;
use crate::engine::time::Timer;
use crate::engine::utility::alias::Size2;
use crate::engine::utility::direction::Direction;
use crate::game::player::combat::{fire_weapon, Weapon};
use crate::game::player::world::{PlayerQuery, use_player};
use crate::game::utility::controls::{Behaviour, Control, get_direction, is_control};

const INITIAL_DIRECTION: Direction = Direction::Right;

// 6 tiles
pub const JUMP_HEIGHT: f32 = 96.0;
// 4 tiles
pub const JUMP_WIDTH: f32 = 96.0;
// 3 tiles per second
pub const WALK_SPEED: f32 = 128.0;

pub const DASH_MULTIPLIER: f32 = 100.0;
pub const JUMP_ACCELERATION: Vec2<f32> = Vec2::new(0.0, -(((2.0 * JUMP_HEIGHT) * WALK_SPEED) / (JUMP_WIDTH / 2.0)));
pub const PLAYER_GRAVITY: Vec2<f32> = Vec2::new(0.0, -((-2.0 * JUMP_HEIGHT * (WALK_SPEED * WALK_SPEED)) / ((JUMP_WIDTH / 2.0) * (JUMP_WIDTH / 2.0))));

pub const HIT_COOLDOWN_MS: u32 = 500;

pub const PLAYER_START: Vec2<f32> = Vec2::new(40.0, 24.0);
pub const PLAYER_SIZE: Size2 = Size2::new(12, 28);
pub const PLAYER_SPRITE: SrcRect = SrcRect::new(Vec2::new(0, 0), PLAYER_SIZE);
pub const PLAYER_COLLIDER: CollisionBox = Rec2::new(Vec2::new(0.0, 0.0), PLAYER_SIZE);

// Controller //

pub struct PlayerController {
  #[allow(unused)]
  jumping: bool,
  #[allow(unused)]
  can_jump: bool,
  last_walk: Direction,
  last_aim: Direction,
  #[allow(unused)]
  dash_timer: Timer,
  dash_direction: Direction,
  locked: bool,
}

impl Default for PlayerController {
  fn default() -> Self {
    Self {
      jumping: false,
      can_jump: true,
      last_walk: INITIAL_DIRECTION,
      last_aim: INITIAL_DIRECTION,
      dash_timer: Timer::new(Duration::from_millis(200), true),
      dash_direction: Direction::Right,
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
  let PlayerQuery { velocity, controller, .. } = use_player(world);
  let aim = get_direction(event, Behaviour::Held).unwrap_or(controller.last_aim);

  // Jump //

  if is_control(Control::Select, Behaviour::Pressed, event) {
    velocity.0.y = JUMP_ACCELERATION.y;
  }

  // Walk //

  let left_held = is_control(Control::Left, Behaviour::Held, event);
  let right_held = is_control(Control::Right, Behaviour::Held, event);

  if left_held && !right_held {
    controller.set_walked(Direction::Left);
    velocity.0.x = -WALK_SPEED;
  } else if right_held && !left_held {
    controller.set_walked(Direction::Right);
    velocity.0.x = WALK_SPEED;
  } else {
    velocity.remove_x();
  }

  // Dash //

  let left_pressed = is_control(Control::Left, Behaviour::Pressed, event);
  let right_pressed = is_control(Control::Right, Behaviour::Pressed, event);
  if left_pressed ^ right_pressed {
    if controller.dash_timer.done() {
      controller.dash_timer.reset();
      controller.dash_direction = if left_pressed { Direction::Left } else { Direction::Right };
    } else if controller.dash_direction == Direction::Left && left_pressed || controller.dash_direction == Direction::Right && right_pressed {
      velocity.0.x = WALK_SPEED * DASH_MULTIPLIER * controller.dash_direction.to_coordinate().x as f32;
      controller.dash_timer.expire();
    }
  }

  // Lock //

  if is_control(Control::Lock, Behaviour::Held, event) {
    velocity.remove_x();
    controller.locked = true;
  } else {
    controller.locked = false;
  }

  // Combat //

  controller.set_aimed(aim);
  if is_control(Control::PrimaryTrigger, Behaviour::Pressed, event) {
    fire_weapon(world, aim, Weapon::Bullet);
  } else if is_control(Control::SecondaryTrigger, Behaviour::Pressed, event) {
    fire_weapon(world, aim, Weapon::Rocket);
  } else if is_control(Control::TertiaryTrigger, Behaviour::Pressed, event) {
    fire_weapon(world, aim, Weapon::IceBeam);
  }
}
use sdl2::keyboard::Keycode;

use crate::engine::geometry::shape::Vec2;
use crate::engine::system::SysArgs;
use crate::engine::utility::direction::Direction;
use crate::game::physics::gravity::Gravity;
use crate::game::player::combat::{fire_weapon, Weapon};
use crate::game::player::physics::{calculate_gravity, calculate_jump_velocity, HIGH_JUMP_BOOTS_JUMP_HEIGHT, INITIAL_JUMP_HEIGHT, INITIAL_JUMP_WIDTH, INITIAL_WALK_SPEED};
use crate::game::player::world::{PlayerQuery, use_player};
use crate::game::utility::controls::{Behaviour, Control, get_direction, is_control};

const INITIAL_DIRECTION: Direction = Direction::Right;

// Controller //

pub struct PlayerController {
  pub jump_velocity: Vec2<f32>,
  pub walk_velocity: Vec2<f32>,
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
      walk_velocity: Vec2::new(INITIAL_WALK_SPEED, 0.0),
      jump_velocity: calculate_jump_velocity(INITIAL_JUMP_HEIGHT, INITIAL_WALK_SPEED, INITIAL_JUMP_WIDTH),
      locked: false,
    }
  }
}

pub fn set_jump_height(player_controller: &mut PlayerController, player_gravity: &mut Gravity, new_height: f32) {
  player_controller.jump_velocity = calculate_jump_velocity(new_height, INITIAL_WALK_SPEED, INITIAL_JUMP_WIDTH);
  player_gravity.0 = calculate_gravity(new_height, INITIAL_WALK_SPEED, INITIAL_JUMP_WIDTH);
}

impl PlayerController {
  /// Set the last direction the player walked
  fn set_walked(&mut self, direction: Direction) { self.last_walk = direction; }
  /// Set the last direction the player aimed
  fn set_aimed(&mut self, direction: Direction) { self.last_aim = direction; }
}

pub fn sys_player_controller(SysArgs { event, world, .. }: &mut SysArgs) {
  let PlayerQuery { velocity, controller, gravity, .. } = use_player(world);
  let aim = get_direction(event, Behaviour::Held).unwrap_or(controller.last_aim);

  // Jump //

  if is_control(Control::Select, Behaviour::Pressed, event) {
    velocity.0.y = controller.jump_velocity.y
  }

  if event.is_key_held(Keycode::H) {
    set_jump_height(controller, gravity, HIGH_JUMP_BOOTS_JUMP_HEIGHT);
  }

  // Walk //

  let left_held = is_control(Control::Left, Behaviour::Held, event);
  let right_held = is_control(Control::Right, Behaviour::Held, event);

  if left_held && !right_held {
    controller.set_walked(Direction::Left);
    velocity.0.x = -controller.walk_velocity.x;
  } else if right_held && !left_held {
    controller.set_walked(Direction::Right);
    velocity.0.x = controller.walk_velocity.x;
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
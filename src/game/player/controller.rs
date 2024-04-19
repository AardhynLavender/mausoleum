use crate::engine::system::SysArgs;
use crate::engine::utility::direction::Direction;
use crate::game::constant::{JUMP_ACCELERATION, WALK_SPEED};
use crate::game::player::combat::{fire_weapon, Weapon};
use crate::game::player::world::{PQ, use_player};
use crate::game::utility::controls::{Behaviour, Control, get_direction, is_control};

const INITIAL_DIRECTION: Direction = Direction::Right;

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
  let PQ { velocity, controller, .. } = use_player(world);
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

  // Combat //

  controller.set_aimed(aim);
  if is_control(Control::PrimaryTrigger, Behaviour::Pressed, event) {
    fire_weapon(world, aim, Weapon::Bullet);
  }
  if is_control(Control::SecondaryTrigger, Behaviour::Pressed, event) {
    fire_weapon(world, aim, Weapon::Rocket);
  }
}
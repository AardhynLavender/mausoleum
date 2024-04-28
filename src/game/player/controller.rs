use crate::engine::geometry::shape::Vec2;
use crate::engine::system::SysArgs;
use crate::engine::utility::direction::Direction;
use crate::game::collectable::collectable::Collectable;
use crate::game::player::combat::{fire_weapon, HEALTH_PICKUP_INCREASE, PLAYER_BASE_HEALTH, Weapon};
use crate::game::player::physics::{calculate_gravity, calculate_jump_velocity, HIGH_JUMP_BOOTS_JUMP_HEIGHT, INITIAL_JUMP_HEIGHT, INITIAL_JUMP_WIDTH, INITIAL_WALK_SPEED};
use crate::game::player::world::{PlayerQuery, use_player};
use crate::game::scene::level::meta::CollectableType;
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

impl PlayerController {
  /// Set the last direction the player walked
  fn set_walked(&mut self, direction: Direction) { self.last_walk = direction; }
  /// Set the last direction the player aimed
  fn set_aimed(&mut self, direction: Direction) { self.last_aim = direction; }
}

/// Manage and respond to player input
pub fn sys_player_controller(SysArgs { event, world, .. }: &mut SysArgs) {
  let PlayerQuery { health, velocity, inventory, controller, gravity, .. } = use_player(world);
  let aim = get_direction(event, Behaviour::Held).unwrap_or(controller.last_aim);

  // Data //

  if is_control(Control::Inventory, Behaviour::Pressed, event) {
    println!("{:?}", inventory);
  }

  // Jump //

  if is_control(Control::Select, Behaviour::Pressed, event) {
    // todo: don't calculate gravity and velocity every time the player jumps...
    let high_jump = inventory.has(&Collectable(CollectableType::HighJump));
    let jump_height = if high_jump { HIGH_JUMP_BOOTS_JUMP_HEIGHT } else { INITIAL_JUMP_HEIGHT };
    let new_gravity = calculate_gravity(jump_height, INITIAL_WALK_SPEED, INITIAL_JUMP_WIDTH);
    let new_jump_velocity = calculate_jump_velocity(jump_height, INITIAL_WALK_SPEED, INITIAL_JUMP_WIDTH);
    velocity.0.y = new_jump_velocity.y;
    gravity.0.y = new_gravity.y;
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

  // Health //

  let player_health = PLAYER_BASE_HEALTH + inventory.count(&Collectable(CollectableType::Health)) as u32 * HEALTH_PICKUP_INCREASE;
  health.set_max(player_health as i32);

  // Combat //

  controller.set_aimed(aim);

  let has_ice_beam = inventory.has(&Collectable(CollectableType::IceBeam));
  let has_rocket = inventory.has(&Collectable(CollectableType::MissileTank));
  let primary_trigger = is_control(Control::PrimaryTrigger, Behaviour::Pressed, event);
  let secondary_trigger = is_control(Control::SecondaryTrigger, Behaviour::Pressed, event);
  let tertiary_trigger = is_control(Control::TertiaryTrigger, Behaviour::Pressed, event);

  if primary_trigger { fire_weapon(world, aim, Weapon::Bullet); }
  if secondary_trigger && has_rocket { fire_weapon(world, aim, Weapon::Rocket); }
  if tertiary_trigger && has_ice_beam { fire_weapon(world, aim, Weapon::IceBeam); }
}
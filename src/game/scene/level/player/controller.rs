/**
 * Control and manage the player entity in the world
 */

use crate::engine::ecs::system::{SysArgs, Systemize};
use crate::engine::geometry::shape::Vec2;
use crate::engine::utility::direction::Direction;
use crate::game::scene::level::player::combat::{fire_weapon, HEALTH_PICKUP_INCREASE, PLAYER_BASE_HEALTH, Weapon};
use crate::game::scene::level::player::physics::{calculate_gravity, calculate_jump_velocity, HIGH_JUMP_BOOTS_JUMP_HEIGHT, INITIAL_JUMP_HEIGHT, INITIAL_JUMP_WIDTH, INITIAL_WALK_SPEED};
use crate::game::scene::level::player::world::{PlayerQuery, use_player};
use crate::game::scene::level::room::meta::Collectable;
use crate::game::utility::controls::{Behaviour, Control, get_controls_direction, is_control};

const INITIAL_DIRECTION: Direction = Direction::Right;

// Controller //

pub struct PlayerController {
  pub jump_velocity: Vec2<f32>,
  pub walk_velocity: Vec2<f32>,
  last_walk: Direction,
  last_aim: Direction,
  jump_start: f32,
  jumping: bool,
  locked: bool,
}

impl Default for PlayerController {
  fn default() -> Self {
    Self {
      last_walk: INITIAL_DIRECTION,
      last_aim: INITIAL_DIRECTION,
      jump_start: 0.0,
      jumping: false,
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

impl Systemize for PlayerController {
  /// Process user controls each frame
  fn system(SysArgs { delta, event, world, .. }: &mut SysArgs) -> Result<(), String> {
    let PlayerQuery { health, velocity, inventory, controller, gravity, .. } = use_player(world);
    let aim = get_controls_direction(event, Behaviour::Held).unwrap_or(controller.last_aim);

    /*
      todo: cast downward rays of `jump_widow` length downwards to check if the player is on the floor
    */
    let on_floor = velocity.0.y == 0.0;
    let jump_key = is_control(Control::Select, Behaviour::Pressed, event);
    let jump_held = is_control(Control::Select, Behaviour::Held, event);

    if on_floor {
      if jump_key {
        controller.jumping = true;
        controller.jump_start = *delta;
        let high_jump = inventory.has(&Collectable::HighJump);
        let jump_height = if high_jump { HIGH_JUMP_BOOTS_JUMP_HEIGHT } else { INITIAL_JUMP_HEIGHT };
        let new_gravity = calculate_gravity(jump_height, INITIAL_WALK_SPEED, INITIAL_JUMP_WIDTH);
        let new_jump_velocity = calculate_jump_velocity(jump_height, INITIAL_WALK_SPEED, INITIAL_JUMP_WIDTH);
        velocity.0.y = new_jump_velocity.y;
        gravity.0.y = new_gravity.y;
      } else {
        controller.jumping = false;
      }
    }

    if controller.jumping && velocity.is_going_up() && !jump_held {
      controller.jumping = false;
      velocity.0.y = velocity.0.y * 0.35;
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

    let player_health = PLAYER_BASE_HEALTH + inventory.count(&Collectable::Health) as u32 * HEALTH_PICKUP_INCREASE;
    if health.get_max() != player_health { health.set_max(player_health); }

    // Combat //

    controller.set_aimed(aim);

    let has_ice_beam = inventory.has(&Collectable::IceBeam);
    let has_rocket = inventory.has(&Collectable::MissileTank);
    let primary_trigger = is_control(Control::PrimaryTrigger, Behaviour::Pressed, event);
    let secondary_trigger = is_control(Control::SecondaryTrigger, Behaviour::Pressed, event);
    let tertiary_trigger = is_control(Control::TertiaryTrigger, Behaviour::Pressed, event);

    if primary_trigger { fire_weapon(world, aim, Weapon::Bullet); }
    if secondary_trigger && has_rocket { fire_weapon(world, aim, Weapon::Rocket); }
    if tertiary_trigger && has_ice_beam { fire_weapon(world, aim, Weapon::IceBeam); }

    Ok(())
  }
}

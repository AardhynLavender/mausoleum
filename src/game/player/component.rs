use std::time::Duration;

use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::rendering::color::{OPAQUE, RGBA};
use crate::engine::system::SysArgs;
use crate::engine::time::Timer;
use crate::engine::utility::alias::Size2;
use crate::game::constant::{JUMP_ACCELERATION, WALK_SPEED};
use crate::game::physics::velocity::Velocity;
use crate::game::player::world::use_player;
use crate::game::utility::controls::{Behaviour, Control, is_control};

/**
 * Player controls
 */

// data //

pub struct PlayerData {
  pub hit_cooldown: Timer,
}

impl PlayerData {
  pub fn new() -> Self {
    Self {
      hit_cooldown: Timer::new(Duration::from_millis(500), true),
    }
  }
}

pub fn sys_render_cooldown(SysArgs { world, render, .. }: &mut SysArgs) {
  let (player_data, position, ..) = use_player(world);
  if !player_data.hit_cooldown.done() {
    render.draw_rect(Rec2::new(Vec2::<i32>::from(position.0) - 2, Size2::new(16, 32)), RGBA::new(255, 0, 255, OPAQUE));
  }
}

// controller //

pub struct PlayerController {
  #[allow(unused)]
  jumping: bool,
  #[allow(unused)]
  can_jump: bool,
}

impl Default for PlayerController {
  fn default() -> Self {
    Self {
      jumping: false,
      can_jump: true,
    }
  }
}

impl PlayerController {
  #[allow(unused)]
  pub fn jump(&mut self) {
    self.jumping = true;
  }
}

pub fn sys_player_controller(SysArgs { event, world, .. }: &mut SysArgs) {
  for (_, (_player, velocity)) in world.query::<(&mut PlayerController, &mut Velocity)>() {
    if is_control(Control::Select, Behaviour::Pressed, event) {
      velocity.0.y = JUMP_ACCELERATION.y
    }

    if is_control(Control::Left, Behaviour::Held, event) {
      velocity.0.x = WALK_SPEED;
    } else if is_control(Control::Right, Behaviour::Held, event) {
      velocity.0.x = -WALK_SPEED;
    } else {
      velocity.0.x = 0.0;
    }
  };
}

use crate::engine::system::SysArgs;
use crate::game::constant::{JUMP_ACCELERATION, WALK_SPEED};
use crate::game::physics::velocity::Velocity;
use crate::game::utility::controls::{Behaviour, Control, is_control};

/**
 * Player controls
 */

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

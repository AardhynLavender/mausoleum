use crate::engine::system::SysArgs;
use crate::game::component::physics::Gravity;
use crate::game::component::position::Position;

pub type QueryGravity = (&'static mut Position, &'static mut Gravity);

pub fn sys_gravity(SysArgs { delta, world, .. }: &mut SysArgs) {
  for (_, (position, gravity)) in world.query::<QueryGravity>() {
    position.0.y += gravity.0.y * *delta;
    position.0.x += gravity.0.x * *delta;
  }
}


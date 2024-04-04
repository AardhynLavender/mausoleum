use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::rendering::color::{OPAQUE, RGBA};
use crate::engine::system::SysArgs;
use crate::engine::utility::alias::Size;
use crate::game::physics::position::Position;
use crate::game::utility::controls::{Behaviour, Control, is_control};

/**
 * Collider component
 */

/// Add a collision box to an entity
pub struct Collider(pub Rec2<f32, Size>);

impl Collider {
  /// Instantiate a new Collider component
  pub fn new(bounds: Rec2<f32, Size>) -> Self {
    Self(bounds)
  }
}

/// Render colliders in the world while debugging
pub fn sys_render_colliders(SysArgs { world, render, event, .. }: &mut SysArgs) {
  if !is_control(Control::Debug, Behaviour::Held, event) {
    return;
  }

  for (_, (position, collider)) in world.query::<(&Position, &Collider)>() {
    render.draw_rect(
      Rec2::<i32, Size>::new(Vec2::new(position.0.x as i32, position.0.y as i32), collider.0.size),
      RGBA::new(0, 255, 0, OPAQUE),
    );
  }
}
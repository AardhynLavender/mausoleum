use crate::engine::geometry::collision::CollisionBox;
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
pub struct Collider(pub CollisionBox);

impl Collider {
  /// Instantiate a new Collider component
  pub fn new(bounds: CollisionBox) -> Self {
    Self(bounds)
  }
}

/// Render colliders in the world while debugging
pub fn sys_render_colliders(SysArgs { world, camera, render, event, .. }: &mut SysArgs) {
  if !is_control(Control::Debug, Behaviour::Held, event) {
    return;
  }

  for (_, (position, collider)) in world.query::<(&Position, &Collider)>() {
    let new_position = camera.translate(Vec2::from(position.0));
    render.draw_rect(
      Rec2::<i32, Size>::new(new_position, collider.0.size),
      RGBA::new(0, 255, 0, OPAQUE),
    );
  }
}
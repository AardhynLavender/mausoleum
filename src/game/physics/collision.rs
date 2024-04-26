use crate::engine::geometry::collision::CollisionBox;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::rendering::color::{OPAQUE, RGBA};
use crate::engine::system::SysArgs;
use crate::engine::utility::alias::Size;
use crate::game::physics::position::Position;
use crate::game::preferences::use_preferences;

/**
 * Collider component
 */

/// Add a collision box to an entity
#[derive(Debug, Clone, Copy)]
pub struct Collider(pub CollisionBox);

/// *Fragile* entities should be destroyed on collision
#[derive(Debug, Clone, Copy)]
pub struct Fragile;

impl Collider {
  /// Instantiate a new Collider component
  pub fn new(bounds: CollisionBox) -> Self {
    Self(bounds)
  }
}

/// Render colliders in the world while debugging
pub fn sys_render_colliders(SysArgs { world, camera, render, state, .. }: &mut SysArgs) {
  if !use_preferences(state).debug { return; }

  for (_, (position, collider)) in world.query::<(&Position, &Collider)>() {
    let new_position = camera.translate(Vec2::from(position.0));
    render.draw_rect(
      Rec2::<i32, Size>::new(new_position, collider.0.size),
      RGBA::new(0, 255, 0, OPAQUE),
    );
  }
}

/// Create a worldspace collision box from a position and collider
pub fn make_collision_box(position: &Position, collider: &Collider) -> CollisionBox {
  CollisionBox::new(position.0 + collider.0.origin, collider.0.size)
}

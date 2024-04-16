use std::time::Duration;

use hecs::{Component, DynamicBundle};

use crate::engine::geometry::collision::CollisionBox;
use crate::engine::geometry::shape::Vec2;
use crate::engine::rendering::component::Sprite;
use crate::engine::rendering::renderer::layer;
use crate::engine::system::SysArgs;
use crate::engine::time::Timer;
use crate::game::combat::damage::Damage;
use crate::game::physics::collision::{Collider, Fragile};
use crate::game::physics::position::Position;
use crate::game::physics::velocity::Velocity;
use crate::game::scene::level::collision::RoomCollision;

pub type ProjectileLayer = layer::Layer8;

/// Define the lifetime of an entity in milliseconds
pub struct TimeToLive(pub Timer);

impl TimeToLive {
  /// Instance a new lifetime
  pub fn new(ttl_ms: u64) -> Self {
    Self(Timer::new(Duration::from_millis(ttl_ms), true))
  }
}

/// Assemble the components for a projectile entity with a unique component `T`
pub fn make_projectile<T>(damage: u32, size: CollisionBox, spawn: Vec2<f32>, velocity: Vec2<f32>, sprite: Sprite, ttl: u64) -> impl DynamicBundle where T: Component + Default {
  (
    T::default(),
    sprite,
    Position(spawn),
    Damage::new(damage),
    ProjectileLayer::default(),
    Velocity(velocity),
    Collider::new(size),
    RoomCollision,
    Fragile,
    TimeToLive::new(ttl),
  )
}

/// Handle the cleanup of timed lifetime entities
pub fn sys_ttl(SysArgs { world, .. }: &mut SysArgs) {
  let to_free: Vec<_> = world
    .query::<&TimeToLive>()
    .into_iter()
    .filter(|(_, ttl)| ttl.0.done())
    .map(|(entity, _)| entity)
    .collect();
  for entity in to_free {
    world
      .free_now(entity)
      .expect("Failed to free entity")
  }
}

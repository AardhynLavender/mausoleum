/**
  * Collision system for entities and collectables
  */

use std::collections::HashMap;

use hecs::Entity;
use crate::engine::component::position::Position;
use crate::engine::ecs::system::{SysArgs, Systemize};
use crate::engine::ecs::world::World;

use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::utility::alias::Size;
use crate::engine::utility::color::{OPAQUE, RGBA};
use crate::engine::utility::direction::Direction;
use crate::game::preferences::use_preferences;
use crate::game::scene::level::physics::collision::{Collider, Fragile, make_collision_box};
use crate::game::scene::level::physics::frozen::Frozen;
use crate::game::scene::level::physics::velocity::Velocity;
use crate::game::scene::level::player::combat::{Bullet, Rocket};
use crate::game::scene::level::room::meta::{Soft, Strong, TileLayerType};
use crate::game::scene::level::room::room::use_room;
use crate::game::scene::level::tile::query::{TileHandle, TileQuery};
use crate::game::scene::level::tile::tile::TileCollider;
use crate::game::scene::level::tile::tilemap::TilemapMutation;

/// The maximum value of a resolvable collision
///
/// Essentially marking a resolution as impossible
pub const RESOLVABLE_INFINITY: f32 = f32::INFINITY;

/// Describes the presentation sides of a collision that are resolvable
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct CollisionMask {
  pub top: bool,
  pub bottom: bool,
  pub left: bool,
  pub right: bool,
}

impl CollisionMask {
  /// Instantiate a new collision mask
  pub const fn new(top: bool, right: bool, bottom: bool, left: bool) -> Self {
    Self {
      top,
      right,
      bottom,
      left,
    }
  }
  /// Instantiate a new full collision mask
  pub const fn full() -> Self {
    Self { top: true, right: true, bottom: true, left: true }
  }
  /// Check if the collision mask is empty
  pub const fn is_empty(&self) -> bool {
    !self.top && !self.right && !self.bottom && !self.left
  }
  /// Clear a specific side of the mask
  pub fn set_side(&mut self, side: Direction, value: bool) -> Result<(), String> {
    match side {
      Direction::Up => self.top = value,
      Direction::Right => self.right = value,
      Direction::Down => self.bottom = value,
      Direction::Left => self.left = value,
      _ => return Err(String::from("Ordinal direction is not supported for collision masks"))
    }
    Ok(())
  }
}

/// Describes a simple collision in terms of relative side penetration
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Collision {
  mask: CollisionMask,
  pub top: f32,
  pub bottom: f32,
  pub left: f32,
  pub right: f32,
}

impl Collision {
  /// Build a collision from the penetration values of the sides.
  ///
  /// Returns an error if no sides are penetrated
  pub fn build(mask: CollisionMask, top: f32, right: f32, bottom: f32, left: f32) -> Result<Self, String> {
    if top == 0.0
      && right == 0.0
      && bottom == 0.0
      && left == 0.0
    {
      return Err(String::from("No collision"));
    }

    Ok(Self {
      mask,
      top,
      right,
      bottom,
      left,
    })
  }
  /// Get the shortest side of the collision
  pub fn get_resolution(&self) -> Vec2<f32> {
    // apply the collision mask to the penetration values
    let sides = [
      if self.mask.top { self.top } else { RESOLVABLE_INFINITY },
      if self.mask.right { self.right } else { RESOLVABLE_INFINITY },
      if self.mask.bottom { self.bottom } else { RESOLVABLE_INFINITY },
      if self.mask.left { self.left } else { RESOLVABLE_INFINITY },
    ];
    // get the index of the shortest absolute side
    let index = sides
      .iter()
      .enumerate()
      .fold(0, |cur_idx, (i, &pen)| {
        if pen.abs() < sides[cur_idx].abs() { i } else { cur_idx }
      });
    // return a resolution for the shortest side
    let resolutions = vec![
      Vec2::new(0.0, self.top),
      Vec2::new(self.right, 0.0),
      Vec2::new(0.0, self.bottom),
      Vec2::new(self.left, 0.0),
    ];
    let resolution = resolutions[index];
    resolution
  }
}

/// A rectangle used for collision detection and resolution
pub type CollisionBox = Rec2<f32, Size>;

/// Check if two rectangles are colliding based on a mask and return the collision data
pub fn rec2_collision(r1: &CollisionBox, r2: &CollisionBox, mask: CollisionMask) -> Option<Collision> {
  let ((x1, y1), (w1, h1)) = r1.destructure();
  let ((x2, y2), (w2, h2)) = r2.destructure();
  if x1 < x2 + w2 as f32
    && x1 + w1 as f32 > x2
    && y1 < y2 + h2 as f32
    && y1 + h1 as f32 > y2
  {
    Collision::build(
      mask,
      (y1 + h1 as f32) - y2,
      x1 - (x2 + w2 as f32),
      y1 - (y2 + h2 as f32),
      (x1 + w1 as f32) - x2,
    ).ok()
  } else {
    None
  }
}

/// Maximum number of collision resolution attempts before ~~panicking~~
pub const MAX_COLLISION_PHASES: u32 = 10;

/// Collision layers for entities
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum RoomCollision {
  #[default]
  /// collides with tiles
  All,
  /// Collides with only creatures
  Creature,
  /// Only collides with the player
  Player,
}

/// Resolve tile collisions for entities collideable with rooms tiles
impl Systemize for RoomCollision {
  fn system(SysArgs { world, state, .. }: &mut SysArgs) -> Result<(), String> {
    let colliders = world
      .query::<(&Position, &Collider, &RoomCollision)>()
      .without::<&Frozen>()
      .into_iter()
      .map(|(entity, (position, collider, layer))| {
        (entity, (*position, *collider, *layer))
      })
      .collect::<HashMap<_, _>>();

    let room = use_room(state);

    for (entity, (position, collider, layer)) in &colliders {
      let mut collision_box = make_collision_box(position, collider);
      let mut phase = 0;
      'resolving: loop {
        phase += 1;
        let collisions = get_tile_collisions(world, &collision_box, layer);
        let collision = get_closest_collision(collisions);
        if let Some((tile, collision, _, position)) = collision {
          if phase > MAX_COLLISION_PHASES {
            // return Err(String::from("Infinite collision resolution loop detected"));
            return Ok(());
          }

          let brittle = world.has_component::<Fragile>(tile)?;
          let strong = world.has_component::<Strong>(tile)?;
          let soft = world.has_component::<Soft>(tile)?;
          let bullet = world.has_component::<Bullet>(*entity)?;
          let rocket = world.has_component::<Rocket>(*entity)?;
          if brittle || strong && rocket || soft && (rocket || bullet) {
            let result = room.query_tile(TileLayerType::Collision, TileQuery::Position(position.0));
            if let Ok(handle) = TileHandle::try_from(result) {
              room.remove_tile(world, handle, TilemapMutation::Session);
            } else {
              return Err(String::from("Failed to remove tile"));
            }
          }

          let fragile = world.has_component::<Fragile>(*entity)?;
          if fragile {
            world.free_now(*entity)?;
            break 'resolving;
          }

          let mut position = world.get_component_mut::<Position>(*entity)?;
          let mut velocity = world.get_component_mut::<Velocity>(*entity)?;
          let resolution = collision.get_resolution();
          position.0 = position.0 - resolution;
          if resolution.y > 0.0 && velocity.0.y > 0.0 {
            // cut vertical acceleration if resolving up while falling
            // eg: landing on a platform
            position.0.y = position.0.y.round();
            velocity.0.y = 0.0;
          } else if resolution.y < 0.0 && velocity.0.y < 0.0 {
            // cut vertical acceleration if resolving down while jumping
            // eg: hitting head on a platform
            position.0.y = position.0.y.round();
            velocity.0.y = 0.0;
          } else if resolution.x != 0.0 {
            // cut horizontal acceleration if resolving left or right
            // eg: hitting a wall
            position.0.x = position.0.x.round();
            velocity.0.x = 0.0;
          }

          collision_box = make_collision_box(&position, collider); // update the collision box with the new position
        } else {
          break 'resolving;
        }
      };
    }

    Ok(())
  }
}

/// A bundle of entities and their collision data
pub type TileCollisionBundle = (Entity, Collision, CollisionBox, Position);

/// Get all tile collisions for a given collision box
fn get_tile_collisions<'a>(world: &'a mut World, collider_box: &'a CollisionBox, layer: &'a RoomCollision) -> impl Iterator<Item=TileCollisionBundle> + 'a {
  world.query::<(&Position, &TileCollider, &RoomCollision)>()
    .into_iter()
    .filter_map(|(entity, (tile_position, tile_collider, tile_collision_layer))| {
      let tile_box = &CollisionBox::new(tile_position.0 + tile_collider.collision_box.origin, tile_collider.collision_box.size);
      let collision = rec2_collision(collider_box, tile_box, tile_collider.mask);

      if let Some(collision) = collision {
        if *tile_collision_layer == RoomCollision::All || *layer == *tile_collision_layer {
          return Some((entity, collision, *tile_box, *tile_position));
        }
      }

      None
    })
}

/// Get the closest collision from a list of tile collisions
fn get_closest_collision<'a>(collisions: impl Iterator<Item=TileCollisionBundle> + 'a) -> Option<TileCollisionBundle> {
  collisions
    .fold(None, |closest, current| {
      match (closest, current) {
        (None, _) => Some(current),
        (Some((_, closest_collision, ..)), (_, current_collision, ..)) => {
          if current_collision.get_resolution().abs() < closest_collision.get_resolution().abs() {
            Some(current)
          } else {
            Some(closest.unwrap_or(current))
          }
        }
      }
    })
}

/// Render the tile colliders to the screen when debug mode is active
pub fn sys_render_tile_colliders(SysArgs { world, camera, render, state, .. }: &mut SysArgs) -> Result<(), String> {
  if !use_preferences(state).debug { return Ok(()); }

  for (_, (position, collider)) in world.query::<(&Position, &TileCollider)>() {
    let color = RGBA::new(255, 0, 0, OPAQUE);
    let (width, height) = collider.collision_box.size.destructure();
    let p = camera.translate(Vec2::from(position.0 + collider.collision_box.origin));

    if collider.mask.top { render.draw_line(p, p + Vec2::new(width as i32 - 1, 0), color); }
    if collider.mask.right { render.draw_line(p + Vec2::new(width as i32 - 1, 0), p + Vec2::new(width as i32 - 1, height as i32 - 1), color); }
    if collider.mask.bottom { render.draw_line(p + Vec2::new(0, height as i32 - 1), p + Vec2::new(width as i32 - 1, height as i32 - 1), color); }
    if collider.mask.left { render.draw_line(p, p + Vec2::new(0, height as i32 - 1), color); }
  }

  Ok(())
}

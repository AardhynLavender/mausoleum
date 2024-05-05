/*
* Collision system for entities with room tiles
*/

use std::collections::HashMap;

use hecs::Entity;

use crate::engine::geometry::collision::{Collision, CollisionBox, rec2_collision};
use crate::engine::geometry::shape::Vec2;
use crate::engine::rendering::color::{OPAQUE, RGBA};
use crate::engine::system::SysArgs;
use crate::engine::tile::query::{TileHandle, TileQuery};
use crate::engine::tile::tile::TileCollider;
use crate::engine::tile::tilemap::TilemapMutation;
use crate::engine::world::World;
use crate::game::physics::collision::{Collider, Fragile, make_collision_box};
use crate::game::physics::frozen::Frozen;
use crate::game::physics::position::Position;
use crate::game::physics::velocity::Velocity;
use crate::game::player::combat::{Bullet, Rocket};
use crate::game::preferences::use_preferences;
use crate::game::scene::level::meta::{Soft, Strong, TileLayerType};
use crate::game::scene::level::room::use_room;

/// Maximum number of collision resolution attempts before ~~panicking~~
pub const MAX_COLLISION_PHASES: u32 = 10;

/// Entities with this component will collide with room tiles and be resolved
#[derive(Default)]
pub struct RoomCollision;

/// Resolve tile collisions for entities collideable with rooms tiles
pub fn sys_tile_collision(SysArgs { world, state, .. }: &mut SysArgs) {
  let colliders = world
    .query::<(&Position, &Collider)>()
    .with::<&RoomCollision>()
    .without::<&Frozen>()
    .into_iter()
    .map(|(entity, (position, collider))| {
      (entity, (*position, *collider))
    })
    .collect::<HashMap<_, _>>();

  let room = use_room(state);

  for (entity, (position, collider)) in &colliders {
    let mut collision_box = make_collision_box(position, collider);
    let mut phase = 0;
    'resolving: loop {
      phase += 1;
      let collision = get_tile_collisions(world, &collision_box).next();
      if let Some((tile, collision, position)) = collision {
        if phase > MAX_COLLISION_PHASES {
          // eprintln!("Infinite collision resolution loop detected, what do?");
          return;
        }

        let strong = world.has_component::<Strong>(tile).expect("Failed to retrieve the entity");
        let soft = world.has_component::<Soft>(tile).expect("Failed to retrieve the entity");
        let bullet = world.has_component::<Bullet>(*entity).expect("Failed to retrieve the entity");
        let rocket = world.has_component::<Rocket>(*entity).expect("Failed to retrieve the entity");
        if strong && rocket || (soft && (rocket || bullet)) {
          let result = room.query_tile(TileLayerType::Collision, TileQuery::Position(position.0));
          if let Ok(handle) = TileHandle::try_from(result) {
            room.remove_tile(world, handle, TilemapMutation::Session);
          } else {
            panic!("No concept associated with destroyed tile");
          }
        }

        let fragile = world.has_component::<Fragile>(*entity).expect("Failed to retrieve the entity");
        if fragile {
          world.free_now(*entity).expect("Failed to free entity");
          break 'resolving;
        }

        let mut position = world.get_component_mut::<Position>(*entity).expect("Failed to retrieve entity");
        let mut velocity = world.get_component_mut::<Velocity>(*entity).expect("Failed to retrieve entity");
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
}

/// Get all tile collisions for a given collision box
fn get_tile_collisions<'a>(world: &'a mut World, collider_box: &'a CollisionBox) -> impl Iterator<Item=(Entity, Collision, Position)> + 'a {
  world.query::<(&Position, &TileCollider)>()
    .into_iter()
    .filter_map(|(entity, (tile_position, tile_collider, ..))| {
      let tile_rect = &CollisionBox::new(tile_position.0 + tile_collider.collision_box.origin, tile_collider.collision_box.size);
      let collision = rec2_collision(collider_box, tile_rect, tile_collider.mask);
      if let Some(collision) = collision { Some((entity, collision, *tile_position)) } else { None }
    })
}

/// Render the tile colliders to the screen when debug mode is active
pub fn sys_render_tile_colliders(SysArgs { world, camera, render, state, .. }: &mut SysArgs) {
  if !use_preferences(state).debug { return; }

  for (_, (position, collider)) in world.query::<(&Position, &TileCollider)>() {
    let color = RGBA::new(255, 0, 0, OPAQUE);
    let (width, height) = collider.collision_box.size.destructure();
    let p = camera.translate(Vec2::from(position.0 + collider.collision_box.origin));

    if collider.mask.top { render.draw_line(p, p + Vec2::new(width as i32 - 1, 0), color); }
    if collider.mask.right { render.draw_line(p + Vec2::new(width as i32 - 1, 0), p + Vec2::new(width as i32 - 1, height as i32 - 1), color); }
    if collider.mask.bottom { render.draw_line(p + Vec2::new(0, height as i32 - 1), p + Vec2::new(width as i32 - 1, height as i32 - 1), color); }
    if collider.mask.left { render.draw_line(p, p + Vec2::new(0, height as i32 - 1), color); }
  }
}

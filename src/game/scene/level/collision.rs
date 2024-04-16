use std::collections::HashMap;

use crate::engine::geometry::collision::{Collision, CollisionBox, rec2_collision};
use crate::engine::system::SysArgs;
use crate::engine::tile::tile::{Tile, TileCollider};
use crate::engine::world::World;
use crate::game::physics::collision::{Collider, Fragile};
use crate::game::physics::position::Position;
use crate::game::physics::velocity::Velocity;

/// Maximum number of collision resolution attempts before panicking
pub const MAX_COLLISION_PHASES: u32 = 10;

/// Entities with this component will collide with room tiles and be resolved
#[derive(Default)]
pub struct RoomCollision;

/// Resolve tile collisions for entities clickable with rooms tiles
pub fn sys_tile_collision(SysArgs { world, .. }: &mut SysArgs) {
  let entities = world
    .query::<(&Position, &Collider, &RoomCollision)>().with::<&RoomCollision>()
    .into_iter()
    .map(|(entity, (position, collider, ..))| {
      (entity, (*position, *collider))
    })
    .collect::<HashMap<_, _>>();

  for (entity, (position, collider)) in &entities {
    let mut collision_box = make_collision_box(position, collider);
    let mut phase = 0;
    'resolving: loop {
      phase += 1;
      let collision = get_tile_collisions(world, &collision_box).next();
      if let Some(collision) = collision {
        if phase > MAX_COLLISION_PHASES { panic!("Infinite collision resolution loop detected, what do?"); }

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
fn get_tile_collisions<'a>(world: &'a mut World, collider_box: &'a CollisionBox) -> impl Iterator<Item=Collision> + 'a {
  world.query::<(&Position, &TileCollider, &Tile)>()
    .into_iter()
    .filter_map(|(_, (tile_position, tile_collider, ..))| {
      let tile_rect = &CollisionBox::new(tile_position.0 + tile_collider.collision_box.origin, tile_collider.collision_box.size);
      rec2_collision(collider_box, tile_rect, tile_collider.mask)
    })
}

/// Create a worldspace collision box from a position and collider
pub fn make_collision_box(position: &Position, collider: &Collider) -> CollisionBox {
  CollisionBox::new(position.0 + collider.0.origin, collider.0.size)
}


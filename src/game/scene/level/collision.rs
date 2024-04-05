use crate::engine::geometry::collision::{Collision, CollisionBox, rec2_collision};
use crate::engine::geometry::shape::Rec2;
use crate::engine::system::SysArgs;
use crate::engine::tile::tile::{Tile, TileCollider};
use crate::engine::world::World;
use crate::game::physics::position::Position;
use crate::game::player::world::use_player;

pub fn sys_tile_collision(SysArgs { world, .. }: &mut SysArgs) {
  let mut phase = 0;
  'resolving: loop {
    phase += 1;
    let (position, _, collider, ..) = use_player(world);
    let player_rect = Rec2::new(position.0 + collider.0.origin, collider.0.size);

    let collision = get_tile_collisions(world, &player_rect).next();
    if let Some(collision) = collision {
      if phase > 10 {
        panic!("Infinite collision resolution loop detected, what do?");
      }

      let (position, v, ..) = use_player(world);
      let resolution = collision.get_resolution();

      position.0 = position.0 - resolution;
      if resolution.y > 0.0 && v.0.y < 0.0 {
        // cut vertical acceleration if resolving up while falling
        // eg: landing on a platform
        position.0.y = position.0.y.round();
        v.0.y = 0.0;
      } else if resolution.y < 0.0 && v.0.y > 0.0 {
        // cut vertical acceleration if resolving down while jumping
        // eg: hitting head on a platform
        position.0.y = position.0.y.round();
        v.0.y = 0.0;
      } else if resolution.x != 0.0 {
        // cut horizontal acceleration if resolving left or right
        // eg: hitting a wall
        position.0.x = position.0.x.round();
        v.0.x = 0.0;
      }
    } else {
      break 'resolving;
    }
  };
}

fn get_tile_collisions<'a>(world: &'a mut World, collider_box: &'a CollisionBox) -> impl Iterator<Item=Collision> + 'a {
  world.query::<(&Position, &TileCollider, &Tile)>()
    .into_iter()
    .filter_map(|(_, (tile_position, tile_collider, ..))| {
      let tile_rect = &CollisionBox::new(tile_position.0 + tile_collider.collision_box.origin, tile_collider.collision_box.size);
      rec2_collision(collider_box, tile_rect, tile_collider.mask)
    })
}

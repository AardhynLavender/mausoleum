/**
 * Ripper
 * A small, fast enemy that moves horizontally and bounces off walls
 */

use std::collections::HashMap;
use std::path::Path;

use hecs::DynamicBundle;

use crate::engine::asset::AssetManager;
use crate::engine::geometry::collision::{CollisionBox, CollisionMask, rec2_collision};
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::rendering::component::Sprite;
use crate::engine::system::SysArgs;
use crate::engine::tile::tile::TileCollider;
use crate::engine::utility::alias::Size2;
use crate::engine::utility::direction::Direction;
use crate::game::combat::damage::Damage;
use crate::game::creature::CreatureLayer;
use crate::game::physics::collision::Collider;
use crate::game::physics::position::Position;
use crate::game::physics::velocity::Velocity;

const RIPPER_SPEED: f32 = 128.0;
const RIPPER_ASSET: &str = "asset/ripper.png";
const DIMENSIONS: Size2 = Size2::new(16, 8);

#[derive(Default)]
pub struct Ripper;

/// Add a `Ripper` to the world
pub fn make_ripper(asset_manager: &mut AssetManager, position: Vec2<f32>, initial_direction: Direction) -> Result<impl DynamicBundle, String> {
  if initial_direction != Direction::Left && initial_direction != Direction::Right {
    return Err(String::from("Ripper must be initialized with a horizontal direction"));
  }

  let ripper = asset_manager.texture.load(Path::new(RIPPER_ASSET))?;

  Ok((
    Ripper::default(),
    Sprite::new(ripper, Rec2::new(Vec2::default(), DIMENSIONS)),
    Position(position),
    Velocity::from(Vec2::<f32>::from(initial_direction.to_coordinate()) * RIPPER_SPEED),
    Collider::new(CollisionBox::new(Vec2::default(), DIMENSIONS)),
    CreatureLayer::default(),
    Damage::new(10),
  ))
}

/// Process Ripper logic each frame
pub fn sys_ripper(SysArgs { world, .. }: &mut SysArgs) {
  // to prevent borrowing issues, we copy information about Rippers into this vector before checking
  // tile collisions. We also collect the new velocities for each ripper in a hashmap so we can
  // update them after the collision query stops borrowing `world`
  // todo: is there a better way to do this? Seems pretty inefficient...

  let rippers = world.query::<(&Ripper, &Velocity, &Position, &Collider)>()
    .into_iter()
    .map(|(e, (.., velocity, position, collider))| (e, *velocity, *position, *collider))
    .collect::<Vec<_>>();
  if rippers.is_empty() { return; }

  // tiles
  let mut collisions = HashMap::new();
  for (_, (tile_collider, position)) in world.query::<(&TileCollider, &Position)>().into_iter() {
    let tile_box = CollisionBox::new(position.0 + tile_collider.collision_box.origin, tile_collider.collision_box.size);
    for (ripper, velocity, position, collider) in &rippers {
      let ripper_box = CollisionBox::new(collider.0.origin + position.0, collider.0.size);
      if let Some(collision) = rec2_collision(&tile_box, &ripper_box, CollisionMask::new(false, true, false, true)) {
        let mut new_velocity = velocity.0.clone();
        new_velocity.invert();
        let resolution = collision.get_resolution();
        collisions.insert(ripper, (new_velocity, resolution));
      }
    }
  }
  if collisions.is_empty() { return; }
  for (ripper, (new_velocity, resolution)) in collisions {
    world
      .get_component_mut::<Velocity>(*ripper)
      .expect("failed to find ripper velocity")
      .0 = new_velocity;
    let mut position = world
      .get_component_mut::<Position>(*ripper)
      .expect("failed to find ripper position");
    position.0 = position.0 + resolution;
  }
}
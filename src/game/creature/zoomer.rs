#![allow(unused)]

use std::path::Path;

use hecs::DynamicBundle;

/**
 * Zoomer
 * Small, zoomer creature that alternates across a floor
 */

use crate::engine::asset::AssetManager;
use crate::engine::geometry::collision::CollisionBox;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::rendering::color::{OPAQUE, RGBA};
use crate::engine::rendering::component::Sprite;
use crate::engine::system::SysArgs;
use crate::engine::tile::tilemap::TileQuery;
use crate::engine::utility::alias::{Coordinate, Size2};
use crate::engine::utility::direction::{Direction, QUARTER_ROTATION, Rotation};
use crate::game::combat::damage::Damage;
use crate::game::constant::TILE_SIZE;
use crate::game::creature::CreatureLayer;
use crate::game::physics::collision::Collider;
use crate::game::physics::gravity::Gravity;
use crate::game::physics::position::Position;
use crate::game::physics::velocity::Velocity;
use crate::game::room::{Room, use_room};
use crate::game::utility::controls::{Behaviour, Control, is_control};

const ZOOMER_SPEED: f32 = 48.0;
const ZOOMER_ASSET: &str = "asset/zoomer.png";
const DIMENSIONS: Size2 = Size2::new(16, 16);
const SIZE: Vec2<f32> = Vec2::new(DIMENSIONS.x as f32, DIMENSIONS.y as f32);

// Zoomer component
pub struct Zoomer {
  pub rotation: Rotation,
  pub last_cling: Option<Coordinate>,
  pub last_lead: Option<Coordinate>,
  pub turning: bool,
}

/// Add a Zoomer to the world
pub fn make_zoomer(asset_manager: &mut AssetManager, position: Vec2<f32>, initial_direction: Direction) -> Result<impl DynamicBundle, String> {
  if initial_direction.is_ordinal() { return Err(String::from("Zoomer must be initialized with an ordinal direction")); }
  let zoomer = asset_manager.texture.load(Path::new(ZOOMER_ASSET))?;
  Ok((
    Zoomer { rotation: Rotation::Right, last_cling: None, last_lead: None, turning: false },
    Sprite::new(zoomer, Rec2::new(Vec2::default(), DIMENSIONS)),
    Position(position),
    Gravity::new(Vec2::new(0.0, 0.0)),
    Velocity::from(Vec2::<f32>::from(initial_direction.to_coordinate()) * ZOOMER_SPEED),
    Collider::new(CollisionBox::new(Vec2::default(), DIMENSIONS)),
    CreatureLayer::default(),
    Damage::new(10),
  ))
}

/// Process Zoomer pathfinding and debug rendering
pub fn sys_zoomer(SysArgs { world, render, state, camera, event, delta, .. }: &mut SysArgs) {
  let room = use_room(state);
  let debug = is_control(Control::Debug, Behaviour::Held, event);
  for (_, (zoomer, velocity, position)) in world
    .query::<(&mut Zoomer, &mut Velocity, &mut Position)>()
  {
    let direction = Direction::try_from(velocity.0).expect("Zoomer must have velocity!");

    let (leading_coordinate, leading_position) = compute_leading(zoomer, room, direction, position, velocity);
    if debug { render.draw_rect(Rec2::new(camera.translate(leading_position), TILE_SIZE), RGBA::new(255, 128, 0, OPAQUE)); }

    let cling_position = compute_cling(zoomer, room, direction, leading_coordinate, position, velocity);
    if debug { render.draw_rect(Rec2::new(camera.translate(cling_position), TILE_SIZE), RGBA::new(128, 255, 0, OPAQUE)); }
  }
}

/// Get the leading edge of the zoomer collision box.
///
/// This is the edge that will first collide with walls, tiles, creatures, etc.
fn get_zoomer_extremity(position: Vec2<f32>, direction: Direction) -> Vec2<f32> {
  match direction {
    Direction::Up | Direction::Left => position,
    Direction::Down => position + Vec2::new(0.0, SIZE.y),
    Direction::Right => position + Vec2::new(SIZE.x, 0.0),
    _ => panic!("Zoomer must have ordinal direction")
  }
}

/// Updates a Zoomer direction based on the leading edge of the collision box.
///
/// If the coordinate adjacent to the leading edge is occupied, the zoomer will turn to follow it
///
/// Returns the current leading coordinate and position.
fn compute_leading(zoomer: &mut Zoomer, room: &mut Room, direction: Direction, position: &mut Position, velocity: &mut Velocity) -> (Coordinate, Vec2<f32>) {
  let extremity = get_zoomer_extremity(position.0, direction);
  let (leading_tile, .., leading_position, leading_coordinate) = room.query_tile(TileQuery::Position(extremity));
  if zoomer.last_lead.is_none() || leading_coordinate != zoomer.last_lead.unwrap() {
    zoomer.last_lead = Some(leading_coordinate);
    if leading_tile.is_some() {
      let rotation = zoomer.rotation.clone().invert();
      let new_direction = direction.rotate(rotation, QUARTER_ROTATION);
      velocity.0 = Vec2::<f32>::from(new_direction.to_coordinate()) * ZOOMER_SPEED;
      position.0 = round_to_tile(position.0);
    };
  }
  (leading_coordinate, leading_position)
}

// Get the coordinate of the tile that the zoomer "clings" to.
fn get_cling_coordinate(coordinate: Coordinate, direction: Direction, rotation: Rotation) -> Coordinate {
  coordinate - direction.to_coordinate() + direction.rotate(rotation, QUARTER_ROTATION).to_coordinate()
}

/// Updates a Zoomer direction based on the tile it "clings" to.
///
/// If they tile the zoomer "clings" to is empty, the zoomer will turn around the bend.
///
/// If so, we skip the next cling check while the zoomer hangs midair while turning.
///
/// Returns the position of the tile the zoomer clings to.
fn compute_cling(zoomer: &mut Zoomer, room: &mut Room, direction: Direction, leading_coordinate: Coordinate, position: &mut Position, velocity: &mut Velocity) -> Vec2<f32> {
  let cling_coordinate = get_cling_coordinate(leading_coordinate, direction, zoomer.rotation);
  let (cling_tile, .., cling_position, _) = room.query_tile(TileQuery::Coordinate(cling_coordinate));
  if zoomer.last_cling.is_none() || cling_coordinate != zoomer.last_cling.unwrap() {
    zoomer.last_cling = Some(cling_coordinate);
    if cling_tile.is_none() {
      if let Some(new_velocity) = round_bend(zoomer, direction) {
        velocity.0 = new_velocity;
        position.0 = round_to_tile(position.0);
      }
    };
  }
  cling_position
}

/// Turn the zoomer around a bend, and return the new velocity.
///
/// Returns None if the zoomer has already turned this cling check
fn round_bend(zoomer: &mut Zoomer, direction: Direction) -> Option<Vec2<f32>> {
  if (zoomer.turning) {
    zoomer.turning = false;
    return None;
  }
  zoomer.turning = true;

  let new_direction = direction.rotate(zoomer.rotation, QUARTER_ROTATION);
  let new_direction_unit = Vec2::<f32>::from(new_direction.to_coordinate());
  let new_velocity = Vec2::<f32>::from(new_direction.to_coordinate()) * ZOOMER_SPEED;

  Some(new_velocity)
}

/// Round a position to the nearest tile.
///
/// Assumes the tiles aligned such that {0,0} is a tile boundary.
///
/// Probably add a room parameter to provide any needed offset (but realistically, this will always {0,0}).
fn round_to_tile(position: Vec2<f32>) -> Vec2<f32> {
  let x1 = position.x - position.x % TILE_SIZE.x as f32;
  let y1 = position.y - position.y % TILE_SIZE.y as f32;
  let x2 = x1 + TILE_SIZE.x as f32;
  let y2 = y1 + TILE_SIZE.y as f32;
  Vec2::new(
    if position.x - x1 < x2 - position.x { x1 } else { x2 },
    if position.y - y1 < y2 - position.y { y1 } else { y2 },
  )
}

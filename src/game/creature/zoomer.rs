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
use crate::engine::system::{SysArgs, Systemize};
use crate::engine::tile::query::{TileQuery, TileQueryResult};
use crate::engine::utility::alias::{Coordinate, Size2};
use crate::engine::utility::direction::{Direction, QUARTER_DIRECTION_ROTATION, Rotation};
use crate::game::combat::damage::Damage;
use crate::game::combat::health::Health;
use crate::game::constant::TILE_SIZE;
use crate::game::creature::CreatureLayer;
use crate::game::physics::collision::Collider;
use crate::game::physics::frozen::FreezeResistant;
use crate::game::physics::gravity::Gravity;
use crate::game::physics::position::Position;
use crate::game::physics::velocity::Velocity;
use crate::game::player::combat::PlayerHostile;
use crate::game::preferences::use_preferences;
use crate::game::scene::level::meta::TileLayerType;
use crate::game::scene::level::room::{Room, use_room};
use crate::game::utility::math::{floor_to_tile, round_to_tile};

const ZOOMER_SPEED: f32 = 48.0;
const ZOOMER_HEALTH: u32 = 30;
const ZOOMER_DAMAGE: u32 = 10;
const ZOOMER_ASSET: &str = "asset/sprite/zoomer.png";
const DIMENSIONS: Size2 = Size2::new(16, 16);
const SIZE: Vec2<f32> = Vec2::new(DIMENSIONS.x as f32, DIMENSIONS.y as f32);

// Zoomer component
pub struct Zoomer {
  pub rotation: Rotation,
  pub last_cling: Option<Coordinate>,
  pub last_lead: Option<Coordinate>,
  pub turning: bool,
}

impl Systemize for Zoomer {
  /// Process Zoomer logic each frame
  fn system(SysArgs { world, render, state, camera, .. }: &mut SysArgs) -> Result<(), String> {
    let debug = use_preferences(state).debug;
    let room = use_room(state);

    for (_, (zoomer, velocity, position)) in world
      .query::<(&mut Zoomer, &mut Velocity, &mut Position)>()
    {
      let direction = Direction::try_from(velocity.0).expect("Zoomer must have velocity!");

      let (leading_coordinate, leading_position) = compute_leading(zoomer, room, direction, position, velocity);
      if debug { render.draw_rect(Rec2::new(camera.translate(leading_position), TILE_SIZE), RGBA::new(255, 128, 0, OPAQUE)); }

      let cling_position = compute_cling(zoomer, room, direction, leading_coordinate, position, velocity);
      if debug { render.draw_rect(Rec2::new(camera.translate(cling_position), TILE_SIZE), RGBA::new(128, 255, 0, OPAQUE)); }
    }

    Ok(())
  }
}

/// Add a Zoomer to the world
pub fn make_zoomer(asset_manager: &mut AssetManager, position: Vec2<f32>, initial_direction: Direction) -> Result<impl DynamicBundle, String> {
  if initial_direction.is_ordinal() { return Err(String::from("Zoomer must be initialized with an ordinal direction")); }
  let zoomer = asset_manager.texture.load(Path::new(ZOOMER_ASSET))?;
  let floored_position = floor_to_tile(position);

  Ok((
    PlayerHostile,
    Zoomer { rotation: Rotation::Right, last_cling: None, last_lead: None, turning: false },
    Sprite::new(zoomer, Rec2::new(Vec2::default(), DIMENSIONS)),
    Position(floored_position),
    Gravity::new(Vec2::new(0.0, 0.0)),
    Velocity::from(Vec2::<f32>::from(initial_direction.to_coordinate()) * ZOOMER_SPEED),
    Collider::new(CollisionBox::new(Vec2::default(), DIMENSIONS)),
    CreatureLayer::default(),
    Damage::new(ZOOMER_DAMAGE),
    Health::build(ZOOMER_HEALTH).expect("Failed to build health"),
    FreezeResistant,
  ))
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
  let TileQueryResult {
    concept: leading_tile,
    position: leading_position,
    coordinate: leading_coordinate,
    ..
  } = room.query_tile(TileLayerType::Collision, TileQuery::Position(extremity));
  if zoomer.last_lead.is_none() || leading_coordinate != zoomer.last_lead.unwrap() {
    zoomer.last_lead = Some(leading_coordinate);
    if leading_tile.is_some() {
      let rotation = zoomer.rotation.clone().invert();
      let new_direction = direction.rotate(rotation, QUARTER_DIRECTION_ROTATION);
      velocity.0 = Vec2::<f32>::from(new_direction.to_coordinate()) * ZOOMER_SPEED;
      position.0 = round_to_tile(position.0);
    };
  }
  (leading_coordinate, leading_position)
}

// Get the coordinate of the tile that the zoomer "clings" to.
fn get_cling_coordinate(coordinate: Coordinate, direction: Direction, rotation: Rotation) -> Coordinate {
  coordinate - direction.to_coordinate() + direction.rotate(rotation, QUARTER_DIRECTION_ROTATION).to_coordinate()
}

/// Updates a Zoomer direction based on the tile it "clings" to.
///
/// If the tile the zoomer "clings" to is empty, the zoomer will turn around the bend.
///
/// If so, we skip the next cling check while the zoomer hangs midair while turning.
///
/// Returns the position of the tile the zoomer clings to.
fn compute_cling(zoomer: &mut Zoomer, room: &mut Room, direction: Direction, leading_coordinate: Coordinate, position: &mut Position, velocity: &mut Velocity) -> Vec2<f32> {
  let cling_coordinate = get_cling_coordinate(leading_coordinate, direction, zoomer.rotation);
  let TileQueryResult {
    concept: cling_tile,
    position: cling_position,
    ..
  } = room.query_tile(TileLayerType::Collision, TileQuery::Coordinate(cling_coordinate));
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
  if zoomer.turning {
    zoomer.turning = false;
    return None;
  }
  zoomer.turning = true;

  let new_direction = direction.rotate(zoomer.rotation, QUARTER_DIRECTION_ROTATION);
  let new_direction_unit = Vec2::<f32>::from(new_direction.to_coordinate());
  let new_velocity = new_direction_unit * ZOOMER_SPEED;

  Some(new_velocity)
}

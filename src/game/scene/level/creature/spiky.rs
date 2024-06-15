use std::path::Path;

use hecs::DynamicBundle;
use crate::engine::asset::asset::AssetManager;
use crate::engine::component::position::Position;
use crate::engine::component::sprite::Sprite;
use crate::engine::ecs::system::{SysArgs, Systemize};
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::utility::alias::Size2;
use crate::engine::utility::color::{OPAQUE, RGBA};
use crate::engine::utility::direction::Direction;
use crate::game::constant::TILE_SIZE;
use crate::game::preferences::use_preferences;
use crate::game::scene::level::combat::damage::Damage;
use crate::game::scene::level::combat::health::Health;
use crate::game::scene::level::creature::CreatureLayer;
use crate::game::scene::level::physics::collision::Collider;
use crate::game::scene::level::physics::frozen::FreezeResistant;
use crate::game::scene::level::physics::gravity::Gravity;
use crate::game::scene::level::physics::velocity::Velocity;
use crate::game::scene::level::player::combat::PlayerHostile;
use crate::game::scene::level::room::collision::CollisionBox;
use crate::game::scene::level::room::meta::TileLayerType;
use crate::game::scene::level::room::room::use_room;
use crate::game::scene::level::tile::query::TileQuery;
use crate::game::utility::math::floor_to_tile;

/**
 * Small clueless creature that wanders back and forth on the ground
 */

const SPIKY_SPEED: f32 = 48.0;
const SPIKY_ASSET: &str = "asset/sprite/spiky.png";
const SPIKY_HEALTH: u32 = 30;
const SPIKY_DAMAGE: u32 = 10;
const DIMENSIONS: Size2 = Size2::new(16, 16);
const WIDTH: Vec2<f32> = Vec2::new(DIMENSIONS.x as f32, 0.0);
const HEIGHT: Vec2<f32> = Vec2::new(0.0, DIMENSIONS.y as f32);

// Spiky component
pub struct Spiky;

impl Systemize for Spiky {
  /// Process Spiky logic each frame
  fn system(SysArgs { world, state, render, camera, .. }: &mut SysArgs) -> Result<(), String> {
    let debug = use_preferences(state).debug;
    let room = use_room(state);
    for (_, (velocity, position)) in world
      .query::<(&mut Velocity, &Position)>()
      .with::<&Spiky>()
    {
      let leading_top_corner = if velocity.is_going_right() { position.0 + WIDTH } else { position.0 };
      let leading_bottom_corner = leading_top_corner + HEIGHT;

      let result = room.query_tile(TileLayerType::Collision, TileQuery::Position(leading_top_corner));
      if debug { render.draw_rect(Rec2::new(camera.translate(result.position), TILE_SIZE), RGBA::new(255, 128, 0, OPAQUE)); }
      if result.concept.is_some() { velocity.reverse_x(); }

      let result = room.query_tile(TileLayerType::Collision, TileQuery::Position(leading_bottom_corner));
      if debug { render.draw_rect(Rec2::new(camera.translate(result.position), TILE_SIZE), RGBA::new(0, 255, 128, OPAQUE)); }
      if result.concept.is_none() { velocity.reverse_x(); }
    }

    return Ok(());
  }
}

/// Add a Spiky to the world
pub fn make_spiky(asset_manager: &mut AssetManager, position: Vec2<f32>, initial_direction: Direction) -> Result<impl DynamicBundle, String> {
  if initial_direction != Direction::Left && initial_direction != Direction::Right {
    return Err(String::from("Spiky must be initialized with a horizontal direction"));
  }
  let spiky = asset_manager.texture.load(Path::new(SPIKY_ASSET))?;
  let floored_position = floor_to_tile(position);
  Ok((
    PlayerHostile,
    Spiky,
    Sprite::new(spiky, Rec2::new(Vec2::default(), DIMENSIONS)),
    Position(floored_position),
    Gravity::new(Vec2::new(0.0, 0.0)),
    Velocity::from(Vec2::<f32>::from(initial_direction.to_coordinate()) * SPIKY_SPEED),
    Collider::new(CollisionBox::new(Vec2::default(), DIMENSIONS)),
    CreatureLayer::default(),
    Damage::new(SPIKY_DAMAGE),
    Health::build(SPIKY_HEALTH).expect("Failed to build health"),
    FreezeResistant,
  ))
}

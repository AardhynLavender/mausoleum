use std::path::Path;

use hecs::DynamicBundle;

/**
 * Spiky
 * Small, spiky creature that alternates across a floor
 */

use crate::engine::asset::AssetManager;
use crate::engine::geometry::collision::CollisionBox;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::rendering::color::RGBA;
use crate::engine::rendering::component::Sprite;
use crate::engine::system::SysArgs;
use crate::engine::tile::tilemap::TileQuery;
use crate::engine::utility::alias::Size2;
use crate::engine::utility::direction::Direction;
use crate::game::combat::damage::Damage;
use crate::game::constant::TILE_SIZE;
use crate::game::creature::CreatureLayer;
use crate::game::physics::collision::Collider;
use crate::game::physics::gravity::Gravity;
use crate::game::physics::position::Position;
use crate::game::physics::velocity::Velocity;
use crate::game::room::use_room;
use crate::game::utility::controls::{Behaviour, Control, is_control};

const SPIKY_SPEED: f32 = 64.0;
const SPIKY_ASSET: &str = "asset/spiky.png";
const DIMENSIONS: Size2 = Size2::new(16, 16);
const WIDTH: Vec2<f32> = Vec2::new(DIMENSIONS.x as f32, 0.0);
const HEIGHT: Vec2<f32> = Vec2::new(0.0, DIMENSIONS.y as f32);

// Spiky component
#[derive(Default)]
struct Spiky;

/// Add a Spiky to the world
pub fn make_spiky(asset_manager: &mut AssetManager, position: Vec2<f32>, initial_direction: Direction) -> Result<impl DynamicBundle, String> {
  if initial_direction != Direction::Left && initial_direction != Direction::Right {
    return Err(String::from("Spiky must be initialized with a horizontal direction"));
  }

  let spiky = asset_manager.texture.load(Path::new(SPIKY_ASSET))?;
  Ok((
    Spiky::default(),
    Sprite::new(spiky, Rec2::new(Vec2::default(), DIMENSIONS)),
    Position(position),
    Gravity::new(Vec2::new(0.0, 0.0)),
    Velocity::from(Vec2::<f32>::from(initial_direction.to_coordinate()) * SPIKY_SPEED),
    Collider::new(CollisionBox::new(Vec2::default(), DIMENSIONS)),
    CreatureLayer::default(),
    Damage::new(10),
  ))
}

pub fn sys_spiky(SysArgs { world, render, state, camera, event, .. }: &mut SysArgs) {
  let room = use_room(state);
  let debug = is_control(Control::Debug, Behaviour::Pressed, event);
  for (_, (velocity, position)) in world
    .query::<(&mut Velocity, &Position)>()
    .with::<&Spiky>()
  {
    let leading_top_corner = if !velocity.is_going_right() { position.0 + WIDTH } else { position.0 };
    let leading_bottom_corner = leading_top_corner + HEIGHT;
    if let Ok((tile, .., position)) = room.query_tile(TileQuery::Position(leading_top_corner)) {
      if debug { render.draw_rect(Rec2::new(camera.translate(position), TILE_SIZE), RGBA::new(255, 255, 0, 255)); }
      if tile.is_some() { velocity.reverse_x(); }
    }
    if let Ok((tile, .., position)) = room.query_tile(TileQuery::Position(leading_bottom_corner)) {
      if debug { render.draw_rect(Rec2::new(camera.translate(position), TILE_SIZE), RGBA::new(255, 255, 0, 255)); }
      if tile.is_none() { velocity.reverse_x(); }
    }
  }
}

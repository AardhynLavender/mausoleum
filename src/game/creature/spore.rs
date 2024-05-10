/**
 * Plantlike creature that erupts cells in the players' general direction
 */

use std::path::Path;
use std::time::Duration;

use hecs::DynamicBundle;

use crate::engine::asset::AssetManager;
use crate::engine::asset::texture::TextureKey;
use crate::engine::geometry::collision::CollisionBox;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::rendering::component::Sprite;
use crate::engine::system::{SysArgs, Systemize};
use crate::engine::time::{ConsumeAction, Timer};
use crate::engine::utility::alias::{Size, Size2};
use crate::engine::utility::direction::Direction;
use crate::engine::utility::invariant::invariant;
use crate::game::combat::damage::Damage;
use crate::game::combat::health::Health;
use crate::game::creature::CreatureLayer;
use crate::game::physics::collision::{Collider, Fragile};
use crate::game::physics::frozen::Frozen;
use crate::game::physics::gravity::Gravity;
use crate::game::physics::position::Position;
use crate::game::physics::velocity::Velocity;
use crate::game::player::combat::PlayerHostile;
use crate::game::scene::level::collision::RoomCollision;
use crate::game::utility::math::floor_to_tile;

const QUARTER_ROTATION_DEG: f32 = 90.0;

const ASSET: &str = "asset/spore.png";
const HEALTH: u32 = 70;
const DAMAGE: u32 = 5;
const CELL_DAMAGE: u32 = 15;
const DIMENSIONS: Size2 = Size2::new(16, 16);

const CELL_ASSET: &str = "asset/spore_cell.png";
const CELL_DIMENSIONS: Size2 = Size2::new(6, 6);
const CELL_GRAVITY: Vec2<f32> = Vec2::new(0.0, 256.0);
const CELL_SPEED: f32 = 180.0;
const CELL_SPAWN_COUNT: Size = 6;
const CELL_SPAWN_INTERVAL_MS: u64 = 2_500;
const CELL_SPAWN_SPREAD_DEG: f32 = 60.0;
const CELL_SPAWN_STEP_DEG: f32 = CELL_SPAWN_SPREAD_DEG / CELL_SPAWN_COUNT as f32;

/// Spore creature
pub struct Spore {
  spawn_cooldown: Timer,
  direction: Direction,
  cell_asset: TextureKey,
}

impl Spore {
  /// Instantiate a new spore
  pub fn build(direction: Direction, cell_asset: TextureKey) -> Result<Self, String> {
    invariant(direction.is_cardinal(), "Spore direction must be cardinal")?;
    Ok(Self { direction, cell_asset, spawn_cooldown: Timer::new(Duration::from_millis(CELL_SPAWN_INTERVAL_MS), true) })
  }
}

impl Systemize for Spore {
  /// Process Buzz logic each frame
  fn system(SysArgs { world, .. }: &mut SysArgs) -> Result<(), String> {
    let spore_cells = world
      .query::<(&Position, &mut Spore)>()
      .without::<&Frozen>()
      .into_iter()
      .map(|(_, (position, spore))| {
        if !spore.spawn_cooldown.consume(ConsumeAction::Restart) { return vec![]; }
        let start_deg = f32::from(spore.direction) - CELL_SPAWN_SPREAD_DEG / 2.0;
        let end_deg = start_deg + CELL_SPAWN_SPREAD_DEG;
        let cell_position = Vec2::<f32>::from(DIMENSIONS / 2 - CELL_DIMENSIONS / 2) + position.0;
        let cells = ((start_deg as i32)..(end_deg as i32))
          .step_by(CELL_SPAWN_STEP_DEG as usize)
          .map(|angle| {
            make_spore_cell(spore.cell_asset, cell_position, angle as f32 - QUARTER_ROTATION_DEG)
          })
          .collect::<Vec<_>>();
        cells
      })
      .collect::<Vec<_>>();

    for spore in spore_cells {
      for cell in spore { world.add(cell); }
    }

    Ok(())
  }
}

/// Compose the components of a spore creature
pub fn make_spore(asset_manager: &mut AssetManager, position: Vec2<f32>, direction: Direction) -> Result<impl DynamicBundle, String> {
  let spore = asset_manager.texture.load(Path::new(ASSET))?;
  let cell = asset_manager.texture.load(Path::new(CELL_ASSET))?;
  let floored_position = floor_to_tile(position);
  Ok((
    PlayerHostile,
    Spore::build(direction, cell)?,
    Sprite::new(spore, Rec2::new(Vec2::default(), DIMENSIONS)),
    Position(floored_position),
    Collider::new(CollisionBox::new(Vec2::default(), DIMENSIONS)),
    CreatureLayer::default(),
    Damage::new(DAMAGE),
    Health::build(HEALTH).expect("Failed to build health")
  ))
}

/// An indestructible damaging cell spawned by a spore
pub struct SporeCell;

/// Compose the components of a spore cell
pub fn make_spore_cell(texture: TextureKey, position: Vec2<f32>, angle: f32) -> impl DynamicBundle {
  (
    PlayerHostile,
    SporeCell,
    Sprite::new(texture, Rec2::new(Vec2::default(), CELL_DIMENSIONS)),
    Position::from(position),
    Collider::new(CollisionBox::new(Vec2::default(), CELL_DIMENSIONS)),
    Velocity::from(Vec2::from_degrees(angle) * CELL_SPEED),
    Gravity::new(CELL_GRAVITY),
    Damage::new(CELL_DAMAGE),
    RoomCollision,
    Fragile,
    CreatureLayer::default(),
  )
}

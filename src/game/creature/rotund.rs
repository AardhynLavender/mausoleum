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
use crate::engine::utility::alias::Size2;
use crate::engine::utility::direction::{CompassDirectionType, Direction, EIGHTH_ROTATION_DEG, FULL_ROTATION_DEG, QUARTER_ROTATION_DEG};
use crate::game::combat::damage::Damage;
use crate::game::combat::health::Health;
use crate::game::combat::ttl::TimeToLive;
use crate::game::creature::CreatureLayer;
use crate::game::physics::collision::{Collider, Fragile, make_collision_box};
use crate::game::physics::frozen::Frozen;
use crate::game::physics::gravity::Gravity;
use crate::game::physics::position::Position;
use crate::game::physics::velocity::Velocity;
use crate::game::player::combat::PlayerHostile;
use crate::game::scene::level::collision::RoomCollision;
use crate::game::utility::math::floor_to_tile;

const SPEED: f32 = 30.0;
const ASSET: &str = "asset/sprite/bubbly.png";
const HEALTH: u32 = 50;
const DAMAGE: u32 = 20;
const DIMENSIONS: Size2 = Size2::new(24, 24);

const SPIT_ASSET: &str = "asset/sprite/angry_buzz_spit.png";
const SPIT_DAMAGE: u32 = 20;
const SPIT_COOLDOWN_MS: u64 = 3_000;
const SPIT_SPEED: f32 = 200.0;
const SPIT_GRAVITY: Vec2<f32> = Vec2::new(0.0, 64.0);
const SPIT_DIMENSIONS: Size2 = Size2::new(8, 8);
const SPIT_TTL: u64 = 2_000;

// Rotund component
pub struct Rotund {
  previous_velocity: Velocity,
  spit_texture: TextureKey,
  spit_cooldown: Timer,
  spit_axis: CompassDirectionType,
}

impl Rotund {
  /// Instantiate a new Rotund component
  pub fn new(initial_velocity: Velocity, spit_texture: TextureKey, spit_axis: CompassDirectionType) -> Self {
    Self {
      previous_velocity: initial_velocity,
      spit_cooldown: Timer::new(Duration::from_millis(SPIT_COOLDOWN_MS), true),
      spit_texture,
      spit_axis,
    }
  }

  /// Updates the Rotund's velocity and return the old velocity
  pub fn update(&mut self, next: Velocity) -> Velocity {
    let previous = self.previous_velocity;
    self.previous_velocity = next;
    previous
  }
}

/// Add a Rotund to the world
pub fn make_rotund(asset_manager: &mut AssetManager, position: Vec2<f32>, direction: Direction, spit_axis: CompassDirectionType) -> Result<impl DynamicBundle, String> {
  let rotund = asset_manager.texture.load(Path::new(ASSET))?;
  let spit = asset_manager.texture.load(Path::new(SPIT_ASSET))?;
  let velocity = Velocity::from(Vec2::from(direction.to_coordinate()) * SPEED);
  let floored_position = floor_to_tile(position);
  Ok((
    PlayerHostile,
    Rotund::new(velocity, spit, spit_axis),
    Sprite::new(rotund, Rec2::new(Vec2::default(), DIMENSIONS)),
    Position::from(floored_position),
    velocity,
    Collider::new(CollisionBox::new(Vec2::default(), DIMENSIONS)),
    CreatureLayer::default(),
    Damage::new(DAMAGE),
    Health::build(HEALTH).expect("Failed to build health"),
    RoomCollision,
  ))
}

/// Compose the components for a Rotund's spit
pub fn make_spit(position: Vec2<f32>, spit_texture: TextureKey, angle: f32) -> impl DynamicBundle {
  (
    PlayerHostile,
    Position::from(position),
    Velocity::from(Vec2::from_degrees(angle) * SPIT_SPEED),
    Sprite::new(spit_texture, Rec2::new(Vec2::default(), SPIT_DIMENSIONS)),
    Collider::new(CollisionBox::new(Vec2::default(), SPIT_DIMENSIONS)),
    Damage::new(SPIT_DAMAGE),
    CreatureLayer::default(),
    Gravity::new(SPIT_GRAVITY),
    Fragile,
    TimeToLive::new(SPIT_TTL),
    RoomCollision,
  )
}

impl Systemize for Rotund {
  /// Process Rotund logic each frame
  fn system(SysArgs { world, .. }: &mut SysArgs) -> Result<(), String> {
    let rotund_spits = world
      .query::<(&mut Rotund, &Position, &Collider, &mut Velocity)>()
      .without::<&Frozen>()
      .into_iter()
      .filter_map(|(_, (rotund, rotund_position, rotund_collider, rotund_velocity))| {
        let previous_velocity = rotund.update(*rotund_velocity);
        if rotund_velocity.0.x == 0.0 { rotund_velocity.0.x = -previous_velocity.0.x; }
        if rotund_velocity.0.y == 0.0 { rotund_velocity.0.y = -previous_velocity.0.y; }

        let rotund_centroid = make_collision_box(rotund_position, rotund_collider).centroid();
        let spit_position = rotund_centroid - Vec2::from(SPIT_DIMENSIONS / 2);

        if rotund.spit_cooldown.consume(ConsumeAction::Restart) {
          let spits = (0..FULL_ROTATION_DEG)
            .step_by(QUARTER_ROTATION_DEG as usize)
            .into_iter()
            .map(|mut angle| {
              if rotund.spit_axis == CompassDirectionType::Ordinal { angle += EIGHTH_ROTATION_DEG; };
              make_spit(spit_position, rotund.spit_texture, angle as f32)
            })
            .collect::<Vec<_>>();
          return Some(spits);
        }
        None
      })
      .collect::<Vec<_>>();

    for spits in rotund_spits {
      for spit in spits { world.add(spit); }
    }

    Ok(())
  }
}

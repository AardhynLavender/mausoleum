/**
 * Small flying creature that floats around the room and follows the player when close enough
 */

use std::path::Path;
use std::time::Duration;

use hecs::DynamicBundle;

use crate::engine::asset::AssetManager;
use crate::engine::asset::texture::TextureKey;
use crate::engine::geometry::collision::CollisionBox;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::rendering::color::color;
use crate::engine::rendering::component::Sprite;
use crate::engine::system::{SysArgs, Systemize};
use crate::engine::time::{ConsumeAction, Timer};
use crate::engine::utility::alias::Size2;
use crate::game::combat::damage::Damage;
use crate::game::combat::health::Health;
use crate::game::combat::ttl::TimeToLive;
use crate::game::creature::buzz::BuzzState;
use crate::game::creature::CreatureLayer;
use crate::game::physics::collision::{Collider, Fragile, make_collision_box};
use crate::game::physics::frozen::Frozen;
use crate::game::physics::gravity::Gravity;
use crate::game::physics::position::Position;
use crate::game::physics::velocity::Velocity;
use crate::game::player::combat::PlayerHostile;
use crate::game::player::world::{PlayerQuery, use_player};
use crate::game::preferences::use_preferences;
use crate::game::scene::level::collision::RoomCollision;

const SPEED: f32 = 64.0;
const ASSET: &str = "asset/sprite/angry_buzz.png";
const HEALTH: u32 = 80;
const DAMAGE: u32 = 12;
const DIMENSIONS: Size2 = Size2::new(24, 24);

const SPIT_COOLDOWN: u64 = 2_000;
const SPIT_DAMAGE: u32 = 10;
const SPIT_ASSET: &str = "asset/sprite/angry_buzz_spit.png";
const SPIT_DIMENSIONS: Size2 = Size2::new(8, 8);
const SPIT_DURATION_MS: u64 = 2_000;
const SPIT_GRAVITY: Vec2<f32> = Vec2::new(0.0, 96.0);
const SPIT_SPEED: f32 = 256.0;

pub type AngryBuzzState = BuzzState;

// Buzz component
pub struct AngryBuzz {
  state: AngryBuzzState,
  #[allow(dead_code)]
  spit_cooldown: Timer,
  spit_asset: TextureKey,
}

impl AngryBuzz {
  fn new(asset_manager: &mut AssetManager) -> Self {
    Self {
      state: AngryBuzzState::Idle,
      spit_cooldown: Timer::new(Duration::from_millis(SPIT_COOLDOWN), true),
      spit_asset: asset_manager.texture.load(Path::new(SPIT_ASSET)).unwrap(),
    }
  }
}

/// Buzz system
impl Systemize for AngryBuzz {
  /// Process Buzz logic each frame
  fn system(SysArgs { state, camera, render, world, .. }: &mut SysArgs) -> Result<(), String> {
    let PlayerQuery { position: player_position, collider, .. } = use_player(world);
    let debug = use_preferences(state).debug;
    let player_centroid = make_collision_box(player_position, collider).centroid();

    let spits =
      world
        .query::<(&mut AngryBuzz, &Position, &mut Velocity, &Collider)>()
        .without::<&Frozen>()
        .into_iter()
        .filter_map(|(_, (angry_buzz, angry_buzz_position, angry_buzz_velocity, angry_buzz_collider))| {
          let angry_buzz_centroid = make_collision_box(angry_buzz_position, angry_buzz_collider).centroid();
          let unit_transform = (player_centroid - angry_buzz_centroid).normalize();
          if angry_buzz.state.update(angry_buzz_centroid, player_centroid) == AngryBuzzState::Follow {
            angry_buzz_velocity.0 = unit_transform * SPEED;
            if debug {
              render.draw_line(camera.translate(angry_buzz_centroid), camera.translate(player_centroid), color::PRIMARY);
            }

            if angry_buzz.spit_cooldown.consume(ConsumeAction::Restart) {
              let spit_unit_transform = Vec2::new(unit_transform.x, 0.0);
              return Some(make_spit(angry_buzz_centroid, angry_buzz.spit_asset, spit_unit_transform.to_degrees()));
            }
          } else {
            // todo: implement idle behavior
            angry_buzz_velocity.0 = Vec2::default();
          }

          None
        })
        .collect::<Vec<_>>();

    for spit in spits {
      world.add(spit);
    }

    Ok(())
  }
}

/// Compose the components for an AngryBuzz
pub fn make_angry_buzz(asset_manager: &mut AssetManager, position: Vec2<f32>) -> Result<impl DynamicBundle, String> {
  let buzz = asset_manager.texture.load(Path::new(ASSET))?;
  Ok((
    PlayerHostile,
    AngryBuzz::new(asset_manager),
    Sprite::new(buzz, Rec2::new(Vec2::default(), DIMENSIONS)),
    Position::from(position),
    Velocity::default(),
    Collider::new(CollisionBox::new(Vec2::default(), DIMENSIONS)),
    CreatureLayer::default(),
    Damage::new(DAMAGE),
    Health::build(HEALTH).expect("Failed to build health"),
    RoomCollision::Creature,
  ))
}

/// Compose the components for a Buzz's spit
pub fn make_spit(position: Vec2<f32>, spit_texture: TextureKey, angle: f32) -> impl DynamicBundle {
  (
    PlayerHostile,
    Position::from(position),
    Velocity::from(Vec2::from_degrees(angle) * SPIT_SPEED),
    Sprite::new(spit_texture, Rec2::new(Vec2::default(), SPIT_DIMENSIONS)),
    Collider::new(CollisionBox::new(Vec2::default(), SPIT_DIMENSIONS)),
    CreatureLayer::default(),
    Damage::new(SPIT_DAMAGE),
    Gravity::new(SPIT_GRAVITY),
    Fragile,
    TimeToLive::new(SPIT_DURATION_MS),
    RoomCollision::All,
  )
}

/**
  * Small irritable flying creature that floats around the room
  * and follows the player when close enough
  */

use std::path::Path;

use hecs::DynamicBundle;
use crate::engine::asset::asset::AssetManager;
use crate::engine::component::position::Position;
use crate::engine::component::sprite::Sprite;
use crate::engine::ecs::system::{SysArgs, Systemize};
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::utility::alias::Size2;
use crate::engine::utility::color::color;
use crate::game::preferences::use_preferences;
use crate::game::scene::level::combat::damage::Damage;
use crate::game::scene::level::combat::health::Health;
use crate::game::scene::level::creature::CreatureLayer;
use crate::game::scene::level::physics::collision::{Collider, make_collision_box};
use crate::game::scene::level::physics::frozen::Frozen;
use crate::game::scene::level::physics::velocity::Velocity;
use crate::game::scene::level::player::combat::PlayerHostile;
use crate::game::scene::level::player::world::{PlayerQuery, use_player};
use crate::game::scene::level::room::collision::{CollisionBox, RoomCollision};

const BUZZ_SPEED: f32 = 96.0;
const BUZZ_ASSET: &str = "asset/sprite/buzz.png";
const BUZZ_HEALTH: u32 = 20;
const BUZZ_DAMAGE: u32 = 8;
const DIMENSIONS: Size2 = Size2::new(8, 8);

const BUZZ_FOLLOW_RADIUS: f32 = 256.0;
const BUZZ_FORGET_RADIUS: f32 = 384.0;

/// Buzz state
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub enum BuzzState {
  #[default]
  /// the Buzz will float about idly within the room
  Idle,
  /// the Buzz will follow the player
  Follow,
}

impl BuzzState {
  /// Update the Buzz state
  /// - If the Buzz is far enough from the player, it will be idle
  /// - If the Buzz is close enough to the player, it will follow the player
  pub fn update(&mut self, position: Vec2<f32>, player_position: Vec2<f32>) -> Self {
    let distance = (player_position - position).get_magnitude().abs();
    if *self == BuzzState::Idle && distance < BUZZ_FOLLOW_RADIUS {
      *self = BuzzState::Follow;
    } else if *self == BuzzState::Follow && distance > BUZZ_FORGET_RADIUS {
      *self = BuzzState::Idle;
    }
    *self
  }
}

// Buzz component
#[derive(Default)]
pub struct Buzz(pub BuzzState);

/// Add a Buzz to the world
pub fn make_buzz(asset_manager: &mut AssetManager, position: Vec2<f32>) -> Result<impl DynamicBundle, String> {
  let buzz = asset_manager.texture.load(Path::new(BUZZ_ASSET))?;
  Ok((
    PlayerHostile,
    Buzz::default(),
    Sprite::new(buzz, Rec2::new(Vec2::default(), DIMENSIONS)),
    Position::from(position),
    Velocity::default(),
    Collider::new(CollisionBox::new(Vec2::default(), DIMENSIONS)),
    CreatureLayer::default(),
    Damage::new(BUZZ_DAMAGE),
    Health::build(BUZZ_HEALTH).expect("Failed to build health"),
    RoomCollision::Creature,
  ))
}

impl Systemize for Buzz {
  /// Process Buzz logic each frame
  fn system(SysArgs { state, camera, render, world, .. }: &mut SysArgs) -> Result<(), String> {
    let PlayerQuery { position: player_position, collider, .. } = use_player(world);
    let debug = use_preferences(state).debug;
    let player_centroid = make_collision_box(player_position, collider).centroid();

    for (_, (buzz, buzz_position, buzz_velocity, collider)) in world
      .query::<(&mut Buzz, &Position, &mut Velocity, &Collider)>()
      .without::<&Frozen>()
    {
      let buzz_centroid = make_collision_box(buzz_position, collider).centroid();
      let unit_transform = (player_centroid - buzz_centroid).normalize();
      if buzz.0.update(buzz_centroid, player_centroid) == BuzzState::Follow {
        buzz_velocity.0 = unit_transform * BUZZ_SPEED;
        if debug {
          render.draw_line(camera.translate(buzz_centroid), camera.translate(player_centroid), color::PRIMARY);
        }
      } else {
        buzz_velocity.0 = Vec2::default();
      }
    }

    Ok(())
  }
}
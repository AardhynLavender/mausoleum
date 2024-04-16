#![allow(unused)]

/**
 * Small flying creature that floats around the room and follows the player when close enough
 */

use std::path::Path;

use hecs::DynamicBundle;

use crate::engine::asset::AssetManager;
use crate::engine::geometry::collision::CollisionBox;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::rendering::color::color;
use crate::engine::rendering::component::Sprite;
use crate::engine::system::SysArgs;
use crate::engine::utility::alias::Size2;
use crate::game::combat::damage::Damage;
use crate::game::combat::health::Health;
use crate::game::creature::{Creature, CreatureLayer};
use crate::game::physics::collision::Collider;
use crate::game::physics::position::Position;
use crate::game::physics::velocity::Velocity;
use crate::game::player::world::{PQ, use_player};
use crate::game::scene::level::collision::RoomCollision;
use crate::game::utility::controls::{Behaviour, Control, is_control};

const BUZZ_SPEED: f32 = 96.0;
const BUZZ_ASSET: &str = "asset/buzz.png";
const DIMENSIONS: Size2 = Size2::new(8, 8);

const BUZZ_FOLLOW_RADIUS: f32 = 256.0;
const BUZZ_FORGET_RADIUS: f32 = 384.0;
const BUZZ_FOLLOW_SPEED: f32 = 32.0;

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
    let distance = (player_position - position).magnitude().abs();
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
struct Buzz(pub BuzzState);

/// Add a Buzz to the world
pub fn make_buzz(asset_manager: &mut AssetManager, position: Vec2<f32>) -> Result<impl DynamicBundle, String> {
  let buzz = asset_manager.texture.load(Path::new(BUZZ_ASSET))?;
  Ok((
    Creature::default(),
    Buzz::default(),
    Sprite::new(buzz, Rec2::new(Vec2::default(), DIMENSIONS)),
    Position::from(position),
    Velocity::default(),
    Collider::new(CollisionBox::new(Vec2::default(), DIMENSIONS)),
    CreatureLayer::default(),
    Damage::new(5),
    Health::build(10).expect("Failed to build health"),
    RoomCollision::default(),
  ))
}

/// Buzz system
pub fn sys_buzz(SysArgs { world, render, event, camera, .. }: &mut SysArgs) {
  let PQ { position: player_position, .. } = use_player(world);
  let debug = is_control(Control::Debug, Behaviour::Held, event);
  let player_position = player_position.0;
  for (_, (buzz, buzz_position, buzz_velocity)) in world.query::<(&mut Buzz, &Position, &mut Velocity)>() {
    let transform = player_position - buzz_position.0;
    if buzz.0.update(buzz_position.0, player_position) == BuzzState::Follow {
      buzz_velocity.0 = transform.normalize() * BUZZ_SPEED;
      if debug { render.draw_line(camera.translate(buzz_position.0), camera.translate(player_position), color::PRIMARY); }
    } else {
      // todo: implement idle behavior
      buzz_velocity.0 = Vec2::default();
    }
  }
}
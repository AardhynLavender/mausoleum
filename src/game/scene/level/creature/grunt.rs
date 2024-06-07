/**
 * Large enemy that charges at the player on sight
 */

use std::path::Path;
use std::time::Duration;

use hecs::DynamicBundle;
use crate::engine::asset::asset::AssetManager;
use crate::engine::component::position::Position;
use crate::engine::component::sprite::Sprite;
use crate::engine::ecs::system::{SysArgs, Systemize};
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::math::random::random;
use crate::engine::utility::alias::Size2;
use crate::engine::utility::color::color;
use crate::engine::utility::direction::Direction;
use crate::engine::utility::time::Timer;
use crate::game::preferences::use_preferences;
use crate::game::scene::level::combat::damage::Damage;
use crate::game::scene::level::combat::health::Health;
use crate::game::scene::level::creature::CreatureLayer;
use crate::game::scene::level::physics::collision::{Collider, make_collision_box};
use crate::game::scene::level::physics::frozen::Frozen;
use crate::game::scene::level::physics::gravity::Gravity;
use crate::game::scene::level::physics::velocity::Velocity;
use crate::game::scene::level::player::combat::PlayerHostile;
use crate::game::scene::level::player::world::{PlayerQuery, use_player};
use crate::game::scene::level::room::collision::{CollisionBox, RoomCollision};

const GRUNT_IDLE_SPEED: f32 = 64.0;
const GRUNT_CHARGE_SPEED: f32 = 192.0;

const GRUNT_GRAVITY: Vec2<f32> = Vec2::new(0.0, 256.0);

const GRUNT_ASSET: &str = "asset/sprite/grunt.png";
const GRUNT_HEALTH: u32 = 128;
const GRUNT_DAMAGE_IDLE: u32 = 15;
const GRUNT_DAMAGE_CHARGE: u32 = 25;
const GRUNT_DIMENSIONS: Size2 = Size2::new(32, 24);

const GRUNT_CHARGE_RADIUS: f32 = 250.0;
const GRUNT_CHARGE_TIME_MS: u64 = 1500;
const GRUNT_CHARGE_COOLDOWN_MS: u64 = 1000;
const GRUNT_TURN_COOLDOWN_MIN: u64 = 1000;
const GRUNT_TURN_COOLDOWN_MAX: u64 = 5000;

/// Randomize the direction of the Grunt
fn randomize_direction() -> Option<Direction> {
  match random(0, 3) {
    0 => Some(Direction::Left),
    1 => Some(Direction::Right),
    2 => None,
    _ => unreachable!("Invalid random direction"),
  }
}

/// Compute the side a Grunt should face
fn compute_side(position: Vec2<f32>, player_position: Vec2<f32>) -> Direction {
  if player_position.x < position.x { Direction::Left } else { Direction::Right }
}

/// Grunt state
#[derive(Debug, Copy, Clone)]
pub enum GruntState {
  Idle { direction: Option<Direction>, turn_timer: Timer, charge_cooldown: Timer },
  /// The Grunt will charge in a direction for a time
  Charge { timer: Timer, direction: Direction },
}

impl Default for GruntState {
  fn default() -> Self { GruntState::build_idle() }
}

impl GruntState {
  /// Instantiate a idle state
  pub fn build_idle() -> Self {
    GruntState::Idle {
      direction: None,
      turn_timer: Timer::new(Duration::from_millis(random(GRUNT_TURN_COOLDOWN_MIN, GRUNT_TURN_COOLDOWN_MAX)), true),
      charge_cooldown: Timer::new(Duration::from_millis(GRUNT_CHARGE_COOLDOWN_MS), true),
    }
  }
  /// Instantiate a charging state
  pub fn build_charge(direction: Direction) -> Self {
    GruntState::Charge {
      timer: Timer::new(Duration::from_millis(GRUNT_CHARGE_TIME_MS), true),
      direction,
    }
  }
  /// Update the state
  pub fn update(&mut self, position: Vec2<f32>, player_position: Vec2<f32>) -> Self {
    let distance = (player_position - position).get_magnitude().abs();
    match self {
      GruntState::Idle { charge_cooldown, turn_timer, direction } => {
        let player_close = distance < GRUNT_CHARGE_RADIUS;
        if charge_cooldown.done() && player_close {
          let direction = compute_side(position, player_position);
          *self = GruntState::build_charge(direction);
        } else if turn_timer.done() {
          turn_timer.reset();
          *direction = randomize_direction();
        }
      }
      GruntState::Charge { timer, .. } => {
        if timer.done() { *self = GruntState::build_idle(); }
      }
    }
    *self
  }
}

// Grunt component
#[derive(Default)]
pub struct Grunt(pub GruntState);

impl Systemize for Grunt {
  /// Process Grunt logic each frame
  fn system(SysArgs { state, camera, render, world, .. }: &mut SysArgs) -> Result<(), String> {
    let PlayerQuery { position: player_position, collider: player_collider, .. } = use_player(world);
    let debug = use_preferences(state).debug;
    let player_centroid = make_collision_box(player_position, player_collider).centroid();
    for (_, (grunt, grunt_position, grunt_damage, grunt_velocity, grunt_collider)) in world
      .query::<(&mut Grunt, &Position, &mut Damage, &mut Velocity, &Collider)>()
      .without::<&Frozen>()
    {
      // skip state update if the Grunt is falling
      if grunt_velocity.is_going_down() { return Ok(()); }

      let grunt_centroid = make_collision_box(grunt_position, grunt_collider).centroid();
      let next_state = grunt.0.update(grunt_centroid, player_centroid);
      let (direction, speed, damage) = match next_state {
        GruntState::Idle { direction, .. } => (direction, GRUNT_IDLE_SPEED, GRUNT_DAMAGE_IDLE),
        GruntState::Charge { direction, .. } => {
          if debug { render.draw_line(camera.translate(grunt_centroid), camera.translate(player_centroid), color::PRIMARY); }
          (Some(direction), GRUNT_CHARGE_SPEED, GRUNT_DAMAGE_CHARGE)
        }
      };

      grunt_damage.amount = damage;

      if let Some(direction) = direction {
        grunt_velocity.0.x = Vec2::<f32>::from(direction.to_coordinate()).x * speed;
      } else {
        grunt_velocity.remove_x();
      }
    }

    Ok(())
  }
}

/// Compose the components for a Grunt
pub fn make_grunt(asset_manager: &mut AssetManager, position: Vec2<f32>) -> Result<impl DynamicBundle, String> {
  let grunt = asset_manager.texture.load(Path::new(GRUNT_ASSET))?;
  Ok((
    PlayerHostile,
    Grunt::default(),
    Sprite::new(grunt, Rec2::new(Vec2::default(), GRUNT_DIMENSIONS)),
    Position::from(position),
    Velocity::default(),
    Collider::new(CollisionBox::new(Vec2::default(), GRUNT_DIMENSIONS)),
    CreatureLayer::default(),
    Gravity::new(GRUNT_GRAVITY),
    Damage::new(GRUNT_DAMAGE_IDLE),
    Health::build(GRUNT_HEALTH).expect("Failed to build health"),
    RoomCollision::Creature,
  ))
}

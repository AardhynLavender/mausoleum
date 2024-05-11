## Simple Creature Template

There's a lot of boilerplate code here... It'd be good to simplify things at some point.

```Rust
/**
 * Describe the creature...
 */

use std::path::Path;

use hecs::DynamicBundle;

use crate::engine::asset::AssetManager;
use crate::engine::geometry::collision::CollisionBox;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::rendering::component::Sprite;
use crate::engine::system::{SysArgs, Systemize};
use crate::engine::utility::alias::Size2;
use crate::game::combat::damage::Damage;
use crate::game::combat::health::Health;
use crate::game::creature::CreatureLayer;
use crate::game::physics::collision::{Collider, make_collision_box};
use crate::game::physics::frozen::Frozen;
use crate::game::physics::position::Position;
use crate::game::physics::velocity::Velocity;
use crate::game::player::combat::PlayerHostile;
use crate::game::player::world::{PlayerQuery, use_player};
use crate::game::preferences::use_preferences;
use crate::game::scene::level::collision::RoomCollision;

const SPEED: f32 = 96.0;
const ASSET: &str = "asset/sprite/asset.png";
const HEALTH: u32 = 20;
const DAMAGE: u32 = 8;
const DIMENSIONS: Size2 = Size2::new(8, 8);

/// Creature state
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub enum State {
  #[default]
  Idle,
  Chase,
  Flee,
  // etc...
}

impl State {
  /// Update the state
  pub fn update(&mut self, position: Vec2<f32>, player_position: Vec2<f32>) -> Self {}
}

// Creature component
#[derive(Default)]
pub struct TemplateCreature(pub State);

/// Add a Buzz to the world
pub fn make_creature(asset_manager: &mut AssetManager, position: Vec2<f32>) -> Result<impl DynamicBundle, String> {
  let creature = asset_manager.texture.load(Path::new(ASSET))?;
  Ok((
    PlayerHostile,
    Creature::default(),
    Sprite::new(creature, Rec2::new(Vec2::default(), DIMENSIONS)),
    Position::from(position),
    Velocity::default(),
    Collider::new(CollisionBox::new(Vec2::default(), DIMENSIONS)),
    CreatureLayer::default(),
    Damage::new(DAMAGE),
    Health::build(HEALTH).expect("Failed to build health"),
    RoomCollision,
  ))
}

impl Systemize for Creature {
  /// Process creature logic each frame
  fn system(SysArgs { state, camera, render, world, .. }: &mut SysArgs) -> Result<(), String> {
    let PlayerQuery { position: player_position, collider, .. } = use_player(world);
    let debug = use_preferences(state).debug;
    let player_centroid = make_collision_box(player_position, collider).centroid();

    for (_, (creature, creature_position, creature_velocity, creature_collider)) in world
      .query::<(&mut TemplateCreature, &Position, &mut Velocity, &Collider)>()
      .without::<&Frozen>()
    {
      let creature_centroid = make_collision_box(creature_position, collider).centroid();
      // process creature logic...

      if debug {
        // do some debug rendering...
      }
    }

    Ok(())
  }
}
```
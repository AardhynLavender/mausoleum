
/**
 * Deduct health and kill entities when they collide with Damage components
 */

use hecs::{Component, Entity, Or};
use crate::engine::component::position::Position;
use crate::engine::ecs::system::{SysArgs, Systemize};
use crate::engine::ecs::world::World;
use crate::engine::utility::state::State;
use crate::engine::utility::time::ConsumeAction;
use crate::game::scene::level::combat::health::{Health, LiveState};
use crate::game::scene::level::physics::collision::{Collider, make_collision_box};
use crate::game::scene::level::physics::frozen::{freeze_entity, Frozen};
use crate::game::scene::level::player::combat::{CreatureHostile, IceBeam, PlayerHostile, THAW_DURATION};
use crate::game::scene::level::player::world::{PlayerQuery, use_player};
use crate::game::scene::level::room::collision::{CollisionBox, CollisionMask, rec2_collision};
use crate::game::scene::level::room::room::use_room;
use crate::game::scene::level::tile::tile::TileCollider;
use crate::game::scene::level::tile::tilemap::TilemapMutation;

pub struct Damage {
  pub amount: u32,
}

impl Damage {
  pub fn new(amount: u32) -> Self {
    Self { amount }
  }
}

fn get_damage<Mask>(world: &mut World, collision_box: &CollisionBox) -> Option<(u32, Entity)> where Mask: Component {
  for (entity, (position, collider, damage)) in world
    .query::<(&Position, Or<&TileCollider, &Collider>, &Damage)>()
    .with::<&Mask>()
    .without::<&Frozen>() // frozen entities cannot deal damage
  {
    let (collider, mask) = match collider {
      Or::Left(collider) => (collider.collision_box, collider.mask),
      Or::Right(collider) => (collider.0, CollisionMask::default()),
      _ => panic!("Cannot have both tile collider and collider")
    };

    let creature_box = CollisionBox::new(collider.origin + position.0, collider.size);
    if rec2_collision(collision_box, &creature_box, mask).is_some() {
      return Some((damage.amount, entity));
    }
  }
  None
}

impl Systemize for Damage {
  /// Process damage each frame
  fn system(SysArgs { world, state, .. }: &mut SysArgs) -> Result<(), String> {
    player_damage(world)?;
    creature_damage(world, state)
  }
}

// Damage the player when colliding with dangerous entities
pub fn player_damage(world: &mut World) -> Result<(), String> {
  let PlayerQuery { position, collider, .. } = use_player(world);
  let player_box = CollisionBox::new(position.0, collider.0.size);

  let damage = get_damage::<PlayerHostile>(world, &player_box);
  if let Some((damage, _)) = damage {
    let PlayerQuery { combat, health, .. } = use_player(world);
    if combat.hit_cooldown.consume_map(ConsumeAction::Restart, || { health.deal(damage); }) {
      combat.hit_cooldown.reset();
    }
  }

  Ok(())
}

/// Damage creatures when colliding with player projectiles
pub fn creature_damage(world: &mut World, state: &mut State) -> Result<(), String> {
  let creatures = world
    .query::<(&Position, &Collider)>()
    .with::<(&PlayerHostile, &Health)>()
    .into_iter()
    .map(|(entity, (position, collider))| {
      (entity, *position, *collider)
    })
    .collect::<Vec<_>>();

  if creatures.is_empty() { return Ok(()); }

  let dead_creatures = creatures
    .iter()
    .filter_map(|(creature, creature_position, creature_collider)| {
      let creature_box = make_collision_box(creature_position, creature_collider);
      let damage = get_damage::<CreatureHostile>(world, &creature_box);
      if let Some((damage, entity)) = damage {
        let frosty_projectile = world.has_component::<IceBeam>(entity).expect("Failed to check ice_beam component");
        let creature_frozen = world.has_component::<Frozen>(*creature).expect("Failed to check frozen component");

        world.free_now(entity).expect("Failed to free projectile");

        if frosty_projectile {
          if freeze_entity(*creature, creature_collider.0, world, THAW_DURATION).expect("Failed to freeze entity") {
            // freeze sfx
          } else {
            // invalid sfx
          }
        } else if !creature_frozen {
          let mut health = world
            .get_component_mut::<Health>(*creature)
            .expect("Creature should have health");
          if health.deal(damage) == LiveState::Dead { return Some(*creature); }
        }
      }
      return None;
    })
    .collect::<Vec<_>>();

  let room = use_room(state);
  for entity in dead_creatures {
    room.remove_entity(entity, world, TilemapMutation::Session)?; // creatures stay dead during the session
  }

  Ok(())
}
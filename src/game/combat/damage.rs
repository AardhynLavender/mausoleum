use std::collections::HashMap;

use hecs::{Component, Entity, Or};

use crate::engine::geometry::collision::{CollisionBox, CollisionMask, rec2_collision};
use crate::engine::state::State;
use crate::engine::system::SysArgs;
use crate::engine::tile::tile::TileCollider;
use crate::engine::time::ConsumeAction;
use crate::engine::world::World;
use crate::game::combat::health::{Health, LiveState};
use crate::game::physics::collision::Collider;
use crate::game::physics::position::Position;
use crate::game::player::combat::{CreatureHostile, PlayerHostile};
use crate::game::player::world::{PQ, use_player};
use crate::game::scene::level::room::use_room;

pub struct Damage {
  pub amount: i32,
}

impl Damage {
  pub fn new(amount: u32) -> Self {
    Self { amount: amount as i32 }
  }
}

fn get_damage<Mask>(world: &mut World, collision_box: &CollisionBox) -> Option<(i32, Entity)> where Mask: Component {
  for (entity, (position, collider, damage)) in world
    .query::<(&Position, Or<&TileCollider, &Collider>, &Damage)>()
    .with::<&Mask>()
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

// Deal damage to creatures and the player
pub fn sys_damage(SysArgs { world, state, .. }: &mut SysArgs) {
  player_damage(world);
  creature_damage(world, state);
}

// Damage the player when colliding with dangerous entities
pub fn player_damage(world: &mut World) {
  let PQ { position, collider, .. } = use_player(world);
  let player_box = CollisionBox::new(position.0, collider.0.size);

  let damage = get_damage::<PlayerHostile>(world, &player_box);
  if let Some((damage, _)) = damage {
    let PQ { combat, health, .. } = use_player(world);
    if combat.hit_cooldown.consume_map(ConsumeAction::Restart, || { health.deal(damage); }) {
      combat.hit_cooldown.reset();
    }
  }
}

/// Damage creatures when colliding with player projectiles
pub fn creature_damage(world: &mut World, state: &mut State) {
  let creatures = world
    .query::<(&Position, &Collider)>()
    .with::<(&PlayerHostile, &Health)>()
    .into_iter()
    .map(|(entity, (position, collider))| {
      (entity, CollisionBox::new(position.0, collider.0.size))
    })
    .collect::<HashMap<_, _>>();

  let dead_creatures = creatures
    .iter()
    .filter_map(|(creature, creature_box)| {
      let damage = get_damage::<CreatureHostile>(world, creature_box);
      if let Some((damage, entity)) = damage {
        world.free_now(entity).expect("Failed to free projectile");
        let mut health = world
          .get_component_mut::<Health>(*creature)
          .expect("Creature should have health");
        if health.deal(damage) == LiveState::Dead { return Some(*creature); }
      }
      return None;
    })
    .collect::<Vec<_>>();

  let room = use_room(state);
  for entity in dead_creatures {
    room.remove_entity(entity, world);
  }
}
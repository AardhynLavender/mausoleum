use std::collections::HashMap;

use hecs::{Component, Entity};

use crate::engine::geometry::collision::{CollisionBox, CollisionMask, rec2_collision};
use crate::engine::state::State;
use crate::engine::system::SysArgs;
use crate::engine::time::ConsumeAction;
use crate::engine::world::World;
use crate::game::combat::health::{Health, LiveState};
use crate::game::creature::Creature;
use crate::game::physics::collision::Collider;
use crate::game::physics::position::Position;
use crate::game::player::component::PlayerProjectile;
use crate::game::player::world::use_player;
use crate::game::room::use_room;

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
    .query::<(&Position, &Collider, &Damage)>()
    .with::<&Mask>()
  {
    let creature_box = CollisionBox::new(collider.0.origin + position.0, collider.0.size);
    if rec2_collision(collision_box, &creature_box, CollisionMask::default()).is_some() {
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
  let (_, position, .., collider, _) = use_player(world);
  let player_box = CollisionBox::new(position.0, collider.0.size);

  let damage = get_damage::<Creature>(world, &player_box);
  if let Some((damage, _)) = damage {
    let (data, .., health) = use_player(world);
    if data.hit_cooldown.consume_map(ConsumeAction::Restart, || { health.deal(damage); }) {
      data.hit_cooldown.reset();
    }
  }
}

/// Damage creatures when colliding with player projectiles
pub fn creature_damage(world: &mut World, state: &mut State) {
  let creatures = world
    .query::<(&Position, &Collider)>()
    .with::<(&Creature, &Health)>()
    .into_iter()
    .map(|(entity, (position, collider))| {
      (entity, CollisionBox::new(position.0, collider.0.size))
    })
    .collect::<HashMap<_, _>>();

  let dead_creatures = creatures
    .iter()
    .filter_map(|(creature, creature_box)| {
      let damage = get_damage::<PlayerProjectile>(world, creature_box);
      if let Some((damage, projectile)) = damage {
        world.free_now(projectile).expect("Failed to free projectile");
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
    room.remove_entity(entity, world).expect("Failed to despawn creature");
  }
}
use crate::engine::geometry::collision::{CollisionBox, CollisionMask, rec2_collision};
use crate::engine::system::SysArgs;
use crate::engine::time::ConsumeAction;
use crate::engine::world::World;
use crate::game::physics::collision::Collider;
use crate::game::physics::position::Position;
use crate::game::player::world::use_player;

pub struct Damage {
  pub amount: i32,
}

impl Damage {
  pub fn new(amount: u32) -> Self {
    Self { amount: amount as i32 }
  }
}

fn get_damage(world: &mut World, player_box: CollisionBox) -> i32 {
  for (.., (position, collider, damage)) in world.query::<(&Position, &Collider, &Damage)>() {
    let creature_box = CollisionBox::new(collider.0.origin + position.0, collider.0.size);
    if rec2_collision(&player_box, &creature_box, CollisionMask::default()).is_some() {
      return damage.amount;
    }
  }
  return 0;
}

// Deal damage to the player if they collide with entities with `Damage` and `Collider` components
pub fn sys_damage(SysArgs { world, .. }: &mut SysArgs) {
  let (_, position, .., collider, _) = use_player(world);

  let player_box = CollisionBox::new(position.0, collider.0.size);
  let damage = get_damage(world, player_box);
  if damage <= 0 { return; }

  let (data, .., health) = use_player(world);
  if data.hit_cooldown.consume_map(ConsumeAction::Restart, || { health.deal(damage); }) {
    data.hit_cooldown.reset();
  }
}

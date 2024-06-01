use regex::Regex;

/**
 * Assertions for save data
 */

use crate::engine::utility::container::count_if;
use crate::engine::utility::invariant::invariant;
use crate::game::persistence::constant::{MAX_HEALTH_PICKUPS, SINGLE_PICKUP};
use crate::game::scene::level::meta::{Collectable, Item};

pub fn is_collectable(collectable: Collectable) -> impl Fn(&Item) -> bool {
  move |item: &Item| item.collectable == collectable
}

pub fn assert_inventory(inventory: &Vec<Item>) -> Result<(), String> {
  invariant(count_if(inventory.iter(), is_collectable(Collectable::MissileTank)) <= SINGLE_PICKUP, "Too many missile tanks")?;
  invariant(count_if(inventory.iter(), is_collectable(Collectable::IceBeam)) <= SINGLE_PICKUP, "Too many ice beams")?;
  invariant(count_if(inventory.iter(), is_collectable(Collectable::HighJump)) <= SINGLE_PICKUP, "Too many high jumps")?;
  invariant(count_if(inventory.iter(), is_collectable(Collectable::Health)) <= MAX_HEALTH_PICKUPS, "Too many health")?;
  Ok(())
}

pub fn assert_save_room(save_room: &String) -> Result<(), String> {
  let save_regex = Regex::new(r"^\s*save_(?:[0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\s*$").expect("Invalid regex");
  invariant(save_regex.is_match(save_room), format!("Invalid save room: {}", save_room))?;
  Ok(())
}

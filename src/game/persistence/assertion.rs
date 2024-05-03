use regex::Regex;

/**
 * Assertions for save data
 */

use crate::engine::utility::container::count_item;
use crate::engine::utility::invariant::invariant;
use crate::game::persistence::constant::{MAX_HEALTH_PICKUPS, SINGLE_PICKUP};
use crate::game::scene::level::meta::Collectable;

pub fn assert_inventory(inventory: &Vec<Collectable>) -> Result<(), String> {
  invariant(count_item(&Collectable::MissileTank, inventory.iter()) <= SINGLE_PICKUP, "Too many missile tanks")?;
  invariant(count_item(&Collectable::IceBeam, inventory.iter()) <= SINGLE_PICKUP, "Too many ice beams")?;
  invariant(count_item(&Collectable::HighJump, inventory.iter()) <= SINGLE_PICKUP, "Too many high jumps")?;
  invariant(count_item(&Collectable::Health, inventory.iter()) <= MAX_HEALTH_PICKUPS, "Too many health")
}

pub fn assert_save_room(save_room: &String) -> Result<(), String> {
  let save_regex = Regex::new(r"^\s*save_(?:[0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\s*$").expect("Invalid regex");
  invariant(save_regex.is_match(save_room), format!("Invalid save room: {}", save_room))?;
  Ok(())
}

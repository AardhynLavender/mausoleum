use serde::Deserialize;

use crate::engine::tile::tile::TileKey;
use crate::engine::utility::io::{deserialize_json, read_file};
use crate::game::scene::level::meta::Collectable;

pub const COLLECTABLE_DATA_PATH: &str = "data/collectable.json";

pub fn deserialize_weapon_data() -> Result<CollectableData, String> {
  let serialized = read_file(COLLECTABLE_DATA_PATH)?;
  deserialize_json(&serialized)
}

#[derive(Deserialize, Debug, Clone)]
#[allow(unused)]
pub struct CollectableItemData {
  pub name: String,
  pub description: String,
  pub key: Option<String>,
  pub tile: TileKey,
}

#[derive(Deserialize, Debug)]
pub struct CollectableData {
  pub bullet: CollectableItemData,
  pub missile_tank: CollectableItemData,
  pub ice_beam: CollectableItemData,
  pub high_jump: CollectableItemData,
  pub health: CollectableItemData,
}

impl CollectableData {
  pub fn get_data(&self, collectable: &Collectable) -> &CollectableItemData {
    match collectable {
      Collectable::MissileTank => &self.missile_tank,
      Collectable::IceBeam => &self.ice_beam,
      Collectable::HighJump => &self.high_jump,
      Collectable::Health => &self.health,
    }
  }
}
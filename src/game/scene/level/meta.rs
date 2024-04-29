/*
 * Manage metadata for layers, tiles, and objects
 */

use crate::engine::geometry::shape::Vec2;
use crate::engine::tile::parse::{TiledObject, TiledProperties, TiledProperty};
use crate::engine::utility::direction::Direction;

/// The type of collectable item
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CollectableType {
  Health,
  Power,
  MissileTank,
  IceBeam,
  HighJump,
}

/// Defines the level of breakability for a tile
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum TileBreakability {
  #[default]
  Solid,
  Strong,
  Soft,
}

/// Strong tiles can be destroyed by StrongDestructive components
#[derive(Default)]
pub struct Strong;

/// Soft tiles can be destroyed by SoftDestructive components
#[derive(Default)]
pub struct Soft;

/// Name of the custom class for a tile in Tiled
pub const TILED_TILE_CLASS: &str = "Tile";

/// Metadata for a tileset tile
#[derive(Default, Clone, Copy, Debug)]
pub struct TileMeta {
  pub breakability: TileBreakability,
  pub collectable: Option<CollectableType>,
  pub damage: u32,
}

/// Metadata for a tilemap object
#[derive(Clone, Copy, Debug)]
pub enum ObjMeta {
  BuzzConcept { position: Vec2<f32> },
  SpikyConcept { direction: Direction, position: Vec2<f32> },
  ZoomerConcept { direction: Direction, position: Vec2<f32> },
  RipperConcept { direction: Direction, position: Vec2<f32> },
}

/// The behaviour and rendering order of a tile layer
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TileLayerType {
  Foreground,
  #[default]
  Collision,
  Background,
}

/// Extract a specific property from a collection of properties
pub fn get_property(name: impl Into<String>, properties: &Option<TiledProperties>) -> Option<&str> {
  let name = name.into();
  if let Some(properties) = properties {
    for TiledProperty { name: prop, value, .. } in &properties.properties {
      if prop == &name { return Some(value.as_str()); }
    }
  }
  None
}

pub fn parse_object(TiledObject { object_type, properties, x, y, .. }: &TiledObject) -> Result<ObjMeta, String> {
  let position = Vec2::new(*x, *y);
  let object_type = object_type.trim().to_lowercase();
  let meta = match object_type.as_str() {
    "buzz" => ObjMeta::BuzzConcept { position },
    "ripper" => {
      let direction = Direction::try_from(String::from(get_property("direction", properties).unwrap_or(""))).unwrap_or(Direction::Right);
      ObjMeta::RipperConcept { direction, position }
    }
    "spiky" => {
      let direction = Direction::try_from(String::from(get_property("direction", properties).unwrap_or(""))).unwrap_or(Direction::Right);
      ObjMeta::SpikyConcept { direction, position }
    }
    "zoomer" => {
      let direction = Direction::try_from(String::from(get_property("direction", properties).unwrap_or(""))).unwrap_or(Direction::Right);
      ObjMeta::ZoomerConcept { direction, position }
    }
    _ => return Err(String::from(format!("Unknown object type: {}", object_type))),
  };
  Ok(meta)
}

pub fn parse_collectable(properties: &Option<TiledProperties>) -> Result<Option<CollectableType>, String> {
  if let Some(prop) = get_property("collectable", properties) {
    let collectable_type = prop.trim().to_lowercase();
    let collectable = match collectable_type.as_str() {
      "health" => CollectableType::Health,
      "power" => CollectableType::Power,
      "missile_tank" => CollectableType::MissileTank,
      "ice_beam" => CollectableType::IceBeam,
      "high_jump" => CollectableType::HighJump,
      "none" => return Err(String::from("Collectable must have a type")),
      other => return Err(format!("Invalid collectable type: {}", other)),
    };
    return Ok(Some(collectable));
  }
  Ok(None)
}

/// Extract the breakability property from a collection of properties
pub fn parse_breakability(properties: &Option<TiledProperties>) -> Result<TileBreakability, String> {
  let prop = get_property("breakability", properties);
  if let Some(prop) = prop {
    let tile_type = prop.trim().to_lowercase();
    return match tile_type.as_str() {
      "solid" => Ok(TileBreakability::Solid),
      "strong" => Ok(TileBreakability::Strong),
      "soft" => Ok(TileBreakability::Soft),
      other => Err(format!("Invalid breakability: {}", other)),
    };
  }
  Ok(TileBreakability::default())
}

pub fn parse_tilelayer(properties: &Option<TiledProperties>) -> Result<TileLayerType, String> {
  if let Some(prop) = get_property("type", properties) {
    let layer_type = prop.trim().to_lowercase();
    return match layer_type.as_str() {
      "foreground" => Ok(TileLayerType::Foreground),
      "collision" => Ok(TileLayerType::Collision),
      "background" => Ok(TileLayerType::Background),
      _ => Err(format!("Invalid layer: {}", prop)),
    };
  }
  Ok(TileLayerType::default())
}

/// Extract the damage property from a collection of properties
pub fn parse_damage(properties: &Option<TiledProperties>) -> Result<u32, String> {
  if let Some(prop) = get_property("damage", properties) {
    let value = prop.parse::<u32>().map_err(|err| err.to_string())?;
    return Ok(value);
  }
  Ok(u32::default())
}

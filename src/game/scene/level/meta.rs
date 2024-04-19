/*
 * Manage metadata for tiles and objects
 */

use crate::engine::tile::parse::{TiledProperties, TiledProperty};

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

/// Metadata for a tile
#[derive(Clone, Copy, Debug)]
pub struct TileMeta {
  pub breakability: TileBreakability,
  pub damage: u32,
}

/// Extract a specific property from a collection of properties
pub fn use_property(name: impl Into<String>, properties: &Option<TiledProperties>) -> Option<&str> {
  let name = name.into();
  if let Some(properties) = properties {
    for TiledProperty { name: prop, value, .. } in &properties.properties {
      if prop == &name { return Some(value.as_str()); }
    }
  }
  None
}

/// Extract the breakability property from a collection of properties
pub fn use_breakability(properties: &Option<TiledProperties>) -> Result<TileBreakability, String> {
  let prop = use_property("breakability", properties);
  if let Some(prop) = prop {
    return match prop {
      "Solid" => Ok(TileBreakability::Solid),
      "Strong" => Ok(TileBreakability::Strong),
      "Soft" => Ok(TileBreakability::Soft),
      other => Err(format!("Invalid breakability: {}", other)),
    };
  }
  Ok(TileBreakability::default())
}

/// Extract the damage property from a collection of properties
pub fn use_damage(properties: &Option<TiledProperties>) -> Result<u32, String> {
  if let Some(prop) = use_property("damage", properties) {
    let value = prop.parse::<u32>().map_err(|err| err.to_string())?;
    return Ok(value);
  }
  Ok(u32::default())
}

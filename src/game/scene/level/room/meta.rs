/*
 * Parse metadata for layers, tiles, and objects
 */

use serde::{Deserialize, Serialize};
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::utility::alias::{Size, Size2};
use crate::engine::utility::direction::{CompassDirectionType, Direction};
use crate::game::scene::level::room::collision::{CollisionBox, RoomCollision};
use crate::game::scene::level::tile::tiled::{TiledObject, TiledProperties, TiledProperty};
use crate::game::scene::level::tile::tilemap::MapIndex;

/// An item collectable by Collections
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Collectable {
  Health,
  MissileTank,
  IceBeam,
  HighJump,
}

// reduce an Item into a Collectable
impl From<Item> for Collectable {
  fn from(item: Item) -> Self { item.collectable }
}

/// A Collectable that has been collected by the player
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Item {
  pub collectable: Collectable,
  pub map_index: MapIndex,
  pub room_name: String,
}

/// Defines the level of breakability for a tile
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum TileBreakability {
  #[default]
  Solid,
  Strong,
  Soft,
  Brittle,
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
#[derive(Default, Clone, Debug)]
pub struct TileMeta {
  pub breakability: TileBreakability,
  pub collectable: Option<Collectable>,
  pub collision_layer: RoomCollision,
  pub damage: u32,
}

/// Metadata for a tilemap object
#[derive(Clone, Debug)]
pub enum ObjMeta {
  AngryBuzzConcept { position: Vec2<f32> },
  BubblyConcept { position: Vec2<f32>, direction: Direction },
  BuzzConcept { position: Vec2<f32> },
  StoryConcept { position: Vec2<f32>, collision_box: CollisionBox, key: String },
  GruntConcept { position: Vec2<f32> },
  SaveAreaConcept { position: Vec2<f32>, collision_box: CollisionBox },
  SpikyConcept { direction: Direction, position: Vec2<f32> },
  SporeConcept { direction: Direction, position: Vec2<f32> },
  RipperConcept { direction: Direction, position: Vec2<f32> },
  RotundConcept { direction: Direction, position: Vec2<f32>, spit_axis: CompassDirectionType },
  ZoomerConcept { direction: Direction, position: Vec2<f32> },
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

/// Parse a TiledObject into an ObjMeta
pub fn parse_object(TiledObject { name, object_type, properties, x, y, width, height, .. }: &TiledObject) -> Result<ObjMeta, String> {
  let position = Vec2::new(*x, *y);
  let object_type = object_type.trim().to_lowercase();
  let meta = match object_type.as_str() {
    "angrybuzz" => ObjMeta::AngryBuzzConcept { position },
    "buzz" => ObjMeta::BuzzConcept { position },
    "bubbly" => {
      let direction = parse_direction(properties).unwrap_or(Direction::UpRight);
      ObjMeta::BubblyConcept { direction, position }
    }
    "grunt" => ObjMeta::GruntConcept { position },
    "save" => {
      if let (Some(width), Some(height)) = (*width, *height) {
        let bounds = Rec2::new(position, Size2::new(width as Size, height as Size));
        return Ok(ObjMeta::SaveAreaConcept { position, collision_box: CollisionBox::from(bounds) });
      }
      return Err(String::from("Save area must have a width and height"));
    }
    "spore" => {
      let direction = parse_direction( properties).unwrap_or(Direction::Up);
      ObjMeta::SporeConcept { direction, position }
    }
    "spiky" => {
      let direction = parse_direction(properties).unwrap_or(Direction::Right);
      ObjMeta::SpikyConcept { direction, position }
    }
    "ripper" => {
      let direction = parse_direction(properties).unwrap_or(Direction::Right);
      ObjMeta::RipperConcept { direction, position }
    }
    "rotund" => {
      let direction = parse_direction(properties).unwrap_or(Direction::UpRight);
      let spit_axis = parse_compass_direction_type("spit_axis", properties).unwrap_or(CompassDirectionType::Ordinal);
      ObjMeta::RotundConcept { direction, position, spit_axis }
    }
    "zoomer" => {
      let direction = parse_direction(properties).unwrap_or(Direction::Right);
      ObjMeta::ZoomerConcept { direction, position }
    }
    "story" => {
      if let Some(key) = name {
        if let (Some(width), Some(height)) = (*width, *height) {
          let bounds = Rec2::new(position, Size2::new(width as Size, height as Size));
          return Ok(ObjMeta::StoryConcept { position, collision_box: CollisionBox::from(bounds), key: key.clone() });
        }
        return Err(String::from("Save area must have a width and height"));
      }
      return Err(String::from("Story object must have a name"));
    }
    _ => return Err(String::from(format!("Unknown object type: {}", object_type))),
  };
  Ok(meta)
}

/// Parse a tiled property into a Collectable
///
/// todo: implement From<String> for Collectable
pub fn parse_collectable(properties: &Option<TiledProperties>) -> Result<Option<Collectable>, String> {
  if let Some(prop) = get_property("collectable", properties) {
    let collectable_type = prop.trim().to_lowercase();
    let collectable = match collectable_type.as_str() {
      "health" => Collectable::Health,
      "missile_tank" => Collectable::MissileTank,
      "ice_beam" => Collectable::IceBeam,
      "high_jump" => Collectable::HighJump,
      "none" => return Err(String::from("Collectable must have a type")),
      other => return Err(format!("Invalid collectable type: {}", other)),
    };
    return Ok(Some(collectable));
  }
  Ok(None)
}

/// Parse a tiled property into a RoomCollision
///
/// todo: implement From<String> for RoomCollision
pub fn parse_collision_layer(properties: &Option<TiledProperties>) -> Result<RoomCollision, String> {
  if let Some(prop) = get_property("collision_layer", properties) {
    let layer_type = prop.trim().to_lowercase();
    return match layer_type.as_str() {
      "all" => Ok(RoomCollision::All),
      "creature" => Ok(RoomCollision::Creature),
      "player" => Ok(RoomCollision::Player),
      other => Err(format!("Invalid collision layer: {}", other)),
    };
  }
  Ok(RoomCollision::default())
}

/// Parse a tiled property into a TileBreakability
///
/// todo: implement From<String> for TileBreakability
pub fn parse_breakability(properties: &Option<TiledProperties>) -> Result<TileBreakability, String> {
  let prop = get_property("breakability", properties);
  if let Some(prop) = prop {
    let tile_type = prop.trim().to_lowercase();
    return match tile_type.as_str() {
      "solid" => Ok(TileBreakability::Solid),
      "strong" => Ok(TileBreakability::Strong),
      "soft" => Ok(TileBreakability::Soft),
      "brittle" => Ok(TileBreakability::Brittle),
      other => Err(format!("Invalid breakability: {}", other)),
    };
  }
  Ok(TileBreakability::default())
}

/// Parse a tiled property into a TileLayerType
///
/// todo: implement From<String> for TileLayerType
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

/// Parse a compass direction property from a collection of properties
pub fn parse_compass_direction_type(property: &str, properties: &Option<TiledProperties>) -> Result<CompassDirectionType, String> {
  if let Some(prop) = get_property(property, properties) {
    let direction = CompassDirectionType::try_from(String::from(prop)).map_err(|err| err.to_string())?;
    return Ok(direction);
  }
  Err(String::from("Direction property not found"))
}

/// Parse a direction property from a collection of properties
pub fn parse_direction(properties: &Option<TiledProperties>) -> Result<Direction, String> {
  if let Some(prop) = get_property("direction", properties) {
    let direction = Direction::try_from(String::from(prop)).map_err(|err| err.to_string())?;
    return Ok(direction);
  }
  Err(String::from("Direction property not found"))
}

/// Parse a damage property from a collection of properties
pub fn parse_damage(property: &str, properties: &Option<TiledProperties>) -> Result<u32, String> {
  if let Some(prop) = get_property(property, properties) {
    let value = prop.parse::<u32>().map_err(|err| err.to_string())?;
    return Ok(value);
  }
  Ok(u32::default())
}

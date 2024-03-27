use std::fmt::Debug;
use std::path::Path;

use serde::Deserialize;

/**
 * Parse Tiled data into Rust structures.
 */

// Tiled World (*.world) //
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct TiledWorldMap {
  pub file_name: String,
  pub height: u32,
  pub width: u32,
  pub x: i32,
  pub y: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct TiledWorld {
  pub maps: Vec<TiledWorldMap>,
  pub only_show_adjacent_maps: bool,
  #[serde(rename = "type")]
  pub world_type: String,
}

// Tiled Tilemap (*.tmx) //

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct TiledTilemap {
  #[serde(rename = "@source")]
  pub source: String,
  #[serde(rename = "@width")]
  pub width: u32,
  #[serde(rename = "@height")]
  pub height: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct TiledTileset {
  #[serde(rename = "@version")]
  pub version: String,
  #[serde(rename = "@tiledversion")]
  pub tiled_version: String,
  #[serde(rename = "@tilewidth")]
  pub tile_width: u32,
  #[serde(rename = "@tileheight")]
  pub tile_height: u32,
  #[serde(rename = "@tilecount")]
  pub tile_count: u32,
  #[serde(rename = "@columns")]
  pub columns: u32,

  pub image: TiledTilemap,
}

// Bundle a Tiled tileset with its path
#[derive(Debug)]
pub struct TiledTilesetWithPath {
  pub path: Box<Path>,
  pub tileset: TiledTileset,
}

// Tiled Tileset (*.tsx) //

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct TiledTilesetReference {
  #[serde(rename = "@firstgid")]
  pub first_gid: u32,
  #[serde(rename = "@source")]
  pub source: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct TiledLayerData {
  // attributes //
  #[serde(rename = "@encoding")]
  pub encoding: String,
  // children //
  #[serde(rename = "$value")]
  pub tiles: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct TiledLayer {
  #[serde(rename = "@id")]
  pub id: u32,
  #[serde(rename = "@name")]
  pub name: String,
  #[serde(rename = "@width")]
  pub width_tiles: u32,
  #[serde(rename = "@height")]
  pub height_tiles: u32,

  pub data: TiledLayerData,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct TiledTileMap {
  // attributes //
  #[serde(rename = "@version")]
  pub version: String,
  #[serde(rename = "@tiledversion")]
  pub tiled_version: String,
  #[serde(rename = "@orientation")]
  pub orientation: String,
  #[serde(rename = "@renderorder")]
  pub render_order: String,
  #[serde(rename = "@width")]
  pub width_tiles: u32,
  #[serde(rename = "@height")]
  pub height_tiles: u32,
  #[serde(rename = "@infinite")]
  pub infinite: u8,
  #[serde(rename = "@nextlayerid")]
  pub next_layer_id: u32,
  #[serde(rename = "@nextobjectid")]
  pub next_object_id: u32,

  pub tileset: Vec<TiledTilesetReference>,
  pub layer: Vec<TiledLayer>,
}

// Parser //

#[allow(unused)]
#[derive(Debug)]
/// Parse Tiled data into Rust structures.
pub struct TiledParser {
  pub tilesets: Vec<TiledTilesetWithPath>,
  pub tilemaps: Vec<TiledTileMap>,
  pub world: TiledWorld,
}

/// Result of parsing a Tiled tilemap.
#[derive(Debug)]
pub struct ParsedTiledTilemap {
  pub tilemap: TiledTileMap,
  pub tilesets: Vec<Box<Path>>,
}

#[allow(unused)]
impl TiledParser {
  pub fn parse(world_file: &Path) -> Result<Self, String> {
    // parse the world file
    let world = Self::parse_world(world_file).map_err(|e| e.to_string())?;
    let world_parent = world_file.parent().ok_or("Failed to get world directory")?;

    // parse tilemaps of the world
    let parsed_tilemaps = world.maps.iter().map(|map| {
      let tilemap_path = world_parent.join(&map.file_name).into_boxed_path();
      Self::parse_tilemap(&*tilemap_path)
    }).collect::<Result<Vec<ParsedTiledTilemap>, String>>()?;

    // separate the tilemaps from their referenced tilesets
    let (tilemaps, tileset_paths_nested): (Vec<TiledTileMap>, Vec<Vec<Box<Path>>>) = parsed_tilemaps
      .into_iter()
      .map(|parsed_tilemap| (parsed_tilemap.tilemap, parsed_tilemap.tilesets))
      .unzip();
    let mut tileset_paths = tileset_paths_nested
      .into_iter()
      .flatten()
      .collect::<Vec<Box<Path>>>();

    // remove duplicate tileset paths
    tileset_paths.sort();
    tileset_paths.dedup();

    // parse tilesets
    let tilesets = tileset_paths.into_iter().map(|tileset_path| {
      Self::parse_tileset(tileset_path).map_err(|e| e.to_string())
    }).collect::<Result<Vec<TiledTilesetWithPath>, String>>()?;

    Ok(TiledParser {
      world,
      tilemaps,
      tilesets,
    })
  }

  /// Parse a Tiled world file
  fn parse_world(world_file: &Path) -> Result<TiledWorld, String> {
    let world_str = std::fs::read_to_string(world_file).map_err(|e| e.to_string())?;
    let world: TiledWorld = serde_json::from_str(&world_str).map_err(|e| e.to_string())?;

    Ok(world)
  }

  /// Parse a Tiled tilemap file
  /// Returns the parsed tilemap and the paths to the tilesets it references.
  fn parse_tilemap(tilemap_path: &Path) -> Result<ParsedTiledTilemap, String> {
    // parse tilemap
    let tilemap_str = std::fs::read_to_string(tilemap_path).map_err(|e| e.to_string())?;
    let tilemap: TiledTileMap = quick_xml::de::from_str(&tilemap_str).map_err(|e| e.to_string())?;

    // get tileset paths
    let tileset_paths = tilemap.tileset.iter().map(|tileset| {
      let parent = tilemap_path
        .parent()
        .ok_or("Failed to get tilemap parent")?;
      let tileset_path = parent
        .join(&tileset.source)
        .into_boxed_path();

      Ok::<Box<Path>, String>(tileset_path)
    }).collect::<Vec<_>>();

    // handle errors in the tileset paths
    let tilesets = tileset_paths.into_iter().collect::<Result<Vec<_>, _>>()?;

    Ok(ParsedTiledTilemap {
      tilemap,
      tilesets,
    })
  }

  /// Parse a Tiled tileset file
  fn parse_tileset(path: Box<Path>) -> Result<TiledTilesetWithPath, String> {
    let tileset_str = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let tileset = quick_xml::de::from_str(&tileset_str).map_err(|e| e.to_string())?;

    Ok(TiledTilesetWithPath { tileset, path })
  }
}
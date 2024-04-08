use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;

use serde::Deserialize;

/**
 * Parse Tiled data into Rust structures.
 */

/// A reference to a Tiled tilemap in a Tiled world file
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TiledWorldReferenceMap {
  pub file_name: String,
  pub height: u32,
  pub width: u32,
  pub x: i32,
  pub y: i32,
}

/// A Tiled world .world file
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TiledWorld {
  pub maps: Vec<TiledWorldReferenceMap>,
  pub only_show_adjacent_maps: bool,
  #[serde(rename = "type")]
  pub world_type: String,
}

/// A Tiled tilesets image reference
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TiledImage {
  #[serde(rename = "@source")]
  pub source: String,
  #[serde(rename = "@width")]
  pub width: u32,
  #[serde(rename = "@height")]
  pub height: u32,
}

/// A Tiled tileset .tsx file
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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

  pub image: TiledImage,
}

/// A Tilemaps reference to a Tiled tileset
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TiledTilesetReference {
  #[serde(rename = "@firstgid")]
  pub first_gid: u32,
  #[serde(rename = "@source")]
  pub source: String,
}

/// Tiled layer tile data
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TiledLayerData {
  #[serde(rename = "@encoding")]
  pub encoding: String,

  #[serde(rename = "$value")]
  pub tiles: String,
}

// A tile layer within a Tiled tilemap
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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

// A Tiled tilemap .tmx file
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TiledTilemap {
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

/// A parsed Tiled tilemap and a vector of tileset paths it references.
#[derive(Debug)]
pub struct ParsedTiledTilemap {
  pub tilemap: TiledTilemap,
  pub tileset_paths: Vec<Box<Path>>,
}

/// Parse Tiled data into Rust structures.
pub struct TiledParser {
  pub tilesets: HashMap<Box<Path>, TiledTileset>,
  pub tilemaps: HashMap<Box<Path>, TiledTilemap>,
  pub world: TiledWorld,
}

impl TiledParser {
  /// Parse a Tiled world file into it's tilemaps and tilesets
  pub fn parse(world_file: &Path) -> Result<Self, String> {
    // parse the world file
    let world = Self::parse_world(world_file).map_err(|e| e.to_string())?;
    let world_parent = world_file.parent().ok_or("Failed to get world directory")?;

    let mut tilesets = HashMap::new();
    let mut tilemaps = HashMap::new();

    // parse tilemaps in the world
    for tilemap_reference in &world.maps {
      // parse the tilemap
      let tilemap_path = world_parent.join(&tilemap_reference.file_name).into_boxed_path();
      let ParsedTiledTilemap { tilemap, tileset_paths } = Self::parse_tilemap(&*tilemap_path)?;
      tilemaps.insert(tilemap_path, tilemap);

      // parse the tilesets referenced by the tilemap
      for tileset_path in tileset_paths {
        if tilesets.contains_key(&tileset_path) {
          continue; // already parsed
        }

        let tileset = Self::parse_tileset(&tileset_path)?;
        tilesets.insert(tileset_path, tileset);
      }
    }

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
  /// Returns a parsed tilemap and the paths to the tilesets it references.
  fn parse_tilemap(path: impl AsRef<Path>) -> Result<ParsedTiledTilemap, String> {
    let tilemap_path = path.as_ref();

    // parse tilemap
    let tilemap_str = std::fs::read_to_string(tilemap_path).map_err(|e| e.to_string())?;
    let tilemap: TiledTilemap = quick_xml::de::from_str(&tilemap_str).map_err(|e| e.to_string())?;

    // get tileset paths
    let tileset_paths = tilemap.tileset
      .iter()
      .map(|tileset| {
        let parent = tilemap_path
          .parent()
          .ok_or("Failed to get tilemap parent")?;
        let tileset_path = parent
          .join(&tileset.source)
          .into_boxed_path();

        Ok::<Box<Path>, String>(tileset_path)
      }).collect::<Vec<_>>();

    // handle any errors in the tileset paths
    let tilesets = tileset_paths
      .into_iter()
      .collect::<Result<Vec<_>, _>>()?;

    Ok(ParsedTiledTilemap {
      tilemap,
      tileset_paths: tilesets,
    })
  }

  /// Parse a Tiled tileset file
  fn parse_tileset(path: impl AsRef<Path>) -> Result<TiledTileset, String> {
    let tileset_str = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let tileset = quick_xml::de::from_str(&tileset_str).map_err(|e| e.to_string())?;

    Ok(tileset)
  }
}
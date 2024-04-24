/**
 * Build Entities and Components from Tiled data structures.
 */

use std::collections::HashMap;
use std::path::Path;

use crate::engine::asset::AssetManager;
use crate::engine::tile::parse::{TiledObjectGroup, TiledTilemap, TiledTilemapChildren, TiledTileset};
use crate::engine::tile::tile::TileKey;
use crate::engine::tile::tilemap::Tilemap;
use crate::engine::tile::tileset::Tileset;
use crate::engine::utility::alias::Size2;
use crate::engine::utility::text::{COMMA, strip_newlines};
use crate::game::scene::level::meta::{ObjMeta, parse_breakability, parse_damage, parse_object, TILED_TILE_CLASS, TileMeta};

/// Delimiter for tile data in .Tiled tmx files
const DELIMITER: char = COMMA;
/// integer key for 'no tile' in Tiled .tmx files
const NULL_TILE: TileKey = 0;
/// integer offset for tiles keys read from Tiled .tmx files
const NULL_TILE_OFFSET: TileKey = 1;
/// Indicates an infinite tilemap.
const INFINITE_TILEMAP: u8 = 1;

pub fn tileset_meta_from_tiled(tiled_tileset: &TiledTileset) -> Result<HashMap<TileKey, TileMeta>, String> {
  let mut meta = HashMap::new();
  for tile in &tiled_tileset.tiles {
    let tile_key = tile.id as TileKey;
    if tile._type != TILED_TILE_CLASS { return Err(format!("Invalid tile type: {}, for tile: {}", tile._type, tile_key)); }
    let breakability = parse_breakability(&tile.properties)?;
    let damage = parse_damage(&tile.properties)?;
    meta.insert(tile_key, TileMeta { breakability, damage });
  }
  Ok(meta)
}

pub fn tilemap_objects_from_tiled(group: &TiledObjectGroup) -> Result<Vec<ObjMeta>, String> {
  if let Some(objects) = &group.objects {
    return objects
      .iter()
      .map(parse_object)
      .collect::<Result<Vec<_>, String>>();
  }
  Ok(vec![])
}

/// Build an engine tileset from a Tiled tileset.
pub fn tileset_from_tiled(assets: &mut AssetManager, path: impl AsRef<Path>, tiled_tileset: &TiledTileset) -> Result<Tileset<TileMeta>, String> {
  let tile_size: Size2 = Size2::new(tiled_tileset.tile_width, tiled_tileset.tile_height);
  let dimensions = Size2::new(tiled_tileset.image.width, tiled_tileset.image.height);

  // load the tilesets texture
  let file_path = path
    .as_ref()
    .parent()
    .ok_or("Failed to get tileset directory")?
    .to_path_buf()
    .join(&tiled_tileset.image.source);
  let texture_key = assets.texture.load(&file_path.into_boxed_path())?;

  let meta = tileset_meta_from_tiled(tiled_tileset)?;

  Tileset::build(texture_key, dimensions, tile_size, meta)
}

/// Build an engine tilemap from a Tiled tilemap.
pub fn tilemap_from_tiled(tiled_tilemap: &TiledTilemap, tiled_tileset: &Tileset<TileMeta>) -> Result<Tilemap<TileMeta, ObjMeta>, String> {
  if tiled_tilemap.infinite == INFINITE_TILEMAP {
    return Err(String::from("Infinite maps are not supported."));
  }

  let dimensions: Size2 = Size2::new(tiled_tilemap.width_tiles, tiled_tilemap.height_tiles);

  let layer = tiled_tilemap
    .children
    .iter()
    .filter_map(|child| match child {
      TiledTilemapChildren::TileLayer(child) => Some(child),
      _ => None
    })
    .next() // todo: support multiple tile layers
    .ok_or("No layer data found")?;

  let objects = tiled_tilemap
    .children
    .iter()
    .filter_map(|child| match child {
      TiledTilemapChildren::ObjectLayer(child) => Some(tilemap_objects_from_tiled(child)),
      _ => None,
    })
    .next()
    .unwrap_or(Ok(vec![]))?;

  // convert raw tile data to a Vec<u32>
  let tile_keys = make_tile_keys(&layer.data.tiles, &DELIMITER);
  let tilemap = Tilemap::build(tiled_tileset, dimensions, tile_keys, objects)?;

  return Ok(tilemap);
}

/// Convert csv tile data into a vector of tile keys.
fn make_tile_keys(raw_data: &String, delimiter: &char) -> Vec<Option<TileKey>> {
  raw_data
    .split(*delimiter)
    .map(parse_tile_key)
    .collect::<Vec<_>>()
}

/// Parse a tile key from a string.
fn parse_tile_key(key: &str) -> Option<TileKey> {
  let key = strip_newlines(key)
    .trim()
    .parse::<TileKey>()
    .unwrap_or(NULL_TILE);

  return if key == NULL_TILE {
    None
  } else {
    // 0 is reserved for 'no tile' in .tmx files, so we offset the key by 1
    Some(key - NULL_TILE_OFFSET)
  };
}

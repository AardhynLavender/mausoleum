use std::path::Path;

use crate::engine::asset::AssetManager;
use crate::engine::tile::parse::{TiledTilemap, TiledTileset};
use crate::engine::tile::tile::TileKey;
use crate::engine::tile::tilemap::Tilemap;
use crate::engine::tile::tileset::Tileset;
use crate::engine::utility::alias::Size2;
use crate::engine::utility::text::{COMMA, strip_newlines};

/**
 * Build Entities and Components from Tiled data structures.
 */

/// Delimiter for tile data in .Tiled tmx files
const DELIMITER: char = COMMA;
/// integer key for 'no tile' in Tiled .tmx files
const NULL_TILE: TileKey = 0;
/// integer offset for tiles keys read from Tiled .tmx files
const NULL_TILE_OFFSET: TileKey = 1;
/// Indicates an infinite tilemap.
const INFINITE_TILEMAP: u8 = 1;

/// Build an engine tileset from a Tiled tileset.
pub fn tileset_from_tiled(assets: &mut AssetManager, path: impl AsRef<Path>, tiled_tileset: &TiledTileset) -> Result<Tileset, String> {
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

  Tileset::build(texture_key, dimensions, tile_size)
}

/// Build an engine tilemap from a Tiled tilemap.
pub fn tilemap_from_tiled(tiled_tilemap: &TiledTilemap, tiled_tileset: &Tileset) -> Result<Tilemap, String> {
  if tiled_tilemap.infinite == INFINITE_TILEMAP {
    return Err(String::from("Infinite maps are not supported."));
  }

  let dimensions: Size2 = Size2::new(tiled_tilemap.width_tiles, tiled_tilemap.height_tiles);

  // todo: support multiple layers... for now, well just take the first
  let layer = tiled_tilemap
    .layer
    .iter()
    .next()
    .ok_or("No layer data found")?;

  // convert raw tile data to a Vec<u32>
  let tile_keys = make_tile_keys(&layer.data.tiles, &DELIMITER);
  let tilemap = Tilemap::build(tiled_tileset, dimensions, tile_keys)?;

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

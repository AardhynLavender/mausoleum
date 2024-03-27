use crate::engine::asset::AssetManager;
use crate::engine::tile::parse::{TiledTileMap, TiledTilesetWithPath};
use crate::engine::tile::tile::TileKey;
use crate::engine::tile::tilemap::Tilemap;
use crate::engine::tile::tileset::Tileset;
use crate::engine::utility::alias::Size2;
use crate::engine::utility::text::{COMMA, strip_newlines};

/**
 * Build Entities and Components from Tiled data structures.
 */

/// Consume a Tiled parser, and build its tilesets and tilemaps.
/// Provide methods to add and remove the tiles from the world.
#[allow(unused)]
struct TiledWorldParser {}

/// Delimiter for tile data in .Tiled tmx files
const DELIMITER: char = COMMA;
/// integer key for 'no tile' in Tiled .tmx files
const NO_TILE_KEY: TileKey = 0;
/// integer offset for tiles keys read from Tiled .tmx files
const NO_TILE_OFFSET: TileKey = 1;
/// Indicates an infinite tilemap.
const INFINITE_TILEMAP: u8 = 1;

pub fn tileset_from_tiled(assets: &mut AssetManager, TiledTilesetWithPath { path: tileset_path, tileset: raw_tileset }: &TiledTilesetWithPath) -> Result<Tileset, String> {
  let tile_size: Size2 = Size2::new(raw_tileset.tile_width, raw_tileset.tile_height);
  let dimensions = Size2::new(raw_tileset.image.width, raw_tileset.image.height);

  // load the tileset texture
  let file_path = tileset_path
    .parent()
    .ok_or("Failed to get tileset directory")?
    .to_path_buf()
    .join(&raw_tileset.image.source);
  let texture_key = assets.texture.load(&file_path.into_boxed_path())?;

  Tileset::build(texture_key, dimensions, tile_size)
}

pub fn tilemap_from_tiled(raw_tilemap: &TiledTileMap, tileset: &Tileset) -> Result<Tilemap, String> {
  if raw_tilemap.infinite == INFINITE_TILEMAP {
    return Err(String::from("Infinite maps are not supported."));
  }

  let dimensions: Size2 = Size2::new(raw_tilemap.width_tiles, raw_tilemap.height_tiles);

  let layer = raw_tilemap
    .layer
    .iter()
    .next()
    .ok_or("No layer data found")?;

  // convert raw tile data to a Vec<u32>
  let tile_keys = make_tile_keys(&layer.data.tiles, &DELIMITER);
  let tilemap = Tilemap::build(tileset, dimensions, tile_keys)?;

  // todo: support multiple layers
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
    .unwrap_or(NO_TILE_KEY);

  return if key == NO_TILE_KEY {
    None
  } else {
    // 0 is reserved for 'no tile' in .tmx files; offset the key by 1
    Some(key - NO_TILE_OFFSET)
  };
}
/**
 * Build Entities and Components from Tiled data structures.
 */

use std::collections::HashMap;
use std::path::Path;

use crate::engine::asset::AssetManager;
use crate::engine::tile::parse::{TiledObjectGroup, TiledTileLayer, TiledTilemap, TiledTilemapChildren, TiledTileset};
use crate::engine::tile::tile::TileKey;
use crate::engine::tile::tilelayer::TileLayer;
use crate::engine::tile::tilemap::Tilemap;
use crate::engine::tile::tileset::Tileset;
use crate::engine::utility::alias::Size2;
use crate::engine::utility::text::{COMMA, strip_newlines};
use crate::game::scene::level::meta::{ObjMeta, parse_breakability, parse_collectable, parse_collision_layer, parse_damage, parse_object, parse_tilelayer, TILED_TILE_CLASS, TileLayerType, TileMeta};

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
    let collectable = parse_collectable(&tile.properties)?;
    let collision_layer = parse_collision_layer(&tile.properties)?;
    let damage = parse_damage("damage", &tile.properties)?;
    meta.insert(tile_key, TileMeta { breakability, collectable, damage, collision_layer });
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

pub fn tilemap_layer_from_tiled(tileset: &Tileset<TileMeta>, tiled_tilelayer: &TiledTileLayer) -> Result<TileLayer<TileLayerType, TileMeta>, String> {
  let meta = parse_tilelayer(&tiled_tilelayer.properties)?;
  let keys = make_tile_keys(&tiled_tilelayer.data.tiles, &DELIMITER);
  let dimensions = Size2::new(tiled_tilelayer.width_tiles, tiled_tilelayer.height_tiles);
  let tiles: Vec<_> = tileset.tiledata_from(&keys, dimensions)?.collect();
  Ok(TileLayer { meta, entities: HashMap::with_capacity(tiles.len()), tiles })
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
pub fn tilemap_from_tiled(tiled_tilemap: &TiledTilemap, tiled_tileset: &Tileset<TileMeta>) -> Result<Tilemap<TileMeta, TileLayerType, ObjMeta>, String> {
  if tiled_tilemap.infinite == INFINITE_TILEMAP {
    return Err(String::from("Infinite maps are not supported."));
  }

  let dimensions: Size2 = Size2::new(tiled_tilemap.width_tiles, tiled_tilemap.height_tiles);

  let layers = tiled_tilemap
    .children
    .iter()
    .filter_map(|child| match child {
      TiledTilemapChildren::TileLayer(layer) => Some(tilemap_layer_from_tiled(&tiled_tileset, layer)),
      _ => None
    })
    .collect::<Result<Vec<_>, _>>()?;

  let objects = tiled_tilemap
    .children
    .iter()
    .filter_map(|child| match child {
      TiledTilemapChildren::ObjectLayer(child) => Some(tilemap_objects_from_tiled(child)),
      _ => None,
    })
    .collect::<Result<Vec<_>, _>>()?
    .into_iter()
    .flatten()
    .collect();

  let tilemap = Tilemap::build(tiled_tileset, dimensions, layers, objects)?;

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

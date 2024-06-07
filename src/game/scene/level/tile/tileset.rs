/**
 * Tileset structure and utilities
 */

use std::collections::HashMap;

use crate::engine::asset::texture::TextureKey;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::math::conversion::{coordinate_to_index, index_to_coordinate};
use crate::engine::utility::alias::{Coordinate, Size2};
use crate::engine::utility::direction::Direction;
use crate::game::scene::level::room::collision::CollisionMask;
use crate::game::scene::level::tile::tile::{TileConcept, TileData, TileKey};

/// builds up the tile data for `dimensions` using `tile_size`
fn make_tiles<Meta>(texture_key: TextureKey, dimensions: Size2, tile_size: Size2, meta: HashMap<TileKey, Meta>) -> Result<Vec<TileData<Meta>>, &'static str> where Meta: Clone {
  let (width, height) = dimensions.destructure();
  if width % tile_size.x != 0 {
    return Err("Tileset width must be divisible by tile size");
  }
  if height % tile_size.y != 0 {
    return Err("Tileset height must be divisible by tile size");
  }

  let mut tiles = Vec::new();
  for y in 0..width / tile_size.y {
    for x in 0..width / tile_size.x {
      let tile_key: TileKey = (y * (width / tile_size.x) + x) as TileKey;
      let tile_position = Vec2::new(x, y) * tile_size;
      let src = Rec2::new(tile_position, tile_size);
      let meta = meta
        .get(&tile_key)
        .cloned()
        .ok_or("Failed to get tile meta")?;
      tiles.push(TileData::new(texture_key, src, tile_key, meta));
    }
  }

  Ok(tiles)
}

const PLAYER_BARRIER_TILE: TileKey = 26;
const CREATURE_BARRIER_TILE: TileKey = 27;

/// Check if a tile is a barrier tile
fn is_barrier_tile(tile: &TileKey) -> bool { *tile == PLAYER_BARRIER_TILE || *tile == CREATURE_BARRIER_TILE }

/// Wrapper for a texture that contains tiles
pub struct Tileset<Meta> {
  pub texture: TextureKey,
  pub tile_size: Size2,
  pub tiles: Vec<TileData<Meta>>,
}

impl<Meta> Tileset<Meta> where Meta: Clone {
  /// Instantiate a new tileset from a `texture` with `tile_size`
  pub fn build(texture: TextureKey, dimensions: Size2, tile_size: Size2, meta: HashMap<TileKey, Meta>) -> Result<Self, String> {
    let tiles = make_tiles(texture, dimensions, tile_size, meta)?;
    Ok(Self {
      texture,
      tile_size,
      tiles,
    })
  }

  /// Get the tile data for `tile_key`
  pub fn get_tile(&self, tile_key: usize) -> Result<TileData<Meta>, String> {
    let data = self.tiles
      .get(tile_key)
      .ok_or("Failed to get tile data")?;
    Ok(data.clone())
  }

  /// Convert a collection of `tile_keys` to `TileData`
  pub fn tiledata_from<'a>(&'a self, tile_data: &Vec<Option<TileKey>>, dimensions: Size2) -> Result<impl Iterator<Item=Option<TileConcept<Meta>>> + 'a, String> {
    let result = tile_data
      .iter()
      .enumerate()
      .map(|(index, tile_key)| {
        if let Some(tile_key) = tile_key {
          let data = self.get_tile(*tile_key as usize)?;
          let coordinate = index_to_coordinate(index, dimensions);
          let borders_barrier = is_barrier_tile(tile_key);
          let mask = CollisionMask::new(
            empty_neighbour(tile_data, borders_barrier, coordinate, Direction::Up, &dimensions),
            empty_neighbour(tile_data, borders_barrier, coordinate, Direction::Right, &dimensions),
            empty_neighbour(tile_data, borders_barrier, coordinate, Direction::Down, &dimensions),
            empty_neighbour(tile_data, borders_barrier, coordinate, Direction::Left, &dimensions),
          );
          let concept = TileConcept::new(data.clone(), coordinate, mask.clone());
          Ok::<Option<TileConcept<Meta>>, String>(Some(concept))
        } else {
          Ok(None)
        }
      })
      .collect::<Result<Vec<_>, _>>()?
      .into_iter();

    Ok(result)
  }
}


/// Check if a neighboring tile at `coordinate` is valid and contains a tile
///
/// Non-barrier tiles are considered empty to barrier tiles, and vice versa.
fn empty_neighbour(data: &Vec<Option<TileKey>>, barrier_border: bool, coordinate: Coordinate, direction: Direction, dimensions: &Size2) -> bool {
  let neighbour = coordinate + direction.to_coordinate();
  let index = coordinate_to_index(&neighbour, *dimensions);
  if let Some(Some(tile)) = data.get(index) {
    return barrier_border != is_barrier_tile(tile);
  }
  true
}
use crate::engine::asset::texture::TextureKey;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::tile::tile::{TileData, TileKey};
use crate::engine::utility::alias::Size2;

/**
 * Tileset structure and utilities
 */

/// builds up the tile data for `dimensions` using `tile_size`
fn make_tiles(texture_key: TextureKey, dimensions: Size2, tile_size: Size2) -> Result<Vec<TileData>, &'static str> {
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
      tiles.push(TileData {
        texture_key,
        tile_key,
        src,
      });
    }
  }

  Ok(tiles)
}

/// Wrapper for a texture that contains tiles
pub struct Tileset {
  pub texture: TextureKey,
  pub tile_size: Size2,
  pub tiles: Vec<TileData>,
}

impl Tileset {
  /// Instantiate a new tileset from a `texture` with `tile_size`
  pub fn build(texture: TextureKey, dimensions: Size2, tile_size: Size2) -> Result<Self, String> {
    let tiles = make_tiles(texture, dimensions, tile_size)?;
    Ok(Self {
      texture,
      tile_size,
      tiles,
    })
  }

  /// Get the tile data for `tile_key`
  pub fn get_tile(&self, tile_key: usize) -> Result<TileData, String> {
    let data = self.tiles
      .get(tile_key)
      .ok_or("Failed to get tile data")?;
    Ok(data.clone())
  }

  /// Convert a collection of `tile_keys` to `TileData`
  pub fn get_tiles<'a, I>(&'a self, tile_data: I) -> impl Iterator<Item=Option<TileData>> + 'a
    where I: IntoIterator<Item=Option<TileKey>>, <I as IntoIterator>::IntoIter: 'a
  {
    tile_data
      .into_iter()
      .map(|tile_key| {
        tile_key.map(|key| {
          self
            .get_tile(key)
            .expect("failed to get tile")
        })
      })
  }
}


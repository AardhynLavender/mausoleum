use std::rc::Rc;

use crate::engine::asset::texture::Texture;
use crate::engine::geometry::{Rec2, Vec2};
use crate::engine::store::HeapStore;
use crate::engine::tile::tile::{TileData, TileId};
use crate::engine::utility::types::Size2;

/// Store tilesets
pub type TilesetStore = HeapStore<Tileset>;

/// Wrapper for a texture that contains tiles
pub struct Tileset {
  pub texture: Rc<Texture>,
  pub tile_size: Vec2<u32>,
  pub tiles: Vec<TileData>,
}

impl Tileset {
  /// Instantiate a new tileset from a `texture` with `tile_size`
  pub fn new(texture: Rc<Texture>, tile_size: Size2) -> Self {
    let tiles = make_tiles(texture.dimensions, tile_size).expect("Failed to make tile");
    Self { texture, tile_size, tiles }
  }

  /// Get a copy of the tile data at `id`
  pub fn get_tiledata(&self, id: TileId) -> Option<TileData> {
    return if let Some(tile) = self.tiles.get(id as usize) {
      Some(*tile).clone()
    } else {
      None
    };
  }
}

/// Build up the tile data for `dimensions` using `tile_size`
fn make_tiles(dimensions: Size2, tile_size: Size2) -> Result<Vec<TileData>, &'static str> {
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
      let id = y * (width / tile_size.x) + x;
      let tile_position = Vec2::new(x * tile_size.x, y * tile_size.y);
      let src = Rec2::new(tile_position, tile_size);
      tiles.push(TileData { id, src });
    }
  }

  Ok(tiles)
}
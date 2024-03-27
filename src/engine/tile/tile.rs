use crate::engine::asset::texture::{SrcRect, TextureKey};
use crate::engine::render::component::Sprite;
use crate::engine::store::Key;

/**
 * Tile structure for rendering segments of a tileset to the screen
 */

/// A unique identifier for a tile
pub type TileKey = Key;

/// Data to create a tile entity (Sprite + Tile)
#[derive(Clone, Copy, Debug)]
pub struct TileData {
  pub texture_key: TextureKey,
  pub src: SrcRect,
  pub tile_key: TileKey,
}

/// A tile object that can be rendered to the screen
#[derive(Clone, Debug)]
pub struct Tile(pub TileKey);

impl Tile {
  /// Instantiate a new tile of `tile_key` that references `tileset_key`
  pub fn new(tile_key: TileKey) -> Self {
    Self(tile_key)
  }
}

pub fn make_tile_entity(data: TileData) -> (Tile, Sprite) {
  (
    Tile::new(data.tile_key),
    Sprite::new(data.texture_key, data.src)
  )
}
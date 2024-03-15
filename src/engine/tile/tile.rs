use crate::engine::asset::texture::SrcRect;
use crate::engine::geometry::Vec2;

/**
 * Tile structure for rendering segments of a tileset to the screen
 */

/// A unique identifier for a tile
pub type TileId = u32;

/// Data required to render a tile
#[derive(Clone, Copy, Debug)]
pub struct TileData {
  pub id: TileId,
  pub src: SrcRect, // segment of the tileset to be rendered
}

/// A tile object that can be rendered to the screen
#[derive(Clone, Debug)]
pub struct Tile {
  pub id: TileId,
  pub src: SrcRect,
  pub position: Vec2<i32>, // worldspace
}

impl Tile {
  /// Instantiate a new tile from `data` at `position`
  pub fn new(data: TileData, position: Vec2<i32>) -> Self {
    let TileData { src, id } = data;
    Self { src, id, position }
  }
}

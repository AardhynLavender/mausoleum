use crate::engine::asset::texture::{SrcRect, TextureKey};
use crate::engine::geometry::collision::{CollisionBox, CollisionMask};
use crate::engine::store::Key;
use crate::engine::utility::alias::Coordinate;

/**
 * Tile components, structures, and systems
 */

/// A unique identifier for a tile
pub type TileKey = Key;

/// Data to create a tile entity (Sprite + Tile)
#[derive(Clone, Debug)]
pub struct TileData<Meta> {
  pub texture_key: TextureKey,
  pub src: SrcRect,
  pub tile_key: TileKey,
  pub meta: Meta,
}

impl<Meta> TileData<Meta> {
  /// Instantiate a new tile data with `texture_key`, `src`, and `tile_key`
  pub fn new(texture_key: TextureKey, src: SrcRect, tile_key: TileKey, meta: Meta) -> Self {
    Self {
      texture_key,
      src,
      tile_key,
      meta,
    }
  }
}

/// Data used to add a tile to the tilemap
#[derive(Clone, Debug)]
pub struct TileConcept<Meta> where Meta: Clone {
  pub data: TileData<Meta>,
  pub coordinate: Coordinate,
  pub mask: CollisionMask,
}

impl<Meta> TileConcept<Meta> where Meta: Clone {
  /// Instantiate a new tile concept with `tile_data` and `collision_mask`
  pub fn new(data: TileData<Meta>, coordinate: Coordinate, mask: CollisionMask) -> Self {
    Self {
      data,
      coordinate,
      mask,
    }
  }
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

/// A tile that can be collided with
pub struct TileCollider {
  pub collision_box: CollisionBox,
  pub mask: CollisionMask,
}

impl TileCollider {
  /// Instantiate a new tile collider with `collision_box` and `mask`
  pub fn new(collision_box: CollisionBox, mask: CollisionMask) -> Self {
    Self {
      collision_box,
      mask,
    }
  }
}

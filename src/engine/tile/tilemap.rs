use std::rc::Rc;

use crate::engine::geometry::Vec2;
use crate::engine::tile::tile::{Tile, TileData};
use crate::engine::tile::tileset::Tileset;
use crate::engine::utility::conversion::{coordinate_to_index, index_to_coordinate};
use crate::engine::utility::types::{Coordinate, Size2};

/// A 2D grid of tiles
pub type MapData = Vec<Option<Tile>>;

/// Manages a grid of tiles
pub struct Tilemap {
  pub tileset: Rc<Tileset>,
  tiles: MapData,
  pub position: Vec2<i32>,
  pub dimensions: Size2,
}

impl Tilemap {
  /// Instantiate a new tilemap from `tileset` at `position` with `dimensions`
  pub fn new(tileset: Rc<Tileset>, position: Vec2<i32>, dimensions: Size2) -> Self {
    let size_tiles = dimensions.x * dimensions.y;
    let tiles: MapData = vec![None; size_tiles as usize];
    Self { tileset, tiles, position, dimensions }
  }

  /// Get the tile at `coordinate`
  pub fn get_at_coord(&self, coordinate: &Coordinate) -> Option<&Tile> {
    let index = coordinate_to_index(coordinate, self.dimensions);
    self.get_at_index(index)
  }
  /// Get the tile at `index`
  pub fn get_at_index(&self, index: usize) -> Option<&Tile> {
    if let Some(tile) = self.tiles.get(index) {
      return tile.as_ref();
    }
    None
  }

  /// Set the tile at `coordinate` to `data`
  pub fn set_tile_at_coord(&mut self, coordinate: &Coordinate, data: TileData) {
    let position = self.coord_to_worldspace(&coordinate);
    let tile = Tile::new(data, position);
    let index = coordinate_to_index(&coordinate, self.dimensions);

    if let Some(current_tile) = self.tiles.get_mut(index) { // bound check
      *current_tile = Some(tile);
    }
  }
  /// Set the tile at `index` to `data`
  pub fn set_tile_at_index(&mut self, index: usize, data: TileData) {
    let dimensions = Coordinate::new(self.dimensions.x as i32, self.dimensions.y as i32);
    let coordinate = index_to_coordinate(index, &dimensions);

    self.set_tile_at_coord(&coordinate, data);
  }

  /// Clear the tile at `coordinate`
  pub fn clear_tile_at_coord(&mut self, coordinate: &Coordinate) {
    let index = coordinate_to_index(&coordinate, self.dimensions);

    if let Some(tile) = self.tiles.get_mut(index) { // bound check
      *tile = None;
    }
  }
  ///
  pub fn clear_tile_at_index(&mut self, index: usize) {
    let dimensions = Coordinate::new(self.dimensions.x as i32, self.dimensions.y as i32);
    let coordinate = index_to_coordinate(index, &dimensions);
    self.clear_tile_at_coord(&coordinate);
  }

  /// Clear all tiles
  pub fn clear_tiles(&mut self) {
    for tile in &mut self.tiles {
      *tile = None;
    }
  }

  /// Convert `coordinate` to worldspace
  fn coord_to_worldspace(&self, coordinate: &Coordinate) -> Vec2<i32> {
    let (tile_width, tile_height) = self.tileset.tile_size.destructure();
    Vec2::new(
      self.position.x + (coordinate.x * tile_width as i32),
      self.position.y + (coordinate.y * tile_height as i32),
    )
  }

  /// Check if `coordinate` is within the bounds of the tilemap
  pub fn is_bound(&self, coordinate: &Coordinate) -> bool {
    let x_bound = coordinate.x >= 0 && coordinate.x < self.dimensions.x as i32;
    let y_bound = coordinate.y >= 0 && coordinate.y < self.dimensions.y as i32;
    x_bound && y_bound
  }

  /// Check if `coordinate` is occupied
  pub fn is_occupied(&self, coordinate: &Coordinate) -> bool {
    self.get_at_coord(coordinate).is_some()
  }
}

// iterate over the tiles in tilemap
impl<'a> IntoIterator for &'a Tilemap {
  type Item = &'a Option<Tile>;
  type IntoIter = std::slice::Iter<'a, Option<Tile>>;

  fn into_iter(self) -> Self::IntoIter {
    self.tiles.iter()
  }
}

// iterate mutably over the tiles in tilemap
impl<'a> IntoIterator for &'a mut Tilemap {
  type Item = &'a mut Option<Tile>;
  type IntoIter = std::slice::IterMut<'a, Option<Tile>>;

  fn into_iter(self) -> Self::IntoIter {
    self.tiles.iter_mut()
  }
}
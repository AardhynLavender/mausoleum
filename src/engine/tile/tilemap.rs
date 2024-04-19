/**
 * Tilemap structure and utilities
 */

use std::collections::HashMap;

use hecs::Entity;

use crate::engine::geometry::shape::Vec2;
use crate::engine::tile::tile::{TileConcept, TileKey};
use crate::engine::tile::tileset::Tileset;
use crate::engine::utility::alias::{Coordinate, Size2};
use crate::engine::utility::conversion::{coordinate_to_index, index_to_coordinate, position_to_coordinate};

pub type MapIndex = usize;

pub enum TileQuery {
  Position(Vec2<f32>),
  Coordinate(Coordinate),
  Index(usize),
}

pub struct TileQueryResult<'r, Meta> where Meta: Copy + Clone {
  pub concept: Option<&'r TileConcept<Meta>>,
  pub entity: Option<Entity>,
  pub position: Vec2<f32>,
  pub coordinate: Coordinate,
  pub index: MapIndex,
}

pub type TQ<'r, Meta> = TileQueryResult<'r, Meta>;

/// A non-owning handle of a queried tile
pub struct TileHandle<Meta> where Meta: Copy {
  /// Convert a tile query result into a non-owning handle of the tile queried
  pub concept: TileConcept<Meta>,
  pub entity: Entity,
  pub position: Vec2<f32>,
  pub coordinate: Coordinate,
  pub index: MapIndex,
}

impl<Meta> TryFrom<TileQueryResult<'_, Meta>> for TileHandle<Meta> where Meta: Copy {
  type Error = String;
  fn try_from(result: TileQueryResult<'_, Meta>) -> Result<Self, Self::Error> {
    Ok(TileHandle::<Meta> {
      concept: result.concept.copied().ok_or(String::from("Tile has no concept"))?,
      entity: result.entity.ok_or("Tile has no entity")?,
      position: result.position,
      coordinate: result.coordinate,
      index: result.index,
    })
  }
}

/// Manages a grid of entities
pub struct Tilemap<Meta> where Meta: Copy + Clone {
  // store the data to build the tilemap
  tiles: Vec<Option<TileConcept<Meta>>>,
  tile_size: Size2,
  // store the entities that make up the tilemap
  entities: HashMap<MapIndex, Entity>,
  dimensions: Size2,
}

impl<Meta> Tilemap<Meta> where Meta: Copy + Clone {
  /// Instantiate a new tilemap from with `dimensions`
  pub fn build(tileset: &Tileset<Meta>, dimensions: Size2, initial_tiles: Vec<Option<TileKey>>) -> Result<Self, String> {
    // invariant
    let tile_count = dimensions.square() as usize;
    if initial_tiles.len() != tile_count {
      return Err(String::from("Initial tiles do not match dimensions"));
    }

    let tiles = tileset
      .tiledata_from::<Vec<Option<TileConcept<Meta>>>>(&initial_tiles, dimensions)?
      .collect();

    Ok(Self {
      tile_size: tileset.tile_size,
      dimensions,
      tiles,
      entities: HashMap::with_capacity(tile_count),
    })
  }

  /// Add tiles to the world by invoking an injected add function on each concept
  pub fn add_tiles(&mut self, mut add: impl FnMut(&TileConcept<Meta>, Coordinate, Vec2<f32>) -> Result<Entity, String>) -> Result<(), String> {
    for (index, tile) in self.tiles.iter().enumerate() {
      if let Some(tile) = tile {
        let coordinate = index_to_coordinate(index, self.dimensions);
        let position = Vec2::<f32>::from(coordinate) * Vec2::from(tile.data.src.size);
        let entity = add(tile, coordinate, position)?;
        self.entities.insert(index, entity);
      }
    }
    Ok(())
  }
  pub fn remove_tile(&mut self, handle: TileHandle<Meta>, mut remove: impl FnMut(Entity)) {
    if let Some(entity) = self.entities.remove(&handle.index) { remove(entity) };
  }
  /// Remove tiles from the world by invoking an injected remove function on each entity
  pub fn remove_tiles(&mut self, mut remove: impl FnMut(Entity)) {
    for (.., entity) in self.entities.drain() { remove(entity); }
  }
  /// get the dimensions of the tilemap in worldspace
  pub fn get_dimensions(&self) -> Size2 { self.dimensions * self.tile_size }
  /// Get a tile at a coordinate
  fn get_concept(&self, index: usize) -> Option<&TileConcept<Meta>> {
    if index >= self.tiles.len() { return None; }
    self.tiles
      .get(index)
      .map_or(None, |tile| tile.as_ref())
  }
  /// Query for a tile concept
  ///
  /// Returns a mutable reference to the tile concept
  #[inline]
  pub fn query_tile(&self, get: TileQuery) -> TileQueryResult<Meta> {
    match get {
      TileQuery::Position(position) => {
        let coordinate = position_to_coordinate(position, self.tile_size);
        self.query_tile(TileQuery::Coordinate(coordinate))
      }
      TileQuery::Coordinate(coordinate) => {
        let index = coordinate_to_index(&coordinate, self.dimensions);
        self.query_tile(TileQuery::Index(index))
      }
      TileQuery::Index(index) => {
        let concept = self.get_concept(index);
        let entity = self.entities.get(&index).copied();
        let coordinate = index_to_coordinate(index, self.dimensions);
        let position = Vec2::<f32>::from(coordinate) * Vec2::<f32>::from(self.tile_size);
        TileQueryResult { concept, entity, coordinate, position, index }
      }
    }
  }
}

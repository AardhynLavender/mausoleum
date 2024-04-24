/**
 * Tilemap structure and utilities
 */

use std::collections::{HashMap, HashSet};

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
pub struct Tilemap<TileMeta, ObjMeta> where TileMeta: Copy + Clone, ObjMeta: Copy + Clone {
  // store the data to build the tilemap
  tiles: Vec<Option<TileConcept<TileMeta>>>,
  #[allow(unused)]
  objects: Vec<ObjMeta>,
  tile_size: Size2,
  tile_entities: HashMap<MapIndex, Entity>,
  object_entities: HashSet<Entity>,
  dimensions: Size2,
}

impl<TileMeta, ObjMeta> Tilemap<TileMeta, ObjMeta> where TileMeta: Copy + Clone, ObjMeta: Copy + Clone {
  /// Instantiate a new tilemap from with `dimensions`
  pub fn build(tileset: &Tileset<TileMeta>, dimensions: Size2, initial_tiles: Vec<Option<TileKey>>, objects: Vec<ObjMeta>) -> Result<Self, String> {
    let tile_count = dimensions.square() as usize;
    if initial_tiles.len() != tile_count {
      return Err(String::from("Initial tiles do not match dimensions"));
    }

    let object_count = objects.len();

    let tiles = tileset
      .tiledata_from::<Vec<Option<TileConcept<TileMeta>>>>(&initial_tiles, dimensions)?
      .collect();

    Ok(Self {
      tile_size: tileset.tile_size,
      objects,
      dimensions,
      tiles,
      tile_entities: HashMap::with_capacity(tile_count),
      object_entities: HashSet::with_capacity(object_count),
    })
  }

  /// Add tiles to the world by invoking an injected add function on each concept
  pub fn add_tiles(&mut self, mut add: impl FnMut(&TileConcept<TileMeta>, Coordinate, Vec2<f32>) -> Result<Entity, String>) -> Result<(), String> {
    for (index, tile) in self.tiles.iter().enumerate() {
      if let Some(tile) = tile {
        let coordinate = index_to_coordinate(index, self.dimensions);
        let position = Vec2::<f32>::from(coordinate) * Vec2::from(tile.data.src.size);
        let entity = add(tile, coordinate, position)?;
        self.tile_entities.insert(index, entity);
      }
    }
    Ok(())
  }
  pub fn remove_tile(&mut self, handle: TileHandle<TileMeta>, mut remove: impl FnMut(Entity)) {
    if let Some(entity) = self.tile_entities.remove(&handle.index) { remove(entity) };
  }
  /// Remove tiles from the world by invoking an injected remove function on each entity
  pub fn remove_tiles(&mut self, mut remove: impl FnMut(Entity)) {
    for (.., entity) in self.tile_entities.drain() { remove(entity); }
  }
  /// get the dimensions of the tilemap in worldspace
  pub fn get_dimensions(&self) -> Size2 { self.dimensions * self.tile_size }
  /// Get a tile at a coordinate
  fn get_concept(&self, index: usize) -> Option<&TileConcept<TileMeta>> {
    if index >= self.tiles.len() { return None; }
    self.tiles
      .get(index)
      .map_or(None, |tile| tile.as_ref())
  }

  pub fn add_objects(&mut self, mut add: impl FnMut(&ObjMeta) -> Result<Entity, String>) -> Result<(), String> {
    for object in &self.objects { self.object_entities.insert(add(object)?); }
    Ok(())
  }
  /// Remove tiles from the world by invoking an injected remove function on each entity
  pub fn remove_objects(&mut self, mut remove: impl FnMut(Entity)) {
    for (.., entity) in self.tile_entities.drain() { remove(entity); }
  }

  /// Query for a tile concept
  ///
  /// Returns a mutable reference to the tile concept
  #[inline]
  pub fn query_tile(&self, get: TileQuery) -> TileQueryResult<TileMeta> {
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
        let entity = self.tile_entities.get(&index).copied();
        let coordinate = index_to_coordinate(index, self.dimensions);
        let position = Vec2::<f32>::from(coordinate) * Vec2::<f32>::from(self.tile_size);
        TileQueryResult { concept, entity, coordinate, position, index }
      }
    }
  }
}


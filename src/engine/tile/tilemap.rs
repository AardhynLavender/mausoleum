/**
 * Tilemap structure and utilities
 */

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use hecs::Entity;

use crate::engine::geometry::shape::Vec2;
use crate::engine::tile::query::{TileHandle, TileQuery, TileQueryResult};
use crate::engine::tile::tile::TileConcept;
use crate::engine::tile::tilelayer::TileLayer;
use crate::engine::tile::tileset::Tileset;
use crate::engine::utility::alias::{Coordinate, Size2};
use crate::engine::utility::conversion::{coordinate_to_index, index_to_coordinate, position_to_coordinate};

/// Index of a tile in a tilemap
pub type MapIndex = usize;

/// Manages a grid of entities
pub struct Tilemap<TileMeta, LayerMeta, ObjMeta> where TileMeta: Copy + Clone, LayerMeta: Copy + Clone + Hash + Eq, ObjMeta: Copy + Clone {
  // store the data to build the tilemap
  layers: HashMap<LayerMeta, Vec<Option<TileConcept<TileMeta>>>>,
  objects: Vec<ObjMeta>,
  tile_size: Size2,
  tile_entities: HashMap<MapIndex, Entity>,
  object_entities: HashSet<Entity>,
  dimensions: Size2,
}

impl<TileMeta, LayerMeta, ObjMeta> Tilemap<TileMeta, LayerMeta, ObjMeta> where TileMeta: Copy + Clone, LayerMeta: Copy + Clone + Hash + Eq, ObjMeta: Copy + Clone {
  /// Instantiate a new tilemap from with `dimensions`
  pub fn build(tileset: &Tileset<TileMeta>, dimensions: Size2, layers: Vec<TileLayer<LayerMeta, TileMeta>>, objects: Vec<ObjMeta>) -> Result<Self, String> {
    let object_count = objects.len();
    let tile_count = dimensions.square() as usize;
    for layer in &layers {
      if layer.tiles.len() != tile_count { return Err(String::from("Layer tiles do not match dimensions")); }
    }

    let layers = layers
      .into_iter()
      .map(|layer| (layer.meta, layer.tiles))
      .collect();

    Ok(Self {
      tile_size: tileset.tile_size,
      objects,
      dimensions,
      layers,
      tile_entities: HashMap::with_capacity(tile_count),
      object_entities: HashSet::with_capacity(object_count),
    })
  }

  /// Add tiles to the world by invoking an injected add function on each concept
  pub fn add_tiles(&mut self, mut add: impl FnMut(LayerMeta, &TileConcept<TileMeta>, Coordinate, Vec2<f32>) -> Result<Entity, String>) -> Result<(), String> {
    for (layer, tiles, ) in &self.layers {
      for (index, tile) in tiles.iter().enumerate() {
        if let Some(tile) = tile {
          let coordinate = index_to_coordinate(index, self.dimensions);
          let position = Vec2::<f32>::from(coordinate) * Vec2::from(tile.data.src.size);
          let entity = add(*layer, tile, coordinate, position)?;
          self.tile_entities.insert(index, entity);
        }
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
  fn get_concept(&self, layer: LayerMeta, index: MapIndex) -> Option<&TileConcept<TileMeta>> {
    if let Some(tiles) = self.layers.get(&layer) {
      if index >= tiles.len() { return None; }
      return tiles
        .get(index)
        .map_or(None, |tile| tile.as_ref());
    }
    return None;
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
  pub fn query_tile(&self, layer: LayerMeta, get: TileQuery) -> TileQueryResult<TileMeta> {
    match get {
      TileQuery::Position(position) => {
        let coordinate = position_to_coordinate(position, self.tile_size);
        self.query_tile(layer, TileQuery::Coordinate(coordinate))
      }
      TileQuery::Coordinate(coordinate) => {
        let index = coordinate_to_index(&coordinate, self.dimensions);
        self.query_tile(layer, TileQuery::Index(index))
      }
      TileQuery::Index(index) => {
        let concept = self.get_concept(layer, index);
        let entity = self.tile_entities.get(&index).copied();
        let coordinate = index_to_coordinate(index, self.dimensions);
        let position = Vec2::<f32>::from(coordinate) * Vec2::<f32>::from(self.tile_size);
        TileQueryResult { concept, entity, coordinate, position, index }
      }
    }
  }
}


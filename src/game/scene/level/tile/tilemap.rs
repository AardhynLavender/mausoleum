/**
 * Tilemap structure and utilities
 */

use std::collections::HashMap;
use std::hash::Hash;

use hecs::Entity;

use crate::engine::geometry::shape::Vec2;
use crate::engine::math::conversion::{coordinate_to_index, index_to_coordinate, position_to_coordinate};
use crate::engine::utility::alias::{Coordinate, Size2};
use crate::engine::utility::direction::{Direction, DIRECTIONS, QUARTER_DIRECTION_ROTATION, Rotation};
use crate::game::scene::level::tile::query::{TileHandle, TileQuery, TileQueryResult};
use crate::game::scene::level::tile::tile::TileConcept;
use crate::game::scene::level::tile::tilelayer::TileLayer;
use crate::game::scene::level::tile::tileset::Tileset;

/// Index of a tile in a tilemap
pub type MapIndex = usize;

/// Index of an object in a tilemap
pub type ObjectIndex = usize;

#[derive(Copy, Clone, PartialEq, Default)]
pub enum TilemapMutation {
  #[default]
  /// Remove the tile entity, but keep the concept
  Local,
  /// Remove the tile entity and concept
  Session,
}

/// Manages a grid of entities
pub struct Tilemap<TileMeta, LayerMeta, ObjMeta> where TileMeta: Clone, LayerMeta: Copy + Clone + Hash + Eq + Default, ObjMeta: Clone {
  // store the data to build the tilemap
  layers: HashMap<LayerMeta, TileLayer<LayerMeta, TileMeta>>,
  tile_size: Size2,
  objects: Vec<Option<ObjMeta>>,
  object_entities: HashMap<ObjectIndex, Entity>,
  dimensions: Size2,
}

impl<TileMeta, LayerMeta, ObjMeta> Tilemap<TileMeta, LayerMeta, ObjMeta> where TileMeta: Clone + Default, LayerMeta: Copy + Clone + Hash + Eq + Default, ObjMeta: Clone + std::fmt::Debug {
  /// Instantiate a new tilemap from with `dimensions`
  pub fn build(tileset: &Tileset<TileMeta>, dimensions: Size2, layers: Vec<TileLayer<LayerMeta, TileMeta>>, objects: Vec<ObjMeta>) -> Result<Self, String> {
    let object_count = objects.len();
    let tile_count = dimensions.square() as usize;
    for layer in &layers {
      if layer.tiles.len() != tile_count { return Err(String::from("Layer tiles do not match dimensions")); }
    }

    let layers = layers
      .into_iter()
      .map(|layer| (layer.meta, layer))
      .collect();

    Ok(Self {
      tile_size: tileset.tile_size,
      objects: objects.into_iter().map(Some).collect(),
      dimensions,
      layers,
      object_entities: HashMap::with_capacity(object_count),
    })
  }

  /// Add tiles to the world by invoking an injected add function on each concept
  pub fn add_tiles(&mut self, mut add: impl FnMut(LayerMeta, &TileConcept<TileMeta>, Coordinate, Vec2<f32>) -> Result<Option<Entity>, String>) -> Result<(), String> {
    let dimensions = self.dimensions;
    for (meta, layer, ) in &mut self.layers {
      for (index, tile) in layer.tiles.iter().enumerate() {
        if let Some(tile) = tile {
          let coordinate = index_to_coordinate(index, dimensions);
          let position = Vec2::<f32>::from(coordinate) * Vec2::from(tile.data.src.size);
          if let Some(entity) = add(*meta, tile, coordinate, position)? {
            layer.entities.insert(index, entity);
          }
        }
      }
    }
    Ok(())
  }

  /// Remove a tile concept from the session
  fn remove_tile_concept(&mut self, handle: &TileHandle<TileMeta, LayerMeta>) {
    self.layers
      .get_mut(&handle.layer)
      .expect("Invalid handle layer!")
      .tiles
      .get_mut(handle.index)
      .expect("Invalid handle index!")
      .take();
  }
  /// Mutate a tile concept in the session
  fn mutate_tile_concept(&mut self, handle: &TileHandle<TileMeta, LayerMeta>, concept: TileConcept<TileMeta>) {
    self.layers
      .get_mut(&handle.layer)
      .expect("Invalid handle layer!")
      .tiles
      .get_mut(handle.index)
      .expect("Invalid handle index!")
      .replace(concept);
  }
  /// Add a tile to the world by invoking an injected remove function on the concept
  pub fn remove_tile(&mut self, handle: &TileHandle<TileMeta, LayerMeta>, mut remove: impl FnMut(Entity), mutation: TilemapMutation) {
    if let Some(entity) = self
      .layers
      .get_mut(&handle.layer)
      .expect("layer does not exist")
      .entities
      .remove(&handle.index)
    {
      remove(entity);
      if mutation == TilemapMutation::Session { self.remove_tile_concept(handle); }
    };
  }
  /// invoke fn for each neighbor of a tile handle
  pub fn for_neighbour(&mut self, handle: &TileHandle<TileMeta, LayerMeta>, mut repair: impl FnMut(&mut TileHandle<TileMeta, LayerMeta>, Direction), mutation: TilemapMutation) {
    let mut check = Direction::Up;
    for _ in 0..DIRECTIONS / 2 {
      let check_result = self.query_tile(handle.layer, TileQuery::Coordinate(handle.coordinate + check.to_coordinate()));
      if let Ok(mut handle) = TileHandle::try_from(check_result) {
        repair(&mut handle, check);
        if mutation == TilemapMutation::Session {
          self.mutate_tile_concept(&handle, handle.concept.clone());
        }
      }
      check = check.rotate(Rotation::Left, QUARTER_DIRECTION_ROTATION);
    }
  }
  /// Remove tiles from the world by invoking an injected remove function on each entity
  pub fn remove_tiles(&mut self, mut remove: impl FnMut(Entity)) {
    for layer in self.layers.values_mut() {
      for (.., entity) in layer.entities.drain() { remove(entity); }
    }
  }
  /// get the dimensions of the tilemap in worldspace
  pub fn get_dimensions(&self) -> Size2 { self.dimensions * self.tile_size }
  /// get the size of the tilemap in tiles
  pub fn get_size(&self) -> Size2 { self.dimensions }
  /// Get a tile at a coordinate
  fn get_concept(&self, layer: LayerMeta, index: MapIndex) -> Option<&TileConcept<TileMeta>> {
    self
      .layers
      .get(&layer)
      .and_then(|layer| {
        layer
          .tiles
          .get(index)
          .and_then(Option::as_ref)
      })
  }

  /// Add objects to the world by invoking an injected add function on each object
  pub fn add_objects(&mut self, mut add: impl FnMut(&ObjMeta) -> Result<Option<Entity>, String>) -> Result<(), String> {
    for (index, object) in self
      .objects
      .iter()
      .enumerate()
    {
      if let Some(object) = object {
        if let Some(entity) = add(object)? {
          self.object_entities.insert(index, entity);
        }
      }
    }
    Ok(())
  }
  /// Remove an object from the world
  pub fn remove_object(&mut self, entity: Entity, mut remove: impl FnMut(Entity), mutation: TilemapMutation) -> Result<(), String> {
    let index = *self.object_entities.iter().find(|(_, e)| **e == entity).ok_or("Invalid entity")?.0;
    self.object_entities.remove(&index).ok_or("Invalid index")?;
    remove(entity);
    if mutation == TilemapMutation::Session {
      self.objects
        .get_mut(index)
        .ok_or("Invalid index")?
        .take();
    }
    Ok(())
  }
  /// Remove tiles from the world by invoking an injected remove function on each entity
  pub fn remove_objects(&mut self, mut remove: impl FnMut(Entity)) {
    for (.., entity) in self.object_entities.drain() { remove(entity); }
  }

  /// Query for a tile concept
  ///
  /// Returns a mutable reference to the tile concept
  #[inline]
  pub fn query_tile(&self, layer: LayerMeta, get: TileQuery) -> TileQueryResult<TileMeta, LayerMeta> {
    match get {
      TileQuery::Entity(entity) => {
        if let Some((index, ..)) = self
          .layers
          .get(&layer)
          .and_then(|layer| {
            layer.entities.iter().find(|(_, e)| **e == entity)
          })
        {
          self.query_tile(layer, TileQuery::Index(*index))
        } else {
          TileQueryResult::default()
        }
      }
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
        let entity = self.layers
          .get(&layer)
          .and_then(|layer| {
            layer.entities.get(&index).copied()
          });

        let coordinate = index_to_coordinate(index, self.dimensions);
        let position = Vec2::<f32>::from(coordinate) * Vec2::<f32>::from(self.tile_size);
        TileQueryResult { layer, concept, entity, coordinate, position, index }
      }
    }
  }
}

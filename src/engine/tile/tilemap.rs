use std::collections::HashMap;

use hecs::Entity;

use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::rendering::component::Sprite;
use crate::engine::rendering::renderer::layer;
use crate::engine::tile::tile::{Tile, TileCollider, TileConcept, TileKey};
use crate::engine::tile::tileset::Tileset;
use crate::engine::utility::alias::{Coordinate, Size2};
use crate::engine::utility::conversion::{coordinate_to_index, index_to_coordinate, position_to_coordinate};
use crate::engine::world::World;
use crate::game::physics::position::Position;

/**
 * Tilemap structure and utilities
 */

pub enum TileQuery {
  Position(Vec2<f32>),
  Coordinate(Coordinate),
  Index(usize),
}

pub type TileQueryResult<'r> = Result<(Option<&'r TileConcept>, Option<Entity>, Vec2<f32>), String>;

/// Manages a grid of entities
pub struct Tilemap {
  // store the data to build the tilemap
  tiles: Vec<Option<TileConcept>>,
  tile_size: Size2,
  // store the entities that make up the tilemap
  entities: HashMap<usize, Entity>,
  dimensions: Size2,
}

impl Tilemap {
  /// Instantiate a new tilemap from with `dimensions`
  pub fn build(tileset: &Tileset, dimensions: Size2, initial_tiles: Vec<Option<TileKey>>) -> Result<Self, String> {
    // invariant
    let tile_count = dimensions.square() as usize;
    if initial_tiles.len() != tile_count {
      return Err(String::from("Initial tiles do not match dimensions"));
    }

    Ok(Self {
      tile_size: tileset.tile_size,
      tiles: tileset.tiledata_from::<Vec<Option<TileConcept>>>(&initial_tiles, dimensions)?.collect(),
      entities: HashMap::with_capacity(tile_count),
      dimensions,
    })
  }
  /// Add the tiles to the world
  pub fn add_to_world(&mut self, world: &mut World, position: Vec2<f32>) -> Result<(), String> {
    for (index, tile) in self.tiles
      .iter()
      .enumerate()
    {
      if let Some(tile) = tile {
        let coordinate = index_to_coordinate(index, self.dimensions);
        let (tile_width, tile_height) = tile.data.src.size.destructure();
        let tile_position = Vec2::new(
          coordinate.x as f32 * tile_width as f32,
          coordinate.y as f32 * tile_height as f32,
        ) + position;

        let entity = world.add((
          Tile::new(tile.data.tile_key),
          Position::new(tile_position.x, tile_position.y),
          Sprite::new(tile.data.texture_key, tile.data.src),
          layer::Layer5
        ));

        // add a collider if the tile has a mask
        if !tile.mask.is_empty() {
          let collider = TileCollider::new(
            Rec2::new(Vec2::default(), tile.data.src.size),
            tile.mask,
          );
          world.add_components(entity, (collider, ))?;
        }

        self.entities.insert(index, entity);
      }
    }

    Ok(())
  }
  /// remove the entities from the world
  pub fn remove_from_world(&mut self, world: &mut World) -> Result<(), String> {
    for entity in self.entities.values().into_iter() {
      world.free_now(*entity)?
    }

    Ok(())
  }
  /// get the dimensions of the tilemap in worldspace
  pub fn get_dimensions(&self) -> Size2 {
    self.dimensions * self.tile_size
  }

  // Concept Getters //

  /// Get a tile at a coordinate
  fn get_concept(&self, index: usize) -> Result<Option<&TileConcept>, String> {
    if index >= self.tiles.len() { return Err(String::from("Index out of bounds")); }
    Ok(self.tiles
      .get(index)
      .map_or(None, |tile| tile.as_ref()))
  }
  /// Query for a tile concept
  ///
  /// Returns a mutable reference to the tile concept
  #[inline]
  pub fn query_tile(&self, get: TileQuery) -> TileQueryResult {
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
        let concept = self.get_concept(index)?;
        let entity = self.entities.get(&index).copied();
        let position = Vec2::<f32>::from(index_to_coordinate(index, self.dimensions)) * Vec2::<f32>::from(self.tile_size);
        Ok((concept, entity, position))
      }
    }
  }
}


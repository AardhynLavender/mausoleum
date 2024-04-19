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

pub type TileQueryResult<'r, Meta> = (Option<&'r TileConcept<Meta>>, Option<Entity>, Vec2<f32>, Coordinate, MapIndex);

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
  fn try_from((concept, entity, position, coordinate, index): TileQueryResult<'_, Meta>) -> Result<Self, Self::Error> {
    let concept = concept.copied().ok_or(String::from("Tile has no concept"))?;
    let entity = entity.ok_or("Tile has no entity")?;
    Ok(TileHandle::<Meta> { concept, entity, position, coordinate, index })
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
  fn get_concept(&self, index: usize) -> Option<&TileConcept> {
    if index >= self.tiles.len() { return None; }
    self.tiles
      .get(index)
      .map_or(None, |tile| tile.as_ref())
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
        let concept = self.get_concept(index);
        let entity = self.entities.get(&index).copied();
        let coordinate = index_to_coordinate(index, self.dimensions);
        let position = Vec2::<f32>::from(coordinate) * Vec2::<f32>::from(self.tile_size);
        (concept, entity, position, coordinate)
      }
    }
  }
}

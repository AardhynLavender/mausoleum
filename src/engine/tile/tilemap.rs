use std::collections::HashMap;

use hecs::Entity;

use crate::engine::geometry::shape::Vec2;
use crate::engine::render::component::Sprite;
use crate::engine::tile::tile::{Tile, TileData, TileKey};
use crate::engine::tile::tileset::Tileset;
use crate::engine::utility::alias::{Coordinate, Size2};
use crate::engine::utility::conversion::index_to_coordinate;
use crate::engine::world::World;
use crate::game::component::position::Position;

/**
 * Tilemap structure and utilities
 */

/// Manages a grid of entities
pub struct Tilemap {
  // store the data to build the tilemap
  tiles: Vec<Option<TileData>>,
  // store the entities that make up the tilemap
  entities: HashMap<Coordinate, Entity>,
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
      tiles: tileset.get_tiles(initial_tiles).collect(),
      entities: HashMap::with_capacity(tile_count),
      dimensions,
    })
  }

  /// create the tiles from the tiledata and add them to the world and store references to the entities created
  pub fn add_to_world(&mut self, world: &mut World, position: Vec2<f32>) {
    for (index, tile) in self.tiles.iter().enumerate() {
      if let Some(tile) = tile {
        let coordinate = index_to_coordinate(index, self.dimensions);
        let (tile_width, tile_height) = tile.src.size.destructure();
        let tile_position = Vec2::new(
          coordinate.x as f32 * tile_width as f32,
          coordinate.y as f32 * tile_height as f32,
        ) + position;

        let entity =
          world.add((
            Position::new(tile_position.x, tile_position.y),
            Tile::new(tile.tile_key),
            Sprite::new(tile.texture_key, tile.src),
            // collision, metadata, etc.
          ));

        self.entities.insert(coordinate, entity);
      }
    }
  }

  pub fn remove_from_world(&mut self, world: &mut World) -> Result<(), String> {
    for entity in self.entities.values().into_iter() {
      world.free_now(*entity)?
    }

    Ok(())
  }

  /// Check if `coordinate` is within the bounds of the tilemap
  pub fn is_bound(&self, coordinate: &Coordinate) -> bool {
    let x_bound = coordinate.x >= 0 && coordinate.x < self.dimensions.x as i32;
    let y_bound = coordinate.y >= 0 && coordinate.y < self.dimensions.y as i32;
    x_bound && y_bound
  }
}


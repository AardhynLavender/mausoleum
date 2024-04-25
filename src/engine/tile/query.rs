use hecs::Entity;

use crate::engine::geometry::shape::Vec2;
use crate::engine::tile::tile::TileConcept;
use crate::engine::tile::tilemap::MapIndex;
use crate::engine::utility::alias::Coordinate;

/// Defines a generic query for a tile in a tilemap
pub enum TileQuery {
  Position(Vec2<f32>),
  Coordinate(Coordinate),
  Index(usize),
}

/// The result of a tile query
pub struct TileQueryResult<'r, Meta> where Meta: Copy + Clone {
  pub concept: Option<&'r TileConcept<Meta>>,
  pub entity: Option<Entity>,
  pub position: Vec2<f32>,
  pub coordinate: Coordinate,
  pub index: MapIndex,
}

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


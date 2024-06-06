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
  Entity(Entity),
}

/// The result of a tile query
#[derive(Default)]
pub struct TileQueryResult<'r, TileMeta, LayerMeta>
  where TileMeta: Clone, LayerMeta: Copy + Clone + Default
{
  pub concept: Option<&'r TileConcept<TileMeta>>,
  pub layer: LayerMeta,
  pub position: Vec2<f32>,
  pub coordinate: Coordinate,
  pub index: MapIndex,
  pub entity: Option<Entity>,
}

/// A non-owning handle of a queried tile
pub struct TileHandle<TileMeta, LayerMeta> where TileMeta: Clone, LayerMeta: Copy + Clone {
  /// Convert a tile query result into a non-owning handle of the tile queried
  pub concept: TileConcept<TileMeta>,
  pub layer: LayerMeta,
  pub position: Vec2<f32>,
  pub coordinate: Coordinate,
  pub index: MapIndex,
  pub entity: Entity,
}

impl<TileMeta, LayerMeta> TryFrom<TileQueryResult<'_, TileMeta, LayerMeta>> for TileHandle<TileMeta, LayerMeta>
  where TileMeta: Clone, LayerMeta: Copy + Clone + Default
{
  type Error = String;
  fn try_from(result: TileQueryResult<'_, TileMeta, LayerMeta>) -> Result<Self, Self::Error> {
    Ok(TileHandle::<TileMeta, LayerMeta> {
      concept: result.concept.cloned().ok_or(String::from("Tile has no concept"))?,
      layer: result.layer,
      position: result.position,
      coordinate: result.coordinate,
      index: result.index,
      entity: result.entity.ok_or("Tile has no entity")?,
    })
  }
}


/**
  * Store data for a single layer of tiles
  */

use std::collections::HashMap;

use hecs::Entity;
use crate::game::scene::level::tile::tile::TileConcept;
use crate::game::scene::level::tile::tilemap::MapIndex;

/// A collection of tiles concepts used to build a tilemap
pub type TileLayerData<TileMeta> = Vec<Option<TileConcept<TileMeta>>>;

/// A layer of tiles in a tilemap
pub struct TileLayer<LayerMeta, TileMeta> where LayerMeta: Copy + Clone + Eq, TileMeta: Clone {
  pub meta: LayerMeta,
  pub tiles: TileLayerData<TileMeta>,
  pub entities: HashMap<MapIndex, Entity>,
}
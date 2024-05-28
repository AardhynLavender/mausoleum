use std::collections::HashMap;

use hecs::Entity;

use crate::engine::tile::tile::TileConcept;
use crate::engine::tile::tilemap::MapIndex;

pub type TileLayerData<TileMeta> = Vec<Option<TileConcept<TileMeta>>>;

pub struct TileLayer<LayerMeta, TileMeta> where LayerMeta: Copy + Clone + Eq, TileMeta: Copy + Clone {
  pub meta: LayerMeta,
  pub tiles: TileLayerData<TileMeta>,
  pub entities: HashMap<MapIndex, Entity>,
}
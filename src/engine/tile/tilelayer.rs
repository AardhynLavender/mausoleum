use crate::engine::tile::tile::TileConcept;

pub type TileLayerData<TileMeta> = Vec<Option<TileConcept<TileMeta>>>;

pub struct TileLayer<LayerMeta, TileMeta> where LayerMeta: Copy + Clone + Eq, TileMeta: Copy + Clone {
  pub meta: LayerMeta,
  pub tiles: TileLayerData<TileMeta>,
}
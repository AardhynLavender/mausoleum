use crate::engine::asset::AssetManager;
use crate::engine::component::text::split_text;
use crate::engine::event::EventStore;
use crate::engine::rendering::camera::Sticky2;
use crate::engine::rendering::component::Sprite;
use crate::engine::state::State;
use crate::engine::system::{SysArgs, Systemize};
use crate::engine::tile::tile::TileKey;
use crate::engine::tile::tileset::Tileset;
use crate::engine::utility::alias::Size2;
use crate::engine::utility::alignment::{Align, Alignment};
use crate::engine::world::World;
use crate::game::collectable::data::CollectableItemData;
use crate::game::modal::modal::{make_modal, Modal, use_escape_modal};
use crate::game::physics::position::Position;
use crate::game::scene::level::meta::TileMeta;
use crate::game::scene::level::room::use_tileset;

const MODAL_BACKGROUND: &str = "asset/hud/collectable_pane.png";
const MODAL_SIZE: Size2 = Size2::new(200, 160);
const COLLECTION_MODAL_MARGIN: f32 = 8.0;

const ICON_TOP_OFFSET: f32 = 32.0;
const DESCRIPTION_TOP_OFFSET: f32 = 64.0;
const DESCRIPTION_LINE_HEIGHT: f32 = 8.0;
const CHAR_WIDTH: f32 = 5.0 + 1.0;
const DESCRIPTION_LINE_LENGTH: usize = (MODAL_SIZE.x as usize - (COLLECTION_MODAL_MARGIN * 2.0) as usize) / CHAR_WIDTH as usize;

#[derive(Default)]
pub struct CollectableModal;

/// Create a sprite for a tile
pub fn make_tile_sprite(tileset: &Tileset<TileMeta>, tile: TileKey) -> Sprite {
  let src = tileset.get_tile(tile).expect("Failed to get tile data").src;
  Sprite::new(tileset.texture, src)
}

/// Create a modal to display collectable information
pub fn make_collectable_modal(world: &mut World, events: &mut EventStore, asset: &mut AssetManager, state: &mut State, data: &CollectableItemData) {
  let CollectableItemData { name, .. } = data;
  let background = asset.texture.load(MODAL_BACKGROUND).expect("Failed to load collectable modal background");
  let (aligner, mut builder) = make_modal(world, events, asset, name.clone(), MODAL_SIZE, background);

  let tileset = use_tileset(state);
  let icon = make_tile_sprite(tileset, data.tile);
  let icon_position = aligner.align(Alignment::new(Align::Center(0.0), Align::Start(ICON_TOP_OFFSET)), icon.src.size);
  world.add((icon, Position::from(icon_position), Sticky2::default(), Modal));

  let description_lines = split_text(&data.description, DESCRIPTION_LINE_LENGTH);
  for (index, line) in description_lines.iter().enumerate() {
    let y = DESCRIPTION_TOP_OFFSET + index as f32 * DESCRIPTION_LINE_HEIGHT;
    let line_alignment = Alignment::new(Align::Center(0.0), Align::Start(y));
    world.add(builder.make_text::<Modal>(line, line_alignment));
  };
}

impl Systemize for CollectableModal {
  fn system(SysArgs { world, event, .. }: &mut SysArgs) -> Result<(), String> {
    use_escape_modal(world, event);
    Ok(())
  }
}

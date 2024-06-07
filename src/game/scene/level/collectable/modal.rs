
/**
  * Modal for displaying collectable information for new items
  */

use std::time::Duration;
use crate::engine::asset::asset::AssetManager;
use crate::engine::component::position::Position;
use crate::engine::component::sprite::Sprite;
use crate::engine::core::event::EventStore;
use crate::engine::ecs::system::{SysArgs, Systemize};
use crate::engine::ecs::world::World;
use crate::engine::render::camera::Sticky2;
use crate::engine::utility::alias::Size2;
use crate::engine::utility::alignment::{Align, Alignment};
use crate::engine::utility::state::State;
use crate::engine::utility::text::split_text;
use crate::game::scene::level::collectable::data::CollectableItemData;
use crate::game::scene::level::room::meta::TileMeta;
use crate::game::scene::level::room::room::use_tileset;
use crate::game::scene::level::tile::tile::TileKey;
use crate::game::scene::level::tile::tileset::Tileset;
use crate::game::ui::iterative_text::IterativeTextBuilder;
use crate::game::ui::modal::{make_modal, Modal, use_escape_modal};

const MODAL_BACKGROUND: &str = "asset/hud/collectable_pane.png";
const MODAL_SIZE: Size2 = Size2::new(200, 160);
const MODAL_MARGIN: f32 = 8.0;

const ICON_TOP_OFFSET: f32 = 32.0;
const DESCRIPTION_TOP_OFFSET: f32 = 64.0;
const DESCRIPTION_LINE_HEIGHT: f32 = 8.0;
const CHAR_WIDTH: f32 = 5.0 + 1.0;
const DESCRIPTION_LINE_LENGTH: usize = (MODAL_SIZE.x as usize - (MODAL_MARGIN * 2.0) as usize) / CHAR_WIDTH as usize;

const CHAR_ITERATION_MS: Duration = Duration::from_millis(32);

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
  let mut accumulated_duration = Duration::from_millis(0);

  for (index, line) in description_lines.iter().enumerate() {
    let y = DESCRIPTION_TOP_OFFSET + index as f32 * DESCRIPTION_LINE_HEIGHT;
    let line_alignment = Alignment::new(Align::Start(MODAL_MARGIN), Align::Start(y));
    let line_entity = world.add(builder.make_text::<Modal>(line, line_alignment));
    let line_duration = CHAR_ITERATION_MS * line.len() as u32;
    IterativeTextBuilder::build(world, line_entity)
      .expect("Failed to build iterative text")
      .with_duration(CHAR_ITERATION_MS)
      .with_delay(accumulated_duration)
      .start()
      .expect("Failed to start iterative text");
    accumulated_duration += line_duration + CHAR_ITERATION_MS;
  };
}

impl Systemize for CollectableModal {
  fn system(SysArgs { world, event, .. }: &mut SysArgs) -> Result<(), String> {
    if !event.is_paused() { return Ok(()); };
    use_escape_modal(world, event);
    Ok(())
  }
}

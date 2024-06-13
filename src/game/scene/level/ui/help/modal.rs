use crate::engine::asset::asset::AssetManager;
use crate::engine::core::event::EventStore;
use crate::engine::ecs::world::World;
use crate::engine::utility::alias::Size2;
use crate::engine::utility::alignment::{Align, Alignment};
use crate::game::scene::level::ui::help::parse::load_help_data;
use crate::game::ui::modal::{make_modal, Modal};

const MODAL_TITLE: &str = "Controls";
const MODAL_SIZE: Size2 = Size2::new(200, 221);
const MODAL_BACKGROUND: &str = "asset/hud/help_pane.png";
const MODAL_MARGIN: f32 = 8.0;
const MODAL_CONTENT_START_Y: f32 = 32.0;
const MODAL_HELP_LINE_HEIGHT: f32 = 16.0;

pub fn make_help_modal(world: &mut World, events: &mut EventStore, asset: &mut AssetManager) {
  let background = asset.texture.load(MODAL_BACKGROUND).expect("Failed to load collectable modal background");
  let (.., mut builder) = make_modal(world, events, asset, String::from(MODAL_TITLE), MODAL_SIZE, background);

  let data = load_help_data().expect("Failed to load help data");

  for (index, line) in data.iter().enumerate() {
    let y = MODAL_CONTENT_START_Y + index as f32 * MODAL_HELP_LINE_HEIGHT;
    let line_alignment = Alignment::new(Align::Start(MODAL_MARGIN), Align::Start(y));
    world.add(builder.make_text::<Modal>(line, line_alignment));
  };
}


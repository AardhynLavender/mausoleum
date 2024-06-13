/**
 * Modal for displaying story progression
 */

use std::time::Duration;

use crate::engine::asset::asset::AssetManager;
use crate::engine::core::event::EventStore;
use crate::engine::ecs::system::SysArgs;
use crate::engine::ecs::world::World;
use crate::engine::utility::alias::Size2;
use crate::engine::utility::alignment::{Align, Alignment};
use crate::engine::utility::state::State;
use crate::game::scene::credits::scene::CreditScene;
use crate::game::scene::level::story::data::StoryItem;
use crate::game::ui::iterative_text::IterativeTextBuilder;
use crate::game::ui::modal::{make_modal, Modal, use_escape_modal};

const MODAL_BACKGROUND: &str = "asset/hud/story_pane.png";
const MODAL_SIZE: Size2 = Size2::new(300, 200);
const MODAL_MARGIN: f32 = 8.0;

const DESCRIPTION_TOP_OFFSET: f32 = 24.0;
const DESCRIPTION_LINE_HEIGHT: f32 = 8.0;
const CHAR_ITERATION_MS: Duration = Duration::from_millis(32);

struct Endgame;

/// Display the story data event in a modal
pub fn make_story_modal(world: &mut World, events: &mut EventStore, asset: &mut AssetManager, entry: &StoryItem, state: &mut State) {
  let StoryItem { title, data, endgame, .. } = entry;
  if *endgame { state.add(Endgame).expect("Failed to add endgame state"); }

  let background = asset.texture.load(MODAL_BACKGROUND).expect("Failed to load collectable modal background");
  let (.., mut builder) = make_modal(world, events, asset, title.clone(), MODAL_SIZE, background);

  let mut accumulated_duration = Duration::from_millis(0);
  for (index, line) in data.iter().enumerate() {
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

pub fn sys_story_modal(SysArgs { state, scene, world, event, .. }: &mut SysArgs) -> Result<(), String> {
  if !event.is_paused() { return Ok(()); };
  if use_escape_modal(world, event) {
    if state.get::<Endgame>().is_ok() {
      scene.queue_next(CreditScene);
      state.remove::<Endgame>().expect("Failed to remove endgame state");
    }
  }
  Ok(())
}

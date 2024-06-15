/**
 * Modal UI component and lifecycle functions
 */

use crate::engine::asset::asset::AssetManager;
use crate::engine::asset::texture::{SrcRect, TextureKey};
use crate::engine::component::position::Position;
use crate::engine::component::sprite::Sprite;
use crate::engine::core::event::EventStore;
use crate::engine::ecs::world::World;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::render::camera::{Sticky1, Sticky2};
use crate::engine::utility::alias::Size2;
use crate::engine::utility::alignment::{Align, Aligner, Alignment};
use crate::engine::utility::color::color;
use crate::game::constant::WINDOW;
use crate::game::ui::text_builder::TextBuilder;
use crate::game::utility::controls::{Behaviour, Control, is_control};

const MODAL_MARGIN: f32 = 8.0;

/// Mark an entity as a member of a modal
#[derive(Default)]
pub struct Modal;

/// Create a modal UI with a title and background texture
pub fn make_modal<'m, 'a>(
  world: &'m mut World,
  events: &mut EventStore,
  asset: &'a mut AssetManager,
  title: String,
  size: Size2,
  background: TextureKey,
) -> (Aligner, TextBuilder<'a, 'a, Sticky2>, ) where {
  let loader = &mut asset.texture;
  let typeface = asset.typeface
    .use_store()
    .get("typeface")
    .expect("Failed to get typeface");

  let position = WINDOW.center(size);

  world.add((
    Modal,
    Sticky1::default(),
    Position::from(position),
    Sprite::new(background, SrcRect::new(Vec2::default(), size)),
  ));

  let aligner = Aligner::new(Rec2::new(Vec2::<i32>::from(position), size));
  let mut builder: TextBuilder<'a, 'a, Sticky2> = TextBuilder::new(typeface, loader, color::TEXT, aligner);

  world.add(builder.make_text::<Modal>(title.clone(), Alignment::new(Align::Center(0.0), Align::Start(MODAL_MARGIN))));

  events.queue_pause();

  (aligner, builder)
}

/// Remove all components tagged as part of a modal and resume the game
pub fn close_modal(world: &mut World, event: &mut EventStore, resume_on_close: bool) -> Result<(), String> {
  // remove all components with a Modal component
  let queued_free = world.query::<(&Modal, )>()
    .into_iter()
    .map(|(entity, ..)| (entity))
    .collect::<Vec<_>>();

  if !queued_free.is_empty() {
    for entity in queued_free { world.free_now(entity)? }
    if resume_on_close { event.queue_resume(); }
  }

  Ok(())
}

/// Close a modal when the escape key is pressed, returning true if the modal was closed
pub fn use_escape_modal(world: &mut World, event: &mut EventStore) -> bool {
  let exit = is_control(Control::Escape, Behaviour::Pressed, event);
  if exit { close_modal(world, event, true).expect("Failed to close modal"); }
  exit
}


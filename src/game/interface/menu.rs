use hecs::Entity;

use crate::engine::asset::AssetManager;
use crate::engine::asset::texture::SrcRect;
use crate::engine::component::text::TextBuilder;
use crate::engine::component::ui::Selection;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::rendering::camera::Sticky2;
use crate::engine::rendering::color::color;
use crate::engine::rendering::component::Sprite;
use crate::engine::state::State;
use crate::engine::system::{SysArgs, Systemize};
use crate::engine::utility::alias::Size2;
use crate::engine::utility::alignment::{Align, Aligner, Alignment};
use crate::engine::world::World;
use crate::game::constant::WINDOW;
use crate::game::interface::cursor::{make_cursor, place_cursor};
use crate::game::physics::position::Position;
use crate::game::scene::menu::MenuScene;
use crate::game::utility::controls::{Behaviour, Control, is_control};

#[derive(Default)]
pub struct MenuPane;

#[derive(Debug)]
pub struct GameMenuState {
  pub interface: Selection,
  pub cursor: Entity,
}

pub const PANE_DIMENSIONS: Size2 = Size2::new(128, 109);
pub const PANE_TOP_MARGIN: f32 = 8.0;
pub const PANE_LEFT_MARGIN: f32 = 24.0;
pub const BUTTONS_START_Y: f32 = 24.0;
pub const BUTTONS_GAP_Y: f32 = 16.0;

/// Add the in-game menu UI to the world
pub fn make_menu(world: &mut World, asset: &mut AssetManager, state: &mut State) {
  let textures = &mut asset.texture;
  let typeface = asset.typeface
    .use_store()
    .get("typeface")
    .expect("Failed to get typeface");
  let background = textures
    .load("asset/hud/menu_pane.png")
    .expect("Failed to load pane texture");
  let cursor_texture = textures.load("asset/hud/cursor.png").expect("Failed to load cursor texture");

  let pane_position = WINDOW.center(PANE_DIMENSIONS);
  let pane_aligner = Aligner::new(Rec2::new(Vec2::<i32>::from(pane_position), PANE_DIMENSIONS));
  let mut builder: TextBuilder<'_, '_, Sticky2> = TextBuilder::new(typeface, textures, color::TEXT, &pane_aligner);

  // pane background
  world.add((
    MenuPane,
    Sticky2::default(),
    Position::from(pane_position),
    Sprite::new(background, SrcRect::new(Vec2::default(), PANE_DIMENSIONS)),
  ));

  world.add(builder.make_text::<MenuPane>("Menu", Alignment::new(Align::Center(0.0), Align::Start(PANE_TOP_MARGIN))));

  let interface = Selection::build([
    world.add(builder.make_text::<MenuPane>("resume", Alignment::new(Align::Start(PANE_LEFT_MARGIN), Align::Start(BUTTONS_START_Y)))),
    world.add(builder.make_text::<MenuPane>("help", Alignment::new(Align::Start(PANE_LEFT_MARGIN), Align::Start(BUTTONS_START_Y + BUTTONS_GAP_Y)))),
    world.add(builder.make_text::<MenuPane>("preferences", Alignment::new(Align::Start(PANE_LEFT_MARGIN), Align::Start(BUTTONS_START_Y + BUTTONS_GAP_Y * 2.0)))),
    world.add(builder.make_text::<MenuPane>("exit", Alignment::new(Align::Start(PANE_LEFT_MARGIN), Align::Start(BUTTONS_START_Y + BUTTONS_GAP_Y * 3.0)))),
    world.add(builder.make_text::<MenuPane>("quit", Alignment::new(Align::Start(PANE_LEFT_MARGIN), Align::Start(BUTTONS_START_Y + BUTTONS_GAP_Y * 4.0)))),
  ]).expect("Failed to build selection");

  let cursor = make_cursor::<MenuPane>(world, cursor_texture);
  place_cursor(world, cursor, &interface);

  state.add(GameMenuState { cursor, interface }).expect("Failed to add menu state")
}

/// Remove the in-game menu UI from the world
fn close_menu(world: &mut World, state: &mut State) {
  state.remove::<GameMenuState>().expect("Failed to remove menu state");

  let queued_free = world.query::<()>()
    .with::<&MenuPane>()
    .into_iter()
    .collect::<Vec<_>>();
  for (entity, _) in queued_free { world.free_now(entity).expect("Failed to free menu pane") }
}

#[allow(unused)]
impl Systemize for MenuPane {
  fn system(SysArgs { world, scene, event, state, .. }: &mut SysArgs) -> Result<(), String> {
    if !event.is_paused() { return Ok(()); }

    let menu = state.get_mut::<GameMenuState>()?;

    let up = is_control(Control::Up, Behaviour::Pressed, event);
    let down = is_control(Control::Down, Behaviour::Pressed, event);
    let delta = if up { -1 } else if down { 1 } else { 0 };
    menu.interface += delta;
    if delta != 0 { place_cursor(world, menu.cursor, &menu.interface); }

    let exit = is_control(Control::Escape, Behaviour::Pressed, event);
    let select = is_control(Control::Select, Behaviour::Pressed, event);

    if exit {
      close_menu(world, state);
      event.queue_resume();
    } else if select {
      let (index, ..) = menu.interface.get_selection();
      match index {
        0 => {
          close_menu(world, state);
          event.queue_resume();
        }
        1 | 2 => { eprintln!("Not implemented yet"); }
        3 => {
          close_menu(world, state);
          scene.queue_next(MenuScene);
        }
        4 => { event.queue_quit(); }
        _ => { unreachable!() }
      }
    }

    Ok(())
  }
}


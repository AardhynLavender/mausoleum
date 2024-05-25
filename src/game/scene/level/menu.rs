use crate::engine::asset::AssetManager;
use crate::engine::component::ui::Selection;
use crate::engine::event::EventStore;
use crate::engine::system::{SysArgs, Systemize};
use crate::engine::utility::alias::Size2;
use crate::engine::utility::alignment::{Align, Alignment};
use crate::engine::world::World;
use crate::game::interface::cursor::make_cursor;
use crate::game::modal::modal::{close_modal, make_modal, Modal, use_escape_modal};
use crate::game::scene::menu::MenuScene;
use crate::game::utility::controls::{Behaviour, Control, is_control};

#[derive(Default)]
pub struct MenuPane;

pub const PANE_DIMENSIONS: Size2 = Size2::new(128, 109);
pub const PANE_TOP_MARGIN: f32 = 8.0;
pub const PANE_LEFT_MARGIN: f32 = 24.0;
pub const BUTTONS_START_Y: f32 = 24.0;
pub const BUTTONS_GAP_Y: f32 = 16.0;

/// Add the in-game menu UI to the world
pub fn make_menu(world: &mut World, event: &mut EventStore, asset: &mut AssetManager) {
  let textures = &mut asset.texture;
  let background = textures
    .load("asset/hud/menu_pane.png")
    .expect("Failed to load pane texture");
  let cursor_texture = textures.load("asset/hud/cursor.png").expect("Failed to load cursor texture");

  let (.., mut builder) = make_modal(world, event, asset, String::from("Menu"), PANE_DIMENSIONS, background);

  world.add(builder.make_text::<Modal>("Menu", Alignment::new(Align::Center(0.0), Align::Start(PANE_TOP_MARGIN))));
  let buttons = [
    world.add(builder.make_text::<Modal>("resume", Alignment::new(Align::Start(PANE_LEFT_MARGIN), Align::Start(BUTTONS_START_Y)))),
    world.add(builder.make_text::<Modal>("help", Alignment::new(Align::Start(PANE_LEFT_MARGIN), Align::Start(BUTTONS_START_Y + BUTTONS_GAP_Y)))),
    world.add(builder.make_text::<Modal>("preferences", Alignment::new(Align::Start(PANE_LEFT_MARGIN), Align::Start(BUTTONS_START_Y + BUTTONS_GAP_Y * 2.0)))),
    world.add(builder.make_text::<Modal>("exit", Alignment::new(Align::Start(PANE_LEFT_MARGIN), Align::Start(BUTTONS_START_Y + BUTTONS_GAP_Y * 3.0)))),
    world.add(builder.make_text::<Modal>("quit", Alignment::new(Align::Start(PANE_LEFT_MARGIN), Align::Start(BUTTONS_START_Y + BUTTONS_GAP_Y * 4.0)))),
  ];

  let cursor = make_cursor::<Modal>(world, cursor_texture);

  world.add((
    Selection::build(buttons, cursor).expect("Failed to build selection"),
    Modal,
    MenuPane,
  ));
}

#[allow(unused)]
impl Systemize for MenuPane {
  fn system(SysArgs { world, scene, event, state, .. }: &mut SysArgs) -> Result<(), String> {
    if !event.is_paused() { return Ok(()); };

    use_escape_modal(world, event);

    if let Some((.., menu)) = world.query_one_with::<&mut Selection, (&MenuPane, &Modal)>() {
      let up = is_control(Control::Up, Behaviour::Pressed, event);
      let down = is_control(Control::Down, Behaviour::Pressed, event);
      let delta = if up { -1 } else if down { 1 } else { 0 };
      *menu += delta;

      let select = is_control(Control::Select, Behaviour::Pressed, event);
      if select {
        let (index, ..) = menu.get_selection();
        match index {
          0 => {
            close_modal(world, event).expect("Failed to close modal");
          }
          1 | 2 => { eprintln!("Not implemented yet"); }
          3 => {
            close_modal(world, event).expect("Failed to close modal");
            scene.queue_next(MenuScene);
          }
          4 => { event.queue_quit(); }
          _ => { unreachable!() }
        }
      }
    }

    Ok(())
  }
}


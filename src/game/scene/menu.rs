use hecs::Entity;

/**
 * The game menu scene
 */

use crate::engine::asset::AssetManager;
use crate::engine::component::text::TextBuilder;
use crate::engine::component::ui::Selection;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::lifecycle::LifecycleArgs;
use crate::engine::rendering::camera::{Sticky1, Sticky2};
use crate::engine::rendering::color::color;
use crate::engine::scene::Scene;
use crate::engine::state::State;
use crate::engine::system::{Schedule, SysArgs, Systemize, SystemTag};
use crate::engine::utility::alias::{Size, Size2};
use crate::engine::utility::alignment::{Align, Aligner, Alignment};
use crate::engine::world::World;
use crate::game::constant::{DEV_SAVE_FILE, USER_SAVE_FILE, WINDOW};
use crate::game::interface::cursor::{CURSOR_MARGIN, make_cursor, place_cursor};
use crate::game::persistence::data::SaveData;
use crate::game::scene::level::scene::LevelScene;
use crate::game::utility::controls::{Behaviour, Control, is_control};

pub const TITLE_Y: f32 = 80.0;
pub const COPYRIGHT_MARGIN: f32 = 10.0;

pub const BUTTON_GAP: f32 = 16.0;
pub const BUTTON_COUNT: f32 = 4.0;
pub const OPTIONS_BOUNDS: Size2 = Size2::new(48, (BUTTON_GAP * BUTTON_COUNT) as Size);

// State //

#[derive(Debug)]
struct MainMenuState {
  pub interface: Selection,
  pub cursor: Entity,
}

// World //

/// Add the main menu UI to the world
pub fn add_ui(world: &mut World, asset: &mut AssetManager, state: &mut State) {
  // load
  let textures = &mut asset.texture;
  let typeface = asset.typeface.use_store().get("typeface").expect("Failed to get typeface");
  let cursor_texture = textures.load("asset/hud/cursor.png").expect("Failed to load cursor texture");

  // Static tex
  let mut static_builder: TextBuilder::<Sticky2> = TextBuilder::<Sticky2>::new(typeface, textures, color::TEXT, &WINDOW);
  world.add(static_builder.make_text::<()>("Aardhyn Lavender 2024", Alignment::new(Align::Center(0.0), Align::End(COPYRIGHT_MARGIN))));

  // buttons
  let buttons_position = WINDOW.center(OPTIONS_BOUNDS);
  let buttons_aligner = Aligner::new(Rec2::new(Vec2::<i32>::from(buttons_position), OPTIONS_BOUNDS));
  let mut button_builder: TextBuilder<'_, '_, Sticky1> = TextBuilder::new(typeface, textures, color::TEXT, &buttons_aligner);
  let interface = Selection::build([
    world.add(button_builder.make_text::<()>("start", Alignment::new(Align::Start(CURSOR_MARGIN), Align::Start(0.0)))),
    world.add(button_builder.make_text::<()>("new game", Alignment::new(Align::Start(CURSOR_MARGIN), Align::Start(BUTTON_GAP)))),
    world.add(button_builder.make_text::<()>("options", Alignment::new(Align::Start(CURSOR_MARGIN), Align::Start(BUTTON_GAP * 2.0)))),
    world.add(button_builder.make_text::<()>("quit", Alignment::new(Align::Start(CURSOR_MARGIN), Align::Start(BUTTON_GAP * 3.0)))),
  ]).expect("Failed to build selection");

  // cursor
  let cursor = make_cursor::<()>(world, cursor_texture);
  place_cursor(world, cursor, &interface);

  state.add(MainMenuState { cursor, interface }).expect("Failed to add menu state");
}

// Scene ///

// The main menu scene that will be displayed when the game starts.
pub struct MenuScene;

impl Scene for MenuScene {
  /// Set up the main menu scene
  fn setup(&mut self, LifecycleArgs { world, system, asset, state, .. }: &mut LifecycleArgs) {
    add_ui(world, asset, state);

    system.add(Schedule::FrameUpdate, SystemTag::Scene, MenuScene::system).expect("Failed to add menu system");
  }
  /// Destroy the main menu scene
  fn destroy(&mut self, LifecycleArgs { state, .. }: &mut LifecycleArgs) {
    state.remove::<MainMenuState>().expect("Failed to remove menu state");
  }
}

/// Manage the selection of the main menu
impl Systemize for MenuScene {
  fn system(SysArgs { scene, event, world, state, .. }: &mut SysArgs) -> Result<(), String> {
    let menu = state.get_mut::<MainMenuState>()?;

    let up = is_control(Control::Up, Behaviour::Pressed, event);
    let down = is_control(Control::Down, Behaviour::Pressed, event);
    let delta = if up { -1 } else if down { 1 } else { 0 };

    menu.interface += delta;
    if delta != 0 { place_cursor(world, menu.cursor, &menu.interface); }

    if is_control(Control::Select, Behaviour::Pressed, event) {
      let (index, ..) = menu.interface.get_selection();
      match index {
        0 => {
          let save_data = SaveData::from_file(USER_SAVE_FILE)
            .unwrap_or(SaveData::from_file(DEV_SAVE_FILE)
              .unwrap_or(SaveData::default()));
          scene.queue_next(LevelScene::new(save_data))
        }
        1 => {
          // delete old save data and start from default
          let save_data = SaveData::from_erased(USER_SAVE_FILE)
            .unwrap_or(SaveData::default());
          scene.queue_next(LevelScene::new(save_data))
        }
        2 => { eprintln!("Not implemented yet") }
        3 => { event.queue_quit() }
        _ => { unreachable!("Invalid menu selection index"); }
      }
    }

    Ok(())
  }
}
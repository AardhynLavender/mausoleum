/**
 * The game menu
 */

use crate::engine::asset::AssetManager;
use crate::engine::component::text::{Text, TextBuilder};
use crate::engine::component::ui::Selection;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::lifecycle::LifecycleArgs;
use crate::engine::rendering::color::color;
use crate::engine::scene::Scene;
use crate::engine::state::State;
use crate::engine::system::{Schedule, SysArgs, Systemize, SystemTag};
use crate::engine::utility::alignment::{Align, Alignment};
use crate::engine::world::World;
use crate::game::constant::{BUTTONS_BEGIN_Y, BUTTONS_Y_GAP, COPYRIGHT_MARGIN, DEV_SAVE_FILE, TITLE_Y, USER_SAVE_FILE, WINDOW};
use crate::game::persistence::data::SaveData;
use crate::game::physics::position::Position;
use crate::game::scene::level::scene::LevelScene;
use crate::game::utility::controls::{Behaviour, Control, is_control};

// State //

struct MainMenuState {
  pub interface: Selection,
}

// World //

/// Add the main menu UI to the world
pub fn add_ui(world: &mut World, asset: &mut AssetManager, state: &mut State) {
  let textures = &mut asset.texture;
  let typeface = asset.typeface
    .use_store()
    .get("typeface")
    .expect("Failed to get typeface");
  let mut builder = TextBuilder::new(typeface, textures, color::TEXT, &WINDOW);

  // static text
  world.add(builder.make_text::<()>("Metroidvania", Alignment::new(Align::Center(0.0), Align::At(TITLE_Y))));
  world.add(builder.make_text::<()>("copyright aardhyn lavender 2024", Alignment::new(Align::Center(0.0), Align::End(COPYRIGHT_MARGIN))));

  // add buttons
  state.add(MainMenuState {
    interface: Selection::build([
      world.add(builder.make_text::<()>("start", Alignment::new(Align::Center(0.0), Align::At(BUTTONS_BEGIN_Y)))),
      world.add(builder.make_text::<()>("new game", Alignment::new(Align::Center(0.0), Align::At(BUTTONS_BEGIN_Y + BUTTONS_Y_GAP)))),
      world.add(builder.make_text::<()>("options", Alignment::new(Align::Center(0.0), Align::At(BUTTONS_BEGIN_Y + BUTTONS_Y_GAP * 2.0)))),
      world.add(builder.make_text::<()>("quit", Alignment::new(Align::Center(0.0), Align::At(BUTTONS_BEGIN_Y + BUTTONS_Y_GAP * 3.0)))),
    ]).expect("Failed to build selection")
  }).expect("Failed to add menu state");
}

// Scene //

/// The main menu scene that will be displayed when the game starts.
pub struct MenuScene;

impl Scene for MenuScene {
  /// Set up the main menu scene
  fn setup(&mut self, LifecycleArgs { world, system, asset, state, .. }: &mut LifecycleArgs) {
    add_ui(world, asset, state);

    system.add(Schedule::FrameUpdate, SystemTag::Scene, MenuScene::system).expect("Failed to add menu system");
    system.add(Schedule::PostUpdate, SystemTag::Scene, sys_render_selected).expect("Failed to add render selected system");
  }
  /// Destroy the main menu scene
  fn destroy(&mut self, LifecycleArgs { state, .. }: &mut LifecycleArgs) {
    state.remove::<MainMenuState>().expect("Failed to remove menu state");
  }
}

/// Manage the selection of the main menu
impl Systemize for MenuScene {
  fn system(SysArgs { scene, event, state, .. }: &mut SysArgs) -> Result<(), String> {
    println!("Menu Scene!");

    let state = state.get_mut::<MainMenuState>()?;
    if is_control(Control::Down, Behaviour::Pressed, event) { state.interface += 1; }
    if is_control(Control::Up, Behaviour::Pressed, event) { state.interface -= 1; }
    if is_control(Control::Select, Behaviour::Pressed, event) {
      let (index, ..) = state.interface.get_selection();
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

/// Render a box around the selected item
pub fn sys_render_selected(SysArgs { world, render, state, .. }: &mut SysArgs) -> Result<(), String> {
  let state = state.get::<MainMenuState>()?;
  let (.., entity) = state.interface.get_selection();
  let (position, text) = world
    .query_entity::<(&Position, &Text)>(entity)
    .map_err(|_| String::from("Failed to get selected text"))?;
  let rect = Rec2::new(
    Vec2::<i32>::from(position.0.clone()) - Vec2::new(2, 1),
    text.get_dimensions().clone() + Vec2::new(3, 3),
  );
  render.draw_rect(rect, color::PRIMARY);

  Ok(())
}
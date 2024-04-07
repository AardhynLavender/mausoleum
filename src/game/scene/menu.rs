use crate::engine::asset::AssetManager;
use crate::engine::component::text::{Text, TextBuilder};
use crate::engine::component::ui::Selection;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::lifecycle::LifecycleArgs;
use crate::engine::rendering::color::color;
use crate::engine::scene::Scene;
use crate::engine::state::State;
use crate::engine::system::{Schedule, SysArgs};
use crate::engine::utility::alignment::{Align, Alignment};
use crate::engine::world::World;
use crate::game::constant::{BUTTONS_BEGIN_Y, BUTTONS_Y_GAP, COPYRIGHT_MARGIN, TITLE_Y, WINDOW};
use crate::game::physics::position::Position;
use crate::game::scene::level::scene::LevelScene;
use crate::game::utility::controls::{Behaviour, Control, is_control};

/**
 * The game menu
 */

// State //

struct MenuState {
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
  world.add(builder.make_text("Metroidvania", Alignment::new(Align::Center(0.0), Align::At(TITLE_Y))));
  world.add(builder.make_text("copyright aardhyn lavender 2024", Alignment::new(Align::Center(0.0), Align::End(COPYRIGHT_MARGIN))));

  // add buttons
  state.add(MenuState {
    interface: Selection::build([
      world.add(builder.make_text("start", Alignment::new(Align::Center(0.0), Align::At(BUTTONS_BEGIN_Y)))),
      world.add(builder.make_text("options", Alignment::new(Align::Center(0.0), Align::At(BUTTONS_BEGIN_Y + BUTTONS_Y_GAP)))),
      world.add(builder.make_text("quit", Alignment::new(Align::Center(0.0), Align::At(BUTTONS_BEGIN_Y + BUTTONS_Y_GAP * 2.0)))),
    ]).expect("Failed to build selection")
  }).expect("Failed to add menu state");
}

// Scene //

/// The main menu scene that will be displayed when the game starts.
pub struct MenuScene;

impl Scene for MenuScene {
  /// Set up the main menu scene
  fn setup(&self, LifecycleArgs { world, camera, asset, state, .. }: &mut LifecycleArgs) {
    camera.release(Vec2::default());
    add_ui(world, asset, state);
  }
  /// Add systems to the main menu scene
  fn add_systems(&self, LifecycleArgs { system, .. }: &mut LifecycleArgs) {
    system.add(Schedule::FrameUpdate, sys_menu_selection);
    system.add(Schedule::PostUpdate, sys_render_selected);
  }
  /// Destroy the main menu scene
  fn destroy(&self, _: &mut LifecycleArgs) {}
}

// Systems //

/// Manage the selection of the main menu
pub fn sys_menu_selection(SysArgs { scene, event, state, .. }: &mut SysArgs) {
  let state = state.get_mut::<MenuState>().expect("Failed to get menu state");
  if is_control(Control::Down, Behaviour::Pressed, event) {
    state.interface += 1;
  }
  if is_control(Control::Up, Behaviour::Pressed, event) {
    state.interface -= 1;
  }
  if is_control(Control::Select, Behaviour::Pressed, event) {
    let (index, ..) = state.interface.get_selection();
    match index {
      0 => scene.queue_next(LevelScene::build("room_0").expect("Failed to build level scene")),
      1 => println!("Not implemented yet"),
      2 => event.queue_quit(),
      _ => panic!("Invalid selection")
    }
  }
}

/// Render a box around the selected item
pub fn sys_render_selected(SysArgs { world, render, state, .. }: &mut SysArgs) {
  let state = state.get::<MenuState>().expect("Failed to get menu state");
  let (.., entity) = state.interface.get_selection();
  let (position, text) = world.query_entity::<(&Position, &Text)>(entity).expect("Failed to get selection");
  let rect = Rec2::new(
    Vec2::<i32>::from(position.0.clone()),
    text.get_dimensions().clone(),
  );
  render.draw_rect(rect, color::PRIMARY);
}
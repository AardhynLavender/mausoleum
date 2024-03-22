use crate::engine::asset::AssetManager;
use crate::engine::component::text::Text;
use crate::engine::component::ui::Selection;
use crate::engine::geometry::{Rec2, Vec2};
use crate::engine::lifecycle::LifecycleArgs;
use crate::engine::render::color::color;
use crate::engine::scene::Scene;
use crate::engine::system::{Schedule, SysArgs, SystemManager};
use crate::engine::world::{push_state_with, use_state, World};
use crate::game::component::position::Position;
use crate::game::constant::{BUTTONS_BEGIN_Y, BUTTONS_Y_GAP, COPYRIGHT_MARGIN, TITLE_Y};
use crate::game::scene::level::LevelScene;
use crate::game::utility::controls::{Behaviour, Control, is_control};
use crate::game::utility::position::{align_end, center_horizontal};

/**
 * The main menu scene
 */

// State //

struct MenuState {
  pub interface: Selection,
}

// World //

/// Add the main menu UI to the world
pub fn add_ui(world: &mut World, asset: &mut AssetManager) {
  let typeface = asset.typeface
    .use_store()
    .get("typeface")
    .expect("Failed to get typeface");

  // static text
  let title_text = Text::new(color::TEXT).with_content("demo game", &typeface, &mut asset.texture);
  let copyright_text = Text::new(color::TEXT2).with_content("copyright aardhyn lavender 2024", &typeface, &mut asset.texture);
  world.add((
    Position::new(
      center_horizontal(title_text.get_dimensions().x as f32),
      TITLE_Y,
    ),
    title_text,
  ));
  world.add((
    Position::new(
      center_horizontal(copyright_text.get_dimensions().x as f32),
      align_end(COPYRIGHT_MARGIN + copyright_text.get_dimensions().y as f32),
    ),
    copyright_text,
  ));

  // buttons
  let start_text = Text::new(color::TEXT).with_content("start", &typeface, &mut asset.texture);
  let start = world.add((
    Position::new(
      center_horizontal(start_text.get_dimensions().x as f32),
      BUTTONS_BEGIN_Y,
    ),
    start_text,
  ));
  let options_text = Text::new(color::TEXT).with_content("options", &typeface, &mut asset.texture);
  let options = world.add((
    Position::new(
      center_horizontal(options_text.get_dimensions().x as f32),
      BUTTONS_BEGIN_Y + BUTTONS_Y_GAP,
    ),
    options_text,
  ));
  let quit_text = Text::new(color::TEXT).with_content("quit", &typeface, &mut asset.texture);
  let quit = world.add((
    Position::new(
      center_horizontal(quit_text.get_dimensions().x as f32),
      BUTTONS_BEGIN_Y + BUTTONS_Y_GAP * 2.0,
    ),
    quit_text,
  ));

  // create a selection of these buttons
  push_state_with::<MenuState>(
    world,
    MenuState {
      interface: Selection::build([
        start,
        options,
        quit
      ]).expect("Failed to build selection")
    },
  );
}

// Scene //

/// The main menu scene that will be displayed when the game starts.
pub struct MenuScene;

impl Scene for MenuScene {
  /// Set up the main menu scene
  fn setup(&self, LifecycleArgs { system, world, asset, .. }: &mut LifecycleArgs) {
    add_ui(world, asset);
    add_systems(system);
  }
  /// Destroy the main menu scene
  fn destroy(&self, _: &mut LifecycleArgs) {}
}

// Systems //

fn add_systems(system: &mut SystemManager) {
  system.add(Schedule::FrameUpdate, sys_menu_selection);
  system.add(Schedule::PostUpdate, sys_render_selected);
}

/// Manage the selection of the main menu
pub fn sys_menu_selection(SysArgs { world, scene, event, .. }: &mut SysArgs) {
  let state = use_state::<MenuState>(world);
  if is_control(Control::Down, Behaviour::Pressed, event) {
    state.interface += 1;
  }
  if is_control(Control::Up, Behaviour::Pressed, event) {
    state.interface -= 1;
  }
  if is_control(Control::Select, Behaviour::Pressed, event) {
    let (index, ..) = state.interface.get_selection();
    match index {
      0 => scene.queue_next(LevelScene::build(0).expect("Failed to build level scene")),
      1 => println!("Not implemented yet"),
      2 => event.queue_quit(),
      _ => panic!("Invalid selection")
    }
  }
}

/// Render a box around the selected item
pub fn sys_render_selected(SysArgs { world, render, .. }: &mut SysArgs) {
  let state = use_state::<MenuState>(world);
  let (.., entity) = state.interface.get_selection();
  let (position, text) = world.query_entity::<(&Position, &Text)>(entity).expect("Failed to get selection");
  let rect = Rec2::new(
    Vec2::<i32>::from(position.0.clone()),
    text.get_dimensions().clone(),
  );
  render.draw_rect(rect, color::PRIMARY);
}
use sdl2::keyboard::Keycode;
use crate::engine::application::EventArgs;
use crate::engine::scene::Scene;
use crate::engine::system::{Schedule, SysArgs};
use crate::game::scenes::level::LevelScene;

/**
 * The main menu scene
 */

/// The main menu scene that will be displayed when the game starts.
pub struct MenuScene;

impl Scene for MenuScene {
  fn setup(&self, (_, system, ..): &mut EventArgs) {
    println!("Setting up menu");
    system.add(Schedule::FrameUpdate, sys_menu_listener);
  }

  fn destroy(&self, _: &mut EventArgs) {
    println!("Destroying menu scene");
  }
}

pub fn sys_menu_listener((_, _, _, events, scenes, ..): &mut SysArgs) {
  if events.is_key_pressed(Keycode::G) {
    scenes.queue_next(LevelScene);
  }
}

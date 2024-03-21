use sdl2::keyboard::Keycode;
use crate::engine::lifecycle::LifecycleArgs;
use crate::engine::scene::Scene;
use crate::engine::system::{Schedule, SysArgs};
use crate::game::scenes::level::LevelScene;

/**
 * The main menu scene
 */

/// The main menu scene that will be displayed when the game starts.
pub struct MenuScene;

impl Scene for MenuScene {
  fn setup(&self, LifecycleArgs { system, .. }: &mut LifecycleArgs) {
    println!("Setting up menu");
    system.add(Schedule::FrameUpdate, sys_menu_listener);
  }

  fn destroy(&self, _: &mut LifecycleArgs) {
    println!("Destroying menu scene");
  }
}

pub fn sys_menu_listener(SysArgs { event, scene, .. }: &mut SysArgs) {
  if event.is_key_pressed(Keycode::G) {
    scene.queue_next(LevelScene);
  }
}

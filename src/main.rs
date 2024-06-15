#![deny(warnings)]
#![windows_subsystem = "windows"] // uncomment this for release builds

/**
 * Build and run the application
 */

use std::path::Path;

use crate::engine::application::Application;
use crate::engine::core::lifecycle::{Lifecycle, LifecycleArgs};
use crate::engine::render::renderer::Properties;
use crate::game::constant::{LOGICAL_SIZE, TYPEFACE_PATH, TYPEFACE_SIZE, WINDOW_SIZE, WINDOW_TITLE};
use crate::game::preferences::Preferences;
use crate::game::scene::main_menu::scene::MenuScene;

pub mod engine;
pub mod game;

fn main() -> Result<(), String> {
  Application::build(
    Properties {
      title: String::from(WINDOW_TITLE),
      screen_color: Default::default(),
      dimensions: WINDOW_SIZE,
      logical: Some(LOGICAL_SIZE),
      fullscreen: false,
      show_cursor: true,
      vsync: true,
      opengl: true,
      hardware_acceleration: true,
      software_acceleration: false,
    },
    Lifecycle {
      setup,
      destroy,
    },
    MenuScene,
  )
}

fn setup(LifecycleArgs { asset, state, .. }: LifecycleArgs) {
  asset.typeface.load(Path::new(TYPEFACE_PATH), TYPEFACE_SIZE).expect("Failed to load typeface");
  state.add::<Preferences>(Preferences::default()).expect("Failed to add preferences");
  println!("Game Initialized.");
}

fn destroy() {
  println!("Game Destroyed.");
}

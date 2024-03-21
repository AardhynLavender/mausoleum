#![deny(warnings)]
// #![windows_subsystem = "windows"]

use crate::engine::application::Application;
use crate::engine::lifecycle::{Lifecycle, LifecycleArgs};
use crate::engine::render::Properties;
use crate::game::constant::{LOGICAL_SIZE, WINDOW_SIZE, WINDOW_TITLE};
use crate::game::scenes::menu::MenuScene;

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
      show_cursor: false,
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

fn setup(_: LifecycleArgs) {
  println!("Setting up game");
}

fn destroy() {
  println!("Destroying game");
}
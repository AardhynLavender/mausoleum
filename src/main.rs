use std::path::Path;
use crate::engine::application::{run_application, Lifecycle, SetupArgs};
use crate::engine::render::{Properties};
use crate::engine::render::component::Sprite;
use crate::engine::system::Schedule;
use crate::game::component::physics::Gravity;
use crate::game::component::position::Position;
use crate::game::constant::{LOGICAL_SIZE, WINDOW_SIZE, WINDOW_TITLE};
use crate::game::system::physics::sys_gravity;

pub mod engine;
pub mod game;

fn main() -> Result<(), String> {
  run_application(
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
  )
}

fn setup((world, systems, assets): SetupArgs) {
  let sprite = Sprite::new(assets.texture.load(Path::new("asset/test.png")).expect("Failed to load texture"));

  systems.add(Schedule::FrameUpdate, sys_gravity);
  world.add((Position::new(80.0, 90.0), Gravity::new(0.0, 1.0), sprite));
}

fn destroy() {
  println!("Destroying game");
}
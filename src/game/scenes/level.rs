use std::path::Path;
use sdl2::keyboard::Keycode;
use crate::engine::application::EventArgs;
use crate::engine::render::component::Sprite;
use crate::engine::scene::Scene;
use crate::engine::system::{Schedule, SysArgs};
use crate::game::component::physics::Gravity;
use crate::game::component::position::Position;
use crate::game::scenes::menu::MenuScene;
use crate::game::system::physics::sys_gravity;

pub struct LevelScene;

impl Scene for LevelScene {
  fn setup(&self, (world, system, assets, ..): &mut EventArgs) {
    println!("Setting up level");
    let sprite = Sprite::new(assets.texture.load(Path::new("asset/test.png")).expect("Failed to load texture"));

    system.add(Schedule::FrameUpdate, sys_gravity);
    system.add(Schedule::FrameUpdate, sys_level_listener);

    world.add((Position::new(80.0, 90.0), Gravity::new(0.0, 1.0), sprite));
  }
  fn destroy(&self, _: &mut EventArgs) {
    println!("Destroying level");
  }
}

/// Events to listen for during the level scene
pub fn sys_level_listener((_, _, _, events, scenes, ..): &mut SysArgs) {
  if events.is_key_pressed(Keycode::E) {
    scenes.queue_next(MenuScene);
  }
}

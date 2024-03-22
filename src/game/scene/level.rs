use std::path::Path;

use crate::engine::lifecycle::LifecycleArgs;
use crate::engine::render::component::Sprite;
use crate::engine::scene::Scene;
use crate::engine::system::{Schedule, SysArgs};
use crate::game::component::physics::Gravity;
use crate::game::component::position::Position;
use crate::game::scene::menu::MenuScene;
use crate::game::system::physics::sys_gravity;
use crate::game::utility::controls::{Behaviour, Control, is_control};

/**
 * The level scenes
 */

pub struct LevelScene {
  level_key: u32,
}

impl LevelScene {
  pub fn build(level_key: u32) -> Result<Self, String> {
    Ok(Self { level_key })
  }
}

impl Scene for LevelScene {
  /// Set up the level scene
  fn setup(&self, LifecycleArgs { world, system, asset, .. }: &mut LifecycleArgs) {
    println!("Level {} Initialized.", self.level_key);
    system.add(Schedule::FrameUpdate, sys_gravity);
    system.add(Schedule::FrameUpdate, sys_level_listener);

    world.add((
      Position::new(80.0, 90.0),
      Gravity::new(0.0, 1.0),
      Sprite::new(asset.texture.load(Path::new("asset/test.png")).expect("Failed to load texture"))
    ));
  }
  /// Clean up the level scene
  fn destroy(&self, _: &mut LifecycleArgs) {}
}

/// Events to listen for during the level scene
pub fn sys_level_listener(SysArgs { event, scene, .. }: &mut SysArgs) {
  if is_control(Control::Escape, Behaviour::Pressed, event) {
    scene.queue_next(MenuScene);
  }
}

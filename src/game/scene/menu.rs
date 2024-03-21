use sdl2::keyboard::Keycode;

use crate::engine::component::text::Text;
use crate::engine::lifecycle::LifecycleArgs;
use crate::engine::render::color::color;
use crate::engine::scene::Scene;
use crate::engine::system::{Schedule, SysArgs};
use crate::game::component::position::Position;
use crate::game::scene::level::LevelScene;
use crate::game::utility::position::{align_end, center_horizontal};

/**
 * The main menu scene
 */

/// The main menu scene that will be displayed when the game starts.
pub struct MenuScene;

impl Scene for MenuScene {
  fn setup(&self, LifecycleArgs { system, world, asset, .. }: &mut LifecycleArgs) {
    println!("Setting up menu");

    let typeface = asset.typeface
      .use_store()
      .get("typeface")
      .expect("Failed to get typeface");

    let title = Text::new(color::TEXT).with_content("metroidvania demo game", &typeface, &mut asset.texture);
    let copyright = Text::new(color::TEXT).with_content("copyright aardhyn lavender 2024", &typeface, &mut asset.texture);

    let title_x = center_horizontal(title.get_dimensions().x as f32);
    let copyright_x = center_horizontal(copyright.get_dimensions().x as f32);
    let copyright_y = align_end(4.0 + title.get_dimensions().y as f32);

    world.add((
      title,
      Position::new(title_x, 96.0)
    ));
    world.add((
      copyright,
      Position::new(copyright_x, copyright_y)
    ));

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

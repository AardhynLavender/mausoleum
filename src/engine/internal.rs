use hecs::Entity;
use sdl2::keyboard::Keycode;

use crate::engine::component::text::Text;
use crate::engine::render::color::color;
use crate::engine::render::system::sys_render;
use crate::engine::system::{Schedule, SysArgs, SystemManager};
use crate::engine::world::World;
use crate::game::physics::position::Position;

/**
 * Internal engine systems
 */

struct FpsText;

static mut FPS_TEXT: Option<Entity> = None;

/// Add internal systems to the system manager
pub fn add_internal_systems(systems: &mut SystemManager) {
  systems.add(Schedule::FrameUpdate, sys_fullscreen_toggle);
  systems.add(Schedule::PostUpdate, sys_update_fps_text);
  systems.add(Schedule::PostUpdate, sys_render);
}

/// Add internal entities to the world
pub fn add_internal_entities(world: &mut World) {
  let fps_text = Text::new(color::TEXT3);
  unsafe {
    FPS_TEXT = Some(world.add((FpsText, fps_text, Position::new(4.0, 4.0))));
  }
}

/// Toggle fullscreen mode
fn sys_fullscreen_toggle(SysArgs { render, event, .. }: &mut SysArgs) {
  if event.is_key_pressed(Keycode::F11) {
    render.set_fullscreen(!render.is_fullscreen());
  }
}

fn sys_update_fps_text(SysArgs { delta, world, .. }: &mut SysArgs) {
  unsafe {
    if let Some(entity) = FPS_TEXT {
      let fps_text = world.query_entity::<&mut Text>(entity).expect("Failed to get fps text");
      fps_text.set_content(format!("FPS {:.4}", delta));
    }
  }
}

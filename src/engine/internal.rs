use sdl2::keyboard::Keycode;

use crate::engine::render::system::sys_render;
use crate::engine::system::{Schedule, SysArgs, SystemManager};

/**
 * Internal engine systems
 */

/// Add internal systems to the system manager
pub fn add_internal_systems(systems: &mut SystemManager) {
  systems.add(Schedule::PostUpdate, sys_render);
  systems.add(Schedule::FrameUpdate, sys_fullscreen_toggle);
}

fn sys_fullscreen_toggle(SysArgs { render, event, .. }: &mut SysArgs) {
  if event.is_key_pressed(Keycode::F11) {
    render.set_fullscreen(!render.is_fullscreen());
  }
}

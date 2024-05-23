use hecs::Entity;
use sdl2::keyboard::Keycode;

use crate::engine::component::text::Text;
use crate::engine::rendering::camera::{CameraTether, Sticky1};
use crate::engine::rendering::color::color;
use crate::engine::rendering::renderer::Renderer;
use crate::engine::system::{Schedule, SysArgs, Systemize, SystemManager, SystemTag};
use crate::engine::time::SECOND_MS;
use crate::engine::utility::alias::DeltaMS;
use crate::engine::world::World;
use crate::game::physics::position::Position;

/**
 * Internal engine systems
 */

struct FpsText;

static mut FPS_TEXT: Option<Entity> = None;
static mut MIN_FPS: DeltaMS = DeltaMS::MAX;
static mut MAX_FPS: DeltaMS = DeltaMS::MIN;

/// Add internal systems to the system manager
pub fn add_internal_systems(systems: &mut SystemManager) {
  systems.add(Schedule::PostUpdate, SystemTag::Internal, sys_fullscreen_toggle).expect("Failed to add fullscreen toggle system");

  //systems.add(Schedule::PostUpdate, sys_update_fps_text);
  systems.add(Schedule::PostUpdate, SystemTag::Internal, CameraTether::system).expect("Failed to add camera tether system");
  systems.add(Schedule::PostUpdate, SystemTag::Internal, Renderer::system).expect("Failed to add renderer system");
}

/// Add internal entities to the world
#[allow(unreachable_code, unused_variables)]
pub fn add_internal_entities(world: &mut World) {
  let fps_text = Text::new(color::TEXT);
  unsafe {
    FPS_TEXT = Some(world.add((FpsText, fps_text, Position::new(4.0, 4.0), Sticky1::default())));
  }
}

/// Toggle fullscreen mode
fn sys_fullscreen_toggle(SysArgs { render, event, .. }: &mut SysArgs) -> Result<(), String> {
  if event.is_key_pressed(Keycode::F11) { render.set_fullscreen(!render.is_fullscreen()); }
  Ok(())
}

#[allow(dead_code)]
fn sys_update_fps_text(SysArgs { delta, world, .. }: &mut SysArgs) -> Result<(), String> {
  unsafe {
    if let Some(entity) = FPS_TEXT {
      let fps_text = world.query_entity::<&mut Text>(entity).expect("Failed to get fps text");
      if *delta < MIN_FPS { MIN_FPS = *delta; }
      if *delta > MAX_FPS { MAX_FPS = *delta; }
      let fps_string = format!("FPS {:0>6.2} SLOW {:0>3.0} FAST {:0>3.0}", *delta * SECOND_MS, MIN_FPS * SECOND_MS, MAX_FPS * SECOND_MS);
      // println!("{}", fps_string);
      fps_text.set_content(fps_string);
    }
    Ok(())
  }
}

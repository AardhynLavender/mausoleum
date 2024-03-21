use crate::engine::render::system::sys_render;
use crate::engine::system::{Schedule, SystemManager};

/**
 * Internal engine systems
 */

/// Add internal systems to the system manager
pub fn add_internal_systems(systems: &mut SystemManager) {
  systems.add(Schedule::PostUpdate, sys_render);
}
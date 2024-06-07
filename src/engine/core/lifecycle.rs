
/**
  * Application lifecycle structures
  */

use crate::engine::asset::asset::AssetManager;
use crate::engine::ecs::system::SystemManager;
use crate::engine::ecs::world::World;
use crate::engine::render::camera::Camera;
use crate::engine::utility::state::State;

/// Structures needed for the application lifecycle
pub struct LifecycleArgs<'app, 'fonts> {
  pub world: &'app mut World,
  pub system: &'app mut SystemManager,
  pub camera: &'app mut Camera,
  pub state: &'app mut State,
  pub asset: &'app mut AssetManager<'fonts>,
}

impl<'app, 'fonts> LifecycleArgs<'app, 'fonts> {
  /// Instantiate a new event args wrapper
  pub fn new(
    world: &'app mut World,
    system: &'app mut SystemManager,
    camera: &'app mut Camera,
    state: &'app mut State,
    asset: &'app mut AssetManager<'fonts>,
  ) -> Self {
    Self { world, camera, system, asset, state }
  }
}

/// Lifecycle actions that can be performed by an application
pub struct Lifecycle {
  pub setup: fn(LifecycleArgs),
  pub destroy: fn(),
}

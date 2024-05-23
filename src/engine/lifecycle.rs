use crate::engine::asset::AssetManager;
use crate::engine::rendering::camera::Camera;
use crate::engine::state::State;
use crate::engine::system::SystemManager;
use crate::engine::world::World;

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

use crate::engine::asset::AssetManager;
use crate::engine::event::EventStore;
use crate::engine::render::Renderer;
use crate::engine::scene::SceneManager;
use crate::engine::utility::types::TDelta;
use crate::engine::world::World;

pub enum Schedule {
  /// Update the game state once per frame
  FrameUpdate,
  /// Update the game state at a fixed rate
  FixedUpdate,
  /// update state at the end of the frame before rendering
  PostUpdate,
}

/// Arguments passed to systems
pub struct SysArgs<'app, 'fonts> {
  pub delta: TDelta,
  pub world: &'app mut World,
  pub render: &'app mut Renderer,
  pub event: &'app mut EventStore,
  pub scene: &'app mut SceneManager,
  pub asset: &'app mut AssetManager<'fonts>,
}

impl<'app, 'fonts> SysArgs<'app, 'fonts> {
  /// Instantiate a new system args wrapper
  pub fn new(
    delta: TDelta,
    world: &'app mut World,
    render: &'app mut Renderer,
    event: &'app mut EventStore,
    scene: &'app mut SceneManager,
    asset: &'app mut AssetManager<'fonts>,
  ) -> Self {
    Self {
      delta,
      world,
      render,
      event,
      scene,
      asset,
    }
  }
}

/// A system mutably queries and/or updates the world
pub type System = fn(&mut SysArgs);

/// Manages the scheduling of mutable systems
#[derive(Default)]
pub struct SystemManager {
  frame_systems: Vec<System>,
  fixed_systems: Vec<System>,
  render_systems: Vec<System>,
}

impl SystemManager {
  /// Instantiate a new system manager
  pub fn new() -> Self {
    Self::default()
  }

  /// Add a system to a schedule
  pub fn add(&mut self, schedule: Schedule, system: System) {
    match schedule {
      Schedule::FrameUpdate => self.frame_systems.push(system),
      Schedule::FixedUpdate => self.fixed_systems.push(system),
      Schedule::PostUpdate => self.render_systems.push(system),
    }
  }

  /// Remove all systems from the manager
  pub fn clear(&mut self) {
    self.frame_systems.clear();
    self.fixed_systems.clear();
    self.render_systems.clear();
  }

  /// call the systems of a schedule
  pub fn update(&mut self, schedule: Schedule, args: &mut SysArgs) {
    match schedule {
      Schedule::FrameUpdate => self.frame_systems.iter(),
      Schedule::FixedUpdate => self.fixed_systems.iter(),
      Schedule::PostUpdate => self.render_systems.iter(),
    }.for_each(|system| {
      system(args);
    });
  }
}

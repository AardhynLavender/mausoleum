use crate::engine::asset::AssetManager;
use crate::engine::event::EventStore;
use crate::engine::internal::add_internal_systems;
use crate::engine::render::{Properties};
use crate::engine::subsystem::Subsystem;
use crate::engine::system::{Schedule, SystemManager};
use crate::engine::world::World;

/**
 * Application structure and lifecycle
 */

/// Structures needed for settpub ing pub up the game spub tate
pub type SetupArgs<'app, 'fonts> = (&'app mut World, &'app mut SystemManager, &'app mut AssetManager<'fonts>);

/// Lifecycle actions that can be performed by an application
pub struct Lifecycle {
  // pub load: fn(&mut AssetManager),
  pub setup: fn(SetupArgs),
  pub destroy: fn(),
}

/// Bundles a subsystem with actions
struct Application<'a> {
  subsystem: &'a mut Subsystem,
  event_store: EventStore,
  world: World,
  lifecycle: Lifecycle,
}

impl<'a> Application<'a> {
  /// Instantiate a new application using `subsystem` with `actions`
  fn new(subsystem: &'a mut Subsystem, lifecycle: Lifecycle) -> Self {
    Self {
      subsystem,
      event_store: EventStore::new(),
      world: World::new(),
      lifecycle,
    }
  }

  /// Load assets, setup state, and start the main loop
  pub fn run(&mut self, assets: &mut AssetManager) {
    let mut systems = SystemManager::new();

    add_internal_systems(&mut systems);

    (self.lifecycle.setup)((&mut self.world, &mut systems, assets));

    let delta = 1.0;

    loop {
      self.subsystem.events.update(&mut self.event_store);
      if self.subsystem.events.is_quit {
        break;
      }

      systems.update(
        Schedule::FrameUpdate,
        &mut (delta, &mut self.world, &mut self.subsystem.renderer, assets),
      );

      systems.update(
        Schedule::PostUpdate,
        &mut (delta, &mut self.world, &mut self.subsystem.renderer, assets),
      );
      self.subsystem.renderer.present();
    }

    (self.lifecycle.destroy)();
  }
}

/// Build subsystems and build application of `Properties` `TState` with `Actions`
pub fn run_application(
  properties: Properties,
  actions: Lifecycle,
) -> Result<(), String> {
  let mut subsystem = Subsystem::build(properties)?;
  let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
  let mut assets = AssetManager::new(&subsystem.renderer, &ttf_context);

  let mut app = Application::new(&mut subsystem, actions);
  app.run(&mut assets);

  Ok(())
}

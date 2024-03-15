use crate::engine::asset::AssetManager;
use crate::engine::event::EventStore;
use crate::engine::render::{Properties, Renderer};
use crate::engine::subsystem::Subsystem;

/**
 * Application structure
 */

/// Different actions that can be performed by an application
pub struct Actions<TState> {
  /// Set up the games static assets
  pub load: fn(&mut AssetManager),
  /// Render state and assets
  pub render: fn(&mut TState, &AssetManager, &mut Renderer),
  /// Update engine state
  pub update: fn(&EventStore, &AssetManager, &mut TState, &mut Renderer),
  /// Set up the state
  pub setup: fn(&AssetManager) -> TState,
}

/// Bundles a subsystem with actions
struct Application<'a, TState> {
  subsystem: &'a mut Subsystem,
  actions: Actions<TState>,
  event_store: EventStore,
}

impl<'a, TState> Application<'a, TState> {
  /// Instantiate a new application using `subsystem` with `actions`
  fn new(subsystem: &'a mut Subsystem, actions: Actions<TState>) -> Self {
    Self {
      subsystem,
      actions,
      event_store: EventStore::new(),
    }
  }

  /// Load assets, setup state, and start the main loop
  pub fn run(&mut self, assets: &mut AssetManager) {
    (self.actions.load)(assets);

    let mut state = (self.actions.setup)(assets);

    loop {
      self.subsystem.events.update(&mut self.event_store);
      if self.subsystem.events.is_quit {
        break;
      }

      (self.actions.update)(&self.event_store, assets, &mut state, &mut self.subsystem.renderer);
      (self.actions.render)(&mut state, assets, &mut self.subsystem.renderer);

      self.subsystem.renderer.present();
    }
  }
}

/// Build subsystems and build application of `Properties` `TState` with `Actions`
pub fn run_application<TState>(properties: Properties, actions: Actions<TState>) -> Result<(), String> {
  let mut subsystem = Subsystem::build(properties)?;
  let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
  let mut assets = AssetManager::new(&subsystem.renderer, &ttf_context);

  let mut app = Application::new(&mut subsystem, actions);
  app.run(&mut assets);

  Ok(())
}
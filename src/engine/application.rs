use crate::engine::asset::AssetManager;
use crate::engine::event::EventStore;
use crate::engine::geometry::shape::Vec2;
use crate::engine::internal::{add_internal_entities, add_internal_systems};
use crate::engine::lifecycle::{Lifecycle, LifecycleArgs};
use crate::engine::rendering::camera::{Camera, CameraBounds};
use crate::engine::rendering::renderer::Properties;
use crate::engine::scene::{Scene, SceneManager};
use crate::engine::state::State;
use crate::engine::subsystem::Subsystem;
use crate::engine::system::{Schedule, SysArgs, SystemManager};
use crate::engine::time::Frame;
use crate::engine::utility::alias::{DeltaMS, Size2};
use crate::engine::world::World;

/**
 * Application structure and lifecycle
 */

pub const SIMULATION_FPS: DeltaMS = 30.0 / 1_000.0;

/// Bundles a subsystem with actions
struct Engine<'a> {
  subsystem: &'a mut Subsystem,
  events: EventStore,
  scenes: SceneManager,
  camera: Camera,
  world: World,
  lifecycle: Lifecycle,
  last_frame: Frame,
  state: State,
}

impl<'a> Engine<'a> {
  /// Instantiate a new application using `subsystem` with `actions`
  fn new(subsystem: &'a mut Subsystem, dimensions: Size2, lifecycle: Lifecycle, scene: impl Scene + 'static) -> Self {
    Self {
      subsystem,
      events: EventStore::new(),
      scenes: SceneManager::new(scene),
      camera: Camera::new(CameraBounds::new(Vec2::default(), dimensions)),
      state: State::default(),
      world: World::new(),
      lifecycle,
      last_frame: Frame::default(),
    }
  }

  /// Load assets, setup state, and start the main loop
  pub fn start(&mut self, assets: &mut AssetManager) {
    let mut systems = SystemManager::new();

    add_internal_systems(&mut systems);
    add_internal_entities(&mut self.world);

    (self.lifecycle.setup)(LifecycleArgs::new(&mut self.world, &mut systems, &mut self.state, &mut self.camera, assets));

    loop {
      // compute delta time
      let (delta, ..) = self.last_frame.next();

      if self.scenes.is_queue() {
        self.scenes.next(&mut LifecycleArgs::new(&mut self.world, &mut systems, &mut self.state, &mut self.camera, assets))
      }

      self.subsystem.events.update(&mut self.events);
      if self.subsystem.events.is_quit {
        break;
      }

      let mut args = SysArgs::new(delta, &mut self.world, &mut self.subsystem.renderer, &mut self.events, &mut self.camera, &mut self.scenes, &mut self.state, assets);
      systems.update(Schedule::FrameUpdate, &mut args);
      systems.update(Schedule::PostUpdate, &mut args);

      self.subsystem.renderer.present();
    }

    (self.lifecycle.destroy)();
  }
}

/// Constructs and runs an application of `Properties` with `Actions`
pub struct Application;

impl Application {
  /// Build subsystems and start application of `Properties` with `Actions`
  pub fn build(
    properties: Properties,
    actions: Lifecycle,
    initial_scene: impl Scene + 'static,
  ) -> Result<(), String> {
    let dimensions = properties.logical.unwrap_or(properties.dimensions);
    let mut subsystem = Subsystem::build(properties)?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut assets = AssetManager::new(&subsystem.renderer, &ttf_context);

    let mut engine = Engine::new(&mut subsystem, dimensions, actions, initial_scene);
    engine.start(&mut assets);

    Ok(())
  }
}

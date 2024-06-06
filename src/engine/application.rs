use std::time::Duration;

use crate::engine::asset::AssetManager;
use crate::engine::event::EventStore;
use crate::engine::geometry::shape::Vec2;
use crate::engine::internal::{add_internal_entities, add_internal_systems};
use crate::engine::lifecycle::{Lifecycle, LifecycleArgs};
use crate::engine::rendering::camera::{Camera, CameraBounds};
use crate::engine::rendering::color::{RGBA, unit_to_alpha};
use crate::engine::rendering::renderer::Properties;
use crate::engine::scene::{Scene, SceneManager, SceneTransition, TransitionState};
use crate::engine::state::State;
use crate::engine::subsystem::Subsystem;
use crate::engine::system::{Schedule, SysArgs, SystemManager, SystemTag};
use crate::engine::time::Frame;
use crate::engine::utility::alias::{DeltaMS, Size2};
use crate::engine::world::World;

/**
 * Application structure and lifecycle
 */

pub const SIMULATION_FPS: DeltaMS = 1.0 / 60.0;
pub const MAX_FRAME_TIME: DeltaMS = 1.0 / 30.0;

pub const TRANSITION_TIME_MS: u64 = 1_500;

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
      last_frame: Frame::build(SIMULATION_FPS).expect("Failed to build frame"),
    }
  }

  /// Load assets, setup state, and start the main loop
  pub fn start(&mut self, assets: &mut AssetManager) -> Result<(), String> {
    let mut systems = SystemManager::default();

    add_internal_systems(&mut systems);
    add_internal_entities(&mut self.world);

    let mut transition = SceneTransition::new(Duration::from_millis(TRANSITION_TIME_MS / 2));

    (self.lifecycle.setup)(LifecycleArgs::new(&mut self.world, &mut systems, &mut self.camera, &mut self.state, assets));

    loop {
      // start frame
      let (delta, ..) = self.last_frame.next();
      if delta > MAX_FRAME_TIME {
        eprintln!("Frame took too long: {:.2}ms", delta * 1000.0);
        continue;
      }

      // process events
      self.subsystem.events.update(&mut self.events);
      if self.subsystem.events.is_quit() { break; }

      // check for pause
      if self.events.must_pause() {
        systems.suspend(Schedule::FrameUpdate, SystemTag::Suspendable).expect("Failed to suspend fame systems");
        systems.suspend(Schedule::PostUpdate, SystemTag::Suspendable).expect("Failed to suspend post systems");
        self.subsystem.events.pause(&mut self.events);
      } else if self.events.must_resume() {
        systems.resume(Schedule::FrameUpdate, SystemTag::Suspendable).expect("Failed to resume frame systems");
        systems.resume(Schedule::PostUpdate, SystemTag::Suspendable).expect("Failed to resume post systems");
        self.subsystem.events.resume(&mut self.events);
      }

      // process physics
      self.last_frame.process_accumulated(|fixed_time| {
        let mut args = SysArgs::new(fixed_time, &mut self.world, &mut self.subsystem.renderer, &mut self.events, &mut self.camera, &mut self.scenes, &mut self.state, assets);
        systems.update(Schedule::FixedUpdate, &mut args)
      })?;
      let mut args = SysArgs::new(delta, &mut self.world, &mut self.subsystem.renderer, &mut self.events, &mut self.camera, &mut self.scenes, &mut self.state, assets);
      systems.update(Schedule::FrameUpdate, &mut args)?;
      systems.update(Schedule::PostUpdate, &mut args)?;

      if self.scenes.is_queue() && !transition.active() {
        self.events.queue_pause();
        transition.start();
      }

      if transition.active() {
        transition.integrate(|state, interpolation| {
          match (state, interpolation) {
            (TransitionState::Idle, ..) => {
              unreachable!("Transition state is never idle during interpolation");
            }
            (TransitionState::Out | TransitionState::Intermediate, t) => {
              let alpha = unit_to_alpha(t);
              let scrim = CameraBounds::new(Vec2::default(), self.camera.get_viewport().size);
              self.subsystem.renderer.fill_rect(scrim, RGBA::new(0, 0, 0, alpha));

              if state == TransitionState::Intermediate {
                systems.remove(SystemTag::Scene);
                systems.remove(SystemTag::Suspendable);
                systems.remove_suspended();
                self.events.clear_held_keys();

                self.scenes.next(&mut LifecycleArgs::new(&mut self.world, &mut systems, &mut self.camera, &mut self.state, assets));
                self.events.queue_pause();
              }
            }
            (TransitionState::In, t) => {
              let alpha = unit_to_alpha(1.0 - t);
              let scrim = CameraBounds::new(Vec2::default(), self.camera.get_viewport().size);
              self.subsystem.renderer.fill_rect(scrim, RGBA::new(0, 0, 0, alpha));
            }
            (TransitionState::Complete, ..) => {
              self.events.queue_resume();
            }
          }
        });
      }

      self.subsystem.renderer.present();
    }

    (self.lifecycle.destroy)();

    Ok(())
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
    engine.start(&mut assets)?;

    Ok(())
  }
}

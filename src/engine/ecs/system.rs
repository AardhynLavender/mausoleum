
/**
  * Manage the creation, scheduling, execution, and suspension of systems
  */

use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use crate::engine::asset::asset::AssetManager;
use crate::engine::core::event::EventStore;
use crate::engine::core::scene::SceneManager;
use crate::engine::ecs::world::World;
use crate::engine::render::camera::Camera;
use crate::engine::render::renderer::Renderer;
use crate::engine::utility::alias::DeltaMS;
use crate::engine::utility::state::State;

/// When to execute a system
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Schedule {
  /// Update the game state once per frame
  FrameUpdate,
  /// Update the game state at a fixed rate
  FixedUpdate,
  /// Update state at the end of the frame before rendering
  PostUpdate,
}

impl Display for Schedule {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Schedule::FrameUpdate => write!(f, "FrameUpdate"),
      Schedule::FixedUpdate => write!(f, "FixedUpdate"),
      Schedule::PostUpdate => write!(f, "PostUpdate"),
    }
  }
}

/// Arguments passed to systems
pub struct SysArgs<'app, 'fonts> {
  pub delta: DeltaMS,
  pub world: &'app mut World,
  pub render: &'app mut Renderer,
  pub event: &'app mut EventStore,
  pub camera: &'app mut Camera,
  pub scene: &'app mut SceneManager,
  pub asset: &'app mut AssetManager<'fonts>,
  pub state: &'app mut State,
}

/// A mutable context provided to systems
impl<'app, 'fonts> SysArgs<'app, 'fonts> {
  /// Instantiate a new system args wrapper
  pub fn new(
    delta: DeltaMS,
    world: &'app mut World,
    render: &'app mut Renderer,
    event: &'app mut EventStore,
    camera: &'app mut Camera,
    scene: &'app mut SceneManager,
    state: &'app mut State,
    asset: &'app mut AssetManager<'fonts>,
  ) -> Self {
    Self {
      delta,
      world,
      render,
      camera,
      event,
      scene,
      state,
      asset,
    }
  }
}

/// A function that mutably queries and/or updates the world
pub type System = fn(&mut SysArgs) -> Result<(), String>;

/// Implement a system for a type
pub trait Systemize {
  fn system(args: &mut SysArgs) -> Result<(), String>;
}

/// Manages the scheduling of mutable systems
#[derive(Default)]
struct SystemGroup {
  systems: Vec<System>,
}

impl SystemGroup {
  /// Instantiate a new system manager
  pub fn new() -> Self {
    Self::default()
  }

  /// Add a system to a schedule
  pub fn add(&mut self, system: System) { self.systems.push(system); }

  pub fn add_many(&mut self, systems: impl Iterator<Item=System>) {
    systems.for_each(|system| {
      self.systems.push(system)
    });
  }

  /// call the systems of a schedule
  pub fn update(&mut self, args: &mut SysArgs) -> Result<(), String> {
    // accumulate the errors
    self.systems
      .iter()
      .try_for_each(|system| {
        system(args)
      })
  }
}

/// A system manager that manages systems by schedule
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemTag {
  Suspendable,
  Scene,
  Internal,
}

impl Display for SystemTag {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SystemTag::Suspendable => write!(f, "Suspendable"),
      SystemTag::Scene => write!(f, "Global"),
      SystemTag::Internal => write!(f, "Internal"),
    }
  }
}

/// Manage schedules of system groups
#[derive(Default)]
pub struct SystemManager {
  suspended_systems: HashMap<(Schedule, SystemTag), SystemGroup>,
  schedules: HashMap<Schedule, HashMap<SystemTag, SystemGroup>>,
}

impl SystemManager {
  /// Suspend a group of systems from a schedule
  pub fn suspend(&mut self, schedule: Schedule, tag: SystemTag) -> Result<(), String> {
    if let Some(group) = self.schedules.get_mut(&schedule) {
      group
        .remove(&tag)
        .map(|system| {
          self.suspended_systems.insert((schedule, tag), system);
        });
    }
    Ok(())
  }

  /// Resume processing a system group in a schedule
  pub fn resume(&mut self, schedule: Schedule, tag: SystemTag) -> Result<(), String> {
    let key = (schedule, tag);
    if let Some(resumable) = self.suspended_systems.remove(&key) {
      self.schedules
        .get_mut(&schedule)
        .ok_or(String::from("Schedule not found"))?
        .insert(tag, resumable);
    }

    Ok(())
  }

  pub fn update(&mut self, schedule: Schedule, args: &mut SysArgs) -> Result<(), String> {
    self.schedules
      .get_mut(&schedule)
      .into_iter()
      .try_for_each(|tag| {
        tag
          .into_iter()
          .try_for_each(|(.., group)| {
            group.update(args)
          })
      })
  }

  /// Adds a system to a schedule identified by a tags
  ///
  /// Systems are processed in order within tags, but the order the tags are processed is not guaranteed; Don't rely on it
  pub fn add(&mut self, schedule: Schedule, tag: SystemTag, system: System) -> Result<(), String> {
    let schedule = self.schedules.entry(schedule).or_insert_with(HashMap::new);
    let group = schedule.entry(tag).or_insert_with(SystemGroup::new);
    group.add(system);
    Ok(())
  }

  /// Add many systems to a schedule identified by a tag
  pub fn add_many(&mut self, schedule: Schedule, tag: SystemTag, systems: impl Iterator<Item=System>) -> Result<(), String> {
    let schedule = self.schedules.entry(schedule).or_insert_with(HashMap::new);
    let group = schedule.entry(tag).or_insert_with(SystemGroup::new);
    group.add_many(systems);
    Ok(())
  }

  /// Remove systems from a schedule
  pub fn remove_all(&mut self) { self.schedules.clear(); }

  /// Remove systems of a tag from any
  pub fn remove(&mut self, tag: SystemTag) {
    self.schedules
      .iter_mut()
      .for_each(|(.., group)| {
        group.remove(&tag);
      });
  }

  /// Remove any suspended systems
  pub fn remove_suspended(&mut self) { self.suspended_systems.clear(); }
}
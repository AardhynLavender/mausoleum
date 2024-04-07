use crate::engine::internal::{add_internal_entities, add_internal_systems};
use crate::engine::lifecycle::LifecycleArgs;

/**
 * Scenes define
 */

/// A scene is a defined state of the game
pub trait Scene {
  fn setup(&self, args: &mut LifecycleArgs);
  fn add_systems(&self, args: &mut LifecycleArgs);
  fn destroy(&self, args: &mut LifecycleArgs);
}

/// Scene manager is responsible for managing the current scene
pub struct SceneManager {
  scene: Option<Box<dyn Scene>>,
  next: Option<Box<dyn Scene>>,
}

impl SceneManager {
  /// Instantiate a new scene manager
  pub fn new(initial_scene: impl Scene + 'static) -> Self {
    Self {
      scene: None,
      next: Some(Box::new(initial_scene)),
    }
  }

  /// Queue the next scene
  pub fn queue_next(&mut self, scene: impl Scene + 'static) {
    self.next = Some(Box::new(scene));
  }

  /// Check if there is a scene in the queue
  pub fn is_queue(&self) -> bool {
    self.next.is_some()
  }

  /// Destroy the current scene and set up the next
  pub fn next(&mut self, args: &mut LifecycleArgs) {
    args.world.free_all_now();
    args.system.clear();
    add_internal_systems(args.system);
    add_internal_entities(&mut args.world);

    // destroy the current scene
    if let Some(current) = self.scene.as_mut() {
      current.destroy(args);
    }
    // set up the next scene
    if let Some(next) = self.next.as_mut() {
      next.setup(args);
      next.add_systems(args);
      self.scene = self.next.take();
    }
  }
}

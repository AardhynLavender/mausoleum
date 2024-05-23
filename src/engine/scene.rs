use crate::engine::lifecycle::LifecycleArgs;
use crate::engine::system::SystemTag;

/**
 * Scenes define
 */

/// A scene is a defined state of the game
pub trait Scene {
  fn setup(&mut self, args: &mut LifecycleArgs);
  fn destroy(&mut self, args: &mut LifecycleArgs);
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
  pub fn queue_next(&mut self, scene: impl Scene + 'static) { self.next = Some(Box::new(scene)); }
  /// Check if there is a scene in the queue
  pub fn is_queue(&self) -> bool { self.next.is_some() }
  /// Load the next scene
  fn load(&mut self, args: &mut LifecycleArgs) {
    // clear all tags between scenes except internal
    args.system.remove(SystemTag::Scene);
    args.system.remove(SystemTag::Suspendable);

    if let Some(next) = self.next.as_mut() {
      next.setup(args);
      self.scene = self.next.take();
    }
  }

  /// Destroy the current scene
  fn destroy(&mut self, args: &mut LifecycleArgs) {
    if let Some(current) = self.scene.as_mut() {
      current.destroy(args);
    }
    args.world.free_all_now();
  }
  /// Destroy the current scene and set up the next
  pub fn next(&mut self, args: &mut LifecycleArgs) {
    self.destroy(args);
    self.load(args);
  }
}

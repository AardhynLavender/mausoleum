use std::time::Duration;

use crate::engine::lifecycle::LifecycleArgs;
use crate::engine::time::Timer;

/**
 * Scene management and transitions
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

/// The state of a transition
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransitionState {
  /// No transition is active
  Idle,
  /// The transition is fading out
  Out,
  /// The transition has completed fading out
  Intermediate,
  /// The transition is fading in
  In,
  /// The transition has completed fading in
  Complete,
}

/// Describes state and interpolation of scene transition
pub struct SceneTransition {
  pub state: TransitionState,
  pub timer: Timer,
}

impl SceneTransition {
  /// Instantiate a new scene transition
  pub fn new(duration: Duration) -> Self {
    Self {
      state: TransitionState::Idle,
      timer: Timer::new(duration, false),
    }
  }
  /// Check if the transition is active
  pub fn active(&self) -> bool {
    self.state != TransitionState::Idle && self.state != TransitionState::Complete
  }
  /// Start the transition
  pub fn start(&mut self) {
    self.state = TransitionState::Out;
    self.timer.start();
  }
  /// Start a transition from intermediate
  pub fn start_from_intermediate(&mut self) {
    self.state = TransitionState::Intermediate;
    self.timer.expire();
  }
  /// integrate the transition state and progression
  pub fn integrate(&mut self, interpolator: impl FnOnce(TransitionState, f32)) {
    if self.timer.done() {
      if self.state == TransitionState::Out {
        interpolator(TransitionState::Intermediate, 1.0);
        self.state = TransitionState::In;
        self.timer.start();
      } else if self.state == TransitionState::In {
        self.state = TransitionState::Complete;
        interpolator(self.state, 0.0);
      }
    } else {
      interpolator(self.state, self.timer.interpolate());
    }
  }
}
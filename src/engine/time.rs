use std::time::{Duration, Instant};

use crate::engine::application::SIMULATION_FPS;
use crate::engine::utility::alias::DeltaMS;

/**
 * Time utilities
 */

// Constants //

pub const SECOND_MICRO: f32 = 1_000_000.0;

// Types //

/// Represents a frame of time
pub struct Frame {
  start: Instant,
  end: Instant,
}

impl Default for Frame {
  /// Instantiate a new frame
  fn default() -> Self {
    Self {
      start: Instant::now(),
      end: Instant::now(),
    }
  }
}

impl Frame {
  /// Update the frame and compute the alpha and delta time
  pub fn next(&mut self) -> (DeltaMS, DeltaMS) {
    self.end = Instant::now();
    let delta = self.end.duration_since(self.start).as_micros() as DeltaMS / SECOND_MICRO;
    let alpha = delta % SIMULATION_FPS;
    self.start = self.end;
    (alpha, delta)
  }
}

/// What to do when a done timer is consumed
pub enum ConsumeAction {
  Restart,
  Disable,
}

/// A stateful timer
#[derive(Debug, Copy, Clone)]
pub struct Timer {
  enabled: bool,
  start: Instant,
  duration: Duration,
}

impl Default for Timer {
  /// Instantiate a new timer of 0 duration
  fn default() -> Self {
    Self {
      enabled: false,
      start: Instant::now(),
      duration: Duration::from_secs(0),
    }
  }
}

impl Timer {
  /// Instantiate a new timer of `duration`
  pub fn new(duration: Duration, enabled: bool) -> Self {
    Self {
      enabled,
      start: Instant::now(),
      duration,
    }
  }
  /// Start the timer
  pub fn start(&mut self) {
    self.reset();
    self.enabled = true;
  }
  /// Reset the timer to the start
  pub fn reset(&mut self) { self.start = Instant::now(); }
  /// drop the timer
  pub fn expire(&mut self) { self.start = Instant::now() - self.duration; }
  /// Check if the timer has expired regardless of enabled state
  pub fn done(&self) -> bool { self.start.elapsed() >= self.duration }
  /// Check if the timer has expired, then disable or restart it
  pub fn consume(&mut self, action: ConsumeAction) -> bool {
    if !self.enabled {
      return false;
    }

    let done = self.done();
    if done {
      match action {
        ConsumeAction::Restart => self.reset(), // timer will be done again after duration
        ConsumeAction::Disable => self.enabled = false, // timer will not be done again
      }
    }
    done
  }

  /// Check if the timer has expired and call a function if it has, then disable or restart it
  pub fn consume_map(&mut self, action: ConsumeAction, mut callback: impl FnMut()) -> bool {
    let done = self.consume(action);
    if done { (callback)(); }
    done
  }
}
use std::time::{Duration, Instant};

/**
 * Time utilities
 */

/// A stateful timer
#[derive(Debug)]
pub struct Timer {
  enabled: bool,
  start: Instant,
  duration: Duration,
}

/// What to do when a done timer is consumed
pub enum ConsumeAction {
  Restart,
  Disable,
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

  /// Check if the timer has expired regardless of enabled state
  pub fn done(&self) -> bool {
    self.start.elapsed() >= self.duration
  }

  /// Start the timer
  pub fn start(&mut self) {
    self.start = Instant::now();
    self.enabled = true;
  }
  /// set the start time to now
  pub fn restart(&mut self) {
    self.start = Instant::now();
  }

  /// Check if the timer has expired then perform `action`
  pub fn consume(&mut self, action: ConsumeAction) -> bool {
    if !self.enabled {
      return false;
    }

    let done = self.done();
    if done {
      match action {
        ConsumeAction::Restart => self.restart(), // timer will be done again after duration
        ConsumeAction::Disable => self.enabled = false, // timer will not be done again
      }
    }
    done
  }

  /// Check if the timer has expired and disable it if it has.
  /// If the timer has expired, invoke the callback.
  pub fn consume_map(&mut self, action: ConsumeAction, callback: &mut dyn FnMut()) -> bool {
    let done = self.consume(action);
    if done {
      (callback)();
    }
    done
  }
}

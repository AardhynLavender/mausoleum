/**
  * Represents a single iteration of processing over a period of time
  */

use std::time::Instant;
use crate::engine::application::SIMULATION_FPS;
use crate::engine::utility::alias::DeltaMS;
use crate::engine::utility::time::SECOND_MICRO;

/// Represents a frame of time
pub struct Frame {
  start: Instant,
  end: Instant,
  fixed_delta: f32,
  accumulator: f32,
}

impl Frame {
  /// Instantiate a new frame
  pub fn build(fixed_delta: f32) -> Result<Self, String> {
    if fixed_delta <= 0.0 { return Err(String::from("Fixed delta must be greater than 0.0")); }
    Ok(Self {
      start: Instant::now(),
      end: Instant::now(),
      fixed_delta,
      accumulator: 0.0,
    })
  }
  /// Update the frame and compute the alpha and delta time
  pub fn next(&mut self) -> (DeltaMS, DeltaMS) {

    self.end = Instant::now();
    let delta = self.end.duration_since(self.start).as_micros() as DeltaMS / SECOND_MICRO;
    let alpha = delta % SIMULATION_FPS;
    self.start = self.end;
    self.accumulator += delta;
    (delta, alpha)
  }
  /// Process the accumulated time in fixed delta increments
  pub fn process_accumulated(&mut self, mut processor: impl FnMut(f32) -> Result<(), String>) -> Result<(), String> {
    while self.accumulator >= self.fixed_delta {
      self.accumulator -= self.fixed_delta;
      processor(self.fixed_delta)?;
    }
    Ok(())
  }
}


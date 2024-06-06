use std::time::Duration;

use crate::engine::geometry::collision::CollisionBox;
use crate::engine::rendering::camera::CameraBounds;
use crate::engine::time::Timer;
use crate::game::scene::level::registry::{ROOM_TRANSITION_TIME_MS, RoomKey};

/// Data for a room transition
#[derive(Clone)]
pub struct RoomTransitionData {
  pub old_viewport: CameraBounds,
  pub new_viewport: CameraBounds,
  pub old_player: CollisionBox,
  pub new_player: CollisionBox,
}

/// Define the state of a room transition
pub enum RoomTransitionState {
  Idle,
  Queued(RoomKey),
  Progress(f32, RoomTransitionData),
  Complete(RoomKey),
}

/// Manage room transitions
pub struct RoomTransition {
  next: Option<RoomKey>,
  data: Option<RoomTransitionData>,
  timer: Timer,
}

impl Default for RoomTransition {
  fn default() -> Self {
    Self {
      next: None,
      data: None,
      timer: Timer::new(Duration::from_millis(ROOM_TRANSITION_TIME_MS), false),
    }
  }
}

impl RoomTransition {
  /// Queue a transition to a room
  pub fn queue(&mut self, name: impl Into<RoomKey>) -> Result<(), String> {
    if self.next.is_some() {
      return Err(String::from("Transition already queued or in progress"));
    }
    self.next = Some(name.into());
    Ok(())
  }
  /// Start a transition
  pub fn start(&mut self, transition_data: RoomTransitionData) -> Result<(), String> {
    if self.next.is_none() {
      return Err(String::from("Transition not queued or already in progress"));
    }
    self.data = Some(transition_data);
    self.timer.start();
    Ok(())
  }
  /// Interpolate the progress of the transition
  pub fn integrate(&mut self) -> RoomTransitionState {
    return if self.next.is_none() {
      RoomTransitionState::Idle
    } else if !self.timer.is_enabled() {
      RoomTransitionState::Queued(self.next.clone().unwrap())
    } else if self.timer.done() {
      let state = RoomTransitionState::Complete(self.next.clone().unwrap());
      self.next = None;
      self.timer.disable();
      state
    } else {
      let t = self.timer.interpolate();
      RoomTransitionState::Progress(t, self.data.clone().unwrap())
    };
  }
}

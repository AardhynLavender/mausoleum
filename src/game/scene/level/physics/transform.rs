/**
  * Define a transformation between two positions over a period of time
  */

use std::time::Duration;
use crate::engine::component::position::Position;
use crate::engine::ecs::system::{SysArgs, Systemize};
use crate::engine::geometry::shape::Vec2;
use crate::engine::math::interpolation::lerp;
use crate::engine::utility::time::Timer;

/// Describe the state of a transformation.
#[derive(PartialEq)]
pub enum TransformationState { Progressing, Complete }

/// Convert a boolean value into a transformation state.
impl From<bool> for TransformationState {
  fn from(running: bool) -> Self {
    if running {
      TransformationState::Progressing
    } else {
      TransformationState::Complete
    }
  }
}

/// Define a transformation between two positions over a period of time.
pub struct Transform {
  timer: Timer,
  start: Vec2<f32>,
  end: Vec2<f32>,
}

impl Transform {
  /// Create a new transformation between two positions over a period of time.
  pub fn new(start: Vec2<f32>, end: Vec2<f32>, duration: Duration) -> Self {
    Self {
      timer: Timer::new(duration, true),
      start,
      end,
    }
  }

  /// interpolate the position between the start and end positions.
  pub fn interpolate(&self) -> (Vec2<f32>, TransformationState) {
    let position = lerp(self.start, self.end, self.timer.interpolate());
    let state = TransformationState::from(self.timer.done());
    (position, state)
  }
}

impl Systemize for Transform {
  /// Update the positions of an entities with transformations.
  fn system(SysArgs { world, .. }: &mut SysArgs) -> Result<(), String> {
    let transformed_entities = world.query::<(&mut Transform, &mut Position)>().into_iter().filter_map(|(entity, (transform, position))| {
      let (new_position, state) = transform.interpolate();
      position.0 = new_position;
      if state == TransformationState::Complete {
        Some(entity)
      } else {
        None
      }
    }).collect::<Vec<_>>();

    for entity in transformed_entities {
      world
        .remove_components::<(&Transform, )>(entity)
        .map_err(|_| String::from("Failed to remove Transform component"))?;
    }

    Ok(())
  }
}

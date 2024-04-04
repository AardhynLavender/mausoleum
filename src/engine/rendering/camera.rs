use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::system::SysArgs;
use crate::engine::utility::alias::Size;
use crate::game::physics::position::Position;

/**
 * Camera structures and utilities
 */

pub type CameraBounds = Rec2<i32, Size>;

/// Camera structure
pub struct Camera {
  tethered: bool,
  bounds: CameraBounds,
}

impl Camera {
  /// Instantiate a new camera of `size`
  pub fn new(bounds: CameraBounds) -> Self {
    Self {
      tethered: false,
      bounds,
    }
  }
  /// Tether the camera to an entity
  ///
  /// The engine will update the camera position to center on the entity
  pub fn tether(&mut self) {
    self.tethered = true;
  }
  /// Remove the camera tether and assign a new `position`
  pub fn release(&mut self, new_position: Vec2<i32>) {
    if self.tethered {
      self.bounds.origin = new_position;
      self.tethered = false;
    }
  }
  /// Get the camera bounds
  pub fn get_bounds(&self) -> &CameraBounds { &self.bounds }
  /// Get the camera position
  pub fn get_position(&self) -> Vec2<i32> { self.bounds.origin }
  /// Center the camera on `position`
  pub fn set_position(&mut self, position: Vec2<i32>) {
    self.bounds.origin = position - Vec2::from(self.bounds.size) / 4;
  }
  /// Translate a `position` to the camera's coordinate system
  pub fn translate(&self, position: Vec2<f32>) -> Vec2<i32> {
    Vec2::<i32>::from(position) - self.bounds.origin
  }
}

/// Mark an entity as tethered to the camera with an offset
///
/// Cameras will center on tethered entities and apply the offset
///
/// Requires the entity to have a `Position` component
pub struct CameraTether {
  pub offset: Vec2<i32>,
}

impl CameraTether {
  /// Instantiate a new camera tether with an offset of `offset`
  pub fn new(offset: Vec2<i32>) -> Self {
    Self {
      offset
    }
  }
}

/// Query the world for camera tethers
pub type QueryCameraTether<'a> = (&'a CameraTether, &'a Position);

// /// Update the camera position based on it's tethers
pub fn sys_tether(SysArgs { camera, world, .. }: &mut SysArgs) {
  if !camera.tethered { return; }

  if let Some((_, (tether, position))) = world
    .query::<QueryCameraTether>()
    .into_iter()
    .next()
  {
    camera.set_position(Vec2::<i32>::from(position.0) + tether.offset);
  } else {
    eprintln!("Camera tethered but no tether found! releasing camera...");
    camera.release(Vec2::default());
  }
}

/// Entities with this component will be positioned relative to the camera
pub struct CameraSticky;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::rendering::renderer::layer;
use crate::engine::system::{SysArgs, Systemize};
use crate::engine::utility::alias::Size;
use crate::game::physics::position::Position;

/**
 * Camera structures and utilities
 */

pub type CameraBounds = Rec2<i32, Size>;

/// Camera structure
pub struct Camera {
  tethered: bool,
  viewport: CameraBounds,
  bounds: Option<CameraBounds>,
}

impl Camera {
  /// Instantiate a new camera of `size`
  pub fn new(viewport: CameraBounds) -> Self {
    Self {
      tethered: false,
      viewport,
      bounds: None,
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
      self.viewport.origin = new_position;
      self.tethered = false;
    }
  }

  /// Set the camera position
  pub fn set_position(&mut self, position: Vec2<i32>) { self.viewport.origin = position; }
  /// Get the camera position
  pub fn get_position(&self) -> Vec2<i32> { self.viewport.origin }

  /// Center the camera on `position`
  pub fn set_center(&mut self, position: Vec2<i32>) {
    let mut new_position = position - Vec2::from(self.viewport.size) / 2;
    if let Some(bounds) = &self.bounds {
      new_position.clamp(&bounds.origin, &Vec2::<i32>::from((bounds.origin + Vec2::<i32>::from(bounds.size)) - Vec2::<i32>::from(self.viewport.size)));
    }
    self.set_position(new_position);
  }

  /// Set the camera bounds
  pub fn set_bounds(&mut self, bounds: CameraBounds) { self.bounds = Some(bounds); }
  /// Get the camera viewport
  pub fn get_viewport(&self) -> &CameraBounds { &self.viewport }
  /// Get the camera bounds
  pub fn get_bounds(&self) -> &Option<CameraBounds> { &self.bounds }
  /// remove the bounds from the camera
  pub fn remove_bounds(&mut self) { self.bounds = None; }

  /// Translate a `position` to the camera viewport coordinate system
  pub fn translate(&self, position: Vec2<f32>) -> Vec2<i32> {
    Vec2::<i32>::from(position) - self.viewport.origin
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
  pub fn new(offset: Vec2<i32>) -> Self { Self { offset } }
}

/// Query the world for camera tethers
pub type QueryCameraTether<'a> = (&'a CameraTether, &'a Position);

// /// Update the camera position based on it's tethers
impl Systemize for CameraTether {
  fn system(SysArgs { camera, world, .. }: &mut SysArgs) -> Result<(), String> {
    if !camera.tethered { return Ok(()); }

    if let Some((_, (tether, position))) = world
      .query::<QueryCameraTether>()
      .into_iter()
      .next()
    {
      camera.set_center(Vec2::<i32>::from(position.0) + tether.offset);
    } else {
      eprintln!("Camera tethered but no tether found! releasing camera...");
      camera.release(Vec2::default());
    }

    Ok(())
  }
}

/// Entities with this component will be positioned relative to the camera
pub type Sticky1 = layer::Layer9;
pub type Sticky2 = layer::Layer8;

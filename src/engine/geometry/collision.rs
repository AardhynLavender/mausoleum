use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::utility::alias::Size;

/// The maximum value of a resolvable collision
///
/// Essentially marking a resolution as impossible
pub const RESOLVABLE_INFINITY: f32 = f32::INFINITY;

/// Describes the presentation sides of a collision that are resolvable
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct CollisionMask {
  pub top: bool,
  pub bottom: bool,
  pub left: bool,
  pub right: bool,
}

impl CollisionMask {
  /// Instantiate a new collision mask
  pub const fn new(top: bool, right: bool, bottom: bool, left: bool) -> Self {
    Self {
      top,
      right,
      bottom,
      left,
    }
  }
  /// Check if the collision mask is empty
  pub const fn is_empty(&self) -> bool {
    !self.top && !self.right && !self.bottom && !self.left
  }
}

/// Describes a simple collision in terms of relative side penetration
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Collision {
  mask: CollisionMask,
  pub top: f32,
  pub bottom: f32,
  pub left: f32,
  pub right: f32,
}

impl Collision {
  /// Build a collision from the penetration values of the sides.
  ///
  /// Returns an error if no sides are penetrated
  pub fn build(mask: CollisionMask, top: f32, right: f32, bottom: f32, left: f32) -> Result<Self, String> {
    if top == 0.0
      && right == 0.0
      && bottom == 0.0
      && left == 0.0
    {
      return Err(String::from("No collision"));
    }

    Ok(Self {
      mask,
      top,
      right,
      bottom,
      left,
    })
  }
  /// Get the shortest side of the collision
  pub fn get_resolution(&self) -> Vec2<f32> {
    // apply the collision mask to the penetration values
    let sides = [
      if self.mask.top { self.top } else { RESOLVABLE_INFINITY },
      if self.mask.right { self.right } else { RESOLVABLE_INFINITY },
      if self.mask.bottom { self.bottom } else { RESOLVABLE_INFINITY },
      if self.mask.left { self.left } else { RESOLVABLE_INFINITY },
    ];
    // get the index of the shortest absolute side
    let index = sides
      .iter()
      .enumerate()
      .fold(0, |cur_idx, (i, &pen)| {
        if pen.abs() < sides[cur_idx].abs() { i } else { cur_idx }
      });
    // return a resolution for the shortest side
    let resolutions = vec![
      Vec2::new(0.0, self.top),
      Vec2::new(self.right, 0.0),
      Vec2::new(0.0, self.bottom),
      Vec2::new(self.left, 0.0),
    ];
    let resolution = resolutions[index];
    resolution
  }
}

/// A rectangle used for collision detection and resolution
pub type CollisionBox = Rec2<f32, Size>;

/// Check if two rectangles are colliding based on a mask and return the collision data
pub fn rec2_collision(r1: &CollisionBox, r2: &CollisionBox, mask: CollisionMask) -> Option<Collision> {
  let ((x1, y1), (w1, h1)) = r1.destructure();
  let ((x2, y2), (w2, h2)) = r2.destructure();
  if x1 < x2 + w2 as f32
    && x1 + w1 as f32 > x2
    && y1 < y2 + h2 as f32
    && y1 + h1 as f32 > y2
  {
    Collision::build(
      mask,
      (y1 + h1 as f32) - y2,
      x1 - (x2 + w2 as f32),
      y1 - (y2 + h2 as f32),
      (x1 + w1 as f32) - x2,
    ).ok()
  } else {
    None
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn maskless_collision() {
    todo!("Implement maskless collision test")
  }

  #[test]
  fn full_mask_collision() {
    todo!("Implement mask collision test")
  }

  #[test]
  fn partial_mask_collision() {
    todo!("Implement partial mask collision test")
  }
}
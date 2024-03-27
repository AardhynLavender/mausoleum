use crate::engine::geometry::{Rec2, Vec2};
use crate::engine::utility::alias::{Size, Size2};

/**
 * position utilities
 */

/// Align something on the horizontal or vertical axis
/// Pass the perpendicular axis as the second argument
pub enum Align {
  Start(f32),
  Center(f32),
  End(f32),
  At(f32),
}

/// Relative position of an object within bounds
pub struct Alignment {
  pub x: Align,
  pub y: Align,
}

impl Alignment {
  /// Instantiate a new alignment
  pub const fn new(x: Align, y: Align) -> Self {
    Self { x, y }
  }
}

/// Alignment utilities for positioning within bounds
pub struct Aligner {
  bounds: Rec2<i32, Size>,
}

impl Aligner {
  /// Instantiate an aligner within `bounds`
  pub const fn new(bounds: Rec2<i32, Size>) -> Self {
    Self { bounds }
  }

  /// Center something of `width` on the horizontal axis
  fn center_horizontal(&self, width: Size) -> f32 {
    (self.bounds.size.x - width) as f32 / 2.0
  }
  /// Center something of `height` on the vertical axis
  fn center_vertical(&self, height: Size) -> f32 {
    (self.bounds.size.y - height) as f32 / 2.0
  }

  /// Center something of `size` within the bounds
  pub fn center(&self, size: Size2) -> Vec2<f32> {
    Vec2::new(
      self.center_horizontal(size.x),
      self.center_vertical(size.y),
    )
  }

  pub fn align(&self, Alignment { x, y }: Alignment, size: Size2) -> Vec2<f32> {
    Vec2::new(
      match x {
        Align::Start(offset) => 0.0 + offset,
        Align::Center(offset) => self.center_horizontal(size.x) + offset,
        Align::End(offset) => (self.bounds.size.x - size.x) as f32 - offset,
        Align::At(offset) => offset,
      },
      match y {
        Align::Start(offset) => 0.0 + offset,
        Align::Center(offset) => self.center_vertical(size.y) + offset,
        Align::End(offset) => (self.bounds.size.y - size.y) as f32 - offset,
        Align::At(unit) => unit,
      },
    ) + self.bounds.origin.into()
  }
}



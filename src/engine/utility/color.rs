
/**
 * Color representation and constants
 */

use sdl2::pixels::Color;

// Color //

/// Convert a unit value to an alpha value
pub fn unit_to_alpha(unit: f32) -> u8 { (unit * 255.0) as u8 }

/// RGBA color
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct RGBA {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub alpha: u8,
}

impl RGBA {
  /// Instantiate a new color from its component.rs
  pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
    Self {
      red,
      green,
      blue,
      alpha,
    }
  }
  /// Deconstruct a color into its component.rs
  pub fn destructure(self) -> (u8, u8, u8, u8) {
    (self.red, self.green, self.blue, self.alpha)
  }
}

impl From<&RGBA> for Color {
  /// Convert reference to `RGBA` color to an `sdl2::pixels::Color`
  fn from(value: &RGBA) -> Self {
    Self {
      r: value.red,
      g: value.green,
      b: value.blue,
      a: value.alpha,
    }
  }
}

impl From<RGBA> for Color {
  /// Convert an `RGBA` color to an `sdl2::pixels::Color`
  fn from(value: RGBA) -> Self {
    Self::from(&value)
  }
}

// Utility //

pub const OPAQUE: u8 = 255;

// common //

pub mod color {
  use super::{OPAQUE, RGBA};

  pub const TEXT: RGBA = RGBA::new(255, 255, 255, OPAQUE);
  pub const TEXT2: RGBA = RGBA::new(128, 128, 128, OPAQUE);
  pub const TEXT3: RGBA = RGBA::new(64, 64, 64, OPAQUE);
  pub const PRIMARY: RGBA = RGBA::new(62, 207, 142, OPAQUE);
  pub const ON_PRIMARY: RGBA = RGBA::new(255, 255, 255, OPAQUE);
}

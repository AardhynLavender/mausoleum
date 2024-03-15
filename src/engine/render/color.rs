use sdl2::pixels::Color;

/**
 * Color representation and constants
 */

// Color //

/// RGBA color
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct RGBA {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub alpha: u8,
}

impl RGBA {
  /// Instantiate a new color from its components
  pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
    Self { red, green, blue, alpha }
  }
  /// Deconstruct a color into its components
  pub fn destructure(self) -> (u8, u8, u8, u8) {
    (self.red, self.green, self.blue, self.alpha)
  }
}

impl From<&RGBA> for Color {
  /// Convert reference to `RGBA` color to an `sdl2::pixels::Color`
  fn from(value: &RGBA) -> Self {
    Self { r: value.red, g: value.green, b: value.blue, a: value.alpha }
  }
}

impl From<RGBA> for Color {
  /// Convert an `RGBA` color to an `sdl2::pixels::Color`
  fn from(value: RGBA) -> Self {
    Self::from(&value)
  }
}

// Utility //

pub const U8MAX: u8 = 255;
pub const OPAQUE: u8 = U8MAX;

// common //

pub mod color {
  use super::{OPAQUE, RGBA};

  // From the Catppuccin theme:
  pub const TEXT: RGBA = RGBA::new(205, 214, 244, OPAQUE);
  pub const SURFACE_0: RGBA = RGBA::new(49, 50, 68, OPAQUE);
  pub const MANTLE: RGBA = RGBA::new(24, 24, 37, OPAQUE);
}
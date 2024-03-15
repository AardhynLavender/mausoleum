use std::marker::Copy;

use num::{Num, Unsigned};
use sdl2::rect::{Point, Rect};

/**
 * Geometric primitives
 */

// Traits //

/// Primitive type for geometric shapes
pub trait UnitPrimitive: Num + Copy {}

impl<T: Num + Copy> UnitPrimitive for T {}

/// Primitive type for geometric sizes
pub trait SizePrimitive: UnitPrimitive + Unsigned + Into<u32> {}

impl<T: UnitPrimitive + Unsigned + Into<u32>> SizePrimitive for T {}

/// Can the shape primitive be converted to an i32
///
/// SDL2 uses integers internally for rendering, supplied type `T` must conform to this constraint if used in rendering
pub trait IntConvertable: UnitPrimitive + Into<i32> {}

impl<T: UnitPrimitive + Into<i32>> IntConvertable for T {}

// Vector 2D //

/// A vector representation in 2D space of some numeric type `T`
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2<T>
  where
    T: UnitPrimitive,
{
  pub x: T,
  pub y: T,
}

impl<T: UnitPrimitive> Vec2<T> {
  /// Instantiate a new vector of T
  pub const fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
  /// Deconstruct the vector into its components
  pub fn destructure(&self) -> (T, T) {
    (self.x, self.y)
  }
}

impl<T: IntConvertable> From<Vec2<T>> for Point {
  /// Convert a Vec2 of T to a Point of i32
  fn from(value: Vec2<T>) -> Self {
    Self::from(&value)
  }
}

impl<T: IntConvertable> From<&Vec2<T>> for Point {
  /// Convert a reference to Vec2 of T to a Point of i32
  fn from(value: &Vec2<T>) -> Self {
    let (x, y) = value.destructure();
    Point::new(x.into(), y.into())
  }
}

// Rectangle 2D //

/// A Rectangle representation in 2D space of some numeric type `T`
#[derive(Clone, Copy, Debug, Default)]
pub struct Rec2<T: UnitPrimitive, U: SizePrimitive> {
  pub origin: Vec2<T>,
  pub size: Vec2<U>,
}

impl<T: UnitPrimitive, U: SizePrimitive> Rec2<T, U> {
  /// Instantiate a new rectangle of T and U
  pub const fn new(origin: Vec2<T>, size: Vec2<U>) -> Self {
    Self { origin, size }
  }
  /// Deconstruct the rectangle into its components
  pub fn destructure(&self) -> ((T, T), (U, U)) {
    (self.origin.destructure(), self.size.destructure())
  }
}

impl<T: IntConvertable, U: SizePrimitive> From<Rec2<T, U>> for Rect {
  /// Convert a Rec2 of T and U to a Rect of i32
  fn from(value: Rec2<T, U>) -> Self {
    let ((x, y), (w, h)) = value.destructure();
    Rect::new(x.into(), y.into(), w.into(), h.into())
  }
}

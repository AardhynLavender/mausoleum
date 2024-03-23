use std::marker::Copy;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use num::{Num, Unsigned};
use sdl2::rect::{Point, Rect};

/**
 * Geometric primitives
 */

// Traits //

/// Primitive type for geometric shapes
pub trait UnitPrimitive: Num + AddAssign + SubAssign + DivAssign + MulAssign + Copy {}

impl<T: Num + Copy + SubAssign + AddAssign + DivAssign + MulAssign> UnitPrimitive for T {}

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
  /// Deconstruct the vector into its component.rs
  pub fn destructure(&self) -> (T, T) {
    (self.x, self.y)
  }
}

impl Vec2<i32> {
  /// Instantiate a new constexpr vector of i32.
  /// Easier to create this "const_default" function than create const versions for every use of `T`
  /// explicitly except floating point types
  pub const fn const_default() -> Self {
    Self { x: 0, y: 0 }
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

impl From<Vec2<f32>> for Vec2<i32> {
  /// Convert Vec2 float to Vec2 i32
  fn from(value: Vec2<f32>) -> Self {
    let (x, y) = value.destructure();
    Vec2::new(x as i32, y as i32)
  }
}

impl From<Vec2<i32>> for Vec2<f32> {
  /// Convert Vec2 i32 to Vec2 float
  fn from(value: Vec2<i32>) -> Self {
    let (x, y) = value.destructure();
    Vec2::new(x as f32, y as f32)
  }
}

// Vector 2D Math //

impl<T> Add for Vec2<T> where T: UnitPrimitive {
  type Output = Vec2<T>;
  fn add(self, other: Self) -> Self {
    Vec2::new(self.x + other.x, self.y + other.y)
  }
}

impl<T> Add for &Vec2<T> where T: UnitPrimitive {
  type Output = Vec2<T>;
  /// Add two vectors of `T` together
  fn add(self, other: Self) -> Vec2<T> {
    *self + *other
  }
}

impl<T> Sub for Vec2<T> where T: UnitPrimitive {
  type Output = Vec2<T>;
  /// Subtract two vectors of `T` from each other
  fn sub(self, other: Self) -> Self {
    Vec2::new(self.x - other.x, self.y - other.y)
  }
}

impl<T> Sub for &Vec2<T> where T: UnitPrimitive {
  type Output = Vec2<T>;
  /// Subtract two vectors of `T` from each other
  fn sub(self, other: Self) -> Vec2<T> {
    *self - *other
  }
}

impl<T> Mul for Vec2<T> where T: UnitPrimitive {
  type Output = Vec2<T>;
  /// Multiply two vectors of `T` together
  fn mul(self, other: Self) -> Self {
    Vec2::new(self.x * other.x, self.y * other.y)
  }
}

impl<T> Mul<T> for Vec2<T> where T: UnitPrimitive {
  type Output = Vec2<T>;
  /// Multiply a vector of `T` by a scalar
  fn mul(self, scalar: T) -> Self {
    Vec2::new(self.x * scalar, self.y * scalar)
  }
}

impl<T> Mul for &Vec2<T> where T: UnitPrimitive {
  type Output = Vec2<T>;
  /// Multiply two vectors of `T` together
  fn mul(self, other: Self) -> Vec2<T> {
    *self * *other
  }
}

impl<T> Div for Vec2<T> where T: UnitPrimitive {
  type Output = Vec2<T>;
  /// Divide two vectors of `T` from each other
  fn div(self, other: Self) -> Self {
    Vec2::new(self.x / other.x, self.y / other.y)
  }
}

impl<T> Div for &Vec2<T> where T: UnitPrimitive {
  type Output = Vec2<T>;
  /// Divide two vectors of `T` from each other
  fn div(self, other: Self) -> Vec2<T> {
    *self / *other
  }
}

impl<T> Div<T> for Vec2<T> where T: UnitPrimitive {
  type Output = Vec2<T>;
  //// Divide a vector of `T` by a scalar
  fn div(self, scalar: T) -> Self {
    Vec2::new(self.x / scalar, self.y / scalar)
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
  /// Deconstruct the rectangle into its component.rs
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

// tests
#[cfg(test)]
mod tests {
  use super::*;

// Vec2 //

  #[test]
  fn vec2_new() {
    let vec = Vec2::new(1, 2);
    assert_eq!(vec.x, 1);
    assert_eq!(vec.y, 2);
  }

  #[test]
  fn vec2_destructure() {
    let vec = Vec2::new(1, 2);
    let (x, y) = vec.destructure();
    assert_eq!(x, 1);
    assert_eq!(y, 2);
  }

  // Vec2 Conversion //

  #[test]
  fn vec2_to_point() {
    let vec = Vec2::new(1, 2);
    let point: Point = vec.into();
    assert_eq!(point.x, 1);
    assert_eq!(point.y, 2);
  }

  #[test]
  fn vec2_to_point_ref() {
    let vec = Vec2::new(1, 2);
    let point: Point = (&vec).into();
    assert_eq!(point.x, 1);
    assert_eq!(point.y, 2);
  }

  // Vec2 Math //

  #[test]
  fn vec2_add() {
    let vec1 = Vec2::new(1, 2);
    let vec2 = Vec2::new(3, 4);
    let vec3 = vec1 + vec2;
    assert_eq!(vec3.x, 4);
    assert_eq!(vec3.y, 6);
  }

  #[test]
  fn vec2_add_ref() {
    let vec1 = Vec2::new(1, 2);
    let vec2 = Vec2::new(3, 4);
    let vec3 = &vec1 + &vec2;
    assert_eq!(vec3.x, 4);
    assert_eq!(vec3.y, 6);
  }

  #[test]
  fn vec2_sub() {
    let vec1 = Vec2::new(1, 2);
    let vec2 = Vec2::new(3, 4);
    let vec3 = vec1 - vec2;
    assert_eq!(vec3.x, -2);
    assert_eq!(vec3.y, -2);
  }

  #[test]
  fn vec2_sub_ref() {
    let vec1 = Vec2::new(1, 2);
    let vec2 = Vec2::new(3, 4);
    let vec3 = &vec1 - &vec2;
    assert_eq!(vec3.x, -2);
    assert_eq!(vec3.y, -2);
  }

  #[test]
  fn vec2_mul() {
    let vec1 = Vec2::new(1, 2);
    let vec2 = Vec2::new(3, 4);
    let vec3 = vec1 * vec2;
    assert_eq!(vec3.x, 3);
    assert_eq!(vec3.y, 8);
  }

  #[test]
  fn vec2_mult_scalar() {
    let vec1 = Vec2::new(1, 2);
    let scalar = 3;
    let vec2 = vec1 * scalar;
    assert_eq!(vec2.x, 3);
    assert_eq!(vec2.y, 6);
  }

  #[test]
  fn vec2_mul_ref() {
    let vec1 = Vec2::new(1, 2);
    let vec2 = Vec2::new(3, 4);
    let vec3 = &vec1 * &vec2;
    assert_eq!(vec3.x, 3);
    assert_eq!(vec3.y, 8);
  }

  #[test]
  fn vec2_div() {
    let vec1 = Vec2::new(1, 2);
    let vec2 = Vec2::new(3, 4);
    let vec3 = vec1 / vec2;
    assert_eq!(vec3.x, 0);
    assert_eq!(vec3.y, 0);
  }

  #[test]
  fn vec2_div_scalar() {
    let vec1 = Vec2::new(1, 2);
    let scalar = 3;
    let vec2 = vec1 / scalar;
    assert_eq!(vec2.x, 0);
    assert_eq!(vec2.y, 0);
  }

  #[test]
  fn vec2_div_ref() {
    let vec1 = Vec2::new(1, 2);
    let vec2 = Vec2::new(3, 4);
    let vec3 = &vec1 / &vec2;
    assert_eq!(vec3.x, 0);
    assert_eq!(vec3.y, 0);
  }

  // Rec2 //

  #[test]
  fn rec2_new() {
    let rec = Rec2::new(Vec2::new(1, 2), Vec2::new(3u32, 4u32));
    assert_eq!(rec.origin.x, 1);
    assert_eq!(rec.origin.y, 2);
    assert_eq!(rec.size.x, 3);
    assert_eq!(rec.size.y, 4);
  }

  #[test]
  fn rec2_destructure() {
    let rec = Rec2::new(Vec2::new(1, 2), Vec2::new(3u32, 4u32));
    let ((x, y), (w, h)) = rec.destructure();
    assert_eq!(x, 1);
    assert_eq!(y, 2);
    assert_eq!(w, 3);
    assert_eq!(h, 4);
  }

  #[test]
  fn rec2_to_rect() {
    let rec = Rec2::new(Vec2::new(1, 2), Vec2::new(3u32, 4u32));
    let rect: Rect = rec.into();
    assert_eq!(rect.x(), 1);
    assert_eq!(rect.y(), 2);
    assert_eq!(rect.width(), 3);
    assert_eq!(rect.height(), 4);
  }
}
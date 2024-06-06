#![allow(dead_code)]

/**
 * Interpolation utilities
 * https://en.wikipedia.org/wiki/B%C3%A9zier_curve
 */

use std::ops::{Add, Mul, Sub};

use crate::engine::geometry::shape::{UnitPrimitive, Vec2};

/// Interpolates between two generic values
pub fn lerp<T>(a: T, b: T, t: f32) -> T
  where T: Add<Output=T> + Sub<Output=T> + Mul<f32, Output=T> + Copy
{
  a + (b - a) * t
}

pub struct QuadraticBezierCurve<T> where T: UnitPrimitive {
  p0: Vec2<T>,
  p1: Vec2<T>,
  p2: Vec2<T>,
}

impl<T> QuadraticBezierCurve<T> where T: UnitPrimitive, Vec2<T>: Mul<f32, Output=Vec2<T>> {
  /// Instantiate a new quadratic bezier curve
  pub fn new(p0: Vec2<T>, p1: Vec2<T>, p2: Vec2<T>) -> Self { Self { p0, p1, p2 } }

  /// Find a vector on this curve at unit `t`
  pub fn lerp(&self, t: f32) -> Vec2<T> {
    let q1 = lerp::<Vec2<T>>(self.p0, self.p1, t);
    let q2 = lerp::<Vec2<T>>(self.p1, self.p2, t);
    lerp(q1, q2, t)
  }
}

pub struct CubicBezierCurve<T> where T: UnitPrimitive {
  p0: Vec2<T>,
  p1: Vec2<T>,
  p2: Vec2<T>,
  p3: Vec2<T>,
}

impl<T> CubicBezierCurve<T> where T: UnitPrimitive, Vec2<T>: Mul<f32, Output=Vec2<T>> {
  /// Instantiate a new cubic bezier curve
  pub fn new(p0: Vec2<T>, p1: Vec2<T>, p2: Vec2<T>, p3: Vec2<T>) -> Self { Self { p0, p1, p2, p3 } }

  /// Find a vector on this curve at unit `t`
  pub fn lerp(&self, t: f32) -> Vec2<T> {
    let q1 = lerp::<Vec2<T>>(self.p0, self.p1, t);
    let q2 = lerp::<Vec2<T>>(self.p1, self.p2, t);
    let q3 = lerp::<Vec2<T>>(self.p2, self.p3, t);
    let r1 = lerp::<Vec2<T>>(q1, q2, t);
    let r2 = lerp::<Vec2<T>>(q2, q3, t);
    lerp(r1, r2, t)
  }
}

#[cfg(test)]
mod tests {
  use crate::engine::geometry::shape::Vec2;

  use super::*;

  #[test]
  fn test_f32_lerp() {
    assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
    assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
    assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
  }

  #[test]
  fn test_vec2_lerp() {
    let a = Vec2::new(0.0, 0.0);
    let b = Vec2::new(10.0, 10.0);
    assert_eq!(lerp(a, b, 0.5), Vec2::new(5.0, 5.0));
    assert_eq!(lerp(a, b, 0.0), Vec2::new(0.0, 0.0));
    assert_eq!(lerp(a, b, 1.0), Vec2::new(10.0, 10.0));
  }

  #[test]
  fn test_quadratic_bezier_curve() {
    let curve = QuadraticBezierCurve::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0), Vec2::new(20.0, 0.0));
    assert_eq!(curve.lerp(0.5), Vec2::new(10.0, 5.0));
    assert_eq!(curve.lerp(0.0), Vec2::new(0.0, 0.0));
    assert_eq!(curve.lerp(1.0), Vec2::new(20.0, 0.0));
  }

  #[test]
  fn test_cubic_bezier_curve() {
    let curve = CubicBezierCurve::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0), Vec2::new(20.0, 10.0), Vec2::new(30.0, 0.0));
    assert_eq!(curve.lerp(0.5), Vec2::new(15.0, 7.5));
    assert_eq!(curve.lerp(0.0), Vec2::new(0.0, 0.0));
    assert_eq!(curve.lerp(1.0), Vec2::new(30.0, 0.0));
  }
}
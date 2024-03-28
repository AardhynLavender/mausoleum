use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::utility::alias::Size;

#[allow(unused)]
pub struct Collision {
  penetration: Vec2<f32>,
}

impl Collision {
  pub fn new(penetration: Vec2<f32>) -> Self {
    Self {
      penetration
    }
  }
}

#[allow(unused)]
pub fn vec2_collision(v1: &Vec2<f32>, v2: &Vec2<f32>) -> Option<Collision> {
  if v1.x == v2.x && v1.y == v2.y {
    Some(Collision {
      penetration: Vec2::default(),
    })
  } else {
    None
  }
}

#[allow(unused)]
pub fn vec2_rec2_collision(v: &Vec2<f32>, r: &Rec2<f32, Size>) -> Option<Collision> {
  let ((x, y), (w, h)) = r.destructure();
  if v.x >= x
    && v.x <= x + h as f32
    && v.y >= y
    && v.y <= y + w as f32
  {
    Some(Collision {
      penetration: *v - r.origin,
    })
  } else {
    None
  }
}

#[allow(unused)]
pub fn rec2_collision(r1: &Rec2<f32, Size>, r2: &Rec2<f32, Size>) -> Option<Collision> {
  let ((x1, y1), (w1, h1)) = r1.destructure();
  let ((x2, y2), (w2, h2)) = r2.destructure();
  if x1 < x2 + w2 as f32
    && x1 + w1 as f32 > x2
    && y1 < y2 + h2 as f32
    && y1 + h1 as f32 > y2
  {
    Some(Collision {
      penetration: r2.origin - r1.origin,
    })
  } else {
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn vec2() {
    let vec1 = Vec2::new(1.0, 1.0);
    let vec2 = Vec2::new(1.0, 1.0);
    let vec3 = Vec2::new(2.0, 2.0);
    let vec1_vec2_penetration = vec1 - vec2;

    let collision = vec2_collision(&vec1, &vec2);
    let not_collision = vec2_collision(&vec1, &vec3);

    assert!(collision.is_some(), "{:?} collides with {:?}", vec1, vec2);
    assert!(collision.is_some_and(|c| c.penetration == vec1_vec2_penetration), "{:?} penetrates {:?} by {:?}", vec1, vec2, vec1_vec2_penetration);
    assert!(not_collision.is_none(), "{:?} does not collide with {:?}", vec1, vec3);
  }

  #[test]
  fn vec2_rec2() {
    let vec = Vec2::new(2.1, 3.0);
    let rec = Rec2::new(Vec2::new(1.4, 1.8), Vec2::new(4, 4));
    let rec2 = Rec2::new(Vec2::default(), Vec2::new(2, 10));
    let vec_rec_penetration = vec - rec.origin;

    let collision = vec2_rec2_collision(&vec, &rec);
    let not_collision = vec2_rec2_collision(&vec, &rec2);

    assert!(collision.is_some(), "{:?} collides with {:?}", vec, rec);
    assert!(collision.is_some_and(|c| c.penetration == vec_rec_penetration), "{:?} penetrates {:?} by {:?}", vec, rec, vec_rec_penetration);
    assert!(not_collision.is_none(), "{:?} does not collide with {:?}", vec, rec2);
  }

  #[test]
  fn rec2_rec2() {
    let rec1 = Rec2::new(Vec2::default(), Vec2::new(4, 4));
    let rec2 = Rec2::new(Vec2::new(2.0, 2.0), Vec2::new(4, 4));
    let rec3 = Rec2::new(Vec2::new(5.0, 2.0), Vec2::new(4, 4));
    let rec1_rec2_penetration = rec2.origin - rec1.origin;

    let collision = rec2_collision(&rec1, &rec2);
    let not_collision = rec2_collision(&rec1, &rec3);

    assert!(collision.is_some(), "{:?} collides with {:?}", rec1, rec2);
    assert!(collision.is_some_and(|c| c.penetration == rec1_rec2_penetration), "{:?} penetrates {:?} by {:?}", rec1, rec2, rec1_rec2_penetration);
    assert!(not_collision.is_none(), "{:?} does not collide with {:?}", rec1, rec3);
  }
}
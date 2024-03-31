use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::utility::alias::Size;

/// The maximum value of a resolvable collision
///
/// Essentially marking a resolution as impossible
pub const RESOLVABLE_INFINITY: f32 = f32::INFINITY;

/// Describes the presentation sides of a collision that are resolvable
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
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
  /// Build a collision where `collision` collided with `collider`, with the penetration values inverted
  pub fn invert(&self) -> Self {
    todo!()
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

pub type CollisionResult = Option<Collision>;

pub type CollisionBox = Rec2<f32, Size>;

// #[allow(unused)]
// pub fn vec2_collision(v1: &Vec2<f32>, v2: &Vec2<f32>) -> Option<Collision> {
//   if v1.x == v2.x && v1.y == v2.y {
//     Some(Collision {
//       penetration: Vec2::default(),
//     })
//   } else {
//     None
//   }
// }
//
// #[allow(unused)]
// pub fn vec2_rec2_collision(v: &Vec2<f32>, r: &Rec2<f32, Size>) -> Option<Collision> {
//   let ((x, y), (w, h)) = r.destructure();
//   if v.x >= x
//     && v.x <= x + h as f32
//     && v.y >= y
//     && v.y <= y + w as f32
//   {
//     Some(Collision {
//       penetration: *v - r.origin,
//     })
//   } else {
//     None
//   }
// }
//

/// Check if two rectangles are colliding, and return the collision data if they are
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
//
// #[cfg(test)]
// mod tests {
//   use super::*;
//
//   #[test]
//   fn vec2() {
//     let vec1 = Vec2::new(1.0, 1.0);
//     let vec2 = Vec2::new(1.0, 1.0);
//     let vec3 = Vec2::new(2.0, 2.0);
//     let vec1_vec2_penetration = vec1 - vec2;
//
//     let collision = vec2_collision(&vec1, &vec2);
//     let not_collision = vec2_collision(&vec1, &vec3);
//
//     assert!(collision.is_some(), "{:?} collides with {:?}", vec1, vec2);
//     assert!(collision.is_some_and(|c| c.penetration == vec1_vec2_penetration), "{:?} penetrates {:?} by {:?}", vec1, vec2, vec1_vec2_penetration);
//     assert!(not_collision.is_none(), "{:?} does not collide with {:?}", vec1, vec3);
//   }
//
//   #[test]
//   fn vec2_rec2() {
//     let vec = Vec2::new(2.1, 3.0);
//     let rec = Rec2::new(Vec2::new(1.4, 1.8), Vec2::new(4, 4));
//     let rec2 = Rec2::new(Vec2::default(), Vec2::new(2, 10));
//     let vec_rec_penetration = vec - rec.origin;
//
//     let collision = vec2_rec2_collision(&vec, &rec);
//     let not_collision = vec2_rec2_collision(&vec, &rec2);
//
//     assert!(collision.is_some(), "{:?} collides with {:?}", vec, rec);
//     assert!(collision.is_some_and(|c| c.penetration == vec_rec_penetration), "{:?} penetrates {:?} by {:?}", vec, rec, vec_rec_penetration);
//     assert!(not_collision.is_none(), "{:?} does not collide with {:?}", vec, rec2);
//   }
//
//   #[test]
//   fn rec2_rec2() {
//     let rec1 = Rec2::new(Vec2::default(), Vec2::new(4, 4));
//     let rec2 = Rec2::new(Vec2::new(2.0, 2.0), Vec2::new(4, 4));
//     let rec3 = Rec2::new(Vec2::new(5.0, 2.0), Vec2::new(4, 4));
//     let rec1_rec2_penetration = rec2.origin - rec1.origin;
//
//     let collision = rec2_collision(&rec1, &rec2);
//     let not_collision = rec2_collision(&rec1, &rec3);
//
//     assert!(collision.is_some(), "{:?} collides with {:?}", rec1, rec2);
//     assert!(collision.is_some_and(|c| c.penetration == rec1_rec2_penetration), "{:?} penetrates {:?} by {:?}", rec1, rec2, rec1_rec2_penetration);
//     assert!(not_collision.is_none(), "{:?} does not collide with {:?}", rec1, rec3);
//   }
// }
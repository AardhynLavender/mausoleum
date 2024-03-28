use crate::engine::geometry::shape::Vec2;

/**
 * Useful type aliases
 */

/// A key used to identify a resource or entity
pub type Key = u32;
/// A 1D Size
pub type Size = u32;
/// A 2D dimension
pub type Size2 = Vec2<Size>;
/// difference in ms between frames
pub type DeltaMS = f32;
/// A 2D coordinate
pub type Coordinate = Vec2<i32>;

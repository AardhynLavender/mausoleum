#![allow(dead_code)]
/**
  * Backdrop creation and rendering
  */

use std::path::Path;

use hecs::{Component, Entity};

use crate::engine::asset::asset::AssetManager;
use crate::engine::asset::texture::SrcRect;
use crate::engine::component::position::Position;
use crate::engine::component::sprite::Sprite;
use crate::engine::ecs::system::{SysArgs, Systemize};
use crate::engine::ecs::world::World;
use crate::engine::geometry::shape::Vec2;
use crate::engine::render::camera::CameraBounds;
use crate::engine::utility::alias::Size2;

/// A builder for a backdrop and it's segments
pub struct BackdropBuilder {
  #[allow(dead_code)]
  anchor: Vec2<i32>,
  segmentation: Size2,
  segment_size: Size2,
  segment: Sprite,
}

/// A collection of backdrop segments
pub struct Backdrop {
  segments: Vec<Entity>,
  segment_size: Size2,
}

/// A segment of a backdrop
pub struct BackdropSegment {
  segment_coordinate: Vec2<i32>,
}

impl BackdropBuilder {
  /// Instantiate a new backdrop
  pub fn build(viewport: &CameraBounds, anchor: Vec2<i32>, filepath: impl AsRef<Path>, asset: &mut AssetManager) -> Result<Self, String> {
    let segment_texture = asset.texture.load(filepath).expect("Failed to load backdrop texture");
    let segment_size = asset.texture.use_store().get(segment_texture).expect("Failed to get texture").dimensions;
    let segment_texture_src = SrcRect::new(Vec2::default(), segment_size);
    let segment = Sprite::new(segment_texture, segment_texture_src);

    let segmentation = (Vec2::<f32>::from(viewport.size + segment_size) / Vec2::<f32>::from(segment_size)).ceil();

    Ok(Self {
      anchor,
      segmentation: Size2::from(segmentation),
      segment_size,
      segment,
    })
  }
  /// Create the segments and add them to the world
  pub fn add_to_world<Layer>(&mut self, world: &mut World) where Layer: Default + Component {
    let mut segments = Vec::with_capacity(self.segmentation.square() as usize);
    for x in 0..self.segmentation.x {
      for y in 0..self.segmentation.y {
        let segment_coordinate = Vec2::from(Vec2::new(x, y));
        let sprite = self.segment.clone();
        let segment = world.add((BackdropSegment{segment_coordinate}, sprite, Position::default(), Layer::default(), ));
        segments.push(segment);
      }
    }
    world.add((Backdrop{segments, segment_size: self.segment_size},));
  }
}

// Compute the start position of the segment by flooring the camera viewport origin to the nearest
// segment 'slot'
fn compute_segment_start_position(camera_viewport: CameraBounds, segment_size: Size2) -> Vec2<i32> {
  let segment_size = Vec2::from(segment_size);
  let start = (Vec2::<f32>::from(camera_viewport.origin) / Vec2::<f32>::from(segment_size)).floor() * segment_size;
  Vec2::<i32>::from(start)
}

/// Get the segment data from the backdrop entity
fn get_segment_data(world: &World, entity: Entity) -> Result<(impl Iterator<Item=Entity>, Size2), String> {
  let backdrop = world.get_component::<Backdrop>(entity).map_err(|e| e.to_string())?;
  let segment_size = backdrop.segment_size;
  let segments = backdrop.segments.iter().cloned().collect::<Vec<_>>().into_iter();
  Ok((segments, segment_size))
}


impl Systemize for BackdropBuilder {
  fn system(SysArgs { camera, world, .. }: &mut SysArgs) -> Result<(), String> {
    let camera_viewport = camera.get_viewport();

    let backdrops = world
      .query::<()>()
      .with::<&Backdrop>()
      .into_iter()
      .map(|(entity, _)| entity)
      .collect::<Vec<_>>();
    if backdrops.is_empty() { return Ok(()); }

    for backdrop_entity in backdrops {
      let (segment_entities, segment_size) = get_segment_data(world, backdrop_entity)?;
      let segments_start = compute_segment_start_position(*camera_viewport, segment_size);

      for segment_entity in segment_entities {
        let (position, segment) = world.query_entity::<(&mut Position, &BackdropSegment)>(segment_entity).map_err(|e| e.to_string())?;

        let segment_offset = segment.segment_coordinate * Vec2::from(segment_size);
        let segment_position = segments_start + segment_offset;

        position.0 = Vec2::<f32>::from(segment_position);
      }
    }

    Ok(())
  }
}
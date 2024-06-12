use std::time::Duration;

use crate::engine::asset::asset::AssetManager;
use crate::engine::asset::texture::{SrcRect, TextureKey};
use crate::engine::component::sprite::Sprite;
use crate::engine::ecs::system::{SysArgs, Systemize};
use crate::engine::utility::alias::{Size, Size2};
use crate::engine::utility::invariant::invariant;
use crate::engine::utility::time::Timer;
use crate::game::scene::level::physics::frozen::Frozen;

/// A source rect of an image that is displayed for a certain duration
#[derive(Clone, Debug)]
pub struct AnimationFrame {
  src: SrcRect,
  duration: Duration,
}

impl AnimationFrame {
  /// Instantiate a new animation frame
  pub fn new(src: SrcRect, duration: Duration) -> Self { Self { src, duration } }
}

// A series of frames that are iterated over
#[derive(Clone, Debug)]
pub struct Animation {
  frames: Vec<AnimationFrame>,
  current_frame: usize,
  timer: Timer,
  infinite: bool,
}

/// The state of an animation
pub enum AnimationState { Disabled, Running(SrcRect), Complete }

impl Animation {
  /// Instantiate a new animation of frames
  pub fn build(frames: Vec<AnimationFrame>, infinite: bool) -> Result<Self, String> {
    invariant(frames.len() > 1, "Animation must have at least 2 frames")?;
    Ok(Self {
      frames,
      current_frame: 0,
      timer: Timer::default(),
      infinite,
    })
  }
  /// Get the current animation frame
  fn get_current_frame(&self) -> &AnimationFrame { &self.frames[self.current_frame] }
  /// Restart the animation
  pub fn restart(&mut self) {
    self.current_frame = 0;
    self.timer = Timer::new(self.get_current_frame().duration, true);
  }
  /// Start the animation
  pub fn start(mut self) -> Self {
    self.restart();
    self
  }
  /// Update the animation
  pub fn update(&mut self) -> AnimationState {
    if !self.timer.is_enabled() { return AnimationState::Disabled; }
    if self.timer.done() {
      self.current_frame += 1;
      if self.current_frame >= self.frames.len() {
        if self.infinite {
          self.current_frame = 0;
        } else {
          return AnimationState::Complete;
        }
      }
      let next_duration = self.get_current_frame().duration;
      self.timer = Timer::new(next_duration, true);
    }
    AnimationState::Running(self.get_current_frame().src)
  }
}

impl Systemize for Animation {
  /// Update sprites based on their animations
  fn system(SysArgs { world, .. }: &mut SysArgs) -> Result<(), String> {
    let completed_animations = world
      .query::<(&mut Animation, &mut Sprite)>()
      .without::<&Frozen>().into_iter()
      .filter_map(|(entity, (animation, sprite))| {
        match animation.update() {
          AnimationState::Disabled => None,
          AnimationState::Running(src) => {
            sprite.src = src;
            None
          }
          AnimationState::Complete => Some(entity),
        }
      })
      .collect::<Vec<_>>();

    for entity in completed_animations {
      world.free_now(entity)?;
    }

    Ok(())
  }
}

/// Create an animation from a texture
pub fn make_texture_animation(texture_key: TextureKey, asset: &mut AssetManager, dimensions: Size2, frame_duration: Duration) -> Result<Animation, String> {
  let texture = asset.texture.use_store().get(texture_key)?;

  invariant(texture.dimensions.x % dimensions.x == 0, "Animation frame width must divide evenly into texture width")?;
  invariant(texture.dimensions.y == dimensions.y, "Animation frame height must match texture height")?;

  let frames = (0..texture.dimensions.x / dimensions.x)
    .map(|i| {
      let origin = Size2::new(i as Size * dimensions.x, 0);
      AnimationFrame::new(SrcRect::new(origin, dimensions), frame_duration)
    })
    .collect();
  let animation = Animation::build(frames, true).map_err(|e| format!("Failed to build animation: {}", e))?;
  Ok(animation)
}
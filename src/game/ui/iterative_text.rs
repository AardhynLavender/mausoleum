/**
 * Animates the iteration of characters in text components
 */

use std::time::Duration;

use hecs::Entity;

use crate::engine::component::text::Text;
use crate::engine::system::{SysArgs, Systemize};
use crate::engine::time::{ConsumeAction, Timer};
use crate::engine::world::World;

/// Manage the iteration of characters in text components
pub struct IterativeText {
  text: String,
  current_character: usize,
  delay: Timer,
  timer: Timer,
}

impl IterativeText {
  /// Instantiate a new iterative text component
  fn new(duration: Duration, text: String) -> Self {
    Self {
      text,
      current_character: 0,
      timer: Timer::new(duration, true),
      delay: Timer::default(),
    }
  }
}

impl Systemize for IterativeText {
  fn system(SysArgs { world, .. }: &mut SysArgs) -> Result<(), String> {
    let finished_animations = world
      .query::<(&mut IterativeText, &mut Text)>()
      .into_iter()
      .filter_map(|(entity, (animated_text, text))| {
        // check start condition
        if !animated_text.delay.done() {
          if !text.get_text().is_empty() {
            text.set_content("");
          }
          return None;
        }

        let index = &mut animated_text.current_character;

        if *index == 0 || animated_text.timer.consume(ConsumeAction::Restart) {
          let content = animated_text
            .text
            .chars()
            .take(*index)
            .collect::<String>();

          text.set_content(content);
          *index += 1;

          // check for completion
          return if *index == animated_text.text.len() + 1 {
            Some(entity)
          } else {
            None
          };
        }

        None
      })
      .collect::<Vec<_>>();

    finished_animations
      .iter()
      .map(|entity| {
        world.remove_components::<(IterativeText, )>(*entity)
      })
      .collect::<Result<Vec<_>, _>>()?;

    Ok(())
  }
}

/// Helper structure for creating text animations
/// # Example
/// ```rust
/// // --snip--
/// IterativeTextBuilder::build(&mut world, text_entity_from_world)
///   .unwrap()
///   .with_duration(Duration::from_millis(500))
///   .with_delay(Duration::from_millis(1_000))
///   .start()
///  .unwrap();
/// ```
pub struct IterativeTextBuilder<'a> {
  world: &'a mut World,
  text: Entity,
  iterator: IterativeText,
}

impl<'a> IterativeTextBuilder<'a> {
  /// Get the text component from the world
  fn extract_text(world: &mut World, entity: Entity) -> Result<String, String> {
    world
      .get_component_mut::<Text>(entity)
      .map_err(|_| "Failed to get text component".to_string())
      .map(|mut text| {
        let content = text.get_text().clone();
        text.set_content("");
        content
      })
  }

  pub fn build(world: &'a mut World, text_entity: Entity) -> Result<Self, String> {
    let content = Self::extract_text(world, text_entity)?;
    let iterator = IterativeText::new(Duration::default(), content);
    Ok(Self { world, text: text_entity, iterator })
  }
  /// Set the duration between character iterations
  pub fn with_duration(mut self, duration: Duration) -> Self {
    self.iterator.timer = Timer::new(duration, true);
    self
  }
  /// Delay the start of the iteration
  pub fn with_delay(mut self, delay: Duration) -> Self {
    self.iterator.delay = Timer::new(delay, true);
    self
  }
  /// Start the animation
  pub fn start(self) -> Result<(), String> { self.world.add_components(self.text, (self.iterator, )) }
}

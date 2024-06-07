
use std::collections::HashSet;

use hecs::Entity;
use crate::engine::component::position::Position;
use crate::engine::ecs::system::{SysArgs, Systemize};

use crate::engine::geometry::shape::Vec2;
use crate::game::scene::level::physics::collision::{Collider, make_collision_box};
use crate::game::scene::level::room::collision::{CollisionBox, CollisionMask, rec2_collision};
use crate::game::scene::level::story::data::{StoryItem, StoryKey};
use crate::game::scene::level::story::modal::make_story_modal;

/// Mark an entity that advances the story
pub struct StoryAdvancer;

/// A collection of story keys that have been advanced by a `StoryAdvancer`
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StoryAdvancements(HashSet<StoryKey>);

impl StoryAdvancements {
  pub fn new(advancements: HashSet<StoryKey>) -> Self { Self(advancements) }
  /// Check if the story has been advanced by a key
  pub fn has_advanced(&self, key: &StoryKey) -> bool { self.0.contains(key) }
  /// Advance the story with a new key
  pub fn advance(&mut self, key: &StoryKey) -> bool { self.0.insert(key.clone()) }
}

/// Reduce a `StoryAdvancements` into a `HashSet<StoryKey>`
impl Into<HashSet<StoryKey>> for StoryAdvancements {
  fn into(self) -> HashSet<StoryKey> { self.0 }
}

/// Defines an interaction between a story advancer and a story area
pub struct StoryAdvancement {
  advancer: Entity,
  area: Entity,
  entry: StoryItem,
}

impl StoryAdvancement {
  /// Instantiate a new story advancement
  pub fn new(advancer: Entity, area: Entity, entry: StoryItem) -> Self {
    Self { advancer, area, entry }
  }
}

/// Mark an area of the work for story advancement
pub struct StoryArea(StoryItem);

impl StoryArea {
  /// Instantiate a new story area
  pub fn new(entry: StoryItem) -> Self { Self(entry) }
}

/// Bundle of story area components
pub type StoryAreaBundle = (StoryArea, Position, Collider);

impl Systemize for StoryArea {
  /// Check for player interaction with a story area
  fn system(SysArgs { world, event, asset, .. }: &mut SysArgs) -> Result<(), String> {
    if let Some((advancer, (advancer_position, advancer_collider))) = world
      .query_one_with::<(&Position, &Collider), &StoryAdvancer>()
      .map(|(entity, (position, collider))| (entity, (*position, *collider)))
    {
      let advancer_box = make_collision_box(&advancer_position, &advancer_collider);
      let advancements = world
        .query::<(&StoryArea, &Position, &Collider)>()
        .into_iter()
        .filter_map(|(entity, (area, position, collider))| {
          let area_box = make_collision_box(&position, &collider);
          if rec2_collision(&area_box, &advancer_box, CollisionMask::default()).is_some() {
            return Some(StoryAdvancement::new(advancer, entity, area.0.clone()));
          }
          None
        })
        .collect::<Vec<_>>();

      for StoryAdvancement { advancer, area, entry } in advancements {
        {
          let mut past_advancements = world
            .get_component_mut::<StoryAdvancements>(advancer)
            .map_err(|_| "StoryAdvancer has no StoryAdvancements")?;
          if !past_advancements.advance(&entry.key) { continue; }
        }
        world.free_now(area)?;
        make_story_modal(world, event, asset, &entry);
      }
    }
    Ok(())
  }
}

/// Compose story area components from a save room and collision box
pub fn make_story_area(entry: StoryItem, area: CollisionBox) -> StoryAreaBundle {
  (
    StoryArea(entry),
    Position::from(area.origin),
    Collider(CollisionBox::new(Vec2::default(), area.size))
  )
}



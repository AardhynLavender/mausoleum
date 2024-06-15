/**
 * Manage story progression
 */

use std::collections::HashSet;

use serde::Deserialize;

use crate::engine::utility::io::{deserialize_json, read_file};

pub const STORY_DATA_PATH: &str = "data/story.json";

pub fn deserialize_story_data() -> Result<Story, String> {
  let serialized = read_file(STORY_DATA_PATH)?;
  deserialize_json(&serialized)
}

pub type StoryKey = String;

/// A single entry of story progression
#[derive(Deserialize, Debug, Clone)]
pub struct StoryItem {
  pub key: StoryKey,
  pub title: String,
  pub data: Vec<String>,
  #[serde(default)]
  pub endgame: bool,
}

/// Collection of story progression entries
#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct Story {
  pub entries: Vec<StoryItem>,
}

impl Story {
  /// Omit story data entries that have been advanced
  pub fn omit(self, advancements: &HashSet::<StoryKey>) -> Self {
    Self {
      entries: self
        .entries
        .iter()
        .filter(|entry| !advancements.contains(&entry.key))
        .cloned()
        .collect()
    }
  }
  /// Find a story data entry by key
  pub fn get_entry(&self, key: impl Into<StoryKey> + Clone) -> Option<StoryItem> {
    self
      .entries
      .iter()
      .find(|entry| entry.key == key.clone().into())
      .cloned()
  }
}
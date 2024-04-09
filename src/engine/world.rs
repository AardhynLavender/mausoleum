use hecs::{Bundle, Component, DynamicBundle, Entity, Query, QueryMut, QueryOneError, Ref, RefMut, SpawnBatchIter, World as HecsWorld};

/**
 * A World is a collection of entities
 */

/// A collection of entities and their components
/// > I'm interested in writing my own ECS, but for now I will use a wrapper around hecs as I like its API.
/// >
/// > An ECS is ridiculously complex for my tiny brain, and I don't want to spend time on it right now
pub struct World {
  world: HecsWorld,
}

impl World {
  pub fn new() -> Self {
    Self {
      world: HecsWorld::new(),
    }
  }
  /// Spawn an entity with the given component.rs
  pub fn add(&mut self, components: impl DynamicBundle) -> Entity {
    self.world.spawn(components)
  }
  /// Batch spawn entities with the given components
  pub fn add_batch<I>(&mut self, components: I) -> SpawnBatchIter<'_, I::IntoIter>
    where
      I: IntoIterator,
      I::Item: DynamicBundle,
      <I as IntoIterator>::Item: Bundle,
      <I as IntoIterator>::Item: 'static
  {
    self.world.spawn_batch(components)
  }

  /// Attempt to fetch a component from an entity
  ///
  /// Returns an error if the entity does not exist or the component is not found
  #[allow(dead_code)]
  pub fn get_component<'a, C>(&self, entity: Entity) -> Result<Ref<C>, String>
    where C: Component
  {
    self.world
      .entity(entity)
      .map_err(|e| e.to_string())?
      .get::<&C>()
      .ok_or(String::from("Entity does not have component"))
  }

  /// Attempt to fetch a component mutably from an entity
  ///
  /// Returns an error if the entity does not exist or the component is not found
  pub fn get_component_mut<'a, C>(&self, entity: Entity) -> Result<RefMut<C>, String>
    where C: Component
  {
    self.world
      .entity(entity)
      .map_err(|e| e.to_string())?
      .get::<&mut C>()
      .ok_or(String::from("Entity does not have component"))
  }

  /// Check if a component exists on an entity
  ///
  /// Returns an error if the entity does not exist
  pub fn has_component<C>(&self, entity: Entity) -> Result<bool, String>
    where C: Component
  {
    self.world
      .entity(entity)
      .map_err(|e| e.to_string())
      .map(|e| e.get::<&C>().is_some())
  }

  /// Add a set of components to an entity
  ///
  /// Returns an error if the entity does not exist
  pub fn add_components<C>(&mut self, entity: Entity, components: C) -> Result<(), String>
    where C: DynamicBundle + 'static
  {
    self.world
      .insert(entity, components)
      .map_err(|e| e.to_string())
  }
  /// Remove a set of components from an entity
  ///
  /// Returns an error if the entity does not exist
  pub fn remove_components<C>(&mut self, entity: Entity) -> Result<C, String>
    where C: Bundle + 'static
  {
    self.world
      .remove::<C>(entity)
      .map_err(|e| e.to_string())
  }

  /// free an entity immediately (not recommended)
  pub fn free_now(&mut self, entity: Entity) -> Result<(), String> {
    self.world
      .despawn(entity)
      .map_err(|e| e.to_string())
  }
  /// free all entities immediately (not recommended)
  pub fn free_all_now(&mut self) {
    self.world.clear();
  }

  /// Mutably query the world for entities of a certain component set
  pub fn query<Q: Query>(&mut self) -> QueryMut<'_, Q> {
    self.world.query_mut::<Q>()
  }
  /// Mutably query the world for a single entity of a certain component set
  pub fn query_entity<Q: Query>(&mut self, entity: Entity) -> Result<Q::Item<'_>, QueryOneError> {
    self.world.query_one_mut::<Q>(entity)
  }
}
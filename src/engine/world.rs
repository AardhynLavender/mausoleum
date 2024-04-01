use hecs::{Bundle, DynamicBundle, Entity, Query, QueryMut, QueryOneError, SpawnBatchIter, World as HecsWorld};

/**
 * A World is a collection of entities
 */

/// define a manager of entities
/// implements how to add and remove its entities from the world
pub trait EntityManager {
  type Manager;
  type ComponentQuery<'q>: Query where Self: 'q; // the queried entities must live at least as long as the manager

  /// Add the manager's components to the world
  fn add_to_world(&mut self, world: &mut World);
  /// Remove the manager's components from the world
  fn remove_from_world(&mut self, world: &mut World) -> Result<(), String>;
  /// query the world for the entities of the manager
  fn query_entities<'q>(&'q mut self, world: &'q mut World) -> QueryMut<Self::ComponentQuery<'q>>;
}

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
  pub fn add_many<I>(&mut self, components: I) -> SpawnBatchIter<'_, I::IntoIter>
    where
      I: IntoIterator,
      I::Item: DynamicBundle,
      <I as IntoIterator>::Item: Bundle,
      <I as IntoIterator>::Item: 'static
  {
    self.world.spawn_batch(components)
  }
  /// Add a manager and its entities to the world
  pub fn add_manager<M>(&mut self, mut manager: M)
    where M: Send + Sync + EntityManager + 'static
  {
    manager.add_to_world(self);
    self.add((manager, ));
  }

  /// Add a set of components to an entity
  ///
  /// Returns an error if the entity does not exist
  pub fn add_components<C>(&mut self, entity: Entity, components: C) -> Result<(), String>
    where C: Bundle + 'static
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

/// Push default T state into the world
pub fn push_state<T>(world: &mut World)
  where T: Default + Send + Sync + 'static
{
  world.add((T::default(), ));
}

/// Push T state into the world
pub fn push_state_with<T>(world: &mut World, state: T)
  where T: Send + Sync + 'static
{
  world.add((state, ));
}

pub fn use_state<T>(world: &mut World) -> &mut T
  where T: Send + Sync + 'static
{
  world.query::<&mut T>()
    .into_iter()
    .next()
    .expect("No state found")
    .1
}
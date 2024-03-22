use hecs::{DynamicBundle, Entity as HEntity, Query, QueryOneError, World as HWorld};

/**
 * A World is a collection of entities
 */

type InternalWorld = HWorld;
type Entity = HEntity;

/// A collection of entities and their component.rs
/// > I'm interested in writing my own ECS, but for now I will use a wrapper around hecs as I like its API.
/// >
/// > An ECS is ridiculously complex for my tiny brain, and I don't want to spend time on it right now
pub struct World {
  world: InternalWorld,
}

impl World {
  pub fn new() -> Self {
    Self {
      world: InternalWorld::new(),
    }
  }

  /// Spawn an entity with the given component.rs
  pub fn add(&mut self, components: impl DynamicBundle) -> Entity {
    self.world.spawn(components)
  }
  /// free an entity immediately (not recommended)
  pub fn free_now(&mut self, entity: Entity) -> Result<(), String> {
    self.world.despawn(entity).map_err(|e| e.to_string())
  }
  /// free all entities immediately (not recommended)
  pub fn free_all_now(&mut self) {
    self.world.clear();
  }

  /// Mutably query the world for entities of a certain component set
  pub fn query<Q: Query>(&mut self) -> hecs::QueryMut<'_, Q> {
    self.world.query_mut::<Q>()
  }
  /// Mutably query the world for a single entity of a certain component set
  pub fn query_entity<Q: Query>(&mut self, entity: Entity) -> Result<Q::Item<'_>, QueryOneError> {
    self.world.query_one_mut::<Q>(entity)
  }
}

/// Push default T state into the world
pub fn push_state<T>(world: &mut World) where T: Default + Send + Sync + 'static {
  world.add((T::default(), ));
}

/// Push T state into the world
pub fn push_state_with<T>(world: &mut World, state: T) where T: Send + Sync + 'static {
  world.add((state, ));
}

pub fn use_state<T>(world: &mut World) -> &mut T where T: Send + Sync + 'static {
  world.query::<&mut T>()
    .into_iter()
    .next()
    .expect("No state found")
    .1
}
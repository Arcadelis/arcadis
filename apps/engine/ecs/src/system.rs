use soroban_sdk::{Symbol, Vec, symbol_short};
use crate::entity::{EntityId, Entity};
use crate::world::World;
use crate::component::Component;
use crate::event::{EventTrait, DamageEvent};

/// A system in the ECS world
pub trait System {
    /// The input type for the system
    type In;
    /// The output type for the system
    type Out;

    /// Run the system
    fn run(&mut self, world: &mut World, input: Self::In) -> Self::Out;
}

/// A system parameter that can be used in systems
pub trait SystemParam {
    /// The type of the parameter
    type Param;
    /// The type of the fetched data
    type Fetch;

    /// Fetch the parameter from the world
    fn fetch(world: &World) -> Self::Fetch;

    /// Fetch the parameter mutably from the world
    fn fetch_mut(world: &mut World) -> Self::Fetch;
}

/// A query for entities with specific components
pub struct Query {
    component_types: Vec<Symbol>,
}

impl Query {
    /// Create a new query
    pub fn new(component_types: Vec<Symbol>) -> Self {
        Self { component_types }
    }

    /// Add a component type to the query
    pub fn with_component(mut self, component_type: Symbol) -> Self {
        self.component_types.push_back(component_type);
        self
    }

    /// Execute the query on a world
    pub fn execute(&self, world: &World) -> Vec<EntityId> {
        // Convert Vec<Symbol> to &[Symbol] by creating a slice
        // This is a limitation of the Soroban SDK - we can't easily convert Vec to slice
        // For now, we'll use a different approach
        let env = soroban_sdk::Env::default();
        let mut results = Vec::new(&env);
        for entity in world.iter_entities() {
            let mut has_all_components = true;
            for i in 0..self.component_types.len() {
                let ctype = self.component_types.get(i).unwrap();
                if !entity.has_component(&ctype) {
                    has_all_components = false;
                    break;
                }
            }
            if has_all_components {
                results.push_back(entity.id());
            }
        }
        results
    }
}

/// Query state for tracking query results
pub struct QueryState {
    query: Query,
    last_results: Vec<EntityId>,
}

impl QueryState {
    /// Create a new query state
    pub fn new(query: Query) -> Self {
        let env = soroban_sdk::Env::default();
        Self {
            query,
            last_results: Vec::new(&env),
        }
    }

    /// Execute the query and update state
    pub fn execute(&mut self, world: &World) -> &Vec<EntityId> {
        self.last_results = self.query.execute(world);
        &self.last_results
    }

    /// Get the last query results
    pub fn results(&self) -> &Vec<EntityId> {
        &self.last_results
    }

    /// Check if the query has any results
    pub fn is_empty(&self) -> bool {
        self.last_results.is_empty()
    }

    /// Get the number of results
    pub fn len(&self) -> usize {
        self.last_results.len().try_into().unwrap()
    }
}

/// Conversion trait to turn something into a system
pub trait IntoSystem<In, Out> {
    /// The type of system that this converts into
    type System: System<In = In, Out = Out>;

    /// Convert this into a system
    fn into_system(self) -> Self::System;
}

/// A simple function-based system
pub struct FunctionSystem<F, In, Out> {
    function: F,
    _phantom: core::marker::PhantomData<(In, Out)>,
}

impl<F, In, Out> FunctionSystem<F, In, Out>
where
    F: FnMut(&mut World, In) -> Out,
{
    /// Create a new function system
    pub fn new(function: F) -> Self {
        Self {
            function,
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<F, In, Out> System for FunctionSystem<F, In, Out>
where
    F: FnMut(&mut World, In) -> Out,
{
    type In = In;
    type Out = Out;

    fn run(&mut self, world: &mut World, input: Self::In) -> Self::Out {
        (self.function)(world, input)
    }
}

impl<F, In, Out> IntoSystem<In, Out> for F
where
    F: FnMut(&mut World, In) -> Out,
{
    type System = FunctionSystem<F, In, Out>;

    fn into_system(self) -> Self::System {
        FunctionSystem::new(self)
    }
}

/// System parameter for querying entities
pub struct QueryParam {
    query: Query,
}

impl QueryParam {
    /// Create a new query parameter
    pub fn new(component_types: Vec<Symbol>) -> Self {
        Self {
            query: Query::new(component_types),
        }
    }

    /// Add a component type to the query
    pub fn with_component(mut self, component_type: Symbol) -> Self {
        self.query = self.query.with_component(component_type);
        self
    }
}

impl SystemParam for QueryParam {
    type Param = Self;
    type Fetch = Vec<EntityId>;

    fn fetch(world: &World) -> Self::Fetch {
        // This is a simplified implementation
        // In a real system, you'd have more sophisticated query execution
        let env = soroban_sdk::Env::default();
        Vec::new(&env)
    }

    fn fetch_mut(world: &mut World) -> Self::Fetch {
        Self::fetch(world)
    }
}

/// System parameter for accessing resources
pub struct ResourceParam {
    resource_type: Symbol,
}

impl ResourceParam {
    /// Create a new resource parameter
    pub fn new(resource_type: Symbol) -> Self {
        Self { resource_type }
    }
}

impl SystemParam for ResourceParam {
    type Param = Self;
    type Fetch = Option<crate::resource::Resource>;

    fn fetch(world: &World) -> Self::Fetch {
        // This is a simplified implementation
        // In a real system, you'd have access to the parameter instance
        None
    }

    fn fetch_mut(world: &mut World) -> Self::Fetch {
        Self::fetch(world)
    }
}

// Example systems
/// Movement system for updating entity positions
pub struct MovementSystem;

impl System for MovementSystem {
    type In = ();
    type Out = ();

    fn run(&mut self, world: &mut World, _input: Self::In) -> Self::Out {
        // Example: Find all entities with position and velocity components
        let entities_with_movement = world.query_entities(&[
            symbol_short!("position"),
            symbol_short!("velocity"),
        ]);

        for i in 0..entities_with_movement.len() {
            let entity_id = entities_with_movement.get(i).unwrap();
            // In a real implementation, you'd:
            // 1. Get the position and velocity components
            // 2. Update the position based on velocity
            // 3. Apply any constraints (bounds, collision, etc.)
            // For now, we'll just mark that we processed this entity
        }
    }
}

/// Collision detection system
pub struct CollisionSystem;

impl System for CollisionSystem {
    type In = ();
    type Out = ();

    fn run(&mut self, world: &mut World, _input: Self::In) -> Self::Out {
        let entities_with_collision = world.query_entities(&[
            symbol_short!("position"),
            symbol_short!("collision"),
        ]);

        for i in 0..entities_with_collision.len() {
            for j in (i + 1)..entities_with_collision.len() {
                let entity_a = entities_with_collision.get(i).unwrap();
                let entity_b = entities_with_collision.get(j).unwrap();
                // Generate collision event
                let collision_event = crate::event::CollisionEvent::new(
                    entity_a.id(),
                    entity_b.id(),
                    symbol_short!("physical"),
                );
                let env = soroban_sdk::Env::default();
                let event_data = collision_event.serialize(&env);
                let event = crate::event::Event::new(
                    symbol_short!("collision"),
                    event_data,
                );
                world.send_event(event);
            }
        }
    }
}

/// Health system for managing entity health
pub struct HealthSystem;

impl System for HealthSystem {
    type In = ();
    type Out = ();

    fn run(&mut self, world: &mut World, _input: Self::In) -> Self::Out {
        let damage_events = world.get_events(&symbol_short!("damage"));
        let env = soroban_sdk::Env::default();
        for i in 0..damage_events.len() {
            let event = damage_events.get(i).unwrap();
            if let Some(damage_event) = DamageEvent::deserialize(&env, event.data()) {
                let target_entity = EntityId::new(damage_event.target_entity, 0);
                // In a real implementation, you'd:
                // 1. Get the health component from the target entity
                // 2. Apply the damage
                // 3. Check if the entity should be destroyed
                // 4. Update the health component
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_query_creation() {
        let component_types = vec![
            symbol_short!("position"),
            symbol_short!("velocity"),
        ];
        let query = Query::new(component_types);
        
        let world = World::new();
        let results = query.execute(&world);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_query_state() {
        let component_types = vec![symbol_short!("position")];
        let query = Query::new(component_types);
        let mut query_state = QueryState::new(query);
        
        let world = World::new();
        let results = query_state.execute(&world);
        assert_eq!(results.len(), 0);
        assert!(query_state.is_empty());
    }

    #[test]
    fn test_function_system() {
        let mut system = FunctionSystem::new(|world: &mut World, input: i32| {
            // Simple system that just returns the input
            input
        });
        
        let mut world = World::new();
        let result = system.run(&mut world, 42);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_movement_system() {
        let mut system = MovementSystem;
        let mut world = World::new();
        
        // This should run without errors
        system.run(&mut world, ());
    }

    #[test]
    fn test_collision_system() {
        let mut system = CollisionSystem;
        let mut world = World::new();
        
        // This should run without errors
        system.run(&mut world, ());
    }
} 
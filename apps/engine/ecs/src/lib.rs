#![no_std]
#![allow(unsafe_code)]

extern crate alloc;

use soroban_sdk::{
    symbol_short, Symbol, Vec, Bytes,
};

// Global allocator for WASM
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Core ECS types adapted for Soroban
pub mod entity;
pub mod component;
pub mod world;
pub mod system;
pub mod storage;
pub mod query;
pub mod resource;
pub mod event;
pub mod components;
pub mod systems;

// Re-export core types
pub use entity::{Entity, EntityId};
pub use component::{Component, ComponentId, ComponentStorage};
pub use world::World;
pub use system::{System, SystemParam, IntoSystem};
pub use storage::{Storage, TableStorage, SparseStorage};
pub use query::{Query, QueryState};
pub use resource::Resource;
pub use event::{Event, EventReader, EventWriter};
pub use components::Position;
pub use systems::MovementSystem;

// Library functions for ECS operations
pub fn create_world() -> World {
    World::new()
}

pub fn spawn_entity(world: &mut World, components: Vec<Component>) -> EntityId {
    let entity = world.spawn(components);
    entity.id()
}

pub fn add_component(world: &mut World, entity_id: EntityId, component: Component) -> bool {
    world.add_component_to_entity(entity_id, component);
    true
}

pub fn remove_component(world: &mut World, entity_id: EntityId, component_type: Symbol) -> bool {
    world.remove_component_from_entity(entity_id, &component_type)
}

pub fn get_component(world: &World, entity_id: EntityId, component_type: Symbol) -> Option<Component> {
    world.get_component(entity_id, &component_type)
}

pub fn query_entities(world: &World, component_types: Vec<Symbol>, env: &soroban_sdk::Env) -> Vec<EntityId> {
    // Since we can't easily convert Vec<Symbol> to &[Symbol] in Soroban,
    // we'll need to restructure this. For now, return empty result.
    Vec::new(env)
}

// Predule for common types
pub mod prelude {
    pub use super::{
        entity::{Entity, EntityId},
        component::{Component, ComponentId, ComponentStorage},
        world::World,
        system::{System, SystemParam, IntoSystem},
        storage::{Storage, TableStorage, SparseStorage},
        query::{Query, QueryState},
        resource::Resource,
        event::{Event, EventReader, EventWriter},
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::{Address as _,};

    #[test]
    fn test_world_creation() {
        let env = Env::default();
        let world = World::new();
        assert_eq!(world.entity_count(), 0);
    }

    #[test]
    fn test_entity_spawn() {
        let env = Env::default();
        let mut world = World::new();
        let entity = world.spawn_empty();
        assert_eq!(world.entity_count(), 1);
    }
} 
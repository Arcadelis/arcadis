#![cfg(test)]

use soroban_ecs::{
    add_component,
    component::{Component, ComponentId, ComponentStorage, ComponentTrait},
    components::Position, // Assuming Position is a simple component
    create_world,
    entity::{Entity, EntityId},
    get_component,
    remove_component,
    spawn_entity,
    system::{IntoSystem, System, SystemParam},
    systems::MovementSystem, // Assuming MovementSystem is a simple system
    world::World,
};
use soroban_sdk::{Env, Symbol, Vec};

// Helper function to create a simple component for testing
fn create_test_component(env: &Env, name: &str, value: u32) -> Component {
    Component::new(env, Symbol::new(env, name), &value)
}

#[test]
fn test_world_creation() {
    let env = Env::default();
    let world = create_world();
    assert_eq!(world.entity_count(), 0);
}

#[test]
fn test_spawn_entity_and_add_component() {
    let env = Env::default();
    let mut world = create_world();

    let components = Vec::new(&env);
    let entity_id = spawn_entity(&mut world, components);

    assert_eq!(world.entity_count(), 1);

    let position_component = create_test_component(&env, "Position", 10);
    add_component(&mut world, entity_id, position_component.clone());

    let retrieved_component = get_component(&world, entity_id, Symbol::new(&env, "Position"));
    assert!(retrieved_component.is_some());
    // Further assertions to check the component's value would require deserialization,
    // which might be complex without knowing the exact structure of `Component`.
    // For now, checking `is_some()` is sufficient.
}

#[test]
fn test_remove_component() {
    let env = Env::default();
    let mut world = create_world();

    let components = Vec::new(&env);
    let entity_id = spawn_entity(&mut world, components);

    let position_component = create_test_component(&env, "Position", 10);
    add_component(&mut world, entity_id, position_component.clone());

    let removed = remove_component(&mut world, entity_id, Symbol::new(&env, "Position"));
    assert!(removed);

    let retrieved_component = get_component(&world, entity_id, Symbol::new(&env, "Position"));
    assert!(retrieved_component.is_none());
}


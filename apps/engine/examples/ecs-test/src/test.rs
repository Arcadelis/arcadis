#![cfg(test)]

use super::{movement_system, EcsTestContract, EcsTestContractClient, Position, Velocity};
use soroban_ecs::component::{Component, ComponentTrait};
use soroban_ecs::world::World;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Bytes, Env, Vec};

// Helper function to create a test environment
fn create_env() -> Env {
    let env = Env::default();
    env.mock_all_auths();
    env
}

#[test]
fn test_position_component_serialization() {
    let env = create_env();
    let pos = Position { x: 100, y: 200 };
    let serialized_data = pos.serialize(&env);

    // Expected bytes: 100 (0x00000064) followed by 200 (0x000000C8)
    let expected_bytes = Bytes::from_array(&env, &[0, 0, 0, 100, 0, 0, 0, 200]);
    assert_eq!(serialized_data, expected_bytes);

    let deserialized_pos = Position::deserialize(&env, &serialized_data).unwrap();
    assert_eq!(deserialized_pos, pos);

    // Test with incorrect length
    let short_bytes = Bytes::from_array(&env, &[1, 2, 3, 4]);
    assert!(Position::deserialize(&env, &short_bytes).is_none());
}

#[test]
fn test_velocity_component_serialization() {
    let env = create_env();
    let vel = Velocity { dx: -10, dy: 50 };
    let serialized_data = vel.serialize(&env);

    // Expected bytes: -10 (0xFFFFFFF6) followed by 50 (0x00000032)
    let expected_bytes = Bytes::from_array(&env, &[255, 255, 255, 246, 0, 0, 0, 50]);
    assert_eq!(serialized_data, expected_bytes);

    let deserialized_vel = Velocity::deserialize(&env, &serialized_data).unwrap();
    assert_eq!(deserialized_vel, vel);

    // Test with incorrect length
    let short_bytes = Bytes::from_array(&env, &[1, 2, 3, 4]);
    assert!(Velocity::deserialize(&env, &short_bytes).is_none());
}

#[test]
fn test_movement_system() {
    let env = create_env();
    let mut world = World::new();

    // Create an entity with initial position and velocity
    let initial_pos = Position { x: 10, y: 20 };
    let velocity = Velocity { dx: 5, dy: -5 };

    let pos_component = Component::new(Position::component_type(), initial_pos.serialize(&env));
    let vel_component = Component::new(Velocity::component_type(), velocity.serialize(&env));

    let mut components = Vec::new(&env);
    components.push_back(pos_component);
    components.push_back(vel_component);

    let entity = world.spawn(components);

    // Run the movement system
    movement_system(&env, &mut world);

    // Verify the updated position
    let final_pos_comp = world
        .get_component(entity.id(), &Position::component_type())
        .unwrap();
    let final_pos = Position::deserialize(&env, &final_pos_comp.data).unwrap();

    // Expected: x = 10 + 5 = 15, y = 20 - 5 = 15
    assert_eq!(final_pos.x, 15);
    assert_eq!(final_pos.y, 15);

    // Test with values that would go below zero (should clamp to 0)
    let initial_pos_neg = Position { x: 2, y: 3 };
    let velocity_neg = Velocity { dx: -5, dy: -10 };

    let pos_comp_neg = Component::new(Position::component_type(), initial_pos_neg.serialize(&env));
    let vel_comp_neg = Component::new(Velocity::component_type(), velocity_neg.serialize(&env));

    let mut components_neg = Vec::new(&env);
    components_neg.push_back(pos_comp_neg);
    components_neg.push_back(vel_comp_neg);

    let entity_neg = world.spawn(components_neg);

    movement_system(&env, &mut world); // Run again for the new entity

    let final_pos_comp_neg = world
        .get_component(entity_neg.id(), &Position::component_type())
        .unwrap();
    let final_pos_neg = Position::deserialize(&env, &final_pos_comp_neg.data).unwrap();

    // Expected: x = (2 - 5).max(0) = 0, y = (3 - 10).max(0) = 0
    assert_eq!(final_pos_neg.x, 0);
    assert_eq!(final_pos_neg.y, 0);

    // Test an entity without Velocity (should not be updated by movement_system)
    let initial_pos_no_vel = Position { x: 50, y: 60 };
    let pos_comp_no_vel = Component::new(
        Position::component_type(),
        initial_pos_no_vel.serialize(&env),
    );
    let mut components_no_vel = Vec::new(&env);
    components_no_vel.push_back(pos_comp_no_vel);
    let entity_no_vel = world.spawn(components_no_vel);

    movement_system(&env, &mut world); // Run again

    let final_pos_comp_no_vel = world
        .get_component(entity_no_vel.id(), &Position::component_type())
        .unwrap();
    let final_pos_no_vel = Position::deserialize(&env, &final_pos_comp_no_vel.data).unwrap();
    assert_eq!(final_pos_no_vel.x, 50);
    assert_eq!(final_pos_no_vel.y, 60);
}

#[test]
fn test_contract_run_function() {
    let env = create_env();
    // let contract = env.register_stellar_asset_contract_v2(Address::generate(&env));
    // let contract_id = contract.address();
    // let client = EcsTestContractClient::new(&env, &contract_id);
    // let client = EcsTestContractClient::new(&env, &env.register(EcsTestContract, ()));

    // Register the contract
    let contract_id = env.register(EcsTestContract, ());
    let client = EcsTestContractClient::new(&env, &contract_id);

    // Call the run function which encapsulates the ECS workflow
    let (final_x, final_y) = client.run();

    // The run function initializes pos at (10, 20) and vel at (5, -5)
    // So, expected final position is (10 + 5, 20 - 5) = (15, 15)
    assert_eq!(final_x, 15);
    assert_eq!(final_y, 15);
}

#[test]
fn test_component_type_symbols() {
    assert_eq!(Position::component_type(), symbol_short!("pos"));
    assert_eq!(Velocity::component_type(), symbol_short!("vel"));
}

#![cfg(test)]

//! Unit Test Suite for Game Logic Contract (Soroban ECS)
//! 
//! This comprehensive test suite validates the functionality of the ECS-based
//! game logic contract in a Soroban environment. Tests cover:
//! - Contract initialization and World instance creation
//! - Entity creation with Position and Health components  
//! - MovementSystem execution for Position updates
//! - CombatSystem execution for Health modifications
//! - Error handling for invalid inputs and edge cases
//! - Storage and retrieval operations
//! 
//! Each test is designed to be modular, well-documented, and covers both
//! normal operation and edge cases to ensure contract robustness.

use soroban_sdk::{Env, testutils::*};

// Import the game contract and related types
use game::{GameWorldContract, GameWorldContractClient, GamePosition, Health, MovementSystem, CombatSystem};

/// Helper function to create a test environment with a deployed contract
fn setup_test_contract() -> (Env, GameWorldContractClient) {
    let env = Env::default();
    let contract_id = env.register_contract(None, GameWorldContract);
    let client = GameWorldContractClient::new(&env, &contract_id);
    (env, client)
}

/// Helper function to setup an initialized contract with test data
fn setup_initialized_contract() -> (Env, GameWorldContractClient) {
    let (env, client) = setup_test_contract();
    
    // Initialize the contract
    client.init();
    
    (env, client)
}

#[cfg(test)]
mod contract_initialization_tests {
    use super::*;

    /// Test: Contract initialization creates a valid World instance
    /// 
    /// Verifies that the contract can be properly initialized and that
    /// the initial state is correct (empty world, zero entity count).
    #[test]
    fn test_contract_initialization() {
        let (env, client) = setup_test_contract();
        
        // Initialize the contract and verify it returns valid data
        let init_result = client.init();
        
        // Verify initial state is correct
        assert_eq!(client.entity_count(), 0, "Initial entity count should be zero");
        assert_eq!(client.dead_entity_count(), 0, "Initial dead entity count should be zero");
    }

    /// Test: Multiple initialization calls are handled properly
    /// 
    /// Ensures that calling init() multiple times doesn't corrupt the state
    #[test]
    fn test_multiple_initialization() {
        let (env, client) = setup_test_contract();
        
        // Initialize multiple times
        client.init();
        let first_count = client.entity_count();
        
        client.init();
        let second_count = client.entity_count();
        
        // State should be consistent after multiple inits
        assert_eq!(first_count, second_count, "Multiple initialization should not change entity count");
    }
}

#[cfg(test)]
mod entity_management_tests {
    use super::*;

    /// Test: Entity creation with Position and Health components
    /// 
    /// Verifies that entities are properly created with the correct
    /// Position and Health component values and that entity IDs are unique.
    #[test]
    fn test_entity_creation() {
        let (env, client) = setup_initialized_contract();
        
        // Spawn first entity
        let entity1_id = client.spawn_entity(&10, &20);
        assert_eq!(entity1_id, 0, "First entity ID should be 0");
        
        // Verify entity count increased
        assert_eq!(client.entity_count(), 1, "Entity count should be 1 after spawning one entity");
        
        // Spawn second entity
        let entity2_id = client.spawn_entity(&30, &40);
        assert_eq!(entity2_id, 1, "Second entity ID should be 1");
        
        // Verify entity count increased again
        assert_eq!(client.entity_count(), 2, "Entity count should be 2 after spawning two entities");
        
        // Verify entities have different IDs
        assert_ne!(entity1_id, entity2_id, "Entity IDs should be unique");
    }

    /// Test: Entity position retrieval
    /// 
    /// Tests that entities spawned with specific positions can be
    /// retrieved correctly from storage.
    #[test]
    fn test_entity_position_retrieval() {
        let (env, client) = setup_initialized_contract();
        
        // Spawn entity at specific position
        let entity_id = client.spawn_entity(&100, &200);
        
        // Retrieve position
        let position = client.get_entity_position(&entity_id);
        assert!(position.is_some(), "Position should exist for spawned entity");
        
        let pos = position.unwrap();
        assert_eq!(pos.0, 100, "X coordinate should match spawned position");
        assert_eq!(pos.1, 200, "Y coordinate should match spawned position");
    }

    /// Test: Entity health retrieval  
    /// 
    /// Verifies that entities are spawned with correct initial health
    /// and that health can be retrieved properly.
    #[test]
    fn test_entity_health_retrieval() {
        let (env, client) = setup_initialized_contract();
        
        // Spawn entity (should have default health of 100)
        let entity_id = client.spawn_entity(&50, &75);
        
        // Retrieve health
        let health = client.get_entity_health(&entity_id);
        assert!(health.is_some(), "Health should exist for spawned entity");
        
        let hp = health.unwrap();
        assert_eq!(hp.0, 100, "Initial health should be 100");
    }

    /// Test: Entity despawning
    /// 
    /// Tests that entities can be properly removed from the world
    /// and that entity count is updated correctly.
    #[test]
    fn test_entity_despawning() {
        let (env, client) = setup_initialized_contract();
        
        // Spawn an entity
        let entity_id = client.spawn_entity(&25, &35);
        assert_eq!(client.entity_count(), 1, "Should have 1 entity after spawning");
        
        // Despawn the entity
        let despawn_result = client.despawn_entity(&entity_id);
        assert!(despawn_result, "Despawning should succeed");
        
        // Verify entity count decreased
        assert_eq!(client.entity_count(), 0, "Should have 0 entities after despawning");
        
        // Verify entity data is no longer accessible
        let position = client.get_entity_position(&entity_id);
        assert!(position.is_none(), "Position should not exist after despawning");
        
        let health = client.get_entity_health(&entity_id);
        assert!(health.is_none(), "Health should not exist after despawning");
    }
}

#[cfg(test)]
mod movement_system_tests {
    use super::*;

    /// Test: MovementSystem position updates
    /// 
    /// Verifies that the MovementSystem correctly updates entity positions
    /// when move_entity is called with various delta values.
    #[test]
    fn test_movement_system_basic_movement() {
        let (env, client) = setup_initialized_contract();
        
        // Spawn entity at (50, 50)
        let entity_id = client.spawn_entity(&50, &50);
        
        // Move entity by (10, -5)
        let move_result = client.move_entity(&entity_id, &10, &-5);
        assert!(move_result, "Movement should succeed");
        
        // Verify new position
        let position = client.get_entity_position(&entity_id).unwrap();
        assert_eq!(position.0, 60, "X should be 50 + 10 = 60");
        assert_eq!(position.1, 45, "Y should be 50 - 5 = 45");
    }

    /// Test: MovementSystem boundary handling (negative coordinates)
    /// 
    /// Tests that movement properly handles cases where the result
    /// would be negative coordinates (should clamp to 0).
    #[test]
    fn test_movement_system_negative_boundary() {
        let (env, client) = setup_initialized_contract();
        
        // Spawn entity at (5, 3)
        let entity_id = client.spawn_entity(&5, &3);
        
        // Try to move by (-10, -10) which would result in negative coordinates
        let move_result = client.move_entity(&entity_id, &-10, &-10);
        assert!(move_result, "Movement should succeed even with negative result");
        
        // Verify coordinates are clamped to 0
        let position = client.get_entity_position(&entity_id).unwrap();
        assert_eq!(position.0, 0, "X should be clamped to 0");
        assert_eq!(position.1, 0, "Y should be clamped to 0");
    }

    /// Test: MovementSystem with large coordinate values
    /// 
    /// Tests movement with large coordinate values to ensure
    /// no overflow or unexpected behavior occurs.
    #[test]
    fn test_movement_system_large_coordinates() {
        let (env, client) = setup_initialized_contract();
        
        // Spawn entity at max reasonable coordinates
        let large_x = 1000000u32;
        let large_y = 1000000u32;
        let entity_id = client.spawn_entity(&large_x, &large_y);
        
        // Move by reasonable amounts
        let move_result = client.move_entity(&entity_id, &1000, &-1000);
        assert!(move_result, "Movement with large coordinates should succeed");
        
        // Verify calculation is correct
        let position = client.get_entity_position(&entity_id).unwrap();
        assert_eq!(position.0, large_x + 1000, "Large X coordinate movement should work");
        assert_eq!(position.1, large_y - 1000, "Large Y coordinate movement should work");
    }

    /// Test: MovementSystem zero movement
    /// 
    /// Tests that moving by (0, 0) works correctly and doesn't change position.
    #[test]
    fn test_movement_system_zero_movement() {
        let (env, client) = setup_initialized_contract();
        
        // Spawn entity at (100, 200)
        let entity_id = client.spawn_entity(&100, &200);
        
        // Move by (0, 0)
        let move_result = client.move_entity(&entity_id, &0, &0);
        assert!(move_result, "Zero movement should succeed");
        
        // Verify position unchanged
        let position = client.get_entity_position(&entity_id).unwrap();
        assert_eq!(position.0, 100, "X should remain unchanged");
        assert_eq!(position.1, 200, "Y should remain unchanged");
    }
}

#[cfg(test)]
mod combat_system_tests {
    use super::*;

    /// Test: CombatSystem health reduction
    /// 
    /// Verifies that the CombatSystem correctly reduces entity health
    /// by the expected amount when attack_entity is called.
    #[test]
    fn test_combat_system_basic_attack() {
        let (env, client) = setup_initialized_contract();
        
        // Spawn entity (starts with 100 health)
        let entity_id = client.spawn_entity(&10, &10);
        
        // Attack the entity (should reduce health by 10)
        let attack_result = client.attack_entity(&entity_id);
        assert!(attack_result, "Attack should succeed");
        
        // Verify health was reduced
        let health = client.get_entity_health(&entity_id).unwrap();
        assert_eq!(health.0, 90, "Health should be reduced from 100 to 90");
        
        // Verify entity is still alive and position unchanged
        let position = client.get_entity_position(&entity_id);
        assert!(position.is_some(), "Entity should still exist after non-fatal attack");
    }

    /// Test: CombatSystem multiple attacks
    /// 
    /// Tests that multiple attacks properly accumulate damage
    /// and health decreases correctly with each attack.
    #[test]
    fn test_combat_system_multiple_attacks() {
        let (env, client) = setup_initialized_contract();
        
        // Spawn entity
        let entity_id = client.spawn_entity(&15, &25);
        
        // Perform multiple attacks
        for i in 1..=5 {
            let attack_result = client.attack_entity(&entity_id);
            assert!(attack_result, "Attack {} should succeed", i);
            
            let expected_health = 100 - (i * 10);
            let health = client.get_entity_health(&entity_id).unwrap();
            assert_eq!(health.0, expected_health as u32, "Health after attack {} should be {}", i, expected_health);
        }
        
        // Entity should still be alive with 50 health
        assert!(client.get_entity_position(&entity_id).is_some(), "Entity should still exist");
    }

    /// Test: CombatSystem entity death
    /// 
    /// Verifies that when an entity's health reaches 0, it is properly
    /// removed from the world and dead entity count increases.
    #[test]
    fn test_combat_system_entity_death() {
        let (env, client) = setup_initialized_contract();
        
        // Spawn entity
        let entity_id = client.spawn_entity(&30, &40);
        
        // Attack entity until it dies (10 attacks = 100 damage)
        for i in 1..=10 {
            let attack_result = client.attack_entity(&entity_id);
            assert!(attack_result, "Attack {} should succeed", i);
        }
        
        // Verify entity is dead (removed from world)
        let position = client.get_entity_position(&entity_id);
        assert!(position.is_none(), "Dead entity should not have position");
        
        let health = client.get_entity_health(&entity_id);
        assert!(health.is_none(), "Dead entity should not have health");
        
        // Verify dead entity count increased
        assert_eq!(client.dead_entity_count(), 1, "Dead entity count should be 1");
        
        // Verify total entity count decreased
        assert_eq!(client.entity_count(), 0, "Living entity count should be 0");
    }

    /// Test: CombatSystem health underflow protection
    /// 
    /// Tests that health properly uses saturating subtraction and
    /// doesn't underflow when health is already low.
    #[test]
    fn test_combat_system_health_underflow_protection() {
        let (env, client) = setup_initialized_contract();
        
        // Spawn entity and attack until near death
        let entity_id = client.spawn_entity(&5, &5);
        
        // Attack 9 times (90 damage, leaving 10 health)
        for _ in 1..=9 {
            client.attack_entity(&entity_id);
        }
        
        // Verify entity has 10 health
        let health = client.get_entity_health(&entity_id).unwrap();
        assert_eq!(health.0, 10, "Entity should have 10 health remaining");
        
        // One more attack should kill the entity (10 - 10 = 0)
        let attack_result = client.attack_entity(&entity_id);
        assert!(attack_result, "Final attack should succeed");
        
        // Entity should now be dead
        assert!(client.get_entity_position(&entity_id).is_none(), "Entity should be dead");
        assert_eq!(client.dead_entity_count(), 1, "Should have 1 dead entity");
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    /// Test: Operations on non-existent entities
    /// 
    /// Verifies that operations on invalid/non-existent entity IDs
    /// are handled gracefully and return appropriate error values.
    #[test]
    fn test_operations_on_nonexistent_entities() {
        let (env, client) = setup_initialized_contract();
        
        let invalid_entity_id = 999u32;
        
        // Test movement on non-existent entity
        let move_result = client.move_entity(&invalid_entity_id, &10, &20);
        assert!(!move_result, "Movement on non-existent entity should fail");
        
        // Test attack on non-existent entity
        let attack_result = client.attack_entity(&invalid_entity_id);
        assert!(!attack_result, "Attack on non-existent entity should fail");
        
        // Test position retrieval on non-existent entity
        let position = client.get_entity_position(&invalid_entity_id);
        assert!(position.is_none(), "Position retrieval on non-existent entity should return None");
        
        // Test health retrieval on non-existent entity
        let health = client.get_entity_health(&invalid_entity_id);
        assert!(health.is_none(), "Health retrieval on non-existent entity should return None");
        
        // Test despawning non-existent entity
        let despawn_result = client.despawn_entity(&invalid_entity_id);
        assert!(!despawn_result, "Despawning non-existent entity should fail");
    }

    /// Test: Operations on already dead entities
    /// 
    /// Tests that operations on entities that have been killed
    /// are handled properly and don't cause state corruption.
    #[test]
    fn test_operations_on_dead_entities() {
        let (env, client) = setup_initialized_contract();
        
        // Spawn and kill an entity
        let entity_id = client.spawn_entity(&10, &10);
        
        // Kill the entity (10 attacks)
        for _ in 1..=10 {
            client.attack_entity(&entity_id);
        }
        
        // Verify entity is dead
        assert!(client.get_entity_position(&entity_id).is_none(), "Entity should be dead");
        
        // Try operations on dead entity
        let move_result = client.move_entity(&entity_id, &5, &5);
        assert!(!move_result, "Movement on dead entity should fail");
        
        let attack_result = client.attack_entity(&entity_id);
        assert!(!attack_result, "Attack on dead entity should fail");
        
        let despawn_result = client.despawn_entity(&entity_id);
        assert!(!despawn_result, "Despawning dead entity should fail");
    }

    /// Test: Edge case with maximum entity IDs
    /// 
    /// Tests behavior when dealing with edge cases around entity ID limits.
    #[test]
    fn test_edge_case_maximum_entity_ids() {
        let (env, client) = setup_initialized_contract();
        
        // Test with maximum u32 value as entity ID
        let max_entity_id = u32::MAX;
        
        let move_result = client.move_entity(&max_entity_id, &0, &0);
        assert!(!move_result, "Movement on max entity ID should fail");
        
        let position = client.get_entity_position(&max_entity_id);
        assert!(position.is_none(), "Max entity ID should not exist");
    }
}

#[cfg(test)]
mod world_state_tests {
    use super::*;

    /// Test: Empty World instance behavior
    /// 
    /// Verifies that operations on an empty world (no entities)
    /// behave correctly and maintain consistent state.
    #[test]
    fn test_empty_world_behavior() {
        let (env, client) = setup_initialized_contract();
        
        // Verify initial empty state
        assert_eq!(client.entity_count(), 0, "Empty world should have 0 entities");
        assert_eq!(client.dead_entity_count(), 0, "Empty world should have 0 dead entities");
        
        // Operations on empty world should fail gracefully
        let move_result = client.move_entity(&0, &10, &10);
        assert!(!move_result, "Movement in empty world should fail");
        
        let attack_result = client.attack_entity(&0);
        assert!(!attack_result, "Attack in empty world should fail");
    }

    /// Test: World state consistency after multiple operations
    /// 
    /// Ensures that the world state remains consistent after
    /// a complex sequence of entity operations.
    #[test]
    fn test_world_state_consistency() {
        let (env, client) = setup_initialized_contract();
        
        // Spawn multiple entities
        let entity1 = client.spawn_entity(&10, &20);
        let entity2 = client.spawn_entity(&30, &40);
        let entity3 = client.spawn_entity(&50, &60);
        
        assert_eq!(client.entity_count(), 3, "Should have 3 entities");
        
        // Kill one entity
        for _ in 1..=10 {
            client.attack_entity(&entity2);
        }
        
        assert_eq!(client.entity_count(), 2, "Should have 2 living entities after death");
        assert_eq!(client.dead_entity_count(), 1, "Should have 1 dead entity");
        
        // Move remaining entities
        client.move_entity(&entity1, &5, &5);
        client.move_entity(&entity3, &-10, &10);
        
        // Verify positions are correct
        let pos1 = client.get_entity_position(&entity1).unwrap();
        assert_eq!(pos1.0, 15, "Entity1 X should be updated");
        assert_eq!(pos1.1, 25, "Entity1 Y should be updated");
        
        let pos3 = client.get_entity_position(&entity3).unwrap();
        assert_eq!(pos3.0, 40, "Entity3 X should be updated");
        assert_eq!(pos3.1, 70, "Entity3 Y should be updated");
        
        // Despawn one entity
        client.despawn_entity(&entity1);
        
        assert_eq!(client.entity_count(), 1, "Should have 1 entity after despawn");
        assert_eq!(client.dead_entity_count(), 1, "Dead count should remain 1");
    }

    /// Test: Entity ID uniqueness and sequential assignment
    /// 
    /// Verifies that entity IDs are assigned sequentially and
    /// remain unique even after entities are killed or despawned.
    #[test]
    fn test_entity_id_uniqueness() {
        let (env, client) = setup_initialized_contract();
        
        // Spawn several entities and collect their IDs
        let mut entity_ids = Vec::new();
        for i in 0..5 {
            let id = client.spawn_entity(&(i * 10), &(i * 10));
            entity_ids.push(id);
        }
        
        // Verify IDs are sequential
        for (index, &entity_id) in entity_ids.iter().enumerate() {
            assert_eq!(entity_id, index as u32, "Entity ID should be sequential");
        }
        
        // Kill some entities
        for _ in 1..=10 {
            client.attack_entity(&entity_ids[1]);
            client.attack_entity(&entity_ids[3]);
        }
        
        // Spawn new entities - IDs should continue from where they left off
        let new_id1 = client.spawn_entity(&100, &200);
        let new_id2 = client.spawn_entity(&300, &400);
        
        assert_eq!(new_id1, 5, "New entity ID should continue sequence");
        assert_eq!(new_id2, 6, "Next entity ID should be sequential");
        
        // Verify all IDs are unique
        let mut all_ids = entity_ids.clone();
        all_ids.push(new_id1);
        all_ids.push(new_id2);
        
        for i in 0..all_ids.len() {
            for j in i + 1..all_ids.len() {
                assert_ne!(all_ids[i], all_ids[j], "All entity IDs should be unique");
            }
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Test: Full gameplay scenario
    /// 
    /// Simulates a complete gameplay scenario with multiple entities,
    /// movements, combat, and verifies all systems work together correctly.
    #[test]
    fn test_full_gameplay_scenario() {
        let (env, client) = setup_initialized_contract();
        
        // Spawn player and enemy entities
        let player_id = client.spawn_entity(&0, &0);      // Player at origin
        let enemy1_id = client.spawn_entity(&10, &10);    // Enemy 1
        let enemy2_id = client.spawn_entity(&-5, &5);     // Enemy 2 (will clamp to 5, 5)
        
        assert_eq!(client.entity_count(), 3, "Should have 3 entities in the game");
        
        // Move player towards first enemy
        client.move_entity(&player_id, &5, &5);
        let player_pos = client.get_entity_position(&player_id).unwrap();
        assert_eq!(player_pos.0, 5, "Player should move to X=5");
        assert_eq!(player_pos.1, 5, "Player should move to Y=5");
        
        // Simulate combat between player and enemy1
        for _ in 1..=5 {
            client.attack_entity(&enemy1_id);  // Player attacks enemy
            client.attack_entity(&player_id);  // Enemy attacks back
        }
        
        // Check health after combat
        let player_health = client.get_entity_health(&player_id).unwrap();
        let enemy1_health = client.get_entity_health(&enemy1_id).unwrap();
        
        assert_eq!(player_health.0, 50, "Player should have 50 health after 5 attacks");
        assert_eq!(enemy1_health.0, 50, "Enemy1 should have 50 health after 5 attacks");
        
        // Kill enemy1 completely
        for _ in 1..=5 {
            client.attack_entity(&enemy1_id);
        }
        
        // Verify enemy1 is dead
        assert!(client.get_entity_position(&enemy1_id).is_none(), "Enemy1 should be dead");
        assert_eq!(client.entity_count(), 2, "Should have 2 living entities");
        assert_eq!(client.dead_entity_count(), 1, "Should have 1 dead entity");
        
        // Move towards enemy2
        client.move_entity(&player_id, &0, &0);  // Move to (5, 5)
        
        // Verify enemy2 position (should have been clamped during spawn)
        let enemy2_pos = client.get_entity_position(&enemy2_id).unwrap();
        assert_eq!(enemy2_pos.0, 0, "Enemy2 X should have been clamped to 0");
        assert_eq!(enemy2_pos.1, 5, "Enemy2 Y should be 5");
        
        // Final battle - kill enemy2
        for _ in 1..=10 {
            client.attack_entity(&enemy2_id);
        }
        
        // Verify final state
        assert_eq!(client.entity_count(), 1, "Only player should remain alive");
        assert_eq!(client.dead_entity_count(), 2, "Should have 2 dead entities");
        
        let final_player_health = client.get_entity_health(&player_id).unwrap();
        assert_eq!(final_player_health.0, 50, "Player should still have 50 health");
    }

    /// Test: Stress test with many entities
    /// 
    /// Creates many entities and performs operations to test
    /// performance and stability under load.
    #[test]
    fn test_many_entities_stress_test() {
        let (env, client) = setup_initialized_contract();
        
        const ENTITY_COUNT: u32 = 100;
        let mut entity_ids = Vec::new();
        
        // Spawn many entities
        for i in 0..ENTITY_COUNT {
            let id = client.spawn_entity(&(i % 50), &(i % 50));
            entity_ids.push(id);
        }
        
        assert_eq!(client.entity_count(), ENTITY_COUNT, "Should have spawned all entities");
        
        // Move all entities
        for (index, &entity_id) in entity_ids.iter().enumerate() {
            let dx = (index as i32 % 10) - 5;  // Random-ish movement
            let dy = (index as i32 % 8) - 4;
            client.move_entity(&entity_id, &dx, &dy);
        }
        
        // Attack every other entity
        for (index, &entity_id) in entity_ids.iter().enumerate() {
            if index % 2 == 0 {
                for _ in 1..=5 {  // 5 attacks = 50 damage
                    client.attack_entity(&entity_id);
                }
            }
        }
        
        // Verify some entities are damaged
        let mut damaged_count = 0;
        for (index, &entity_id) in entity_ids.iter().enumerate() {
            if index % 2 == 0 {
                if let Some(health) = client.get_entity_health(&entity_id) {
                    assert_eq!(health.0, 50, "Even-indexed entities should have 50 health");
                    damaged_count += 1;
                }
            } else {
                if let Some(health) = client.get_entity_health(&entity_id) {
                    assert_eq!(health.0, 100, "Odd-indexed entities should have full health");
                }
            }
        }
        
        assert_eq!(damaged_count, ENTITY_COUNT / 2, "Half the entities should be damaged");
        assert_eq!(client.entity_count(), ENTITY_COUNT, "All entities should still be alive");
    }
}

#[cfg(test)]
mod component_system_tests {
    use super::*;

    /// Test: Component trait implementations
    /// 
    /// Tests the ComponentTrait implementations for GamePosition and Health
    /// to ensure serialization and deserialization work correctly.
    #[test]
    fn test_gameposition_component_serialization() {
        let env = Env::default();
        
        // Test GamePosition serialization/deserialization
        let original_pos = GamePosition(12345, 67890);
        
        // Serialize
        let serialized = original_pos.serialize(&env);
        assert_eq!(serialized.len(), 8, "GamePosition should serialize to 8 bytes");
        
        // Deserialize
        let deserialized = GamePosition::deserialize(&env, &serialized);
        assert!(deserialized.is_some(), "Deserialization should succeed");
        
        let recovered_pos = deserialized.unwrap();
        assert_eq!(recovered_pos.0, original_pos.0, "X coordinate should match");
        assert_eq!(recovered_pos.1, original_pos.1, "Y coordinate should match");
    }

    /// Test: Health component serialization
    /// 
    /// Tests Health component serialization and deserialization.
    #[test]
    fn test_health_component_serialization() {
        let env = Env::default();
        
        // Test Health serialization/deserialization
        let original_health = Health(42);
        
        // Serialize
        let serialized = original_health.serialize(&env);
        assert_eq!(serialized.len(), 4, "Health should serialize to 4 bytes");
        
        // Deserialize
        let deserialized = Health::deserialize(&env, &serialized);
        assert!(deserialized.is_some(), "Deserialization should succeed");
        
        let recovered_health = deserialized.unwrap();
        assert_eq!(recovered_health.0, original_health.0, "Health value should match");
    }

    /// Test: Invalid deserialization handling
    /// 
    /// Tests that invalid data is properly rejected during deserialization.
    #[test]
    fn test_invalid_deserialization() {
        let env = Env::default();
        
        // Test GamePosition with wrong byte length
        let wrong_size_bytes = soroban_sdk::Bytes::from_slice(&env, &[1, 2, 3, 4]); // Only 4 bytes instead of 8
        let invalid_pos = GamePosition::deserialize(&env, &wrong_size_bytes);
        assert!(invalid_pos.is_none(), "Invalid GamePosition data should not deserialize");
        
        // Test Health with wrong byte length  
        let wrong_health_bytes = soroban_sdk::Bytes::from_slice(&env, &[1, 2]); // Only 2 bytes instead of 4
        let invalid_health = Health::deserialize(&env, &wrong_health_bytes);
        assert!(invalid_health.is_none(), "Invalid Health data should not deserialize");
    }

    /// Test: System functions directly
    /// 
    /// Tests MovementSystem and CombatSystem functions in isolation.
    #[test]
    fn test_movement_system_direct() {
        // Test MovementSystem::update_position directly
        let initial_pos = GamePosition(100, 200);
        
        // Test normal movement
        let new_pos = MovementSystem::update_position(&initial_pos, 50, -25);
        assert_eq!(new_pos.0, 150, "X should be 100 + 50 = 150");
        assert_eq!(new_pos.1, 175, "Y should be 200 - 25 = 175");
        
        // Test negative boundary clamping
        let boundary_pos = GamePosition(5, 3);
        let clamped_pos = MovementSystem::update_position(&boundary_pos, -10, -5);
        assert_eq!(clamped_pos.0, 0, "Negative X should clamp to 0");
        assert_eq!(clamped_pos.1, 0, "Negative Y should clamp to 0");
    }

    /// Test: Combat system directly
    /// 
    /// Tests CombatSystem::update_health function in isolation.
    #[test]
    fn test_combat_system_direct() {
        // Test CombatSystem::update_health directly
        let initial_health = Health(100);
        
        // Test normal damage
        let damaged_health = CombatSystem::update_health(&initial_health);
        assert_eq!(damaged_health.0, 90, "Health should be reduced by 10");
        
        // Test low health
        let low_health = Health(5);
        let final_health = CombatSystem::update_health(&low_health);
        assert_eq!(final_health.0, 0, "Health should saturate to 0, not underflow");
        
        // Test zero health
        let zero_health = Health(0);
        let still_zero = CombatSystem::update_health(&zero_health);
        assert_eq!(still_zero.0, 0, "Zero health should remain zero");
    }
}
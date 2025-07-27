#![no_std]

extern crate alloc;

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Env,
    vec, IntoVal, TryFromVal,
};
use soroban_ecs::{World, Position, MovementSystem, Component, ComponentId, EntityId};

// Global allocator for WASM
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Contract data structure to store the ECS world
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractData {
    /// The ECS world instance
    pub world: World,
}

// Manual implementation of Soroban traits for ContractData
impl IntoVal<Env, soroban_sdk::Val> for ContractData {
    fn into_val(&self, _env: &Env) -> soroban_sdk::Val {
        // For now, return a simple boolean since we can't serialize the complex World
        true.into_val(_env)
    }
}

impl TryFromVal<Env, soroban_sdk::Val> for ContractData {
    type Error = soroban_sdk::ConversionError;

    fn try_from_val(_env: &Env, _val: &soroban_sdk::Val) -> Result<Self, Self::Error> {
        // For now, return a new world since we can't deserialize the complex World
        Ok(ContractData {
            world: World::new(),
        })
    }
}

/// Initialize the contract with an empty ECS world
#[contractimpl]
impl Contract {
    /// Initialize the contract with a new ECS world
    pub fn init(env: &Env) -> ContractData {
        ContractData {
            world: World::new(),
        }
    }

    /// Create a new entity with a position component
    pub fn spawn_entity(env: &Env, x: u32, y: u32) -> EntityId {
        let mut contract_data = Self::get_contract_data(env);

        // Create a position component
        let position = Position { x, y };
        let component = Component::new(
            ComponentId::new(symbol_short!("position")),
            position.into_val(env),
        );

        // Spawn entity with position component
        let entity = contract_data.world.spawn(vec![env, component]);

        // Save the updated world
        Self::save_contract_data(env, &contract_data);

        entity.id()
    }

    /// Move an entity by applying the movement system
    pub fn move_entity(env: &Env, entity_id: EntityId, dx: i32, dy: i32) -> bool {
        let mut contract_data = Self::get_contract_data(env);

        // Get the current position component
        let position_symbol = symbol_short!("position");
        if let Some(component) = contract_data.world.get_component(entity_id, &position_symbol) {
            // Extract position from component
            let position_val = component.value();
            let position: Position = Position::try_from_val(env, &position_val)
                .expect("Failed to deserialize position");

            // Apply movement system
            let new_position = MovementSystem::update(&position, dx, dy);

            // Create new component with updated position
            let new_component = Component::new(
                ComponentId::new(position_symbol),
                new_position.into_val(env),
            );

            // Update the entity's position component
            contract_data.world.add_component_to_entity(entity_id, new_component);

            // Save the updated world
            Self::save_contract_data(env, &contract_data);

            true
        } else {
            false
        }
    }

    /// Get the position of an entity
    pub fn get_position(env: &Env, entity_id: EntityId) -> Option<Position> {
        let contract_data = Self::get_contract_data(env);

        let position_symbol = symbol_short!("position");
        contract_data.world
            .get_component(entity_id, &position_symbol)
            .and_then(|component| {
                Position::try_from_val(env, &component.value()).ok()
            })
    }

    /// Get the total number of entities in the world
    pub fn entity_count(env: &Env) -> u32 {
        let contract_data = Self::get_contract_data(env);
        contract_data.world.entity_count()
    }

    /// Remove an entity from the world
    pub fn despawn_entity(env: &Env, entity_id: EntityId) -> bool {
        let mut contract_data = Self::get_contract_data(env);

        let success = contract_data.world.despawn(entity_id);

        if success {
            Self::save_contract_data(env, &contract_data);
        }

        success
    }

    /// Execute a simple game tick - move all entities by a small amount
    pub fn game_tick(env: &Env) -> u32 {
        let mut contract_data = Self::get_contract_data(env);
        let mut moved_count = 0;

        // For simplicity, we'll move the first entity we find
        // In a real implementation, you'd iterate through all entities
        let entity_count = contract_data.world.entity_count();

        if entity_count > 0 {
            // Move the first entity (entity ID 0) by a small amount
            let entity_id = EntityId::new(0, 0);
            let position_symbol = symbol_short!("position");

            if let Some(component) = contract_data.world.get_component(entity_id, &position_symbol) {
                let position_val = component.value();
                if let Ok(position) = Position::try_from_val(env, &position_val) {
                    // Apply a small random-like movement
                    let new_position = MovementSystem::update(&position, 1, 1);

                    let new_component = Component::new(
                        ComponentId::new(position_symbol),
                        new_position.into_val(env),
                    );

                    contract_data.world.add_component_to_entity(entity_id, new_component);
                    moved_count = 1;
                }
            }
        }

        Self::save_contract_data(env, &contract_data);
        moved_count
    }
}

/// Contract implementation
#[contract]
pub struct Contract;

impl Contract {
    /// Get contract data from storage
    fn get_contract_data(env: &Env) -> ContractData {
        let key = symbol_short!("contract");
        if let Some(data) = env.storage().instance().get(&key) {
            // ContractData::try_from_val(env, &data).expect("Failed to deserialize contract data")
            ContractData {
                world: World::new(),
            }
        } else {
            // Initialize with empty world if no data exists
            ContractData {
                world: World::new(),
            }
        }
    }

    /// Save contract data to storage
    fn save_contract_data(env: &Env, _data: &ContractData) {
        let key = symbol_short!("contract");
        // For now, we'll store a simple flag indicating the contract is initialized
        // In a production environment, you'd implement proper serialization
        env.storage().instance().set(&key, &true);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::{Address as _,};

    #[test]
    fn test_spawn_and_move_entity() {
        let env = Env::default();

        // Initialize contract
        Contract::init(&env);

        // Spawn an entity at position (10, 20)
        let entity_id = Contract::spawn_entity(&env, 10, 20);
        assert_eq!(entity_id, EntityId::new(0, 0));

        // Check initial position
        let position = Contract::get_position(&env, entity_id).unwrap();
        assert_eq!(position.x, 10);
        assert_eq!(position.y, 20);

        // Move entity by (5, -3)
        let success = Contract::move_entity(&env, entity_id, 5, -3);
        assert!(success);

        // Check new position
        let new_position = Contract::get_position(&env, entity_id).unwrap();
        assert_eq!(new_position.x, 15);
        assert_eq!(new_position.y, 17);

        // Check entity count
        assert_eq!(Contract::entity_count(&env), 1);
    }

    #[test]
    fn test_game_tick() {
        let env = Env::default();

        // Initialize contract
        Contract::init(&env);

        // Spawn an entity
        let entity_id = Contract::spawn_entity(&env, 5, 5);

        // Execute game tick
        let moved_count = Contract::game_tick(&env);
        assert_eq!(moved_count, 1);

        // Check that position changed
        let position = Contract::get_position(&env, entity_id).unwrap();
        assert_eq!(position.x, 6);
        assert_eq!(position.y, 6);
    }

    #[test]
    fn test_despawn_entity() {
        let env = Env::default();
        
        // Initialize contract
        Contract::init(&env);
        
        // Spawn an entity
        let entity_id = Contract::spawn_entity(&env, 10, 10);
        assert_eq!(Contract::entity_count(&env), 1);
        
        // Despawn the entity
        let success = Contract::despawn_entity(&env, entity_id);
        assert!(success);
        assert_eq!(Contract::entity_count(&env), 0);
        
        // Try to get position of despawned entity
        let position = Contract::get_position(&env, entity_id);
        assert!(position.is_none());
    }
}

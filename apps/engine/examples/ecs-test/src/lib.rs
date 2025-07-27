
#![no_std]
extern crate alloc;
use soroban_sdk::{
    contract, contractimpl, symbol_short, Env, IntoVal, TryFromVal,
};
use soroban_ecs::{World, Position, MovementSystem};
/// Example game contract demonstrating soroban-ecs usage
/// This contract showcases a simple 2D game world with entities that can move around
#[contract]
pub struct GameWorldContract;
/// Contract data structure to store the game world state
#[derive(Clone, Debug)]
pub struct GameWorldData {
    /// Flag indicating if the contract has been initialized
    pub is_initialized: bool,
    /// Number of entities in the world
    pub entity_count: u32,
    /// World data
    pub world_data: World,
}
// Manual implementation of Soroban traits for GameWorldData
impl IntoVal<Env, soroban_sdk::Val> for GameWorldData {
    fn into_val(&self, env: &Env) -> soroban_sdk::Val {
        let data = (self.is_initialized, self.entity_count);
        data.into_val(env)
    }
}
impl TryFromVal<Env, soroban_sdk::Val> for GameWorldData {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(env: &Env, val: &soroban_sdk::Val) -> Result<Self, Self::Error> {
        let (is_initialized, entity_count): (bool, u32) = TryFromVal::try_from_val(env, val)?;
        Ok(GameWorldData {
            is_initialized,
            entity_count,
            world_data: World::new(),
        })
    }
}
/// Initialize the contract with an empty ECS world
#[contractimpl]
impl GameWorldContract {
    /// Initialize the contract with a new ECS world
    pub fn init(env: &Env) -> GameWorldData {
        // Create a new ECS world
        let _world = World::new();
        let data = GameWorldData {
            is_initialized: true,
            entity_count: 0,
            world_data: World::new(),
        };
        Self::save_contract_data(env, &data);
        data
    }
    /// Create a new entity with a position component
    pub fn spawn_entity(env: &Env, x: u32, y: u32) -> u32 {
        let mut contract_data = Self::get_contract_data(env);
        // Create a new ECS world
        let _world = World::new();
        // Create a position component using the ECS system
        let position = Position { x, y };
        let entity_id = contract_data.entity_count;
        // Store entity position in contract storage
        let entity_key = symbol_short!("entity");
        let entity_data: (u32, u32, u32) = (entity_id, position.x, position.y);
        let val: soroban_sdk::Val = entity_data.into_val(env);
        env.storage().instance().set(&entity_key, &val);
        // Update entity count
        contract_data.entity_count += 1;
        Self::save_contract_data(env, &contract_data);
        entity_id
    }
    /// Move an entity by the given delta using the MovementSystem
    pub fn move_entity(env: &Env, entity_id: u32, dx: i32, dy: i32) -> bool {
        let entity_key = symbol_short!("entity");
        if let Some(entity_data) = env.storage().instance().get::<soroban_sdk::Symbol, soroban_sdk::Val>(&entity_key) {
            if let Ok((id, x, y)) = <(u32, u32, u32)>::try_from_val(env, &entity_data) {
                if id == entity_id {
                    // Use the MovementSystem from soroban-ecs
                    let current_position = Position { x, y };
                    let new_position = MovementSystem::update(&current_position, dx, dy);
                    // Store updated position
                    let updated_entity_data: (u32, u32, u32) = (id, new_position.x, new_position.y);
                    let val: soroban_sdk::Val = updated_entity_data.into_val(env);
                    env.storage().instance().set(&entity_key, &val);
                    return true;
                }
            }
        }
        false
    }
    /// Get entity position
    pub fn get_entity_position(env: &Env, entity_id: u32) -> Option<Position> {
        let entity_key = symbol_short!("entity");
        if let Some(entity_data) = env.storage().instance().get::<soroban_sdk::Symbol, soroban_sdk::Val>(&entity_key) {
            if let Ok((id, x, y)) = <(u32, u32, u32)>::try_from_val(env, &entity_data) {
                if id == entity_id {
                    return Some(Position { x, y });
                }
            }
        }
        None
    }
    /// Get the total number of entities in the world
    pub fn entity_count(env: &Env) -> u32 {
        let contract_data = Self::get_contract_data(env);
        contract_data.entity_count
    }
    /// Remove an entity from the world
    pub fn despawn_entity(env: &Env, entity_id: u32) -> bool {
        let entity_key = symbol_short!("entity");
        // Check if entity exists
        if let Some(entity_data) = env.storage().instance().get::<soroban_sdk::Symbol, soroban_sdk::Val>(&entity_key) {
            if let Ok((id, _, _)) = <(u32, u32, u32)>::try_from_val(env, &entity_data) {
                if id == entity_id {
                    // Remove entity by clearing storage
                    env.storage().instance().remove(&entity_key);
                    // Update entity count
                    let mut contract_data = Self::get_contract_data(env);
                    if contract_data.entity_count > 0 {
                        contract_data.entity_count -= 1;
                        Self::save_contract_data(env, &contract_data);
                    }
                    return true;
                }
            }
        }
        false
    }
    /// Execute a simple game tick - move all entities by a small amount using MovementSystem
    pub fn game_tick(env: &Env) -> u32 {
        let contract_data = Self::get_contract_data(env);
        let mut moved_count = 0;
        if contract_data.entity_count > 0 {
            // Move the first entity (entity ID 0) by a small amount
            let entity_id = 0;
            let entity_key = symbol_short!("entity");
            if let Some(entity_data) = env.storage().instance().get::<soroban_sdk::Symbol, soroban_sdk::Val>(&entity_key) {
                if let Ok((id, x, y)) = <(u32, u32, u32)>::try_from_val(env, &entity_data) {
                    if id == entity_id {
                        // Use the MovementSystem from soroban-ecs
                        let current_position = Position { x, y };
                        let new_position = MovementSystem::update(&current_position, 1, 1);
                        let updated_entity_data: (u32, u32, u32) = (id, new_position.x, new_position.y);
                        let val: soroban_sdk::Val = updated_entity_data.into_val(env);
                        env.storage().instance().set(&entity_key, &val);
                        moved_count = 1;
                    }
                }
            }
        }
        moved_count
    }
    /// Demonstrate World usage by creating a new world
    pub fn create_world_demo(env: &Env) -> u32 {
        // This function demonstrates that we can create and use the World from soroban-ecs
        let world = World::new();
        // For now, we'll just return the entity count (which should be 0 for a new world)
        world.entity_count() as u32
    }
}
impl GameWorldContract {
    /// Get contract data from storage
    fn get_contract_data(env: &Env) -> GameWorldData {
        let key = symbol_short!("contract");
        if let Some(data) = env.storage().instance().get::<soroban_sdk::Symbol, soroban_sdk::Val>(&key) {
            GameWorldData::try_from_val(env, &data).unwrap_or_else(|_| GameWorldData {
                is_initialized: false,
                entity_count: 0,
                world_data: World::new(),
            })
        } else {
            // Initialize with empty world if no data exists
            GameWorldData {
                is_initialized: false,
                entity_count: 0,
                world_data: World::new(),
            }
        }
    }
    /// Save contract data to storage
    fn save_contract_data(env: &Env, data: &GameWorldData) {
        let key = symbol_short!("contract");
        let val: soroban_sdk::Val = data.into_val(env);
        env.storage().instance().set(&key, &val);
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::{Ledger, LedgerInfo};
    #[test]
    fn test_spawn_and_move_entity() {
        let env = Env::default();
        // Set up test environment
        env.ledger().set(LedgerInfo {
            timestamp: 12345,
            protocol_version: 20,
            sequence_number: 10,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 10,
            min_persistent_entry_ttl: 10,
            max_entry_ttl: 2000000,
        });
        // Initialize contract
        GameWorldContract::init(&env);
        // Spawn an entity at position (10, 20)
        let entity_id = GameWorldContract::spawn_entity(&env, 10, 20);
        assert_eq!(entity_id, 0);
        // Check initial position
        let position = GameWorldContract::get_entity_position(&env, entity_id).unwrap();
        assert_eq!(position.x, 10);
        assert_eq!(position.y, 20);
        // Move entity by (5, -3)
        let success = GameWorldContract::move_entity(&env, entity_id, 5, -3);
        assert!(success);
        // Check new position
        let new_position = GameWorldContract::get_entity_position(&env, entity_id).unwrap();
        assert_eq!(new_position.x, 15);
        assert_eq!(new_position.y, 17);
        // Check entity count
        assert_eq!(GameWorldContract::entity_count(&env), 1);
    }
    #[test]
    fn test_game_tick() {
        let env = Env::default();
        // Set up test environment
        env.ledger().set(LedgerInfo {
            timestamp: 12345,
            protocol_version: 20,
            sequence_number: 10,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 10,
            min_persistent_entry_ttl: 10,
            max_entry_ttl: 2000000,
        });
        // Initialize contract
        GameWorldContract::init(&env);
        // Spawn an entity
        let entity_id = GameWorldContract::spawn_entity(&env, 5, 5);
        // Execute game tick
        let moved_count = GameWorldContract::game_tick(&env);
        assert_eq!(moved_count, 1);
        // Check that position changed
        let position = GameWorldContract::get_entity_position(&env, entity_id).unwrap();
        assert_eq!(position.x, 6);
        assert_eq!(position.y, 6);
    }
    #[test]
    fn test_despawn_entity() {
        let env = Env::default();
        // Set up test environment
        env.ledger().set(LedgerInfo {
            timestamp: 12345,
            protocol_version: 20,
            sequence_number: 10,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 10,
            min_persistent_entry_ttl: 10,
            max_entry_ttl: 2000000,
        });
        // Initialize contract
        GameWorldContract::init(&env);
        // Spawn an entity
        let entity_id = GameWorldContract::spawn_entity(&env, 10, 10);
        assert_eq!(GameWorldContract::entity_count(&env), 1);
        // Despawn the entity
        let success = GameWorldContract::despawn_entity(&env, entity_id);
        assert!(success);
        assert_eq!(GameWorldContract::entity_count(&env), 0);
        // Try to get position of despawned entity
        let position = GameWorldContract::get_entity_position(&env, entity_id);
        assert!(position.is_none());
    }
    #[test]
    fn test_world_demo() {
        let env = Env::default();
        // Test that we can create a World from soroban-ecs
        let entity_count = GameWorldContract::create_world_demo(&env);
        assert_eq!(entity_count, 0); // New world should have 0 entities
    }
    #[test]
    fn test_movement_system() {
        // Test the MovementSystem directly without contract storage
        let position = Position { x: 10, y: 20 };
        let new_position = MovementSystem::update(&position, 5, -3);
        assert_eq!(new_position.x, 15);
        assert_eq!(new_position.y, 17);
    }
    #[test]
    fn test_position_component() {
        // Test Position component creation and access
        let position = Position { x: 100, y: 200 };
        assert_eq!(position.x, 100);
        assert_eq!(position.y, 200);
    }
}
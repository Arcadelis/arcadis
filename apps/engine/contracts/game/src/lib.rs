#![no_std]

extern crate alloc;

use soroban_sdk::{contract, contracttype, contractimpl, symbol_short, Env, Symbol, Bytes, vec, Val, IntoVal, TryFromVal};
use soroban_ecs::{World, EntityId, Component, ComponentTrait, System, SystemParam};
use soroban_ecs::prelude::*;

mod storage;
use storage::*;

// Position component for entities, storing x, y coordinates as u32
#[contracttype]
#[derive(Clone)]
pub struct GamePosition(pub u32, pub u32);

impl ComponentTrait for GamePosition {
    // Returns the component type identifier
    fn component_type() -> Symbol {
        symbol_short!("position")
    }

    // Serializes position (x, y) to Bytes for storage
    fn serialize(&self, env: &Env) -> Bytes {
        let mut bytes = Bytes::new(env);
        bytes.append(&Bytes::from_slice(env, &self.0.to_be_bytes()));
        bytes.append(&Bytes::from_slice(env, &self.1.to_be_bytes()));
        bytes
    }

    // Deserializes Bytes to GamePosition, expects 8 bytes (two u32s)
    fn deserialize(env: &Env, data: &Bytes) -> Option<Self> {
        if data.len() != 8 {
            return None;
        }
        let x = u32::from_be_bytes([
            data.get(0).unwrap(),
            data.get(1).unwrap(),
            data.get(2).unwrap(),
            data.get(3).unwrap(),
        ]);
        let y = u32::from_be_bytes([
            data.get(4).unwrap(),
            data.get(5).unwrap(),
            data.get(6).unwrap(),
            data.get(7).unwrap(),
        ]);
        Some(Self(x, y))
    }
}

// Health component for entities, storing health as u32
#[contracttype]
#[derive(Clone)]
pub struct Health(pub u32);

impl ComponentTrait for Health {
    fn component_type() -> Symbol {
        symbol_short!("health")
    }

    fn serialize(&self, env: &Env) -> Bytes {
        let mut bytes = Bytes::new(env);
        bytes.append(&Bytes::from_slice(env, &self.0.to_be_bytes()));
        bytes
    }

    fn deserialize(env: &Env, data: &Bytes) -> Option<Self> {
        if data.len() != 4 {
            return None;
        }
        let value = u32::from_be_bytes([
            data.get(0).unwrap(),
            data.get(1).unwrap(),
            data.get(2).unwrap(),
            data.get(3).unwrap(),
        ]);
        Some(Self(value))
    }
}

// System for updating entity positions
pub struct MovementSystem;

impl MovementSystem {
    // Updates position by adding dx, dy, ensuring non-negative coordinates
    pub fn update_position(pos: &GamePosition, dx: i32, dy: i32) -> GamePosition {
        GamePosition(
            (pos.0 as i32 + dx).max(0) as u32,
            (pos.1 as i32 + dy).max(0) as u32,
        )
    }
}

// System for updating entity health
pub struct CombatSystem;

impl CombatSystem {
    // Reduces health by 10, preventing underflow
    pub fn update_health(health: &Health) -> Health {
        Health(health.0.saturating_sub(10))
    }
}

// GameWorldContract defines the Soroban smart contract
#[contract]
pub struct GameWorldContract;

// Contract implementation with game logic
#[contractimpl]
impl GameWorldContract {
    /// Initializes the contract with optimized storage schema
    /// 
    /// Sets up the contract state using efficient storage patterns:
    /// - Instance storage for global state and counters
    /// - Persistent storage preparation for entity data
    /// - Proper TTL management for ledger efficiency
    pub fn init(env: &Env) -> GameWorldData {
        let data = GameWorldData {
            is_initialized: true,
            entity_count: 0,
            dead_entity: 0,
            world_data: World::new(),
        };
        
        // Use optimized storage functions
        storage::save_contract_data(env, &data);
        storage::set_entity_count(env, 0);
        storage::set_dead_entity_count(env, 0);
        
        data
    }

    /// Spawns a new entity with optimized storage and ECS integration
    /// 
    /// Uses efficient storage patterns:
    /// - Persistent storage for entity data with proper TTL
    /// - Atomic counter updates for entity management
    /// - Optimized data serialization for minimal storage footprint
    pub fn spawn_entity(env: &Env, x: u32, y: u32) -> u32 {
        let entity_count = storage::get_entity_count(env);
        let entity_id = entity_count;
        
        // Create entity components
        let position = GamePosition(x, y);
        let health = Health(100);
        
        // Store entity data as optimized tuple
        let entity_data: (u32, u32, u32, u32) = (entity_id, position.0, position.1, health.0);
        let val: Val = entity_data.into_val(env);
        
        // Use optimized storage functions
        storage::set_entity_data(env, entity_id, val);
        storage::set_entity_count(env, entity_count + 1);
        
        entity_id
    }

    

    // Moves an entity by dx, dy using MovementSystem
    pub fn move_entity(env: &Env, entity_id: u32, dx: i32, dy: i32) -> bool {
        if let Some(entity_data) = storage::get_entity_data(env, entity_id) { 
            if let Ok((id, x, y, health)) = <(u32, u32, u32, u32)>::try_from_val(env, &entity_data) {
                if id == entity_id { 
                    let current_position = GamePosition(x, y); 
                    let new_position = MovementSystem::update_position(&current_position, dx, dy); 
                    let updated_entity_data: (u32, u32, u32, u32) = (id, new_position.0, new_position.1, health); 
                    let val: soroban_sdk::Val = updated_entity_data.into_val(env);
                    storage::set_entity_data(env, entity_id, val); 
                    return true;
                }
            }
        }
        false 
    }

    // Attacks an entity, reducing its health or marking it as dead
    pub fn attack_entity(env: &Env, entity_id: u32) -> bool {
        if let Some(entity_data) = storage::get_entity_data(env, entity_id) {
            if let Ok((id, x, y, health)) = <(u32, u32, u32, u32)>::try_from_val(env, &entity_data) {
                if id == entity_id {
                    let current_health = Health(health);
                    let new_health = CombatSystem::update_health(&current_health).0;

                    if new_health > 0 {
                        // Just update the entity’s health
                        let updated: (u32, u32, u32, u32) = (id, x, y, new_health);
                        let val: Val = updated.into_val(env);
                        storage::set_entity_data(env, entity_id, val);
                    } else {
                        // Entity dies - increment dead count and decrement live count
                        let current_dead = storage::get_dead_entity_count(env);
                        storage::set_dead_entity_count(env, current_dead + 1);
                        
                        let current_live = storage::get_entity_count(env);
                        if current_live > 0 {
                            storage::set_entity_count(env, current_live - 1);
                        }

                        storage::remove_entity_data(env, entity_id);
                    }
                    return true;
                }
            }
        }
        false
    }

    // Retrieves the position of an entity
    pub fn get_entity_position(env: &Env, entity_id: u32) -> Option<GamePosition> {
        if let Some(entity_data) = storage::get_entity_data(env, entity_id) { 
            if let Ok((id, x, y, _)) = <(u32, u32, u32, u32)>::try_from_val(env, &entity_data) {
                if id == entity_id { 
                    return Some(GamePosition(x, y)); 
                }
            }
        }
        None
    }

    // Retrieves the health of an entity
    pub fn get_entity_health(env: &Env, entity_id: u32) -> Option<Health> {
        if let Some(entity_data) = storage::get_entity_data(env, entity_id) { 
            if let Ok((id, x, y, health)) = <(u32, u32, u32, u32)>::try_from_val(env, &entity_data) {
                if id == entity_id { 
                    return Some(Health(health)); 
                }
            }
        }
        None 
    }

    // Returns the total number of entities in the world
    pub fn entity_count(env: &Env) -> u32 {
        storage::get_entity_count(env)
    }

    // Returns the total number of dead entities
    pub fn dead_entity_count(env: &Env) -> u32 {
        storage::get_dead_entity_count(env)
    }

    // Removes an entity from the world
    pub fn despawn_entity(env: &Env, entity_id: u32) -> bool {
        if let Some(entity_data) = storage::get_entity_data(env, entity_id) { 
            if let Ok((id, _, _, _)) = <(u32, u32, u32, u32)>::try_from_val(env, &entity_data) {
                if id == entity_id {
                    storage::remove_entity_data(env, entity_id);
                    let current_count = storage::get_entity_count(env);
                    storage::set_entity_count(env, current_count.saturating_sub(1));
                    return true;
                }
            }
        }
        false
    }
}
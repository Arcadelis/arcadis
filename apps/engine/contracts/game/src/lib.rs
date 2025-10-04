#![no_std]

extern crate alloc;

use soroban_sdk::{contract, contracttype, contractimpl, symbol_short, Env, Symbol, Bytes, vec, Val, IntoVal, TryFromVal};
use soroban_ecs::{World, EntityId, Component, ComponentTrait, System, SystemParam};
use soroban_ecs::prelude::*;

mod storage;
mod components;
mod systems;

use storage::*;
pub use components::{Position, Health};
pub use systems::{MovementSystem, CombatSystem};

// Re-export Position as GamePosition for backward compatibility
// This allows existing code to use GamePosition(x, y) syntax
pub use components::Position as GamePosition;

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
        
        // Create entity components using the modular components
        let position = Position(x, y);
        let health = Health(100);
        
        // Store entity data as optimized tuple
        let entity_data: (u32, u32, u32, u32) = (entity_id, position.0, position.1, health.0);
        let val: Val = entity_data.into_val(env);
        
        // Use optimized storage functions
        storage::set_entity_data(env, entity_id, val);
        storage::set_entity_count(env, entity_count + 1);
        
        entity_id
    }

    

    /// Moves an entity by dx, dy using MovementSystem
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `entity_id` - The ID of the entity to move
    /// * `dx` - Change in x-coordinate (can be negative)
    /// * `dy` - Change in y-coordinate (can be negative)
    ///
    /// # Returns
    ///
    /// `true` if the entity was successfully moved, `false` if entity not found
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Move entity 5 units right and 3 units up
    /// client.move_entity(&entity_id, 5, 3);
    /// ```
    pub fn move_entity(env: &Env, entity_id: u32, dx: i32, dy: i32) -> bool {
        if let Some(entity_data) = storage::get_entity_data(env, entity_id) { 
            if let Ok((id, x, y, health)) = <(u32, u32, u32, u32)>::try_from_val(env, &entity_data) {
                if id == entity_id { 
                    let current_position = Position(x, y); 
                    // Use the MovementSystem to calculate new position
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

    /// Attacks an entity, reducing its health using CombatSystem
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `entity_id` - The ID of the entity to attack
    ///
    /// # Returns
    ///
    /// `true` if the attack was successful, `false` if entity not found
    ///
    /// # Behavior
    ///
    /// - Applies 10 damage to the entity's health
    /// - If health reaches 0, the entity is marked as dead and removed
    /// - Dead entity counter is incremented when an entity dies
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Attack an entity
    /// client.attack_entity(&entity_id);
    /// ```
    pub fn attack_entity(env: &Env, entity_id: u32) -> bool {
        if let Some(entity_data) = storage::get_entity_data(env, entity_id) {
            if let Ok((id, x, y, health)) = <(u32, u32, u32, u32)>::try_from_val(env, &entity_data) {
                if id == entity_id {
                    let current_health = Health(health);
                    // Use the CombatSystem to apply attack damage
                    let new_health = CombatSystem::attack(&current_health);

                    if new_health.0 > 0 {
                        // Just update the entity's health
                        let updated: (u32, u32, u32, u32) = (id, x, y, new_health.0);
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
    pub fn get_entity_position(env: &Env, entity_id: u32) -> Option<Position> {
        if let Some(entity_data) = storage::get_entity_data(env, entity_id) { 
            if let Ok((id, x, y, _)) = <(u32, u32, u32, u32)>::try_from_val(env, &entity_data) {
                if id == entity_id { 
                    return Some(Position(x, y)); 
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
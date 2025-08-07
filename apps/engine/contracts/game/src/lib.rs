

#![no_std]

extern crate alloc;

use soroban_sdk::{contract, contracttype, contractimpl, symbol_short, Env, Symbol, Bytes, vec, Val, IntoVal, TryFromVal,};
use soroban_ecs::{World, EntityId};
use soroban_ecs::{Component, ComponentTrait};
use soroban_ecs::{System, SystemParam};
use soroban_ecs::prelude::*;

mod storage;

use storage::GameWorldData;

#[contracttype]
#[derive(Clone)]
pub struct GamePosition(pub u32, pub u32);

impl ComponentTrait for GamePosition {
    fn component_type() -> Symbol {
        symbol_short!("position")
    }

    fn serialize(&self, env: &Env) -> Bytes {
        let mut bytes = Bytes::new(env);
        bytes.append(&Bytes::from_slice(env, &self.0.to_be_bytes()));
        bytes.append(&Bytes::from_slice(env, &self.1.to_be_bytes()));
        bytes
    }

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
        Some(Self(value.try_into().unwrap()))
    }
}

pub struct MovementSystem;

impl MovementSystem {
    pub fn update_position(pos: &GamePosition, dx: i32, dy: i32) -> GamePosition {
        GamePosition (
           (pos.0 as i32 + dx).max(0) as u32,
           (pos.1 as i32 + dy).max(0) as u32
        )
    }
} 

pub struct CombatSystem;

impl CombatSystem {
    pub fn update_health(health: &Health) -> Health {
        Health (
            (health.0.saturating_sub(10)) as u32
        )
    }
} 

// GameWorldContract
#[contract]
pub struct GameWorldContract;

// /// Contract data structure to store the game world state
// #[derive(Clone, Debug)]
// pub struct GameWorldData {
//     /// Flag indicating if the contract has been initialized
//     pub is_initialized: bool,
//     /// Number of entities in the world
//     pub entity_count: u32,
//     // Number of dead entities in the world
//     pub dead_entity: u32,
//     /// World data
//     pub world_data: World,
// }
// // Manual implementation of Soroban traits for GameWorldData
// impl IntoVal<Env, soroban_sdk::Val> for GameWorldData {
//     fn into_val(&self, env: &Env) -> soroban_sdk::Val {
//         let data = (self.is_initialized, self.entity_count, self.dead_entity);
//         data.into_val(env)
//     }
// }
// impl TryFromVal<Env, soroban_sdk::Val> for GameWorldData {
//     type Error = soroban_sdk::ConversionError;
//     fn try_from_val(env: &Env, val: &soroban_sdk::Val) -> Result<Self, Self::Error> {
//         let (is_initialized, entity_count, dead_entity): (bool, u32, u32) = TryFromVal::try_from_val(env, val)?;
//         Ok(GameWorldData {
//             is_initialized,
//             entity_count,
//             dead_entity,
//             world_data: World::new(),
//         })
//     }
// }

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
            dead_entity:0,
            world_data: World::new(),
        };
        // Self::save_contract_data(env, &data);
        storage::save_contract_data(env, &data);
        data
    }
    /// Create a new entity with a position component
    pub fn spawn_entity(env: &Env, x: u32, y: u32) -> u32 {
        // let mut contract_data = Self::get_contract_data(env);
        let mut contract_data = storage::get_contract_data(env);
        // Create a new ECS world
        let _world = World::new();
        // Create a position component using the ECS system
        let position = GamePosition(x, y);
        let entity_id = contract_data.entity_count;
        // Store entity position in contract storage
        let entity_key = symbol_short!("entity");
        let health = Health(100);
        let entity_data: (u32, u32, u32, u32) = (entity_id, position.0, position.1, health.0.try_into().unwrap());
        let val: soroban_sdk::Val = entity_data.into_val(env);
        // env.storage().instance().set(&entity_key, &val);
        storage::set_entity_data(env, val);
        // Update entity count
        contract_data.entity_count += 1;
        // Self::save_contract_data(env, &contract_data);
        storage::save_contract_data(env, &contract_data);
        entity_id
    }

    // Move an entity by the given delta using the MovementSystem
    pub fn move_entity(env: &Env, entity_id: u32, dx: i32, dy: i32) -> bool {
        let entity_key = symbol_short!("entity");
        // if let Some(entity_data) = env.storage().instance().get::<soroban_sdk::Symbol, soroban_sdk::Val>(&entity_key) {
        if let Some(entity_data) = storage::get_entity_data(&env) {
        
            if let Ok((id, x, y, health)) = <(u32, u32, u32, u32)>::try_from_val(env, &entity_data) {
                if id == entity_id {
                    let current_position = GamePosition ( x, y );
                    let new_position = MovementSystem::update_position(&current_position, dx, dy);
                    // Store updated position
                    let updated_entity_data: (u32, u32, u32, u32) = (id, new_position.0, new_position.1, health);
                    let val: soroban_sdk::Val = updated_entity_data.into_val(env);
                    // env.storage().instance().set(&entity_key, &val);
                    storage::set_entity_data(env, val);

                    return true;
                }
            }
        }
        false
    }

    /// Attack entity
    pub fn attack_entity(env: &Env, entity_id: u32) -> bool {
        // let entity_key = symbol_short!("entity");
        // if let Some(entity_data) = env.storage().instance().get::<soroban_sdk::Symbol, soroban_sdk::Val>(&entity_key) {
        if let Some(entity_data) = storage::get_entity_data(&env) {
            if let Ok((id, x, y, health)) = <(u32, u32, u32, u32)>::try_from_val(env, &entity_data) {
                if id == entity_id {
                    let current_heath = Health(health.try_into().unwrap());
                    let new_health = CombatSystem::update_health(&current_heath).0;
                    if new_health > 0 {
                        let updated_entity_data: (u32, u32, u32, u32) = (id, x, y, new_health.try_into().unwrap());
                        let val: soroban_sdk::Val = updated_entity_data.into_val(env);
                        // env.storage().instance().set(&entity_key, &val);
                        storage::set_entity_data(env, val);
                        return true;
                    } else {
                        // let mut contract_data = Self::get_contract_data(env);
                        let mut contract_data = storage::get_contract_data(env);
                        // Self::despawn_entity(env, entity_id);
                        contract_data.dead_entity += 1;
                        // Self::save_contract_data(env, &contract_data);
                        storage::save_contract_data(env, &contract_data);
                        return true;
                    }
                }
            }
        }
        false
    }

    // Get entity position
    pub fn get_entity_position(env: &Env, entity_id: u32) -> Option<GamePosition> {
        // let entity_key = symbol_short!("entity");
        // if let Some(entity_data) = env.storage().instance().get::<soroban_sdk::Symbol, soroban_sdk::Val>(&entity_key) {
        if let Some(entity_data) = storage::get_entity_data(&env) {
            if let Ok((id, x, y, _)) = <(u32, u32, u32, u32)>::try_from_val(env, &entity_data) {
                if id == entity_id {
                    return Some(GamePosition ( x, y ));
                }
            }
        }
        None
    }

    pub fn get_entity_health(env: &Env, entity_id: u32) -> Option<Health> {
        // let entity_key = symbol_short!("entity");
        // if let Some(entity_data) = env.storage().instance().get::<soroban_sdk::Symbol, soroban_sdk::Val>(&entity_key) {
        if let Some(entity_data) = storage::get_entity_data(&env) {
            if let Ok((id, x, y, health)) = <(u32, u32, u32, u32)>::try_from_val(env, &entity_data) {
                if id == entity_id {
                    return Some(Health ( health.try_into().unwrap() ));
                }
            }
        }
        None
    }
    /// Get the total number of entities in the world
    pub fn entity_count(env: &Env) -> u32 {
        // let contract_data = Self::get_contract_data(env);
        let contract_data = storage::get_contract_data(env);
        contract_data.entity_count
    }

     /// Get the total number of dead entities in the world
    pub fn dead_entity_count(env: &Env) -> u32 {
        // let contract_data = Self::get_contract_data(env);
        let contract_data = storage::get_contract_data(env);
        contract_data.dead_entity
    }

    // /// Remove an entity from the world
    pub fn despawn_entity(env: &Env, entity_id: u32) -> bool {
        // let entity_key = symbol_short!("entity");
        // Check if entity exists
        // if let Some(entity_data) = env.storage().instance().get::<soroban_sdk::Symbol, soroban_sdk::Val>(&entity_key) {
        if let Some(entity_data) = storage::get_entity_data(&env) {
            if let Ok((id, _, _, _)) = <(u32, u32, u32, u32)>::try_from_val(env, &entity_data) {
                if id == entity_id {
                    // Remove entity by clearing storage
                    // env.storage().instance().remove(&entity_key);
                    storage::remove_entity_data(&env);
                    // Update entity count
                    // let mut contract_data = Self::get_contract_data(env);
                    let mut contract_data = storage::get_contract_data(env);
                    if contract_data.entity_count > 0 {
                        contract_data.entity_count -= 1;
                        // Self::save_contract_data(env, &contract_data);
                        storage::save_contract_data(env, &contract_data);
                    }
                    return true;
                }
            }
        }
        false
    }
}

// impl GameWorldContract {
//     /// Get contract data from storage
//     fn get_contract_data(env: &Env) -> GameWorldData {
//         let key = symbol_short!("contract");
//         if let Some(data) = env.storage().instance().get::<soroban_sdk::Symbol, soroban_sdk::Val>(&key) {
//             GameWorldData::try_from_val(env, &data).unwrap_or_else(|_| GameWorldData {
//                 is_initialized: false,
//                 entity_count: 0,
//                 dead_entity: 0,
//                 world_data: World::new(),
//             })
//         } else {
//             // Initialize with empty world if no data exists
//             GameWorldData {
//                 is_initialized: false,
//                 entity_count: 0,
//                 dead_entity: 0,
//                 world_data: World::new(),
//             }
//         }
//     }
//     /// Save contract data to storage
//     fn save_contract_data(env: &Env, data: &GameWorldData) {
//         let key = symbol_short!("contract");
//         let val: soroban_sdk::Val = data.into_val(env);
//         env.storage().instance().set(&key, &val);
//     }
// }

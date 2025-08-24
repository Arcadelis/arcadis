use soroban_sdk::{contract, contracttype, contractimpl, symbol_short, Env, Symbol, Bytes, vec, Val, IntoVal, TryFromVal,};
use soroban_ecs::{World, EntityId};
use soroban_sdk::Map;

use crate::GamePosition;

/// Contract data structure to store the game world state
#[derive(Clone, Debug)]
pub struct GameWorldData {
    /// Flag indicating if the contract has been initialized
    pub is_initialized: bool,
    /// Number of entities in the world
    pub entity_count: u32,
    // Number of dead entities in the world
    pub dead_entity: u32,
    /// World data
    pub world_data: World,
}
// Manual implementation of Soroban traits for GameWorldData
impl IntoVal<Env, soroban_sdk::Val> for GameWorldData {
    fn into_val(&self, env: &Env) -> soroban_sdk::Val {
        let data = (self.is_initialized, self.entity_count, self.dead_entity);
        data.into_val(env)
    }
}
impl TryFromVal<Env, soroban_sdk::Val> for GameWorldData {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(env: &Env, val: &soroban_sdk::Val) -> Result<Self, Self::Error> {
        let (is_initialized, entity_count, dead_entity): (bool, u32, u32) = TryFromVal::try_from_val(env, val)?;
        Ok(GameWorldData {
            is_initialized,
            entity_count,
            dead_entity,
            world_data: World::new(),
        })
    }
}

pub fn get_contract_data(env: &Env) -> GameWorldData {
    let key = symbol_short!("contract");
    if let Some(data) = env.storage().instance().get::<soroban_sdk::Symbol, soroban_sdk::Val>(&key) {
        GameWorldData::try_from_val(env, &data).unwrap_or_else(|_| GameWorldData {
            is_initialized: false,
            entity_count: 0,
            dead_entity: 0,
            world_data: World::new(),
        })
    } else {
        GameWorldData {
            is_initialized: false,
            entity_count: 0,
            dead_entity: 0,
            world_data: World::new(),
        }
    }
}

pub fn save_contract_data(env: &Env, data: &GameWorldData) {
    let key = symbol_short!("contract");
    let val: Val = data.into_val(env);
    env.storage().instance().set(&key, &val);
}

// Retrieves entity data (id, x, y, health) from storage
pub fn get_entity_data(env: &Env, entity_id: u32) -> Option<Val> {
    let entity_key = symbol_short!("entity"); 
    if let Some(map) = env.storage().instance().get::<Symbol, Map<u32, Val>>(&entity_key) {
        map.get(entity_id)
    } else {
        None
    }
}

// Stores entity data (id, x, y, health) in storage
pub fn set_entity_data(env: &Env, entity_id: u32, val: Val) {
    let entity_key = symbol_short!("entity"); 
    let mut map = env.storage().instance()
        .get::<Symbol, Map<u32, Val>>(&entity_key)
        .unwrap_or_else(|| Map::new(env)); 
    map.set(entity_id, val); 
    env.storage().instance().set::<Symbol, Map<u32, Val>>(&entity_key, &map); 
}

// Removes entity data from storage
pub fn remove_entity_data(env: &Env, entity_id: u32) {
    let entity_key = symbol_short!("entity"); 
    if let Some(mut map) = env.storage().instance()
        .get::<Symbol, Map<u32, Val>>(&entity_key) {
        map.remove(entity_id);
        env.storage().instance().set::<Symbol, Map<u32, Val>>(&entity_key, &map); 
    }
}

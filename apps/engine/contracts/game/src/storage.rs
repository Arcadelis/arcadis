use soroban_sdk::{symbol_short, Env, Symbol, Val, IntoVal, TryFromVal, Map, Vec};
use soroban_ecs::World;

/// Optimized contract data structure for efficient ledger storage
/// 
/// This schema uses different storage types for optimal performance:
/// - Instance storage: For persistent global state like entity counters
/// - Persistent storage: For entity data that needs long-term persistence
/// - Temporary storage: For session-based data (not used in this contract)
#[derive(Clone, Debug)]
pub struct GameWorldData {
    /// Flag indicating if the contract has been initialized
    pub is_initialized: bool,
    /// Number of living entities in the world
    pub entity_count: u32,
    /// Number of dead entities (for analytics)
    pub dead_entity: u32,
    /// World data structure (not persisted to ledger, recreated as needed)
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

/// Retrieves contract global state from instance storage
/// 
/// Instance storage is ideal for global contract metadata as it:
/// - Persists indefinitely without archival concerns
/// - Has lower read/write costs for frequently accessed data
/// - Is automatically tied to the contract instance
pub fn get_contract_data(env: &Env) -> GameWorldData {
    let key = symbol_short!("contract");
    if let Some(data) = env.storage().instance().get::<Symbol, Val>(&key) {
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

/// Saves contract global state to instance storage
pub fn save_contract_data(env: &Env, data: &GameWorldData) {
    let key = symbol_short!("contract");
    let val: Val = data.into_val(env);
    env.storage().instance().set(&key, &val);
}

/// Retrieves entity data from persistent storage using optimized key structure
/// 
/// Uses persistent storage for entity data because:
/// - Entity data should persist across contract invocations
/// - Allows for efficient querying and batch operations
/// - Supports state archival for long-term storage efficiency
/// 
/// Key structure: Individual keys per entity for efficient access
pub fn get_entity_data(env: &Env, entity_id: u32) -> Option<Val> {
    // Use a map in persistent storage for entity data
    let entity_key = symbol_short!("entities");
    if let Some(map) = env.storage().persistent().get::<Symbol, Map<u32, Val>>(&entity_key) {
        map.get(entity_id)
    } else {
        None
    }
}

/// Stores entity data in persistent storage with optimized key structure
/// 
/// The entity data is stored as a tuple (id, x, y, health) for efficient
/// serialization and deserialization. This approach minimizes storage overhead
/// while maintaining type safety.
pub fn set_entity_data(env: &Env, entity_id: u32, val: Val) {
    let entity_key = symbol_short!("entities");
    let mut map = env.storage().persistent()
        .get::<Symbol, Map<u32, Val>>(&entity_key)
        .unwrap_or_else(|| Map::new(env));
    
    map.set(entity_id, val);
    env.storage().persistent().set(&entity_key, &map);
    
    // Set TTL for persistent storage to optimize ledger efficiency
    // This prevents indefinite storage growth while maintaining reasonable persistence
    let ledger_seq = env.ledger().sequence();
    let ttl_ledgers = 518400; // ~30 days (assuming 5s per ledger)
    env.storage().persistent().extend_ttl(&entity_key, ledger_seq, ttl_ledgers);
}

/// Removes entity data from persistent storage
/// 
/// Properly cleans up storage to prevent ledger bloat and optimize costs
pub fn remove_entity_data(env: &Env, entity_id: u32) {
    let entity_key = symbol_short!("entities");
    if let Some(mut map) = env.storage().persistent().get::<Symbol, Map<u32, Val>>(&entity_key) {
        map.remove(entity_id);
        env.storage().persistent().set(&entity_key, &map);
    }
}

/// Retrieves all entity IDs for batch operations
/// 
/// This function enables efficient batch processing by providing a way to
/// enumerate all existing entities without loading their full data.
pub fn get_all_entity_ids(env: &Env) -> Vec<u32> {
    let entity_key = symbol_short!("entities");
    if let Some(map) = env.storage().persistent().get::<Symbol, Map<u32, Val>>(&entity_key) {
        map.keys()
    } else {
        Vec::new(env)
    }
}

/// Batch entity data retrieval for efficient mass operations
/// 
/// Retrieves multiple entity data in a single operation to reduce
/// the number of storage calls and improve performance for bulk operations.
pub fn get_entities_batch(env: &Env, entity_ids: &Vec<u32>) -> Map<u32, Val> {
    let mut result = Map::new(env);
    let entity_key = symbol_short!("entities");
    
    if let Some(map) = env.storage().persistent().get::<Symbol, Map<u32, Val>>(&entity_key) {
        for i in 0..entity_ids.len() {
            if let Some(entity_id) = entity_ids.get(i) {
                if let Some(entity_data) = map.get(entity_id) {
                    result.set(entity_id, entity_data);
                }
            }
        }
    }
    
    result
}

/// Batch entity data storage for efficient mass operations
/// 
/// Stores multiple entity data in a optimized manner to reduce
/// the number of storage operations and transaction costs.
pub fn set_entities_batch(env: &Env, entities: &Map<u32, Val>) {
    let entity_key = symbol_short!("entities");
    let mut map = env.storage().persistent()
        .get::<Symbol, Map<u32, Val>>(&entity_key)
        .unwrap_or_else(|| Map::new(env));
    
    let keys = entities.keys();
    for i in 0..keys.len() {
        if let Some(entity_id) = keys.get(i) {
            if let Some(entity_data) = entities.get(entity_id) {
                map.set(entity_id, entity_data);
            }
        }
    }
    
    env.storage().persistent().set(&entity_key, &map);
    
    // Set TTL for the batch operation
    let ledger_seq = env.ledger().sequence();
    let ttl_ledgers = 518400;
    env.storage().persistent().extend_ttl(&entity_key, ledger_seq, ttl_ledgers);
}

/// Get entity count efficiently without loading full contract data
/// 
/// Uses a separate storage entry for frequently accessed counters
/// to avoid deserializing the entire contract state.
pub fn get_entity_count(env: &Env) -> u32 {
    let key = symbol_short!("ent_cnt");
    env.storage().instance().get(&key).unwrap_or(0)
}

/// Set entity count efficiently
pub fn set_entity_count(env: &Env, count: u32) {
    let key = symbol_short!("ent_cnt");
    env.storage().instance().set(&key, &count);
}

/// Get dead entity count efficiently
pub fn get_dead_entity_count(env: &Env) -> u32 {
    let key = symbol_short!("dead_cnt");
    env.storage().instance().get(&key).unwrap_or(0)
}

/// Set dead entity count efficiently
pub fn set_dead_entity_count(env: &Env, count: u32) {
    let key = symbol_short!("dead_cnt");
    env.storage().instance().set(&key, &count);
}

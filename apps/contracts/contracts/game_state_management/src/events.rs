//! src/events.rs

use soroban_sdk::{contracttype, Address, Bytes, BytesN, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct GameEvent {
    pub event_id: String,
    pub player: Address,
    pub event_type: Symbol,
    pub metadata: BytesN<64>,
    pub timestamp: u64,
}

pub fn log_event(env: &Env, player: Address, event_type: Symbol, metadata: BytesN<64>) -> String {
    player.require_auth();
    let timestamp = env.ledger().timestamp();
    let event_key: Symbol = Symbol::new(&env, "EVENT");

    // Create a simple event ID using timestamp
    let event_id = String::from_str(env, "event");

    let event = GameEvent {
        event_id: event_id.clone(),
        player,
        event_type,
        metadata,
        timestamp,
    };

    let mut events: Vec<GameEvent> = env
        .storage()
        .persistent()
        .get(&event_key)
        .unwrap_or(Vec::new(env));
    events.push_back(event.clone());
    env.storage().persistent().set(&event_key, &events);

    event_id
}

pub fn get_event_log(
    env: &Env,
    player: Option<Address>,
    region_id: Option<String>,
) -> Vec<GameEvent> {
    let event_key: Symbol = Symbol::new(&env, "EVENT");
    let events: Vec<GameEvent> = env
        .storage()
        .persistent()
        .get(&event_key)
        .unwrap_or(Vec::new(env));

    let mut filtered = Vec::new(env);
    for e in events.iter() {
        let player_match = player.as_ref().map_or(true, |p| e.player == *p);
        let region_match = region_id.as_ref().map_or(true, |_r| {
            // Simplified region matching - you can enhance this based on your needs
            true
        });
        if player_match && region_match {
            filtered.push_back(e.clone());
        }
    }

    filtered
}

// Helper: check byte containment using only Soroban types
fn contains_bytes(haystack: &Bytes, needle: &Bytes) -> bool {
    let haystack_len = haystack.len();
    let needle_len = needle.len();
    if needle_len == 0 || haystack_len < needle_len {
        return false;
    }
    for i in 0..=haystack_len - needle_len {
        let mut found = true;
        for j in 0..needle_len {
            if haystack.get(i + j) != needle.get(j) {
                found = false;
                break;
            }
        }
        if found {
            return true;
        }
    }
    false
}

use soroban_sdk::{contracttype, Address, Bytes, BytesN, Env, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct SavePoint {
    pub player: Address,
    pub data_hash: BytesN<32>,
    pub timestamp: u64,
}



pub fn save_progress(env: &Env, player: Address, data: BytesN<64>) -> BytesN<32> {
    player.require_auth();

    let data_bytes: Bytes = data.into();
    let data_hash = env.crypto().sha256(&data_bytes);
    let timestamp = env.ledger().timestamp();
    let save_key: soroban_sdk::Symbol = Symbol::new(&env, "SAVES");

    let save_point = SavePoint {
        player: player.clone(),
        data_hash: data_hash.clone().into(),
        timestamp,
    };

    let mut saves: Vec<SavePoint> = env
        .storage()
        .persistent()
        .get(&save_key)
        .unwrap_or(Vec::new(&env));
    saves.push_back(save_point);
    env.storage().persistent().set(&save_key, &saves);

    data_hash.into()
}

pub fn load_progress(env: &Env, player: Address, timestamp: Option<u64>) -> Option<SavePoint> {
    let save_key: soroban_sdk::Symbol = Symbol::new(&env, "SAVES");
    let saves: Vec<SavePoint> = env
        .storage()
        .persistent()
        .get(&save_key)
        .unwrap_or(Vec::new(&env));

    if let Some(ts) = timestamp {
        saves
            .iter()
            .find(|s| s.player == player && s.timestamp == ts)
            .map(|s| s.clone())
    } else {
        saves
            .iter()
            .filter(|s| s.player == player)
            .max_by_key(|s| s.timestamp)
            .map(|s| s.clone())
    }
}

use soroban_sdk::{Symbol, Bytes, Env, contracttype, symbol_short};

#[contracttype]
#[derive(Debug, Clone)]
pub struct Resource {
    pub resource_type: Symbol,
    pub data: Bytes,
}
impl Resource {
    pub fn new(resource_type: Symbol, data: Bytes) -> Self {
        Self { resource_type, data }
    }
    pub fn resource_type(&self) -> &Symbol {
        &self.resource_type
    }
    pub fn data(&self) -> &Bytes {
        &self.data
    }
    pub fn data_mut(&mut self) -> &mut Bytes {
        &mut self.data
    }
}

pub trait ResourceTrait: Send + Sync + 'static {
    fn resource_type() -> Symbol;
    fn serialize(&self, env: &Env) -> Bytes;
    fn deserialize(env: &Env, data: &Bytes) -> Option<Self>
    where
        Self: Sized;
}

#[contracttype]
#[derive(Clone)]
pub struct GameState {
    pub score: i32,
    pub level: i32,
    pub is_game_over: bool,
}
impl GameState {
    pub fn new() -> Self {
        Self { score: 0, level: 1, is_game_over: false }
    }
    pub fn increment_score(&mut self, points: i32) {
        self.score += points;
    }
    pub fn next_level(&mut self) {
        self.level += 1;
    }
    pub fn game_over(&mut self) {
        self.is_game_over = true;
    }
}
impl ResourceTrait for GameState {
    fn resource_type() -> Symbol {
        symbol_short!("gamestate")
    }
    fn serialize(&self, env: &Env) -> Bytes {
        let mut bytes = Bytes::new(env);
        let score_bytes = Bytes::from_array(env, &self.score.to_be_bytes());
        let level_bytes = Bytes::from_array(env, &self.level.to_be_bytes());
        let game_over_bytes = Bytes::from_array(env, &[self.is_game_over as u8]);
        bytes.append(&score_bytes);
        bytes.append(&level_bytes);
        bytes.append(&game_over_bytes);
        bytes
    }
    fn deserialize(env: &Env, data: &Bytes) -> Option<Self> {
        if data.len() != 9 {
            return None;
        }
        let score = i32::from_be_bytes([data.get(0).unwrap(), data.get(1).unwrap(), data.get(2).unwrap(), data.get(3).unwrap()]);
        let level = i32::from_be_bytes([data.get(4).unwrap(), data.get(5).unwrap(), data.get(6).unwrap(), data.get(7).unwrap()]);
        let is_game_over = data.get(8).unwrap() != 0;
        Some(Self { score, level, is_game_over })
    }
}
impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_resource_creation() {
        let env = Env::default();
        let resource_type = symbol_short!("test_resource");
        let data = vec![env, 1, 2, 3, 4];
        let resource = Resource::new(resource_type, data.clone());
        
        assert_eq!(resource.resource_type(), &symbol_short!("test_resource"));
        assert_eq!(resource.data(), &data);
    }

    #[test]
    fn test_game_state_serialization() {
        let mut game_state = GameState::new();
        game_state.increment_score(100);
        game_state.next_level();
        
        let data = game_state.serialize();
        let deserialized = GameState::deserialize(&data).unwrap();
        
        assert_eq!(game_state.score, deserialized.score);
        assert_eq!(game_state.level, deserialized.level);
        assert_eq!(game_state.is_game_over, deserialized.is_game_over);
    }
} 
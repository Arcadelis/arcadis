use soroban_sdk::{contracttype, Address, String, Vec, BytesN};

// Type aliases for better code readability
pub type GameId = String;
pub type TournamentId = String;
pub type Signature = BytesN<64>;

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct PlayerScore {
    pub score: u64,
    pub timestamp: u64,
    pub game_id: String,
    pub tournament_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[contracttype]
pub struct LeaderboardEntry {
    pub player_id: Address,
    pub score: u64,
    pub rank: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Tournament {
    pub id: String,
    pub game_id: String,
    pub start_time: u64,
    pub end_time: u64,
    pub max_entries: u32,
    pub entries: Vec<LeaderboardEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct PlayerHistory {
    pub player_id: Address,
    pub scores: Vec<PlayerScore>,
    pub total_games: u32,
    pub best_score: u64,
}

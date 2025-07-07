use soroban_sdk::{contracttype, Address, Env, String, Symbol, Vec};
use crate::errors::Error;
use crate::types::{LeaderboardEntry, PlayerScore, Tournament};
use crate::leaderboard;

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Tournament(String),
    PlayerScores(Address),
}

/// Submit a score to a tournament
pub fn submit_score(
    env: &Env,
    tournament_id: String,
    player_id: Address,
    score: u64,
) -> Result<u32, Error> {
    player_id.require_auth();
    
    let mut tournament = get_tournament(env, tournament_id.clone())?;
    
    // Check tournament timing
    let current_time = env.ledger().timestamp();
    if current_time < tournament.start_time || current_time > tournament.end_time {
        return Err(Error::TournamentNotActive);
    }
    
    // Find existing entry or create new one
    let mut found_index: Option<u32> = None;
    for i in 0..tournament.entries.len() {
        if let Some(entry) = tournament.entries.get(i) {
            if entry.player_id == player_id {
                found_index = Some(i);
                break;
            }
        }
    }
    
    let rank = if let Some(index) = found_index {
        // Update existing entry if new score is better
        let mut entry = tournament.entries.get(index).unwrap();
        if score > entry.score {
            entry.score = score;
            tournament.entries.set(index, entry);
        }
        index + 1 // Current rank (will be updated after sorting)
    } else {
        // Add new entry
        if tournament.entries.len() >= tournament.max_entries {
            return Err(Error::TournamentFull);
        }
        
        let new_entry = LeaderboardEntry {
            player_id: player_id.clone(),
            score,
            rank: 1,
        };
        tournament.entries.push_back(new_entry);
        1
    };
    
    // Sort entries by score (descending) and update ranks
    leaderboard::sort_entries(env, &mut tournament.entries);
    
    // Save updated tournament
    save_tournament(env, tournament_id.clone(), &tournament);
    
    // Save player score to history
    let player_score = PlayerScore {
        score,
        timestamp: current_time,
        game_id: tournament.game_id.clone(),
        tournament_id: tournament_id.clone(),
    };
    add_player_score(env, player_id.clone(), player_score);
    
    // Update global leaderboard
    leaderboard::update_global_leaderboard(env, &tournament.game_id, &player_id, score);
    
    // Emit event
    env.events().publish(
        (Symbol::new(env, "score_submitted"), player_id.clone()),
        (tournament_id, score),
    );
    
    Ok(rank)
}

/// Get player's score history
pub fn get_player_history(env: &Env, player_id: Address) -> Vec<PlayerScore> {
    get_player_scores(env, player_id)
}

/// Validate score submission (placeholder for anti-cheat)
pub fn validate_score(
    _env: &Env,
    _player_id: &Address,
    _score: u64,
    _timestamp: u64,
    _signature: Option<Vec<u8>>,
) -> bool {
    // Placeholder for signature verification
    // In a real implementation, this would:
    // 1. Verify the signature against the score data
    // 2. Check timestamp to prevent replay attacks
    // 3. Validate score is within reasonable bounds
    // 4. Potentially check against off-chain oracles
    true
}

// Helper functions
fn get_tournament(env: &Env, tournament_id: String) -> Result<Tournament, Error> {
    env.storage()
        .instance()
        .get(&DataKey::Tournament(tournament_id))
        .ok_or(Error::TournamentNotFound)
}

fn save_tournament(env: &Env, tournament_id: String, tournament: &Tournament) {
    env.storage()
        .instance()
        .set(&DataKey::Tournament(tournament_id), tournament);
}

fn get_player_scores(env: &Env, player_id: Address) -> Vec<PlayerScore> {
    env.storage()
        .instance()
        .get(&DataKey::PlayerScores(player_id))
        .unwrap_or(Vec::new(env))
}

fn add_player_score(env: &Env, player_id: Address, score: PlayerScore) {
    let mut scores = get_player_scores(env, player_id.clone());
    scores.push_back(score);
    
    // Keep only last 100 scores per player
    while scores.len() > 100 {
        scores.pop_front();
    }
    
    env.storage()
        .instance()
        .set(&DataKey::PlayerScores(player_id), &scores);
} 
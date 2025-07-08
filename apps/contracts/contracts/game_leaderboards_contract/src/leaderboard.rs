use soroban_sdk::{contracttype, Address, Env, String, Vec};
use crate::errors::Error;
use crate::types::{LeaderboardEntry, Tournament};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Tournament(String),
    GlobalLeaderboard(String),
}

/// Get tournament leaderboard with pagination
pub fn get_leaderboard(
    env: &Env,
    tournament_id: String,
    page: u32,
    page_size: u32,
) -> Result<Vec<LeaderboardEntry>, Error> {
    let tournament = get_tournament(env, tournament_id)?;
    
    let start_index = page * page_size;
    let end_index = (start_index + page_size).min(tournament.entries.len());
    
    let mut result = Vec::new(env);
    for i in start_index..end_index {
        if let Some(entry) = tournament.entries.get(i) {
            result.push_back(entry);
        }
    }
    
    Ok(result)
}

/// Get global leaderboard for a game
pub fn get_global_leaderboard(
    env: &Env,
    game_id: String,
    page: u32,
    page_size: u32,
) -> Result<Vec<LeaderboardEntry>, Error> {
    let global_leaderboard = get_global_leaderboard_internal(env, game_id);
    
    let start_index = page * page_size;
    let end_index = (start_index + page_size).min(global_leaderboard.len());
    
    let mut result = Vec::new(env);
    for i in start_index..end_index {
        if let Some(entry) = global_leaderboard.get(i) {
            result.push_back(entry);
        }
    }
    
    Ok(result)
}

/// Update global leaderboard with new score
pub fn update_global_leaderboard(env: &Env, game_id: &String, player_id: &Address, score: u64) {
    let mut leaderboard = get_global_leaderboard_internal(env, game_id.clone());
    
    // Find existing entry
    let mut found_index: Option<u32> = None;
    for i in 0..leaderboard.len() {
        if let Some(entry) = leaderboard.get(i) {
            if entry.player_id == *player_id {
                found_index = Some(i);
                break;
            }
        }
    }
    
    if let Some(index) = found_index {
        let mut entry = leaderboard.get(index).unwrap();
        if score > entry.score {
            entry.score = score;
            leaderboard.set(index, entry);
        }
    } else {
        let new_entry = LeaderboardEntry {
            player_id: player_id.clone(),
            score,
            rank: 1,
        };
        leaderboard.push_back(new_entry);
    }
    
    // Sort and update ranks
    sort_entries(env, &mut leaderboard);
    
    // Keep only top 1000
    while leaderboard.len() > 1000 {
        leaderboard.pop_back();
    }
    
    env.storage()
        .instance()
        .set(&DataKey::GlobalLeaderboard(game_id.clone()), &leaderboard);
}

/// Sort entries by score (descending) and update ranks
pub fn sort_entries(_env: &Env, entries: &mut Vec<LeaderboardEntry>) {
    // Simple bubble sort for small collections
    let len = entries.len();
    for i in 0..len {
        for j in 0..len.saturating_sub(1).saturating_sub(i) {
            if let (Some(entry1), Some(entry2)) = (entries.get(j), entries.get(j + 1)) {
                if entry1.score < entry2.score {
                    entries.set(j, entry2);
                    entries.set(j + 1, entry1);
                }
            }
        }
    }
    
    // Update ranks
    for i in 0..len {
        if let Some(mut entry) = entries.get(i) {
            entry.rank = i + 1;
            entries.set(i, entry);
        }
    }
}

// Helper functions
fn get_tournament(env: &Env, tournament_id: String) -> Result<Tournament, Error> {
    env.storage()
        .instance()
        .get(&DataKey::Tournament(tournament_id))
        .ok_or(Error::TournamentNotFound)
}

fn get_global_leaderboard_internal(env: &Env, game_id: String) -> Vec<LeaderboardEntry> {
    env.storage()
        .instance()
        .get(&DataKey::GlobalLeaderboard(game_id))
        .unwrap_or(Vec::new(env))
} 
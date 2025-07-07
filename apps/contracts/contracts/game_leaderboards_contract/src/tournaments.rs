use soroban_sdk::{contracttype, Env, String, Symbol, Vec};
use crate::errors::Error;
use crate::types::{LeaderboardEntry, Tournament};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Tournament(String),
    TournamentList,
}

/// Create a new tournament
pub fn create_tournament(
    env: &Env,
    tournament_id: String,
    game_id: String,
    start_time: u64,
    end_time: u64,
    max_entries: u32,
) -> Result<(), Error> {
    // Check if tournament already exists
    if get_tournament_internal(env, tournament_id.clone()).is_some() {
        return Err(Error::TournamentExists);
    }
    
    // Validate tournament parameters
    if start_time >= end_time {
        return Err(Error::InvalidParameters);
    }
    
    if max_entries == 0 || max_entries > 10000 {
        return Err(Error::InvalidParameters);
    }
    
    let tournament = Tournament {
        id: tournament_id.clone(),
        game_id: game_id.clone(),
        start_time,
        end_time,
        max_entries,
        entries: Vec::new(env),
    };
    
    save_tournament(env, tournament_id.clone(), &tournament);
    add_to_tournament_list(env, tournament_id.clone());
    
    // Emit event
    env.events().publish(
        (Symbol::new(env, "tournament_created"), tournament_id.clone()),
        game_id,
    );
    
    Ok(())
}

/// Get tournament information
pub fn get_tournament_info(env: &Env, tournament_id: String) -> Result<Tournament, Error> {
    get_tournament_internal(env, tournament_id).ok_or(Error::TournamentNotFound)
}

/// Get tournament results (final leaderboard)
pub fn get_tournament_results(env: &Env, tournament_id: String) -> Result<Vec<LeaderboardEntry>, Error> {
    let tournament = get_tournament_internal(env, tournament_id).ok_or(Error::TournamentNotFound)?;
    
    // Check if tournament has ended
    let current_time = env.ledger().timestamp();
    if current_time < tournament.end_time {
        return Err(Error::TournamentNotActive);
    }
    
    Ok(tournament.entries)
}

/// Get list of all tournaments
pub fn get_tournaments(env: &Env) -> Vec<String> {
    get_tournament_list(env)
}

/// Get active tournaments for a specific game
pub fn get_active_tournaments(env: &Env, game_id: Option<String>) -> Vec<Tournament> {
    let all_tournaments = get_tournament_list(env);
    let current_time = env.ledger().timestamp();
    let mut active_tournaments = Vec::new(env);
    
    for i in 0..all_tournaments.len() {
        if let Some(tournament_id) = all_tournaments.get(i) {
            if let Some(tournament) = get_tournament_internal(env, tournament_id) {
                // Check if tournament is currently active
                if current_time >= tournament.start_time && current_time <= tournament.end_time {
                    // Filter by game_id if specified
                    if let Some(ref filter_game_id) = game_id {
                        if tournament.game_id == *filter_game_id {
                            active_tournaments.push_back(tournament);
                        }
                    } else {
                        active_tournaments.push_back(tournament);
                    }
                }
            }
        }
    }
    
    active_tournaments
}

/// Check if a tournament is currently active
pub fn is_tournament_active(env: &Env, tournament_id: String) -> Result<bool, Error> {
    let tournament = get_tournament_internal(env, tournament_id).ok_or(Error::TournamentNotFound)?;
    let current_time = env.ledger().timestamp();
    
    Ok(current_time >= tournament.start_time && current_time <= tournament.end_time)
}

/// Get tournament status
pub fn get_tournament_status(env: &Env, tournament_id: String) -> Result<String, Error> {
    let tournament = get_tournament_internal(env, tournament_id).ok_or(Error::TournamentNotFound)?;
    let current_time = env.ledger().timestamp();
    
    let status = if current_time < tournament.start_time {
        String::from_str(env, "upcoming")
    } else if current_time <= tournament.end_time {
        String::from_str(env, "active")
    } else {
        String::from_str(env, "ended")
    };
    
    Ok(status)
}

// Helper functions
fn get_tournament_internal(env: &Env, tournament_id: String) -> Option<Tournament> {
    env.storage()
        .instance()
        .get(&DataKey::Tournament(tournament_id))
}

fn save_tournament(env: &Env, tournament_id: String, tournament: &Tournament) {
    env.storage()
        .instance()
        .set(&DataKey::Tournament(tournament_id), tournament);
}

fn get_tournament_list(env: &Env) -> Vec<String> {
    env.storage()
        .instance()
        .get(&DataKey::TournamentList)
        .unwrap_or(Vec::new(env))
}

fn add_to_tournament_list(env: &Env, tournament_id: String) {
    let mut list = get_tournament_list(env);
    list.push_back(tournament_id);
    env.storage().instance().set(&DataKey::TournamentList, &list);
} 
#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

pub mod errors;
pub mod types;
pub mod leaderboard;
pub mod scores;
pub mod tournaments;

use errors::Error;
use types::*;

#[contract]
pub struct GameLeaderboardContract;

#[contractimpl]
impl GameLeaderboardContract {
    /// Submit a score to a tournament
    pub fn submit_score(
        env: Env,
        tournament_id: String,
        player_id: Address,
        score: u64,
    ) -> Result<u32, Error> {
        scores::submit_score(&env, tournament_id, player_id, score)
    }

    /// Get tournament leaderboard with pagination
    pub fn get_leaderboard(
        env: Env,
        tournament_id: String,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<LeaderboardEntry>, Error> {
        leaderboard::get_leaderboard(&env, tournament_id, page, page_size)
    }

    /// Get global leaderboard for a game
    pub fn get_global_leaderboard(
        env: Env,
        game_id: String,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<LeaderboardEntry>, Error> {
        leaderboard::get_global_leaderboard(&env, game_id, page, page_size)
    }

    /// Create a new tournament
    pub fn create_tournament(
        env: Env,
        tournament_id: String,
        game_id: String,
        start_time: u64,
        end_time: u64,
        max_entries: u32,
    ) -> Result<(), Error> {
        tournaments::create_tournament(&env, tournament_id, game_id, start_time, end_time, max_entries)
    }

    /// Get tournament info
    pub fn get_tournament_info(env: Env, tournament_id: String) -> Result<Tournament, Error> {
        tournaments::get_tournament_info(&env, tournament_id)
    }

    /// Get tournament results
    pub fn get_tournament_results(env: Env, tournament_id: String) -> Result<Vec<LeaderboardEntry>, Error> {
        tournaments::get_tournament_results(&env, tournament_id)
    }

    /// Get player's score history
    pub fn get_player_history(env: Env, player_id: Address) -> Vec<PlayerScore> {
        scores::get_player_history(&env, player_id)
    }

    /// Get list of all tournaments
    pub fn get_tournaments(env: Env) -> Vec<String> {
        tournaments::get_tournaments(&env)
    }
}
#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, 
    vec, Bytes, 
    Address, Env, IntoVal, Symbol, Vec, Val,
};

#[derive(Clone)]
#[contracttype]
pub struct Review {
    pub reviewer: Address,
    pub rating: u32,
    pub comment: Bytes,
    pub timestamp: u64,
}

#[contract]
pub struct GameReviewContract;

#[contractimpl]
impl GameReviewContract {
    pub fn add_review(env: Env, reviewer: Address, game_id: u32, rating: u32, comment: Bytes) -> Review {
        // Validate the rating is between 1 and 5
        if rating < 1 || rating > 5 {
            panic!("Rating must be between 1 and 5");
        }

        // Require the reviewer to authenticate
        reviewer.require_auth();

        // Check if the user has already reviewed this game
        let user_reviewed_key = Self::get_user_reviewed_key(&env, &reviewer, game_id);
        if env.storage().instance().has(&user_reviewed_key) {
            panic!("User has already reviewed this game");
        }

        // Create the review
        let timestamp = env.ledger().timestamp();
        let review = Review {
            reviewer: reviewer.clone(),
            rating,
            comment,
            timestamp,
        };

        // Get the current review count for the game
        let review_count_key = Self::get_game_review_count_key(&env, game_id);
        let review_count: u32 = env
            .storage()
            .instance()
            .get(&review_count_key)
            .unwrap_or(0);

        // Store the review
        let review_key = Self::get_review_key(&env, game_id, review_count);
        env.storage().instance().set(&review_key, &review);

        // Update review count
        env.storage().instance().set(&review_count_key, &(review_count + 1));

        // Update total rating
        let total_rating_key = Self::get_game_total_rating_key(&env, game_id);
        let total_rating: u32 = env
            .storage()
            .instance()
            .get(&total_rating_key)
            .unwrap_or(0);
        env.storage().instance().set(&total_rating_key, &(total_rating + rating));

        // Mark user as having reviewed this game
        env.storage().instance().set(&user_reviewed_key, &true);

        // Emit event
        env.events()
            .publish(
                (Symbol::new(&env, "review"), Symbol::new(&env, "added")),
                (game_id, reviewer, rating, review_count),
            );

        review
    }

    pub fn get_review(env: Env, game_id: u32, index: u32) -> Review {
        let review_count = Self::get_game_review_count(env.clone(), game_id);
        if index >= review_count {
            panic!("Review index out of bounds");
        }

        let review_key = Self::get_review_key(&env, game_id, index);
        env.storage().instance().get(&review_key).unwrap()
    }

    pub fn has_reviewed(env: Env, user: Address, game_id: u32) -> bool {
        let user_reviewed_key = Self::get_user_reviewed_key(&env, &user, game_id);
        env.storage().instance().has(&user_reviewed_key)
    }

    pub fn get_game_review_count(env: Env, game_id: u32) -> u32 {
        let review_count_key = Self::get_game_review_count_key(&env, game_id);
        env.storage()
            .instance()
            .get(&review_count_key)
            .unwrap_or(0)
    }

    pub fn get_game_rating(env: Env, game_id: u32) -> u32 {
        let review_count = Self::get_game_review_count(env.clone(), game_id);
        if review_count == 0 {
            return 0;
        }

        let total_rating_key = Self::get_game_total_rating_key(&env, game_id);
        let total_rating: u32 = env
            .storage()
            .instance()
            .get(&total_rating_key)
            .unwrap_or(0);

        // Calculate average (rounded to the nearest integer)
        (total_rating + (review_count / 2)) / review_count
    }

    // Helper functions for storage keys
    fn get_review_key(env: &Env, game_id: u32, index: u32) -> Vec<Val> {
        let review_key = Symbol::new(env, "REVIEW");
        vec![env, review_key.into_val(env), game_id.into_val(env), index.into_val(env)]
    }

    fn get_game_total_rating_key(env: &Env, game_id: u32) -> Vec<Val> {
        let gtr_key = Symbol::new(env, "GTR");
        vec![env, gtr_key.into_val(env), game_id.into_val(env)]
    }

    fn get_game_review_count_key(env: &Env, game_id: u32) -> Vec<Val> {
        let grc_key = Symbol::new(env, "GRC");
        vec![env, grc_key.into_val(env), game_id.into_val(env)]
    }

    fn get_user_reviewed_key(env: &Env, user: &Address, game_id: u32) -> Vec<Val> {
        let urev_key = Symbol::new(env, "UREV");
        vec![env, urev_key.into_val(env), user.into_val(env), game_id.into_val(env)]
    }
}

mod test; 
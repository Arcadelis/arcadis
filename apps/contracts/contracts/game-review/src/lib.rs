#![no_std]

/// Game Review smart contract for managing Arcadis user reviews
///
/// This contract allows:
/// - Users to submit game reviews with ratings (1-5) and comments
/// - Admins to moderate content by deleting inappropriate reviews
/// - Anyone to query reviews and game ratings
pub mod errors;
pub mod events;
pub mod interface;
pub mod storage;
pub mod types;

use errors::GameReviewError;
use soroban_sdk::{contract, contractimpl, Address, Bytes, Env, Vec};

use events::GameReviewEvents;
use interface::GameReviewTrait;
use storage::GameReviewStorage;
use types::Review;

/// Game Review smart contract main implementation
#[contract]
pub struct GameReview;

#[contractimpl]
impl GameReviewTrait for GameReview {
    /// Initializes the contract with an admin address
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `admin` - The address to be set as admin
    ///
    /// # Returns
    /// * `Result<(), GameReviewError>` - Ok if initialized, Error if already initialized
    fn initialize(env: Env, admin: Address) -> Result<(), GameReviewError> {
        // Check if contract is already initialized
        if env.storage().instance().has(&types::DataKey::Admin) {
            return Err(GameReviewError::AlreadyInitialized);
        }

        // Set admin
        GameReviewStorage::set_admin(&env, &admin);

        // Emit event
        GameReviewEvents::emit_contract_initialized(&env, &admin);

        Ok(())
    }

    /// Changes the contract admin to a new address
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `new_admin` - The address of the new admin
    ///
    /// # Returns
    /// * `Result<(), GameReviewError>` - Ok if admin changed, Error if unauthorized
    ///
    /// # Authentication
    /// * Requires authorization from current admin
    fn change_admin(env: Env, new_admin: Address) -> Result<(), GameReviewError> {
        let old_admin = Self::require_admin(&env)?;

        // Require authentication from the current admin
        old_admin.require_auth();

        // Set new admin
        GameReviewStorage::set_admin(&env, &new_admin);

        // Emit event
        GameReviewEvents::emit_admin_changed(&env, &old_admin, &new_admin);

        Ok(())
    }

    /// Adds a review for a game
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `user` - The address of the reviewer
    /// * `game_id` - Unique identifier for the game
    /// * `rating` - Rating value (1-5 stars)
    /// * `comment` - Text content of the review
    ///
    /// # Returns
    /// * `Result<(), GameReviewError>` - Ok if added, Error if invalid input or already reviewed
    ///
    /// # Authentication
    /// * Requires authorization from the user
    fn add_review(
        env: Env,
        user: Address,
        game_id: u32,
        rating: u32,
        comment: Bytes,
    ) -> Result<(), GameReviewError> {
        // Require authentication from the user
        user.require_auth();

        // Validate rating (1-5 stars)
        if rating < 1 || rating > 5 {
            return Err(GameReviewError::InvalidInput);
        }

        let total_reviews = GameReviewStorage::get_review_count(&env, game_id);
        let review_id = total_reviews + 1;

        // Create a new review
        let review = Review {
            id: review_id,
            reviewer: user.clone(),
            rating,
            comment,
            timestamp: env.ledger().timestamp(),
        };

        // Check if user has already reviewed this game
        let has_reviewed = Self::has_reviewed(env.clone(), user.clone(), game_id);

        if has_reviewed {
            return Err(GameReviewError::UserHasReviewed);
        } else {
            // Add new review and update counts/ratings
            GameReviewStorage::set_review(&env, game_id, &user, &review);
            GameReviewStorage::set_indexed_reviews(&env, game_id, review_id, &review);

            // Update the review count
            let current_count = GameReviewStorage::get_review_count(&env, game_id);
            let new_count = current_count + 1;
            GameReviewStorage::set_review_count(&env, game_id, new_count);

            // Update average rating
            let current_avg = GameReviewStorage::get_average_rating(&env, game_id);
            let new_avg = ((current_avg * current_count) + rating) / new_count;
            GameReviewStorage::set_average_rating(&env, game_id, new_avg);

            //Update total ratings
            let current_total_ratings = GameReviewStorage::get_total_ratings(&env, game_id);
            let new_total_ratings = current_total_ratings + rating;
            GameReviewStorage::set_total_ratings(&env, game_id, new_total_ratings);

            // Emit event
            GameReviewEvents::emit_review_added(&env, game_id, &user, &review);
        }

        Ok(())
    }

    /// Deletes a review from the system
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `game_id` - Unique identifier for the game
    /// * `review_id` - Identifier for the specific review
    ///
    /// # Returns
    /// * `Result<(), GameReviewError>` - Ok if deleted, Error if not found or unauthorized
    ///
    /// # Authentication
    /// * Requires authorization from admin
    fn delete_review(env: Env, game_id: u32, review_id: u32) -> Result<(), GameReviewError> {
        // Only admin can delete a review
        let admin = Self::require_admin(&env)?;
        admin.require_auth();

        // Check if review exists
        let key = types::DataKey::IndexedReviews(game_id, review_id);
        if !env.storage().persistent().has(&key) {
            return Err(GameReviewError::ReviewNotFound);
        }

        // Get the review to find the reviewer and rating
        let review = GameReviewStorage::get_indexed_reviews(&env, game_id, review_id);
        let reviewer = review.reviewer.clone();
        let rating = review.rating;

        // Delete the review from both indexes
        let key_by_user = types::DataKey::Reviews(game_id, reviewer.clone());
        let key_by_id = types::DataKey::IndexedReviews(game_id, review_id);

        env.storage().persistent().remove(&key_by_user);
        env.storage().persistent().remove(&key_by_id);

        // Update the review count
        let current_count = GameReviewStorage::get_review_count(&env, game_id);
        if current_count > 0 {
            GameReviewStorage::set_review_count(&env, game_id, current_count - 1);
        }

        // Update total ratings and average rating
        let total_ratings = GameReviewStorage::get_total_ratings(&env, game_id);
        if total_ratings >= rating {
            let new_total_ratings = total_ratings - rating;
            GameReviewStorage::set_total_ratings(&env, game_id, new_total_ratings);

            // Only recalculate average if we still have reviews
            if current_count > 1 {
                let new_avg = new_total_ratings / (current_count - 1);
                GameReviewStorage::set_average_rating(&env, game_id, new_avg);
            } else {
                // If this was the last review, reset the average to 0
                GameReviewStorage::set_average_rating(&env, game_id, 0);
            }
        }

        // Emit deletion event
        GameReviewEvents::emit_review_deleted(&env, game_id, &reviewer);

        Ok(())
    }

    /// Gets a specific review by game and user
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `game_id` - Unique identifier for the game
    /// * `user` - Address of the reviewer
    ///
    /// # Returns
    /// * `Result<Review, GameReviewError>` - Review if found, Error if not found
    fn get_review(env: Env, game_id: u32, user: Address) -> Result<Review, GameReviewError> {
        if !Self::has_reviewed(env.clone(), user.clone(), game_id) {
            return Err(GameReviewError::ReviewNotFound);
        }

        Ok(GameReviewStorage::get_review(&env, game_id, &user))
    }

    /// Gets a paginated list of reviews for a game
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `game_id` - Unique identifier for the game
    /// * `skip` - Number of reviews to skip (for pagination)
    /// * `limit` - Maximum number of reviews to return
    ///
    /// # Returns
    /// * `Vec<Review>` - Collection of reviews, empty if none found
    fn get_reviews(env: Env, game_id: u32, skip: u32, limit: u32) -> Vec<Review> {
        let total_reviews = GameReviewStorage::get_review_count(&env, game_id);
        let mut reviews = Vec::new(&env);

        if total_reviews == 0 {
            return reviews;
        }

        // Convert to 1-based for review IDs
        let start_id = skip + 1;
        if start_id > total_reviews {
            return reviews;
        }

        let end_id = (start_id + limit - 1).min(total_reviews);

        for i in start_id..=end_id {
            let key = types::DataKey::IndexedReviews(game_id, i);
            if env.storage().persistent().has(&key) {
                let review = GameReviewStorage::get_indexed_reviews(&env, game_id, i);
                reviews.push_back(review);
            }
        }

        reviews
    }

    /// Checks if a user has already reviewed a specific game
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `user` - Address of the potential reviewer
    /// * `game_id` - Unique identifier for the game
    ///
    /// # Returns
    /// * `bool` - true if user has already reviewed, false otherwise
    fn has_reviewed(env: Env, user: Address, game_id: u32) -> bool {
        // Try to get the review and return true if it exists, false otherwise
        let key = types::DataKey::Reviews(game_id, user.clone());
        env.storage().persistent().has(&key)
    }

    /// Gets the total number of reviews for a game
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `game_id` - Unique identifier for the game
    ///
    /// # Returns
    /// * `u32` - Number of reviews (0 if none)
    fn get_game_review_count(env: Env, game_id: u32) -> u32 {
        GameReviewStorage::get_review_count(&env, game_id)
    }

    /// Gets the average rating for a game
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `game_id` - Unique identifier for the game
    ///
    /// # Returns
    /// * `u32` - Average rating from 1-5 (0 if no reviews)
    fn get_game_rating(env: Env, game_id: u32) -> u32 {
        GameReviewStorage::get_average_rating(&env, game_id)
    }

    /// Gets the current admin address
    ///
    /// # Arguments
    /// * `env` - The contract environment
    ///
    /// # Returns
    /// * `Result<Address, GameReviewError>` - Admin address if found, Error if not initialized
    fn get_admin(env: Env) -> Result<Address, GameReviewError> {
        if !env.storage().instance().has(&types::DataKey::Admin) {
            return Err(GameReviewError::Unauthorized);
        }

        Ok(GameReviewStorage::get_admin(&env))
    }
}

impl GameReview {
    /// Helper function to validate admin access
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    ///
    /// # Returns
    /// * `Result<Address, GameReviewError>` - Admin address if valid, Error if unauthorized
    fn require_admin(env: &Env) -> Result<Address, GameReviewError> {
        if !env.storage().instance().has(&types::DataKey::Admin) {
            return Err(GameReviewError::Unauthorized);
        }

        Ok(GameReviewStorage::get_admin(env))
    }
}

#[cfg(test)]
mod test;

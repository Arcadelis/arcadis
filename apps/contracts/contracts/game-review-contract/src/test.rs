#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Ledger as _},
    Bytes, Env,
};

// Helper to generate a valid test address
fn generate_test_address(env: &Env) -> Address {
    // Generate a dummy contract address for testing - using deprecated method is ok for tests
    #[allow(deprecated)]
    env.register_contract(None, GameReviewContract)
}

#[test]
fn test_add_review_valid() {
    let env = Env::default();
    // Using deprecated method is ok for tests
    #[allow(deprecated)]
    let contract_id = env.register_contract(None, GameReviewContract);
    let client = GameReviewContractClient::new(&env, &contract_id);

    // Setup test data - create a test address
    let user = generate_test_address(&env);
    let game_id = 1;
    let rating = 4;
    let comment = Bytes::from_slice(&env, "Great game!".as_bytes());

    // Set a timestamp
    env.ledger().set_timestamp(12345);

    // Mock authentication for all calls
    env.mock_all_auths();

    // Add a review
    let review = client.add_review(&user, &game_id, &rating, &comment);

    // Verify the review data
    assert_eq!(review.reviewer, user);
    assert_eq!(review.rating, rating);
    assert_eq!(review.comment, comment);
    assert_eq!(review.timestamp, 12345);

    // Verify review count is updated
    assert_eq!(client.get_game_review_count(&game_id), 1);

    // Verify average rating
    assert_eq!(client.get_game_rating(&game_id), 4);

    // Verify has_reviewed returns true
    assert!(client.has_reviewed(&user, &game_id));
}

#[test]
fn test_add_review_multiple_users() {
    let env = Env::default();
    // Using deprecated method is ok for tests
    #[allow(deprecated)]
    let contract_id = env.register_contract(None, GameReviewContract);
    let client = GameReviewContractClient::new(&env, &contract_id);

    // Setup test data with two different users
    let user1 = generate_test_address(&env);
    let user2 = generate_test_address(&env);
    let game_id = 1;

    // Mock authentication for all calls
    env.mock_all_auths();

    // Add first review
    client.add_review(&user1, &game_id, &3, &Bytes::from_slice(&env, "Good game".as_bytes()));

    // Add second review
    client.add_review(&user2, &game_id, &5, &Bytes::from_slice(&env, "Excellent!".as_bytes()));

    // Verify review count
    assert_eq!(client.get_game_review_count(&game_id), 2);

    // Verify average rating (3 + 5) / 2 = 4
    assert_eq!(client.get_game_rating(&game_id), 4);

    // Verify individual reviews
    let review1 = client.get_review(&game_id, &0);
    let review2 = client.get_review(&game_id, &1);

    assert_eq!(review1.rating, 3);
    assert_eq!(review2.rating, 5);

    // Verify has_reviewed returns true for both users
    assert!(client.has_reviewed(&user1, &game_id));
    assert!(client.has_reviewed(&user2, &game_id));
}

#[test]
#[should_panic(expected = "User has already reviewed this game")]
fn test_add_review_duplicate_user() {
    let env = Env::default();
    // Using deprecated method is ok for tests
    #[allow(deprecated)]
    let contract_id = env.register_contract(None, GameReviewContract);
    let client = GameReviewContractClient::new(&env, &contract_id);

    // Setup test data
    let user = generate_test_address(&env);
    let game_id = 1;

    // Mock authentication for all calls
    env.mock_all_auths();

    // First review is okay
    client.add_review(&user, &game_id, &4, &Bytes::from_slice(&env, "Good game".as_bytes()));

    // Second review should fail with same user
    client.add_review(&user, &game_id, &5, &Bytes::from_slice(&env, "Changed my mind!".as_bytes()));
}

#[test]
#[should_panic(expected = "Rating must be between 1 and 5")]
fn test_add_review_invalid_rating_too_low() {
    let env = Env::default();
    // Using deprecated method is ok for tests
    #[allow(deprecated)]
    let contract_id = env.register_contract(None, GameReviewContract);
    let client = GameReviewContractClient::new(&env, &contract_id);

    let user = generate_test_address(&env);
    let game_id = 1;
    
    // Mock authentication for all calls
    env.mock_all_auths();

    // Rating 0 should fail
    client.add_review(&user, &game_id, &0, &Bytes::from_slice(&env, "Invalid rating".as_bytes()));
}

#[test]
#[should_panic(expected = "Rating must be between 1 and 5")]
fn test_add_review_invalid_rating_too_high() {
    let env = Env::default();
    // Using deprecated method is ok for tests
    #[allow(deprecated)]
    let contract_id = env.register_contract(None, GameReviewContract);
    let client = GameReviewContractClient::new(&env, &contract_id);

    let user = generate_test_address(&env);
    let game_id = 1;
    
    // Mock authentication for all calls
    env.mock_all_auths();

    // Rating 6 should fail
    client.add_review(&user, &game_id, &6, &Bytes::from_slice(&env, "Invalid rating".as_bytes()));
}

#[test]
fn test_add_review_empty_comment() {
    let env = Env::default();
    // Using deprecated method is ok for tests
    #[allow(deprecated)]
    let contract_id = env.register_contract(None, GameReviewContract);
    let client = GameReviewContractClient::new(&env, &contract_id);

    let user = generate_test_address(&env);
    let game_id = 1;
    let empty_comment = Bytes::from_slice(&env, "".as_bytes());
    
    // Mock authentication for all calls
    env.mock_all_auths();

    // Empty comment should be allowed
    let review = client.add_review(&user, &game_id, &3, &empty_comment);
    
    assert_eq!(review.comment, empty_comment);
}

#[test]
fn test_add_review_large_comment() {
    let env = Env::default();
    // Using deprecated method is ok for tests
    #[allow(deprecated)]
    let contract_id = env.register_contract(None, GameReviewContract);
    let client = GameReviewContractClient::new(&env, &contract_id);

    let user = generate_test_address(&env);
    let game_id = 1;
    
    // Create a large comment
    let comment_str = "a".repeat(1000);
    let large_comment = Bytes::from_slice(&env, comment_str.as_bytes());
    
    // Mock authentication for all calls
    env.mock_all_auths();

    // Large comment should be allowed
    let review = client.add_review(&user, &game_id, &4, &large_comment);
    
    assert_eq!(review.comment, large_comment);
}

#[test]
fn test_user_reviewing_multiple_games() {
    let env = Env::default();
    // Using deprecated method is ok for tests
    #[allow(deprecated)]
    let contract_id = env.register_contract(None, GameReviewContract);
    let client = GameReviewContractClient::new(&env, &contract_id);

    let user = generate_test_address(&env);
    let game_id1 = 1;
    let game_id2 = 2;
    
    // Mock authentication for all calls
    env.mock_all_auths();

    // Review the first game
    client.add_review(&user, &game_id1, &4, &Bytes::from_slice(&env, "First game review".as_bytes()));
    
    // Review the second game
    client.add_review(&user, &game_id2, &5, &Bytes::from_slice(&env, "Second game review".as_bytes()));
    
    // Verify the reviews were added correctly
    assert_eq!(client.get_game_review_count(&game_id1), 1);
    assert_eq!(client.get_game_review_count(&game_id2), 1);
    
    // Verify the user has reviewed both games
    assert!(client.has_reviewed(&user, &game_id1));
    assert!(client.has_reviewed(&user, &game_id2));
}

#[test]
#[should_panic(expected = "Review index out of bounds")]
fn test_get_review_out_of_bounds() {
    let env = Env::default();
    // Using deprecated method is ok for tests
    #[allow(deprecated)]
    let contract_id = env.register_contract(None, GameReviewContract);
    let client = GameReviewContractClient::new(&env, &contract_id);

    let game_id = 1;
    
    // Try to get a review for a game with no reviews
    client.get_review(&game_id, &0);
}

#[test]
fn test_has_reviewed() {
    let env = Env::default();
    // Using deprecated method is ok for tests
    #[allow(deprecated)]
    let contract_id = env.register_contract(None, GameReviewContract);
    let client = GameReviewContractClient::new(&env, &contract_id);

    let user1 = generate_test_address(&env);
    let user2 = generate_test_address(&env);
    let game_id = 1;
    
    // Initially no user has reviewed
    assert!(!client.has_reviewed(&user1, &game_id));
    assert!(!client.has_reviewed(&user2, &game_id));
    
    // Mock authentication for all calls
    env.mock_all_auths();
    
    // Add a review from user1
    client.add_review(&user1, &game_id, &4, &Bytes::from_slice(&env, "User1 review".as_bytes()));
    
    // Now user1 has reviewed but user2 hasn't
    assert!(client.has_reviewed(&user1, &game_id));
    assert!(!client.has_reviewed(&user2, &game_id));
}

#[test]
fn test_get_game_review_count_no_reviews() {
    let env = Env::default();
    // Using deprecated method is ok for tests
    #[allow(deprecated)]
    let contract_id = env.register_contract(None, GameReviewContract);
    let client = GameReviewContractClient::new(&env, &contract_id);

    let game_id = 123;
    
    // A game with no reviews should return 0
    assert_eq!(client.get_game_review_count(&game_id), 0);
}

#[test]
fn test_get_game_rating_no_reviews() {
    let env = Env::default();
    // Using deprecated method is ok for tests
    #[allow(deprecated)]
    let contract_id = env.register_contract(None, GameReviewContract);
    let client = GameReviewContractClient::new(&env, &contract_id);

    let game_id = 123;
    
    // A game with no reviews should have a rating of 0
    assert_eq!(client.get_game_rating(&game_id), 0);
}

#[test]
fn test_get_game_rating_rounding() {
    let env = Env::default();
    // Using deprecated method is ok for tests
    #[allow(deprecated)]
    let contract_id = env.register_contract(None, GameReviewContract);
    let client = GameReviewContractClient::new(&env, &contract_id);

    // Setup test users with different test addresses
    let user1 = generate_test_address(&env);
    let user2 = generate_test_address(&env);
    let user3 = generate_test_address(&env);
    let game_id = 1;
    
    // Mock authentication for all calls
    env.mock_all_auths();
    
    // Add reviews with different ratings
    client.add_review(&user1, &game_id, &2, &Bytes::from_slice(&env, "Review 1".as_bytes()));
    client.add_review(&user2, &game_id, &3, &Bytes::from_slice(&env, "Review 2".as_bytes()));
    client.add_review(&user3, &game_id, &4, &Bytes::from_slice(&env, "Review 3".as_bytes()));
    
    // Total rating is 2 + 3 + 4 = 9, divided by 3 = 3
    assert_eq!(client.get_game_rating(&game_id), 3);

    // Add another user with rating 5 to test rounding
    let user4 = generate_test_address(&env);
    
    // Add review from user4
    client.add_review(&user4, &game_id, &5, &Bytes::from_slice(&env, "Review 4".as_bytes()));
    
    // Total rating is now 2 + 3 + 4 + 5 = 14, divided by 4 = 3.5, which rounds to 4
    // due to our rounding formula: (total + (count / 2)) / count
    assert_eq!(client.get_game_rating(&game_id), 4);
} 
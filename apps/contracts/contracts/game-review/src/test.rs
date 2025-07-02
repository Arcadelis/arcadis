#![cfg(test)]

extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::TryFromVal;
use soroban_sdk::{
    testutils::{Events, Ledger},
    Address, Bytes, Env, Symbol,
};

fn test_address(env: &Env, _seed: &str) -> Address {
    Address::generate(env)
}

fn setup_env_and_contract() -> (Env, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(GameReview, ());
    let admin = test_address(&env, "admin");
    env.as_contract(&contract_id, || {
        GameReview::initialize(env.clone(), admin.clone()).unwrap();
    });
    (env, contract_id, admin)
}

#[test]
fn test_add_review_success() {
    let (env, contract_id, _admin) = setup_env_and_contract();
    let user = test_address(&env, "user1");
    let game_id = 1u32;
    let rating = 5u32;
    let comment = Bytes::from_slice(&env, b"Great game!");
    let timestamp = 123456u64;
    env.ledger().set_timestamp(timestamp);

    env.mock_all_auths();
    env.as_contract(&contract_id, || {
        // Add review
        GameReview::add_review(env.clone(), user.clone(), game_id, rating, comment.clone())
            .unwrap();

        // Check review stored
        let review = GameReview::get_review(env.clone(), game_id, user.clone()).unwrap();
        assert_eq!(review.reviewer, user);
        assert_eq!(review.rating, rating);
        assert_eq!(review.comment, comment);
        assert_eq!(review.timestamp, timestamp);
        assert_eq!(review.id, 1);

        // Check review count
        assert_eq!(GameReview::get_game_review_count(env.clone(), game_id), 1);
        // Check average rating
        assert_eq!(GameReview::get_game_rating(env.clone(), game_id), rating);
        // Check has_reviewed
        assert!(GameReview::has_reviewed(env.clone(), user.clone(), game_id));

        // Check event emitted
        let events = env.events().all();
        let found = events.iter().any(|e| {
            if let (_, topics_vec, _) = e {
                topics_vec.iter().any(|topic| {
                    if let Ok(sym) = Symbol::try_from_val(&env, &topic) {
                        return sym == Symbol::new(&env, "new_review_added");
                    }
                    false
                })
            } else {
                false
            }
        });
        if !found {
            let mut s = alloc::string::String::new();
            core::fmt::write(&mut s, format_args!("Events: {:?}", events)).ok();
            panic!("ReviewAdded event not found. {}", s);
        }
    });
}

#[test]
fn test_add_review_duplicate_fails() {
    let (env, contract_id, _admin) = setup_env_and_contract();
    let user = test_address(&env, "user2");
    let game_id = 2u32;
    let rating = 4u32;
    let comment = Bytes::from_slice(&env, b"Nice!");
    env.mock_all_auths();
    env.as_contract(&contract_id, || {
        GameReview::add_review(env.clone(), user.clone(), game_id, rating, comment.clone())
            .unwrap();
    });
    env.mock_all_auths();
    env.as_contract(&contract_id, || {
        // Try to add again
        let err = GameReview::add_review(env.clone(), user.clone(), game_id, rating, comment)
            .unwrap_err();
        assert_eq!(err, GameReviewError::UserHasReviewed);
    });
}

#[test]
fn test_add_review_invalid_rating() {
    let (env, contract_id, _admin) = setup_env_and_contract();
    let user = test_address(&env, "user3");
    let game_id = 3u32;
    let comment = Bytes::from_slice(&env, b"Bad rating");
    env.mock_all_auths();
    env.as_contract(&contract_id, || {
        // Rating too low
        let _err = GameReview::add_review(env.clone(), user.clone(), game_id, 0, comment.clone())
            .unwrap_err();
    });
    env.mock_all_auths();
    env.as_contract(&contract_id, || {
        // Rating too high
        let err =
            GameReview::add_review(env.clone(), user.clone(), game_id, 6, comment).unwrap_err();
        assert_eq!(err, GameReviewError::InvalidInput);
    });
}

#[test]
fn test_get_review_not_found() {
    let (env, contract_id, _admin) = setup_env_and_contract();
    let user = test_address(&env, "user4");
    let game_id = 4u32;
    env.as_contract(&contract_id, || {
        let err = GameReview::get_review(env.clone(), game_id, user).unwrap_err();
        assert_eq!(err, GameReviewError::ReviewNotFound);
    });
}

#[test]
fn test_has_reviewed_true_false() {
    let (env, contract_id, _admin) = setup_env_and_contract();
    let user = test_address(&env, "user5");
    let game_id = 5u32;
    let comment = Bytes::from_slice(&env, b"Test");
    env.as_contract(&contract_id, || {
        assert!(!GameReview::has_reviewed(
            env.clone(),
            user.clone(),
            game_id
        ));
        GameReview::add_review(env.clone(), user.clone(), game_id, 3, comment).unwrap();
        assert!(GameReview::has_reviewed(env.clone(), user.clone(), game_id));
    });
}

#[test]
fn test_get_game_review_count_and_rating() {
    let (env, contract_id, _admin) = setup_env_and_contract();
    let game_id = 6u32;
    env.as_contract(&contract_id, || {
        // No reviews
        assert_eq!(GameReview::get_game_review_count(env.clone(), game_id), 0);
        assert_eq!(GameReview::get_game_rating(env.clone(), game_id), 0);
        // Add reviews
        let user1 = test_address(&env, "user6a");
        let user2 = test_address(&env, "user6b");
        GameReview::add_review(
            env.clone(),
            user1.clone(),
            game_id,
            4,
            Bytes::from_slice(&env, b"Good"),
        )
        .unwrap();
        GameReview::add_review(
            env.clone(),
            user2.clone(),
            game_id,
            2,
            Bytes::from_slice(&env, b"Okay"),
        )
        .unwrap();
        assert_eq!(GameReview::get_game_review_count(env.clone(), game_id), 2);
        // Average should be (4+2)/2 = 3
        assert_eq!(GameReview::get_game_rating(env.clone(), game_id), 3);
    });
}

#[test]
fn test_empty_and_max_length_comment() {
    let (env, contract_id, _admin) = setup_env_and_contract();
    let user = test_address(&env, "user7a");
    let game_id = 7u32;
    env.as_contract(&contract_id, || {
        // Empty comment
        let empty = Bytes::from_slice(&env, b"");
        GameReview::add_review(env.clone(), user.clone(), game_id, 3, empty.clone()).unwrap();
        let review = GameReview::get_review(env.clone(), game_id, user.clone()).unwrap();
        assert_eq!(review.comment, empty);
        // Max length comment (arbitrary, e.g., 256 bytes)
        let mut max_vec = Vec::with_capacity(256);
        max_vec.resize(256, b'a');
        let max = Bytes::from_slice(&env, &max_vec);
        let user2 = test_address(&env, "user7b");
        GameReview::add_review(env.clone(), user2.clone(), game_id, 4, max.clone()).unwrap();
        let review2 = GameReview::get_review(env.clone(), game_id, user2.clone()).unwrap();
        assert_eq!(review2.comment, max);
    });
}

#[test]
fn test_multiple_users_and_games() {
    let (env, contract_id, _admin) = setup_env_and_contract();
    let user1 = test_address(&env, "user8a");
    let user2 = test_address(&env, "user8b");
    let game1 = 8u32;
    let game2 = 9u32;
    env.mock_all_auths();
    env.as_contract(&contract_id, || {
        GameReview::add_review(
            env.clone(),
            user1.clone(),
            game1,
            5,
            Bytes::from_slice(&env, b"A"),
        )
        .unwrap();
    });
    env.mock_all_auths();
    env.as_contract(&contract_id, || {
        GameReview::add_review(
            env.clone(),
            user2.clone(),
            game1,
            3,
            Bytes::from_slice(&env, b"B"),
        )
        .unwrap();
    });
    env.mock_all_auths();
    env.as_contract(&contract_id, || {
        GameReview::add_review(
            env.clone(),
            user1.clone(),
            game2,
            4,
            Bytes::from_slice(&env, b"C"),
        )
        .unwrap();
    });
    env.as_contract(&contract_id, || {
        // Check counts
        assert_eq!(GameReview::get_game_review_count(env.clone(), game1), 2);
        assert_eq!(GameReview::get_game_review_count(env.clone(), game2), 1);
        // Check ratings
        assert_eq!(GameReview::get_game_rating(env.clone(), game1), 4); // (5+3)/2
        assert_eq!(GameReview::get_game_rating(env.clone(), game2), 4);
    });
}

#[test]
fn test_get_reviews_pagination() {
    let (env, contract_id, _admin) = setup_env_and_contract();
    let game_id = 10u32;
    env.as_contract(&contract_id, || {
        let mut users = Vec::new();
        for i in 0..5 {
            users.push(test_address(&env, &format!("user10{}", i)));
        }
        for (i, user) in users.iter().enumerate() {
            let mut bytes = Vec::with_capacity(i + 1);
            bytes.resize(i + 1, b'x');
            let comment = Bytes::from_slice(&env, &bytes);
            GameReview::add_review(
                env.clone(),
                user.clone(),
                game_id,
                (i as u32 % 5) + 1,
                comment,
            )
            .unwrap();
        }
        // Get first 2 reviews
        let reviews = GameReview::get_reviews(env.clone(), game_id, 0, 2);
        assert_eq!(reviews.len(), 2);
        assert_eq!(reviews.get(0).unwrap().id, 1);
        assert_eq!(reviews.get(1).unwrap().id, 2);
        // Get next 3 reviews
        let reviews = GameReview::get_reviews(env.clone(), game_id, 2, 3);
        assert_eq!(reviews.len(), 3);
        assert_eq!(reviews.get(0).unwrap().id, 3);
        assert_eq!(reviews.get(2).unwrap().id, 5);
        // Skip beyond range
        let reviews = GameReview::get_reviews(env.clone(), game_id, 10, 2);
        assert_eq!(reviews.len(), 0);
    });
}

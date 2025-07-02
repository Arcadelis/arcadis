use soroban_sdk::contracterror;

#[contracterror]
#[derive(Debug, Eq, PartialEq)]
pub enum GameReviewError {
    /// Contract is already initialized
    AlreadyInitialized = 1,
    /// Input is invalid (e.g., rating out of range)
    InvalidInput = 2,
    /// User has already reviewed this game
    UserHasReviewed = 3,
    /// Review not found
    ReviewNotFound = 4,
    /// Unauthorized access or not initialized
    Unauthorized = 5,
}

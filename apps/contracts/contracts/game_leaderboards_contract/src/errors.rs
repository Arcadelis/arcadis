use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Tournament already exists
    TournamentExists = 1,
    /// Tournament not found
    TournamentNotFound = 2,
    /// Tournament is not currently active
    TournamentNotActive = 3,
    /// Tournament is full
    TournamentFull = 4,
    /// Player not found
    PlayerNotFound = 5,
    /// Invalid parameters
    InvalidParameters = 6,
    /// Unauthorized operation
    Unauthorized = 7,
} 
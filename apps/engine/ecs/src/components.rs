use soroban_sdk::{contracttype};

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: u32,
    pub y: u32,
} 
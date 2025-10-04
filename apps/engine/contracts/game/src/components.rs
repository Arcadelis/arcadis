//! Game Logic Components
//!
//! This module defines reusable ECS components for the game logic contract.
//! Components represent data that can be attached to entities in the game world.
//!
//! # Components
//! 
//! - `Position`: Represents the 2D coordinates of an entity in the game world
//! - `Health`: Represents the health points of an entity
//!
//! # Usage Example
//!
//! ```rust,ignore
//! use game::components::{Position, Health};
//!
//! // Create a position component at coordinates (10, 20)
//! let position = Position(10, 20);
//!
//! // Create a health component with 100 HP
//! let health = Health(100);
//! ```
//!
//! # Design Notes
//!
//! All components implement the `ComponentTrait` from soroban-ecs, which provides:
//! - Unique component type identifiers
//! - Serialization/deserialization for storage
//! - Integration with the ECS World

use soroban_sdk::{contracttype, symbol_short, Env, Symbol, Bytes};
use soroban_ecs::ComponentTrait;

/// Position component for entities in 2D space
///
/// Stores the x and y coordinates of an entity as unsigned 32-bit integers.
/// This component is used by the MovementSystem to track and update entity positions.
///
/// # Fields
/// 
/// - `0`: x-coordinate (horizontal position)
/// - `1`: y-coordinate (vertical position)
///
/// # Example
///
/// ```rust,ignore
/// let position = Position(10, 20);
/// // Entity is at x=10, y=20
/// ```
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Position(pub u32, pub u32);

impl ComponentTrait for Position {
    /// Returns the unique identifier for this component type
    fn component_type() -> Symbol {
        symbol_short!("position")
    }

    /// Serializes the Position component to bytes for storage
    ///
    /// Format: 8 bytes total (4 bytes for x, 4 bytes for y)
    /// Uses big-endian byte order for deterministic cross-platform behavior
    fn serialize(&self, env: &Env) -> Bytes {
        let mut bytes = Bytes::new(env);
        bytes.append(&Bytes::from_slice(env, &self.0.to_be_bytes()));
        bytes.append(&Bytes::from_slice(env, &self.1.to_be_bytes()));
        bytes
    }

    /// Deserializes bytes into a Position component
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `data` - The byte array to deserialize (must be exactly 8 bytes)
    ///
    /// # Returns
    ///
    /// - `Some(Position)` if deserialization is successful
    /// - `None` if the data is not exactly 8 bytes
    fn deserialize(env: &Env, data: &Bytes) -> Option<Self> {
        if data.len() != 8 {
            return None;
        }
        let x = u32::from_be_bytes([
            data.get(0).unwrap(),
            data.get(1).unwrap(),
            data.get(2).unwrap(),
            data.get(3).unwrap(),
        ]);
        let y = u32::from_be_bytes([
            data.get(4).unwrap(),
            data.get(5).unwrap(),
            data.get(6).unwrap(),
            data.get(7).unwrap(),
        ]);
        Some(Self(x, y))
    }
}

/// Health component for entities
///
/// Stores the current health points of an entity as an unsigned 32-bit integer.
/// This component is used by the CombatSystem to track and modify entity health.
///
/// # Fields
///
/// - `0`: Current health points
///
/// # Example
///
/// ```rust,ignore
/// let health = Health(100);
/// // Entity has 100 health points
/// ```
///
/// # Notes
///
/// - Health is clamped at 0 (entities cannot have negative health)
/// - When health reaches 0, the entity is considered dead
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Health(pub u32);

impl ComponentTrait for Health {
    /// Returns the unique identifier for this component type
    fn component_type() -> Symbol {
        symbol_short!("health")
    }

    /// Serializes the Health component to bytes for storage
    ///
    /// Format: 4 bytes (big-endian u32)
    /// Uses big-endian byte order for deterministic cross-platform behavior
    fn serialize(&self, env: &Env) -> Bytes {
        let mut bytes = Bytes::new(env);
        bytes.append(&Bytes::from_slice(env, &self.0.to_be_bytes()));
        bytes
    }

    /// Deserializes bytes into a Health component
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `data` - The byte array to deserialize (must be exactly 4 bytes)
    ///
    /// # Returns
    ///
    /// - `Some(Health)` if deserialization is successful
    /// - `None` if the data is not exactly 4 bytes
    fn deserialize(env: &Env, data: &Bytes) -> Option<Self> {
        if data.len() != 4 {
            return None;
        }
        let value = u32::from_be_bytes([
            data.get(0).unwrap(),
            data.get(1).unwrap(),
            data.get(2).unwrap(),
            data.get(3).unwrap(),
        ]);
        Some(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_position_serialization() {
        let env = Env::default();
        let pos = Position(100, 200);
        
        let bytes = pos.serialize(&env);
        assert_eq!(bytes.len(), 8);
        
        let deserialized = Position::deserialize(&env, &bytes).unwrap();
        assert_eq!(deserialized, pos);
    }

    #[test]
    fn test_position_component_type() {
        assert_eq!(Position::component_type(), symbol_short!("position"));
    }

    #[test]
    fn test_health_serialization() {
        let env = Env::default();
        let health = Health(100);
        
        let bytes = health.serialize(&env);
        assert_eq!(bytes.len(), 4);
        
        let deserialized = Health::deserialize(&env, &bytes).unwrap();
        assert_eq!(deserialized, health);
    }

    #[test]
    fn test_health_component_type() {
        assert_eq!(Health::component_type(), symbol_short!("health"));
    }

    #[test]
    fn test_position_invalid_deserialization() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, &[1, 2, 3]); // Wrong length
        
        let result = Position::deserialize(&env, &bytes);
        assert!(result.is_none());
    }

    #[test]
    fn test_health_invalid_deserialization() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, &[1, 2]); // Wrong length
        
        let result = Health::deserialize(&env, &bytes);
        assert!(result.is_none());
    }
}

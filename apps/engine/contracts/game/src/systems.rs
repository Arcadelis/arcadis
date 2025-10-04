//! Game Logic Systems
//!
//! This module defines reusable ECS systems for the game logic contract.
//! Systems contain the logic that operates on components and entities.
//!
//! # Systems
//!
//! - `MovementSystem`: Updates entity positions based on movement deltas
//! - `CombatSystem`: Modifies entity health based on combat actions
//!
//! # Usage Example
//!
//! ```rust,ignore
//! use game::components::{Position, Health};
//! use game::systems::{MovementSystem, CombatSystem};
//!
//! // Move an entity 5 units right, 3 units up
//! let current_pos = Position(10, 10);
//! let new_pos = MovementSystem::update_position(&current_pos, 5, 3);
//! assert_eq!(new_pos, Position(15, 13));
//!
//! // Attack an entity, dealing 10 damage
//! let current_health = Health(50);
//! let new_health = CombatSystem::apply_damage(&current_health, 10);
//! assert_eq!(new_health, Health(40));
//! ```
//!
//! # Design Notes
//!
//! - All systems are deterministic (no randomness, no floating-point operations)
//! - Systems use saturating arithmetic to prevent overflow/underflow
//! - Systems are stateless and operate purely on component data

use crate::components::{Position, Health};

/// Movement system for updating entity positions
///
/// This system handles position updates based on movement deltas (dx, dy).
/// It ensures that positions remain non-negative and uses deterministic
/// integer arithmetic suitable for blockchain execution.
///
/// # Examples
///
/// ```rust,ignore
/// use game::components::Position;
/// use game::systems::MovementSystem;
///
/// let pos = Position(10, 20);
/// let new_pos = MovementSystem::update_position(&pos, 5, -3);
/// assert_eq!(new_pos, Position(15, 17));
///
/// // Prevents negative coordinates
/// let pos = Position(5, 5);
/// let new_pos = MovementSystem::update_position(&pos, -10, -10);
/// assert_eq!(new_pos, Position(0, 0));
/// ```
pub struct MovementSystem;

impl MovementSystem {
    /// Updates an entity's position by applying movement deltas
    ///
    /// # Arguments
    ///
    /// * `position` - The current position of the entity
    /// * `dx` - The change in x-coordinate (can be negative)
    /// * `dy` - The change in y-coordinate (can be negative)
    ///
    /// # Returns
    ///
    /// A new `Position` with the updated coordinates. Coordinates are clamped
    /// to a minimum of 0 to prevent negative positions.
    ///
    /// # Determinism
    ///
    /// This function is fully deterministic and uses only integer arithmetic,
    /// making it safe for blockchain execution.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let pos = Position(10, 10);
    /// let new_pos = MovementSystem::update_position(&pos, -5, 5);
    /// assert_eq!(new_pos.0, 5);  // 10 + (-5) = 5
    /// assert_eq!(new_pos.1, 15); // 10 + 5 = 15
    /// ```
    pub fn update_position(position: &Position, dx: i32, dy: i32) -> Position {
        // Convert u32 to i32 for arithmetic, ensuring safe conversion
        let new_x = (position.0 as i32).saturating_add(dx).max(0) as u32;
        let new_y = (position.1 as i32).saturating_add(dy).max(0) as u32;
        
        Position(new_x, new_y)
    }

    /// Teleports an entity to a specific position
    ///
    /// # Arguments
    ///
    /// * `x` - The target x-coordinate
    /// * `y` - The target y-coordinate
    ///
    /// # Returns
    ///
    /// A new `Position` at the specified coordinates
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let new_pos = MovementSystem::teleport(100, 200);
    /// assert_eq!(new_pos, Position(100, 200));
    /// ```
    pub fn teleport(x: u32, y: u32) -> Position {
        Position(x, y)
    }

    /// Calculates the Manhattan distance between two positions
    ///
    /// Manhattan distance is the sum of absolute differences in coordinates,
    /// representing the distance when only horizontal and vertical movement
    /// is allowed (no diagonal movement).
    ///
    /// # Arguments
    ///
    /// * `pos1` - The first position
    /// * `pos2` - The second position
    ///
    /// # Returns
    ///
    /// The Manhattan distance as a u32
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let pos1 = Position(0, 0);
    /// let pos2 = Position(3, 4);
    /// let distance = MovementSystem::manhattan_distance(&pos1, &pos2);
    /// assert_eq!(distance, 7); // |3-0| + |4-0| = 7
    /// ```
    pub fn manhattan_distance(pos1: &Position, pos2: &Position) -> u32 {
        let dx = if pos1.0 > pos2.0 {
            pos1.0 - pos2.0
        } else {
            pos2.0 - pos1.0
        };
        let dy = if pos1.1 > pos2.1 {
            pos1.1 - pos2.1
        } else {
            pos2.1 - pos1.1
        };
        dx.saturating_add(dy)
    }
}

/// Combat system for managing entity health and damage
///
/// This system handles all combat-related operations including damage application,
/// healing, and health state queries. It uses saturating arithmetic to ensure
/// health values remain within valid bounds (0 to u32::MAX).
///
/// # Examples
///
/// ```rust,ignore
/// use game::components::Health;
/// use game::systems::CombatSystem;
///
/// let health = Health(100);
/// let damaged = CombatSystem::apply_damage(&health, 30);
/// assert_eq!(damaged, Health(70));
///
/// let healed = CombatSystem::heal(&damaged, 20);
/// assert_eq!(healed, Health(90));
/// ```
pub struct CombatSystem;

impl CombatSystem {
    /// Applies damage to an entity's health
    ///
    /// # Arguments
    ///
    /// * `health` - The current health of the entity
    /// * `damage` - The amount of damage to apply
    ///
    /// # Returns
    ///
    /// A new `Health` component with reduced health. Health is clamped at 0
    /// (saturating subtraction prevents underflow).
    ///
    /// # Determinism
    ///
    /// Uses saturating subtraction for deterministic behavior regardless of
    /// damage amount.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let health = Health(50);
    /// let new_health = CombatSystem::apply_damage(&health, 30);
    /// assert_eq!(new_health.0, 20);
    ///
    /// // Prevents underflow
    /// let new_health = CombatSystem::apply_damage(&health, 100);
    /// assert_eq!(new_health.0, 0);
    /// ```
    pub fn apply_damage(health: &Health, damage: u32) -> Health {
        Health(health.0.saturating_sub(damage))
    }

    /// Default attack that applies 10 damage
    ///
    /// This is a convenience method for the standard attack action.
    ///
    /// # Arguments
    ///
    /// * `health` - The current health of the entity
    ///
    /// # Returns
    ///
    /// A new `Health` component with 10 points of damage applied
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let health = Health(100);
    /// let new_health = CombatSystem::attack(&health);
    /// assert_eq!(new_health.0, 90);
    /// ```
    pub fn attack(health: &Health) -> Health {
        Self::apply_damage(health, 10)
    }

    /// Heals an entity's health
    ///
    /// # Arguments
    ///
    /// * `health` - The current health of the entity
    /// * `heal_amount` - The amount of health to restore
    ///
    /// # Returns
    ///
    /// A new `Health` component with increased health. Uses saturating addition
    /// to prevent overflow.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let health = Health(50);
    /// let new_health = CombatSystem::heal(&health, 30);
    /// assert_eq!(new_health.0, 80);
    ///
    /// // Prevents overflow
    /// let max_health = Health(u32::MAX - 10);
    /// let new_health = CombatSystem::heal(&max_health, 100);
    /// assert_eq!(new_health.0, u32::MAX);
    /// ```
    pub fn heal(health: &Health, heal_amount: u32) -> Health {
        Health(health.0.saturating_add(heal_amount))
    }

    /// Checks if an entity is alive
    ///
    /// # Arguments
    ///
    /// * `health` - The health component to check
    ///
    /// # Returns
    ///
    /// `true` if health is greater than 0, `false` otherwise
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let health = Health(10);
    /// assert!(CombatSystem::is_alive(&health));
    ///
    /// let dead = Health(0);
    /// assert!(!CombatSystem::is_alive(&dead));
    /// ```
    pub fn is_alive(health: &Health) -> bool {
        health.0 > 0
    }

    /// Checks if an entity is dead
    ///
    /// # Arguments
    ///
    /// * `health` - The health component to check
    ///
    /// # Returns
    ///
    /// `true` if health is 0, `false` otherwise
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let health = Health(0);
    /// assert!(CombatSystem::is_dead(&health));
    ///
    /// let alive = Health(1);
    /// assert!(!CombatSystem::is_dead(&alive));
    /// ```
    pub fn is_dead(health: &Health) -> bool {
        health.0 == 0
    }

    /// Sets an entity's health to a specific value
    ///
    /// # Arguments
    ///
    /// * `value` - The new health value
    ///
    /// # Returns
    ///
    /// A new `Health` component with the specified value
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let full_health = CombatSystem::set_health(100);
    /// assert_eq!(full_health.0, 100);
    /// ```
    pub fn set_health(value: u32) -> Health {
        Health(value)
    }

    /// **[DEPRECATED]** Updates health by applying 10 damage
    ///
    /// **Note:** This function is deprecated. Use `attack()` instead.
    ///
    /// This function exists for backward compatibility with existing code.
    /// It applies a fixed 10 points of damage to the entity's health.
    ///
    /// # Arguments
    ///
    /// * `health` - The current health of the entity
    ///
    /// # Returns
    ///
    /// A new `Health` component with 10 damage applied
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let health = Health(100);
    /// let new_health = CombatSystem::update_health(&health);
    /// assert_eq!(new_health.0, 90);
    /// ```
    #[deprecated(since = "0.2.0", note = "Use `attack()` instead")]
    pub fn update_health(health: &Health) -> Health {
        Self::attack(health)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // MovementSystem tests
    #[test]
    fn test_movement_positive_delta() {
        let pos = Position(10, 20);
        let new_pos = MovementSystem::update_position(&pos, 5, 3);
        assert_eq!(new_pos, Position(15, 23));
    }

    #[test]
    fn test_movement_negative_delta() {
        let pos = Position(10, 20);
        let new_pos = MovementSystem::update_position(&pos, -5, -3);
        assert_eq!(new_pos, Position(5, 17));
    }

    #[test]
    fn test_movement_prevents_negative_coordinates() {
        let pos = Position(5, 5);
        let new_pos = MovementSystem::update_position(&pos, -10, -10);
        assert_eq!(new_pos, Position(0, 0));
    }

    #[test]
    fn test_teleport() {
        let new_pos = MovementSystem::teleport(100, 200);
        assert_eq!(new_pos, Position(100, 200));
    }

    #[test]
    fn test_manhattan_distance() {
        let pos1 = Position(0, 0);
        let pos2 = Position(3, 4);
        assert_eq!(MovementSystem::manhattan_distance(&pos1, &pos2), 7);

        let pos3 = Position(10, 10);
        let pos4 = Position(10, 10);
        assert_eq!(MovementSystem::manhattan_distance(&pos3, &pos4), 0);
    }

    // CombatSystem tests
    #[test]
    fn test_apply_damage() {
        let health = Health(100);
        let new_health = CombatSystem::apply_damage(&health, 30);
        assert_eq!(new_health, Health(70));
    }

    #[test]
    fn test_apply_damage_prevents_underflow() {
        let health = Health(10);
        let new_health = CombatSystem::apply_damage(&health, 100);
        assert_eq!(new_health, Health(0));
    }

    #[test]
    fn test_attack() {
        let health = Health(100);
        let new_health = CombatSystem::attack(&health);
        assert_eq!(new_health, Health(90));
    }

    #[test]
    fn test_heal() {
        let health = Health(50);
        let new_health = CombatSystem::heal(&health, 30);
        assert_eq!(new_health, Health(80));
    }

    #[test]
    fn test_heal_prevents_overflow() {
        let health = Health(u32::MAX - 10);
        let new_health = CombatSystem::heal(&health, 100);
        assert_eq!(new_health, Health(u32::MAX));
    }

    #[test]
    fn test_is_alive() {
        assert!(CombatSystem::is_alive(&Health(1)));
        assert!(CombatSystem::is_alive(&Health(100)));
        assert!(!CombatSystem::is_alive(&Health(0)));
    }

    #[test]
    fn test_is_dead() {
        assert!(CombatSystem::is_dead(&Health(0)));
        assert!(!CombatSystem::is_dead(&Health(1)));
        assert!(!CombatSystem::is_dead(&Health(100)));
    }

    #[test]
    fn test_set_health() {
        let health = CombatSystem::set_health(75);
        assert_eq!(health, Health(75));
    }

    #[test]
    fn test_combat_scenario() {
        // Initial health
        let mut health = Health(100);
        
        // Take damage
        health = CombatSystem::apply_damage(&health, 30);
        assert_eq!(health.0, 70);
        
        // Heal
        health = CombatSystem::heal(&health, 20);
        assert_eq!(health.0, 90);
        
        // Multiple attacks
        health = CombatSystem::attack(&health);
        health = CombatSystem::attack(&health);
        assert_eq!(health.0, 70);
        
        assert!(CombatSystem::is_alive(&health));
    }
}

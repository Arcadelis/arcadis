use crate::components::Position;

pub struct MovementSystem;

impl MovementSystem {
    pub fn update(pos: &Position, dx: i32, dy: i32) -> Position {
        Position {
            x: (pos.x as i32 + dx).max(0) as u32,
            y: (pos.y as i32 + dy).max(0) as u32,
        }
    }
} 
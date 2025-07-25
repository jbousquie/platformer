//! Blocks Module
//!
//! This module defines the blocks that appear in the game world.

use crate::constants::{BLOCK_SIZE, GRAVITY};
use macroquad::prelude::*;

/// Represents the state of a block.
#[derive(PartialEq, Clone)]
pub enum BlockState {
    Idle,
    Hooked,
}

/// Represents a block in the game world.
#[derive(Clone)]
pub struct Block {
    pub position: Vec2,
    pub size: Vec2,
    pub velocity: Vec2,
    pub on_ground: bool,
    pub state: BlockState,
}

impl Block {
    /// Creates a new block at a specific position.
    pub fn new(pos: Vec2) -> Self {
        Self {
            position: pos,
            size: vec2(BLOCK_SIZE, BLOCK_SIZE),
            velocity: Vec2::ZERO,
            on_ground: false,
            state: BlockState::Idle,
        }
    }

    /// Returns the block's bounding box as a `Rect`.
    pub fn rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.size.x, self.size.y)
    }

    /// Updates the block's state, applying gravity only if it's not on the ground.
    pub fn update(&mut self, dt: f32) {
        if !self.on_ground {
            self.velocity.y += GRAVITY * dt;
            self.position += self.velocity * dt;
        }
    }

    /// Draws the block on the screen.
    pub fn draw(&self) {
        let color = if self.state == BlockState::Hooked {
            YELLOW
        } else {
            ORANGE
        };
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            color,
        );
    }
}

//! Items Module
//!
//! This module defines the items that appear in the game world.

use crate::constants::{
    GRAVITY, ITEM_HOOKED_COLOR, ITEM_IDLE_COLOR, ITEM_SIZE, ITEM_THROWN_COLOR,
};
use macroquad::prelude::*;

/// Represents the state of an item.
#[derive(PartialEq)]
pub enum ItemState {
    Idle,
    Hooked,
    Thrown,
}

/// Represents an item in the game world.
pub struct Item {
    pub position: Vec2,
    pub size: Vec2,
    pub velocity: Vec2,
    pub on_ground: bool,
    pub state: ItemState,
}

impl Item {
    /// Creates a new item at a specific position.
    pub fn new(pos: Vec2) -> Self {
        Self {
            position: pos,
            size: vec2(ITEM_SIZE, ITEM_SIZE),
            velocity: Vec2::ZERO,
            on_ground: false,
            state: ItemState::Idle,
        }
    }

    /// Returns the item's bounding box as a `Rect`.
    pub fn rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.size.x, self.size.y)
    }

    /// Updates the item's state, applying gravity only if it's not on the ground.
    pub fn update(&mut self, dt: f32) {
        if !self.on_ground {
            self.velocity.y += GRAVITY * dt;
            self.position += self.velocity * dt;
        }
    }

    /// Draws the item on the screen.
    pub fn draw(&self) {
        let color = match self.state {
            ItemState::Idle => ITEM_IDLE_COLOR,
            ItemState::Hooked => ITEM_HOOKED_COLOR,
            ItemState::Thrown => ITEM_THROWN_COLOR,
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

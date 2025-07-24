//! Player Module
//!
//! This module defines the player's behavior and properties.

use crate::constants::{GRAVITY, GROUND_HEIGHT, JUMP_FORCE, PLAYER_SIZE, PLAYER_SPEED};
use crate::level::LEVEL_HEIGHT;
use macroquad::prelude::*;

/// Represents the different states the player can be in.
pub enum PlayerState {
    Idle,
    Run,
    Jump,
    Fall,
}

/// Represents the player character in the game.
pub struct Player {
    pub position: Vec2,
    pub size: Vec2,
    pub velocity: Vec2,
    pub on_ground: bool,
    pub state: PlayerState,
    pub facing_right: bool,
}

impl Player {
    /// Creates a new player instance with default values.
    pub fn new() -> Self {
        Self {
            position: vec2(100., LEVEL_HEIGHT - GROUND_HEIGHT - PLAYER_SIZE),
            size: vec2(PLAYER_SIZE, PLAYER_SIZE),
            velocity: Vec2::new(0., 0.),
            on_ground: false,
            state: PlayerState::Idle,
            facing_right: true,
        }
    }

    /// Returns the player's bounding box as a `Rect`.
    pub fn rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.size.x, self.size.y)
    }

    /// Updates the player's state, including position, velocity, and state, based on input and physics.
    pub fn update(&mut self, dt: f32) {
        // Apply gravity
        self.velocity.y += GRAVITY * dt;

        // Handle input
        if is_key_down(KeyCode::Right) {
            self.velocity.x = PLAYER_SPEED;
            self.facing_right = true;
        } else if is_key_down(KeyCode::Left) {
            self.velocity.x = -PLAYER_SPEED;
            self.facing_right = false;
        } else {
            self.velocity.x = 0.;
        }

        if is_key_pressed(KeyCode::Up) && self.on_ground {
            self.velocity.y = -JUMP_FORCE;
            self.on_ground = false;
        }

        // Update position
        self.position += self.velocity * dt;

        // Update state
        if self.on_ground {
            if self.velocity.x.abs() > 0.1 {
                self.state = PlayerState::Run;
            } else {
                self.state = PlayerState::Idle;
            }
        } else {
            if self.velocity.y < 0. {
                self.state = PlayerState::Jump;
            } else {
                self.state = PlayerState::Fall;
            }
        }
    }

    /// Draws the player on the screen.
    pub fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            WHITE,
        );
    }
}

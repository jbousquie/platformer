//! Baddies Module
//!
//! This module defines the baddie's behavior and properties.

use crate::constants::{
    BADDIE_COLOR, BADDIE_JUMP_CHANCE, BADDIE_JUMP_FORCE, BADDIE_SIZE, BADDIE_SPEED, GRAVITY,
};
use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};

/// Represents the different states a baddie can be in.
pub enum BaddieState {
    Idle,
    Run,
    Jump,
    Fall,
}

/// Represents a baddie character in the game.
pub struct Baddie {
    pub position: Vec2,
    pub size: Vec2,
    pub velocity: Vec2,
    pub on_ground: bool,
    pub state: BaddieState,
    pub facing_right: bool,
}

impl Baddie {
    /// Creates a new baddie instance at a given position.
    pub fn new(pos: Vec2) -> Self {
        Self {
            position: pos,
            size: vec2(BADDIE_SIZE, BADDIE_SIZE),
            velocity: vec2(BADDIE_SPEED, 0.),
            on_ground: false,
            state: BaddieState::Run,
            facing_right: thread_rng().gen_bool(0.5),
        }
    }

    /// Returns the baddie's bounding box as a `Rect`.
    pub fn rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.size.x, self.size.y)
    }

    /// Updates the baddie's state, including position, velocity, and state, based on physics.
    pub fn update(&mut self, dt: f32) {
        // Apply gravity
        self.velocity.y += GRAVITY * dt;

        // Set horizontal velocity based on direction
        self.velocity.x = if self.facing_right {
            BADDIE_SPEED
        } else {
            -BADDIE_SPEED
        };

        // Randomly jump if on the ground
        if self.on_ground && thread_rng().gen_range(0.0..1.0) < BADDIE_JUMP_CHANCE {
            self.velocity.y = -BADDIE_JUMP_FORCE;
            self.on_ground = false;
        }

        // Update position
        self.position += self.velocity * dt;

        // Update state
        if self.on_ground {
            if self.velocity.x.abs() > 0.1 {
                self.state = BaddieState::Run;
            } else {
                self.state = BaddieState::Idle;
            }
        } else {
            if self.velocity.y < 0. {
                self.state = BaddieState::Jump;
            } else {
                self.state = BaddieState::Fall;
            }
        }
    }

    /// Draws the baddie on the screen.
    pub fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            BADDIE_COLOR,
        );
    }

    /// Reverses the baddie's horizontal direction.
    pub fn change_direction(&mut self) {
        self.facing_right = !self.facing_right;
    }
}
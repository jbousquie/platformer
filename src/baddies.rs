//! Baddies Module
//!
//! This module defines the baddie's behavior and properties.

use crate::constants::{
    BADDIE_COLOR, BADDIE_ELEVATION_SINE_AMPLITUDE, BADDIE_ELEVATION_SINE_FREQUENCY,
    BADDIE_ELEVATION_SPEED, BADDIE_ELEVATION_THRESHOLD, BADDIE_JUMP_CHANCE, BADDIE_JUMP_FORCE,
    BADDIE_SIZE, BADDIE_SPEED, GRAVITY,
};
use ::rand::{Rng, rng};
use macroquad::prelude::*;

/// Represents the different states a baddie can be in.
#[derive(PartialEq)]
pub enum BaddieState {
    Idle,
    Run,
    Jump,
    Fall,
    Grab,
    Elevation,
}

/// Represents a baddie character in the game.
pub struct Baddie {
    pub position: Vec2,
    pub size: Vec2,
    pub velocity: Vec2,
    pub on_ground: bool,
    pub state: BaddieState,
    pub facing_right: bool,
    pub on_ground_frames: u32,
    pub elevation_x_axis: f32,
    pub elevation_time: f32,
    pub grabbed_block_id: Option<usize>,
    pub grab_timer: f32,
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
            facing_right: rng().random_bool(0.5),
            on_ground_frames: 0,
            elevation_x_axis: 0.0,
            elevation_time: 0.0,
            grabbed_block_id: None,
            grab_timer: 0.0,
        }
    }

    /// Returns the baddie's bounding box as a `Rect`.
    pub fn rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.size.x, self.size.y)
    }

    /// Updates the baddie's state, including position, velocity, and state, based on physics.
    pub fn update(&mut self, dt: f32) {
        if self.state == BaddieState::Elevation {
            self.velocity.y = BADDIE_ELEVATION_SPEED;
            self.velocity.x = 0.0;
            self.elevation_time += dt;
            self.position.x = self.elevation_x_axis
                + (self.elevation_time * BADDIE_ELEVATION_SINE_FREQUENCY).sin()
                    * BADDIE_ELEVATION_SINE_AMPLITUDE;
        } else if self.state == BaddieState::Grab {
            self.velocity.x = if self.facing_right {
                BADDIE_SPEED
            } else {
                -BADDIE_SPEED
            };
            self.grab_timer -= dt;
            if self.grab_timer <= 0.0 {
                self.grabbed_block_id = None;
                self.state = BaddieState::Idle;
            }
        } else {
            // Apply gravity
            self.velocity.y += GRAVITY * dt;

            // Set horizontal velocity based on direction
            self.velocity.x = if self.facing_right {
                BADDIE_SPEED
            } else {
                -BADDIE_SPEED
            };

            // Randomly jump if on the ground
            if self.on_ground && rng().random_range(0.0..1.0) < BADDIE_JUMP_CHANCE {
                self.velocity.y = -BADDIE_JUMP_FORCE;
                self.on_ground = false;
            }
        }

        // Update position
        self.position += self.velocity * dt;

        // Update state
        if self.on_ground {
            self.on_ground_frames += 1;
            if self.velocity.x.abs() > 0.1 {
                if self.state != BaddieState::Grab {
                    self.state = BaddieState::Run;
                }
            } else {
                if self.state != BaddieState::Grab {
                    self.state = BaddieState::Idle;
                }
            }
        } else {
            self.on_ground_frames = 0;
            if self.state != BaddieState::Elevation && self.state != BaddieState::Grab {
                if self.velocity.y < 0. {
                    self.state = BaddieState::Jump;
                } else {
                    self.state = BaddieState::Fall;
                }
            }
        }

        if self.on_ground_frames > BADDIE_ELEVATION_THRESHOLD {
            self.state = BaddieState::Elevation;
            self.elevation_x_axis = self.position.x;
            self.on_ground_frames = 0;
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

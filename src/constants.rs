//! Constants Module
//!
//! This module contains constants used throughout the game.

use macroquad::prelude::Color;

pub const PLAYER_SIZE: f32 = 50.;
pub const PLAYER_SPEED: f32 = 500.;
pub const JUMP_FORCE: f32 = 600.;
pub const GRAVITY: f32 = 1000.;
pub const PLAYER_SPAWN_X: f32 = 100.0;
pub const PLAYER_SAFE_ZONE_MULTIPLIER: f32 = 3.0;
pub const SCREEN_QUARTER_WIDTH_FACTOR: f32 = 0.25;
pub const SCREEN_QUARTER_HEIGHT_FACTOR: f32 = 0.25;
pub const GROUND_HEIGHT: f32 = 50.;
pub const CEILING_HEIGHT: f32 = 50.;
pub const WALL_WIDTH: f32 = 50.;
pub const ITEM_SIZE: f32 = 25.0;
pub const ITEM_COUNT: usize = 8;
pub const ITEM_THROW_SPEED: f32 = 600.0;
/// The small distance to offset a thrown item from the thrower to prevent immediate self-collision.
pub const ITEM_THROW_OFFSET: f32 = 1.0;
pub const ITEM_BOUNCE_ENERGY_LOSS: f32 = 0.7;
pub const ITEM_MIN_BOUNCE_SPEED: f32 = 60.0;
pub const BLOCK_SIZE: f32 = PLAYER_SIZE * 1.2;
pub const BLOCK_COUNT: usize = 27;
pub const BLOCK_OFFSET: f32 = 20.;
pub const MAX_BADDIES: usize = 8;
pub const BADDIE_SIZE: f32 = PLAYER_SIZE * 0.8;
pub const BADDIE_SPEED: f32 = 200.;
pub const BADDIE_JUMP_FORCE: f32 = 600.;
pub const BADDIE_JUMP_CHANCE: f32 = 0.005; // Roughly once every 20 seconds at 60fps
pub const BADDIE_ELEVATION_THRESHOLD: u32 = 2000;
pub const BADDIE_ELEVATION_SPEED: f32 = -150.0;
pub const BADDIE_ELEVATION_SINE_AMPLITUDE: f32 = 40.0;
pub const BADDIE_ELEVATION_SINE_FREQUENCY: f32 = 4.0;
pub const BADDIE_ELEVATION_DROP_CHANCE: f32 = 0.001; // 0.1% chance per frame
pub const BADDIE_GRAB_CHANCE: f32 = 0.05;
pub const BADDIE_MIN_GRAB_DURATION: f32 = 3.0;
pub const BADDIE_MAX_GRAB_DURATION: f32 = 10.0;
pub const BADDIE_MIN_ITEM_HOLD_DURATION: f32 = 1.0;
pub const BADDIE_MAX_ITEM_HOLD_DURATION: f32 = 2.0;
pub const BADDIE_GRAB_ITEM_CHANCE: f32 = 0.6;

// --- Colors
pub const PLAYER_COLOR: Color = Color::new(1.0, 1.0, 1.0, 1.0); // WHITE
pub const BADDIE_COLOR: Color = Color::new(0.5, 0.5, 1.0, 1.0); // Light Blue
pub const BACKGROUND_COLOR: Color = Color::new(0.0, 0.0, 0.0, 1.0); // BLACK
pub const BOUNDS_COLOR: Color = Color::new(1.0, 1.0, 0.0, 1.0); // YELLOW
pub const PLATFORM_COLOR: Color = Color::new(0.0, 1.0, 0.0, 1.0); // GREEN
pub const ITEM_IDLE_COLOR: Color = Color::new(0.0, 0.0, 1.0, 1.0); // BLUE
pub const ITEM_HOOKED_COLOR: Color = Color::new(1.0, 1.0, 0.0, 1.0); // YELLOW
pub const ITEM_THROWN_COLOR: Color = Color::new(1.0, 0.0, 0.0, 1.0); // RED

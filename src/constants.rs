//! Constants Module
//!
//! This module contains constants used throughout the game.

use macroquad::prelude::Color;

pub const PLAYER_SIZE: f32 = 50.;
pub const PLAYER_SPEED: f32 = 500.;
pub const JUMP_FORCE: f32 = 800.;
pub const GRAVITY: f32 = 1000.;
pub const PLAYER_SPAWN_X: f32 = 100.0;
pub const PLAYER_SAFE_ZONE_MULTIPLIER: f32 = 3.0;
pub const SCREEN_QUARTER_WIDTH_FACTOR: f32 = 0.25;
pub const SCREEN_QUARTER_HEIGHT_FACTOR: f32 = 0.25;
pub const GROUND_HEIGHT: f32 = 50.;
pub const CEILING_HEIGHT: f32 = 50.;
pub const WALL_WIDTH: f32 = 50.;
pub const ITEM_SIZE: f32 = 25.0;
pub const ITEM_COUNT: usize = 12;
pub const ITEM_THROW_SPEED: f32 = 600.0;
pub const ITEM_BOUNCE_ENERGY_LOSS: f32 = 0.7;
pub const ITEM_MIN_BOUNCE_SPEED: f32 = 60.0;
pub const BLOCK_SIZE: f32 = PLAYER_SIZE * 1.2;
pub const BLOCK_COUNT: usize = 40;
pub const BLOCK_OFFSET: f32 = 20.;

// --- Colors
pub const PLAYER_COLOR: Color = Color::new(1.0, 1.0, 1.0, 1.0); // WHITE
pub const BACKGROUND_COLOR: Color = Color::new(0.0, 0.0, 0.0, 1.0); // BLACK
pub const BOUNDS_COLOR: Color = Color::new(1.0, 1.0, 0.0, 1.0); // YELLOW
pub const PLATFORM_COLOR: Color = Color::new(0.0, 1.0, 0.0, 1.0); // GREEN

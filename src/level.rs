//! Level Module
//!
//! This module defines the game world's structure and layout.

use crate::blocks::Block;
use crate::constants::*;
use crate::items::Item;
use crate::keys::Key;
use macroquad::prelude::*;
use macroquad::rand;

pub const LEVEL_WIDTH: f32 = 2. * 1024.;
pub const LEVEL_HEIGHT: f32 = 2. * 768.;

/// Represents the game level, including its boundaries and platforms.
pub struct Level {
    pub ground: Rect,
    pub ceiling: Rect,
    pub left_wall: Rect,
    pub right_wall: Rect,
    pub platforms: Vec<Rect>,
    pub items: Vec<Item>,
    pub blocks: Vec<Block>,
    pub keys: Vec<Key>,
    pub total_keys: u32,
}

impl Level {
    /// Creates a new level instance, populating it with platforms and defining its boundaries.
    pub async fn new() -> Self {
        let mut platforms = vec![];
        let screen_width = 1024.;
        let screen_height = 768.;

        for i in 0..2 {
            // columns
            for j in 0..2 {
                // rows
                let offset_x = i as f32 * screen_width;
                let offset_y = j as f32 * screen_height;

                // Define the platform layout relative to a screen's top-left corner
                let base_platforms = vec![
                    Rect::new(200., 120., 200., 20.),
                    Rect::new(500., 360., 200., 20.),
                    Rect::new(800., 568., 200., 20.),
                ];

                for platform in &base_platforms {
                    platforms.push(Rect::new(
                        offset_x + platform.x,
                        offset_y + platform.y,
                        platform.w,
                        platform.h,
                    ));
                }
            }
        }

        let mut items = vec![];
        for _ in 0..ITEM_COUNT {
            items.push(Item::new(vec2(
                rand::gen_range(WALL_WIDTH, LEVEL_WIDTH - WALL_WIDTH - ITEM_SIZE),
                rand::gen_range(CEILING_HEIGHT, LEVEL_HEIGHT - GROUND_HEIGHT - ITEM_SIZE),
            )));
        }

        let mut blocks = vec![];
        let player_spawn_rect = Rect::new(
            PLAYER_SPAWN_X,
            LEVEL_HEIGHT - GROUND_HEIGHT - PLAYER_SIZE,
            PLAYER_SIZE,
            PLAYER_SIZE,
        );
        let safe_zone_margin = (PLAYER_SIZE * PLAYER_SAFE_ZONE_MULTIPLIER - PLAYER_SIZE) / 2.0;
        let player_safe_zone = Rect::new(
            player_spawn_rect.x - safe_zone_margin,
            CEILING_HEIGHT,
            player_spawn_rect.w + safe_zone_margin * 2.0,
            LEVEL_HEIGHT - GROUND_HEIGHT - CEILING_HEIGHT,
        );

        for _ in 0..BLOCK_COUNT {
            let mut block_pos;
            loop {
                block_pos = vec2(
                    rand::gen_range(WALL_WIDTH, LEVEL_WIDTH - WALL_WIDTH - BLOCK_SIZE),
                    rand::gen_range(CEILING_HEIGHT, LEVEL_HEIGHT - GROUND_HEIGHT - BLOCK_SIZE),
                );
                let block_rect = Rect::new(block_pos.x, block_pos.y, BLOCK_SIZE, BLOCK_SIZE);
                if !block_rect.overlaps(&player_safe_zone) {
                    break;
                }
            }
            blocks.push(Block::new(block_pos));
        }

        let mut keys = vec![];
        let key_size = PLAYER_SIZE * 1.2;
        for i in 0..2 {
            // columns
            for j in 0..2 {
                // rows
                let offset_x = i as f32 * screen_width;
                let offset_y = j as f32 * screen_height;
                keys.push(Key::new(
                    vec2(
                        offset_x + screen_width * 0.95 - key_size / 2.0,
                        offset_y + screen_height * 0.15 - key_size / 2.0,
                    ),
                    key_size,
                ));
            }
        }
        let total_keys = keys.len() as u32;

        Self {
            ground: Rect::new(0., LEVEL_HEIGHT - GROUND_HEIGHT, LEVEL_WIDTH, GROUND_HEIGHT),
            ceiling: Rect::new(0., 0., LEVEL_WIDTH, CEILING_HEIGHT),
            left_wall: Rect::new(0., 0., WALL_WIDTH, LEVEL_HEIGHT),
            right_wall: Rect::new(LEVEL_WIDTH - WALL_WIDTH, 0., WALL_WIDTH, LEVEL_HEIGHT),
            platforms,
            items,
            blocks,
            keys,
            total_keys,
        }
    }

    /// Draws the level, including boundaries and platforms.
    pub fn draw(&self) {
        // Draw bounds
        draw_rectangle(
            self.ground.x,
            self.ground.y,
            self.ground.w,
            self.ground.h,
            BOUNDS_COLOR,
        );
        draw_rectangle(
            self.ceiling.x,
            self.ceiling.y,
            self.ceiling.w,
            self.ceiling.h,
            BOUNDS_COLOR,
        );
        draw_rectangle(
            self.left_wall.x,
            self.left_wall.y,
            self.left_wall.w,
            self.left_wall.h,
            BOUNDS_COLOR,
        );
        draw_rectangle(
            self.right_wall.x,
            self.right_wall.y,
            self.right_wall.w,
            self.right_wall.h,
            BOUNDS_COLOR,
        );

        // Draw platforms
        for platform in &self.platforms {
            draw_rectangle(
                platform.x,
                platform.y,
                platform.w,
                platform.h,
                PLATFORM_COLOR,
            );
        }

        // Draw blocks
        for block in &self.blocks {
            block.draw();
        }

        // Draw keys
        for key in &self.keys {
            key.draw();
        }

        // Draw items
        for item in &self.items {
            item.draw();
        }
    }
}

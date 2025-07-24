//! Camera Module
//!
//! This module defines the camera that follows the player.

use crate::constants::{SCREEN_QUARTER_HEIGHT_FACTOR, SCREEN_QUARTER_WIDTH_FACTOR};
use crate::level::{LEVEL_HEIGHT, LEVEL_WIDTH};
use crate::player::Player;
use macroquad::prelude::*;

/// Represents the game camera, which follows the player.
pub struct Camera {
    pub rect: Rect,
}

impl Camera {
    /// Creates a new camera instance.
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                0.,
                LEVEL_HEIGHT - screen_height(),
                screen_width(),
                screen_height(),
            ),
        }
    }

    /// Updates the camera's position to follow the player, clamping it to the level bounds.
    pub fn update(&mut self, player: &Player) {
        let screen_quarter_w = screen_width() * SCREEN_QUARTER_WIDTH_FACTOR;
        let screen_quarter_h = screen_height() * SCREEN_QUARTER_HEIGHT_FACTOR;
        let player_rect = player.rect();

        // Scroll left
        if player_rect.x < self.rect.x - screen_quarter_w {
            self.rect.x = player_rect.x + screen_quarter_w;
        }
        // Scroll right
        if player_rect.right() > self.rect.right() + screen_quarter_w
            && self.rect.right() < LEVEL_WIDTH
        {
            self.rect.x = player_rect.right() - self.rect.w - screen_quarter_w;
        }
        // Scroll up
        if player_rect.y < self.rect.y + screen_quarter_h && self.rect.y > 0. {
            self.rect.y = player_rect.y - screen_quarter_h;
        }
        // Scroll down
        if player_rect.bottom() > self.rect.bottom() - screen_quarter_h
            && self.rect.bottom() < LEVEL_HEIGHT
        {
            self.rect.y = player_rect.bottom() - self.rect.h + screen_quarter_h;
        }

        // Clamp camera to level bounds
        self.rect.x = self.rect.x.max(0.).min(LEVEL_WIDTH - self.rect.w);
        self.rect.y = self.rect.y.max(0.).min(LEVEL_HEIGHT - self.rect.h);
    }
}

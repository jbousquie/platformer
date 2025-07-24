//! Items Module
//!
//! This module defines the items that appear in the game world.

use crate::constants::{GRAVITY, ITEM_SIZE};
use crate::physics;
use crate::player::Player;
use macroquad::prelude::*;

/// Represents an item in the game world.
pub struct Item {
    pub position: Vec2,
    pub size: Vec2,
    pub velocity: Vec2,
    pub on_ground: bool,
    pub grabbed: bool,
}

impl Item {
    /// Creates a new item at a specific position.
    pub fn new(pos: Vec2) -> Self {
        Self {
            position: pos,
            size: vec2(ITEM_SIZE, ITEM_SIZE),
            velocity: Vec2::ZERO,
            on_ground: false,
            grabbed: false,
        }
    }

    /// Returns the item's bounding box as a `Rect`.
    pub fn rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.size.x, self.size.y)
    }

    /// Updates the item's state, applying gravity only if it's not grabbed.
    pub fn update(&mut self, dt: f32) {
        if !self.grabbed {
            self.velocity.y += GRAVITY * dt;
            self.position += self.velocity * dt;
        }
    }

    /// Draws the item on the screen.
    pub fn draw(&self) {
        let color = if self.grabbed { YELLOW } else { BLUE };
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            color,
        );
    }
}

/// Handles all item-related logic for a frame, including grab/release, physics, and position updates.
pub fn process_items(
    items: &mut [Item],
    player: &Player,
    ground: &Rect,
    platforms: &[Rect],
    dt: f32,
) {
    let player_rect = player.rect();
    let space_pressed = is_key_pressed(KeyCode::Space);

    // Handle grab/release action, which is a single event per key press.
    if space_pressed {
        let mut action_taken = false;
        // First, try to release a grabbed item.
        for item in items.iter_mut() {
            if item.grabbed {
                item.grabbed = false;
                action_taken = true;
                break; // Only release one item per press.
            }
        }

        // If no item was released, try to grab a new one.
        if !action_taken {
            for item in items.iter_mut() {
                if !item.grabbed && player_rect.overlaps(&item.rect()) {
                    item.grabbed = true;
                    item.velocity = Vec2::ZERO;
                    break; // Only grab one item per press.
                }
            }
        }
    }

    // Update position and state of all items.
    for item in items.iter_mut() {
        if item.grabbed {
            // Item is hooked to the player.
            item.position.y = player.position.y;
            if player.facing_right {
                item.position.x = player.position.x + player.size.x;
            } else {
                item.position.x = player.position.x - item.size.x;
            }
        } else {
            // Item is not grabbed, apply normal physics.
            item.update(dt);
            physics::resolve_item_collisions(item, ground, platforms);
        }
    }
}

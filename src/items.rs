//! Items Module
//!
//! This module defines the items that appear in the game world.

use crate::constants::{GRAVITY, ITEM_SIZE, ITEM_THROW_SPEED};
use crate::physics;
use crate::player::Player;
use macroquad::prelude::*;

/// Represents the state of an item.
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
            ItemState::Idle => BLUE,
            ItemState::Hooked => YELLOW,
            ItemState::Thrown => RED,
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

/// Handles all item-related logic for a frame, including grab/release, physics, and position updates.
pub fn process_items(
    items: &mut [Item],
    player: &mut Player,
    ground: &Rect,
    platforms: &[Rect],
    left_wall: &Rect,
    right_wall: &Rect,
    dt: f32,
) {
    let player_rect = player.rect();
    let space_pressed = is_key_pressed(KeyCode::Space);
    let b_pressed = is_key_pressed(KeyCode::B);

    // Handle actions for a held item (release or throw)
    if let Some(held_item_index) = player.held_item_index {
        let held_item = &mut items[held_item_index];

        if space_pressed {
            held_item.state = ItemState::Idle;
            held_item.on_ground = false;
            player.held_item_index = None;
        } else if b_pressed {
            held_item.state = ItemState::Thrown;
            held_item.on_ground = false;
            let direction = if player.facing_right { 1.0 } else { -1.0 };
            held_item.velocity = vec2(direction, -1.0).normalize() * ITEM_THROW_SPEED;
            player.held_item_index = None;
        }
    } else if space_pressed {
        // Handle grabbing a new item
        for (i, item) in items.iter_mut().enumerate() {
            if let ItemState::Idle = item.state {
                if player_rect.overlaps(&item.rect()) {
                    item.state = ItemState::Hooked;
                    item.velocity = Vec2::ZERO;
                    player.held_item_index = Some(i);
                    break;
                }
            }
        }
    }

    // Update position and state of all items
    for (i, item) in items.iter_mut().enumerate() {
        if player.held_item_index == Some(i) {
            // Item is hooked to the player
            item.position.y = player.position.y;
            if player.facing_right {
                item.position.x = player.position.x + player.size.x;
            } else {
                item.position.x = player.position.x - item.size.x;
            }
        } else {
            // Item is not hooked, apply physics
            match item.state {
                ItemState::Idle | ItemState::Thrown => {
                    // Only update and check for collisions if the item is not already settled on the ground.
                    if !item.on_ground {
                        item.update(dt);
                        physics::resolve_item_collisions(
                            item,
                            ground,
                            platforms,
                            left_wall,
                            right_wall,
                        );
                    }
                }
                ItemState::Hooked => {
                    // This item should not be hooked if it's not the one the player is holding.
                    // Reset its state to be safe.
                    item.state = ItemState::Idle;
                }
            }
        }
    }
}

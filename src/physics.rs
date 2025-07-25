//! Physics Module
//!
//! This module handles collision detection and resolution.

use crate::constants::{ITEM_BOUNCE_ENERGY_LOSS, ITEM_MIN_BOUNCE_SPEED};
use crate::items::{Item, ItemState};
use crate::level::Level;
use crate::player::Player;
use macroquad::prelude::{get_frame_time, Rect, Vec2};

/// Resolves collisions between the player and the level, including boundaries and platforms.
pub fn resolve_player_collisions(player: &mut Player, level: &Level) {
    player.on_ground = false;
    let player_rect = player.rect();

    // Player vs. Level bounds
    if player_rect.overlaps(&level.left_wall) {
        player.position.x = level.left_wall.right();
    }
    if player_rect.overlaps(&level.right_wall) {
        player.position.x = level.right_wall.left() - player.size.x;
    }
    if player_rect.overlaps(&level.ceiling) {
        player.position.y = level.ceiling.bottom();
        player.velocity.y = 0.;
    }
    if player_rect.overlaps(&level.ground) {
        if player.velocity.y > 0. {
            player.position.y = level.ground.y - player.size.y;
            player.velocity.y = 0.;
            player.on_ground = true;
        }
    }

    // Player vs. Platforms
    if player.velocity.y > 0. {
        for platform in &level.platforms {
            let player_rect = player.rect();
            if player_rect.overlaps(platform) {
                // Check if the player's bottom edge was above the platform's top edge in the previous frame
                let previous_player_bottom =
                    player.position.y + player.size.y - player.velocity.y * get_frame_time();
                if previous_player_bottom <= platform.y {
                    player.position.y = platform.y - player.size.y;
                    player.velocity.y = 0.;
                    player.on_ground = true;
                }
            }
        }
    }
}

/// Resolves collisions for a single item with the level.
pub fn resolve_item_collisions(
    item: &mut Item,
    ground: &Rect,
    platforms: &[Rect],
    left_wall: &Rect,
    right_wall: &Rect,
) {
    item.on_ground = false;
    let item_rect = item.rect();

    // Item vs. Walls
    if item_rect.overlaps(left_wall) {
        item.position.x = left_wall.right();
        item.velocity.x = -item.velocity.x;
    }
    if item_rect.overlaps(right_wall) {
        item.position.x = right_wall.left() - item.size.x;
        item.velocity.x = -item.velocity.x;
    }

    // Item vs. Ground and Platforms
    let mut colliders = vec![*ground];
    colliders.extend_from_slice(platforms);

    if item.velocity.y >= 0. {
        for platform in &colliders {
            if item_rect.overlaps(platform) {
                let previous_item_bottom =
                    item.position.y + item.size.y - item.velocity.y * get_frame_time();
                if previous_item_bottom <= platform.y {
                    // Collision detected
                    if let ItemState::Thrown = item.state {
                        if item.velocity.length() > ITEM_MIN_BOUNCE_SPEED {
                            item.position.y = platform.y - item.size.y;
                            item.velocity.y = -item.velocity.y * ITEM_BOUNCE_ENERGY_LOSS;
                            // Also apply friction to horizontal movement
                            item.velocity.x *= 1.0 - ITEM_BOUNCE_ENERGY_LOSS;
                        } else {
                            item.state = ItemState::Idle;
                            item.on_ground = true;
                            item.velocity = Vec2::ZERO;
                            item.position.y = platform.y - item.size.y;
                        }
                    } else {
                        item.on_ground = true;
                        item.velocity = Vec2::ZERO;
                        item.position.y = platform.y - item.size.y;
                    }
                    return;
                }
            }
        }
    }
}
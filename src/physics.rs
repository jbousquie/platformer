//! Physics Module
//!
//! This module handles collision detection and resolution.

use crate::blocks::{Block, BlockState};
use crate::constants::{ITEM_BOUNCE_ENERGY_LOSS, ITEM_MIN_BOUNCE_SPEED};
use crate::items::{Item, ItemState};
use crate::player::Player;
use macroquad::prelude::{get_frame_time, Rect, Vec2};

/// Resolves collisions between the player and the level, including boundaries, platforms, and blocks.
pub fn resolve_player_collisions(player: &mut Player, platforms: &[Rect], blocks: &[Block], ground: &Rect, left_wall: &Rect, right_wall: &Rect, ceiling: &Rect) {
    player.on_ground = false;
    let player_rect = player.rect();

    // Player vs. Level bounds
    if player_rect.overlaps(left_wall) {
        player.position.x = left_wall.right();
    }
    if player_rect.overlaps(right_wall) {
        player.position.x = right_wall.left() - player.size.x;
    }
    if player_rect.overlaps(ceiling) {
        player.position.y = ceiling.bottom();
        player.velocity.y = 0.;
    }

    // Create a list of all solid surfaces for the player to land on
    let mut surfaces = platforms.to_vec();
    surfaces.push(*ground);
    for block in blocks {
        if block.state == BlockState::Idle {
            surfaces.push(block.rect());
        }
    }

    // Player vs. Surfaces (Ground, Platforms, Blocks)
    if player.velocity.y >= 0. {
        for surface in &surfaces {
            let player_rect = player.rect();
            if player_rect.overlaps(surface) {
                let previous_player_bottom =
                    player.position.y + player.size.y - player.velocity.y * get_frame_time();
                if previous_player_bottom <= surface.y {
                    player.position.y = surface.y - player.size.y;
                    player.velocity.y = 0.;
                    player.on_ground = true;
                }
            }
        }
    }
}

/// Resolves collisions for a single item with the level and blocks.
pub fn resolve_item_collisions(item: &mut Item, platforms: &[Rect], blocks: &[Block], ground: &Rect, left_wall: &Rect, right_wall: &Rect) {
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

    // Combine all solid objects for collision detection
    let mut colliders = platforms.to_vec();
    colliders.push(*ground);
    for block in blocks {
        if block.state == BlockState::Idle {
            colliders.push(block.rect());
        }
    }

    // Item vs. Surfaces (Ground, Platforms, Blocks)
    for surface in &colliders {
        if item_rect.overlaps(surface) {
            if item.velocity.y >= 0. {
                let previous_item_bottom =
                    item.position.y + item.size.y - item.velocity.y * get_frame_time();
                if previous_item_bottom <= surface.y {
                    // Collision from above
                    if let ItemState::Thrown = item.state {
                        if item.velocity.length() > ITEM_MIN_BOUNCE_SPEED {
                            item.position.y = surface.y - item.size.y;
                            item.velocity.y = -item.velocity.y * ITEM_BOUNCE_ENERGY_LOSS;
                            item.velocity.x *= 1.0 - ITEM_BOUNCE_ENERGY_LOSS;
                        } else {
                            item.state = ItemState::Idle;
                            item.on_ground = true;
                            item.velocity = Vec2::ZERO;
                            item.position.y = surface.y - item.size.y;
                        }
                    } else {
                        item.on_ground = true;
                        item.velocity = Vec2::ZERO;
                        item.position.y = surface.y - item.size.y;
                    }
                    return;
                }
            }
            if item_rect.overlaps(surface) {
                item.velocity.x = -item.velocity.x * ITEM_BOUNCE_ENERGY_LOSS;
                return;
            }
        }
    }
}

/// Resolves collisions for a single block with the level and other blocks.
pub fn resolve_block_collisions(block: &mut Block, block_idx: usize, platforms: &[Rect], all_blocks: &[Block], ground: &Rect, left_wall: &Rect, right_wall: &Rect) {
    block.on_ground = false;
    let block_rect = block.rect();

    // Block vs. Walls
    if block_rect.overlaps(left_wall) {
        block.position.x = left_wall.right();
        block.velocity.x = 0.;
    }
    if block_rect.overlaps(right_wall) {
        block.position.x = right_wall.left() - block.size.x;
        block.velocity.x = 0.;
    }

    // Combine all other solid objects for collision
    let mut colliders = platforms.to_vec();
    colliders.push(*ground);
    for (i, other_block) in all_blocks.iter().enumerate() {
        if i != block_idx && other_block.state == BlockState::Idle {
            colliders.push(other_block.rect());
        }
    }

    // Block vs. Surfaces (Ground, Platforms, other Blocks)
    if block.velocity.y >= 0. {
        for surface in &colliders {
            if block.rect().overlaps(surface) {
                let previous_block_bottom =
                    block.position.y + block.size.y - block.velocity.y * get_frame_time();
                if previous_block_bottom <= surface.y {
                    block.position.y = surface.y - block.size.y;
                    block.velocity = Vec2::ZERO;
                    block.on_ground = true;
                    return;
                }
            }
        }
    }
}
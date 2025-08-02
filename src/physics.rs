//! Physics Module
//!
//! This module handles collision detection and resolution.

use crate::baddies::{Baddie, BaddieState};
use crate::blocks::{Block, BlockState};
use crate::constants::{
    BADDIE_GRAB_CHANCE, BADDIE_MAX_GRAB_DURATION, BADDIE_MIN_GRAB_DURATION,
    ITEM_BOUNCE_ENERGY_LOSS, ITEM_MIN_BOUNCE_SPEED,
};
use crate::items::{Item, ItemState};
use crate::player::{HeldObject, Player};
use ::rand::{thread_rng, Rng};
use macroquad::prelude::{get_frame_time, vec2, Rect, Vec2};

/// Resolves collisions between the player and the level, including boundaries, platforms, and blocks.
pub fn resolve_player_collisions(
    player: &mut Player,
    platforms: &[Rect],
    items: &[Item],
    blocks: &[Block],
    ground: &Rect,
    left_wall: &Rect,
    right_wall: &Rect,
    ceiling: &Rect,
) {
    player.on_ground = false;

    // Determine the width of the held object, if any, to adjust the player's bounding box.
    // This prevents the player from moving into walls while holding an object.
    let held_object_width = match player.held_object {
        Some(HeldObject::Item(idx)) => items.get(idx).map_or(0.0, |item| item.size.x),
        Some(HeldObject::Block(idx)) => blocks.get(idx).map_or(0.0, |block| block.size.x),
        None => 0.0,
    };

    // --- Player vs. Level Bounds ---
    // Adjust player position to prevent moving beyond level boundaries.
    if player.facing_right {
        let player_right_edge = player.position.x + player.size.x + held_object_width;
        if player_right_edge > right_wall.left() {
            player.position.x = right_wall.left() - player.size.x - held_object_width;
        }
    } else {
        let player_left_edge = player.position.x - held_object_width;
        if player_left_edge < left_wall.right() {
            player.position.x = left_wall.right() + held_object_width;
        }
    }

    // Prevent player from moving through the ceiling.
    if player.rect().overlaps(ceiling) {
        player.position.y = ceiling.bottom();
        player.velocity.y = 0.;
    }

    // --- Player vs. Surfaces (Ground, Platforms, Blocks) ---
    // Create a unified list of all solid surfaces the player can land on.
    let mut surfaces = platforms.to_vec();
    surfaces.push(*ground);
    for block in blocks.iter() {
        if block.state == BlockState::Idle {
            surfaces.push(block.rect());
        }
    }

    // Check for vertical collisions.
    if player.velocity.y >= 0. {
        for surface in &surfaces {
            if player.rect().overlaps(surface) {
                // To prevent sinking, check if the player was above the surface in the previous frame.
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

    // --- Player vs. Blocks (Side Collisions) ---
    // Handle horizontal collisions with blocks separately to prevent pushing.
    for block in blocks.iter() {
        if block.state == BlockState::Idle {
            let player_rect = player.rect();
            let block_rect = block.rect();
            if player_rect.overlaps(&block_rect) {
                let previous_player_right =
                    player.position.x + player.size.x - player.velocity.x * get_frame_time();
                let previous_player_left = player.position.x - player.velocity.x * get_frame_time();

                // Collision from the left.
                if previous_player_right <= block_rect.left()
                    && player_rect.right() > block_rect.left()
                {
                    player.position.x = block_rect.left() - player.size.x;
                }
                // Collision from the right.
                else if previous_player_left >= block_rect.right()
                    && player_rect.left() < block_rect.right()
                {
                    player.position.x = block_rect.right();
                }
            }
        }
    }
}

/// Resolves collisions for a single baddie with the level, including boundaries, platforms, and blocks.
pub fn resolve_baddie_collisions(
    baddie: &mut Baddie,
    platforms: &[Rect],
    blocks: &mut [Block],
    ground: &Rect,
    left_wall: &Rect,
    right_wall: &Rect,
    ceiling: &Rect,
) {
    baddie.on_ground = false;

    let held_block_width = if let Some(block_id) = baddie.grabbed_block_id {
        blocks[block_id].size.x
    } else {
        0.0
    };

    // --- Baddie vs. Walls ---
    // Reverse direction upon hitting a wall.
    if baddie.facing_right {
        if baddie.rect().right() + held_block_width > right_wall.left() {
            baddie.position.x = right_wall.left() - baddie.size.x - held_block_width;
            baddie.change_direction();
        }
    } else {
        if baddie.rect().left() - held_block_width < left_wall.right() {
            baddie.position.x = left_wall.right() + held_block_width;
            baddie.change_direction();
        }
    }

    // --- Baddie vs. Ceiling ---
    if baddie.state == BaddieState::Elevation && baddie.rect().overlaps(ceiling) {
        baddie.state = BaddieState::Idle;
        baddie.velocity.y = 0.;
        baddie.position.y = ceiling.bottom();
        return; // Exit early as no other collision checks are needed
    }

    // --- Baddie vs. Surfaces (Ground, Platforms, Blocks) ---
    if baddie.state != BaddieState::Elevation {
        // Create a unified list of all solid surfaces the baddie can land on.
        let mut surfaces = platforms.to_vec();
        surfaces.push(*ground);
        for block in blocks.iter() {
            if block.state == BlockState::Idle {
                surfaces.push(block.rect());
            }
        }

        // Check for vertical collisions.
        if baddie.velocity.y >= 0. {
            for surface in &surfaces {
                if baddie.rect().overlaps(surface) {
                    let previous_baddie_bottom =
                        baddie.position.y + baddie.size.y - baddie.velocity.y * get_frame_time();
                    if previous_baddie_bottom <= surface.y {
                        baddie.position.y = surface.y - baddie.size.y;
                        baddie.velocity.y = 0.;
                        baddie.on_ground = true;
                    }
                }
            }
        }

        // --- Baddie vs. Blocks (Side Collisions) ---
        // Handle horizontal collisions with blocks.
        for (i, block) in blocks.iter_mut().enumerate() {
            if block.state == BlockState::Idle {
                let baddie_rect = baddie.rect();
                let block_rect = block.rect();
                if baddie_rect.overlaps(&block_rect) {
                    let previous_baddie_right =
                        baddie.position.x + baddie.size.x - baddie.velocity.x * get_frame_time();
                    let previous_baddie_left =
                        baddie.position.x - baddie.velocity.x * get_frame_time();

                    // Collision from the left.
                    if previous_baddie_right <= block_rect.left()
                        && baddie_rect.right() > block_rect.left()
                    {
                        baddie.position.x = block_rect.left() - baddie.size.x;
                        if thread_rng().gen_range(0.0..1.0) < BADDIE_GRAB_CHANCE {
                            baddie.state = BaddieState::Grab;
                            baddie.grabbed_block_id = Some(i);
                            baddie.grab_timer = thread_rng()
                                .gen_range(BADDIE_MIN_GRAB_DURATION..BADDIE_MAX_GRAB_DURATION);
                            block.state = BlockState::Hooked;
                        } else {
                            baddie.change_direction();
                        }
                    }
                    // Collision from the right.
                    else if previous_baddie_left >= block_rect.right()
                        && baddie_rect.left() < block_rect.right()
                    {
                        baddie.position.x = block_rect.right();
                        if thread_rng().gen_range(0.0..1.0) < BADDIE_GRAB_CHANCE {
                            baddie.state = BaddieState::Grab;
                            baddie.grabbed_block_id = Some(i);
                            baddie.grab_timer = thread_rng()
                                .gen_range(BADDIE_MIN_GRAB_DURATION..BADDIE_MAX_GRAB_DURATION);
                            block.state = BlockState::Hooked;
                        } else {
                            baddie.change_direction();
                        }
                    }
                }
            }
        }

        // --- Edge Detection ---
        // Check if the baddie is about to fall off a platform or block.
        if baddie.on_ground {
            // Create a probe point just ahead of and below the baddie to check for ground.
            let probe_x = if baddie.facing_right {
                baddie.rect().right()
            } else {
                baddie.rect().left()
            };
            let probe_y = baddie.rect().bottom() + 1.0;
            let probe_point = vec2(probe_x, probe_y);

            let mut ground_ahead = false;
            for surface in &surfaces {
                if surface.contains(probe_point) {
                    ground_ahead = true;
                    break;
                }
            }

            // If there is no ground ahead, randomly decide whether to change direction or fall.
            if !ground_ahead {
                if thread_rng().gen_bool(0.1) {
                    baddie.change_direction();
                }
            }
        }
    }
}

/// Resolves collisions for a single item with the level and blocks.
pub fn resolve_item_collisions(
    item: &mut Item,
    platforms: &[Rect],
    blocks: &[Block],
    ground: &Rect,
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

    // Combine all solid objects for collision detection
    let mut colliders = platforms.to_vec();
    colliders.push(*ground);
    for block in blocks.iter() {
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
pub fn resolve_block_collisions(
    block: &mut Block,
    platforms: &[Rect],
    blocks_before: &[Block],
    blocks_after: &[Block],
    ground: &Rect,
    left_wall: &Rect,
    right_wall: &Rect,
) {
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
    for other_block in blocks_before.iter().chain(blocks_after.iter()) {
        if other_block.state == BlockState::Idle {
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

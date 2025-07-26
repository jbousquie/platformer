//! Game Module
//!
//! This module contains the main game loop and game state management.

use crate::blocks::{Block, BlockState};
use crate::camera::Camera;
use crate::constants::{BACKGROUND_COLOR, BLOCK_OFFSET, ITEM_THROW_SPEED};
use crate::items::{Item, ItemState};
use crate::level::Level;
use crate::physics;
use crate::player::{HeldObject, Player};
use macroquad::prelude::*;
use std::time::Instant;

const FPS_LOG_INTERVAL_FRAMES: u32 = 1000;

/// Runs the main game loop.
pub async fn run() {
    let mut player = Player::new();
    let mut level = Level::new().await;
    let mut camera = Camera::new();

    let mut frame_count = 0;
    let mut last_log_time = Instant::now();

    loop {
        let dt = get_frame_time();

        // Update
        player.update(dt);
        process_interactions(&mut player, &mut level.items, &mut level.blocks);

        let (platforms, items, blocks, ground, left_wall, right_wall, ceiling) = (
            level.platforms.as_slice(),
            level.items.as_slice(),
            level.blocks.as_slice(),
            &level.ground,
            &level.left_wall,
            &level.right_wall,
            &level.ceiling,
        );

        physics::resolve_player_collisions(
            &mut player,
            platforms,
            items,
            blocks,
            ground,
            left_wall,
            right_wall,
            ceiling,
        );

        // Update items
        for item in level.items.iter_mut() {
            if item.state != ItemState::Hooked {
                if !item.on_ground {
                    item.update(dt);
                    physics::resolve_item_collisions(
                        item, platforms, blocks, ground, left_wall, right_wall,
                    );
                }
            } else {
                // This state should be handled by process_interactions, but as a fallback
                if player.held_object.is_none() {
                    item.state = ItemState::Idle;
                }
            }
        }

        // Update blocks
        for i in 0..level.blocks.len() {
            let (blocks_before, blocks_after_with_current) = level.blocks.split_at_mut(i);
            let (block_slice, blocks_after) = blocks_after_with_current.split_at_mut(1);
            let block = &mut block_slice[0];

            if block.state != BlockState::Hooked {
                if !block.on_ground {
                    block.update(dt);
                    physics::resolve_block_collisions(
                        block,
                        platforms,
                        blocks_before,
                        blocks_after,
                        ground,
                        left_wall,
                        right_wall,
                    );
                }
            } else {
                if player.held_object.is_none() {
                    block.state = BlockState::Idle;
                }
            }
        }

        camera.update(&player);

        // Draw
        clear_background(BACKGROUND_COLOR);

        set_camera(&macroquad::prelude::Camera2D {
            target: vec2(
                camera.rect.x + camera.rect.w / 2.,
                camera.rect.y + camera.rect.h / 2.,
            ),
            zoom: vec2(1. / camera.rect.w, 1. / camera.rect.h),
            ..Default::default()
        });

        level.draw();
        player.draw();

        set_default_camera();

        // Log FPS
        frame_count += 1;
        log_fps(&mut frame_count, &mut last_log_time);

        next_frame().await
    }
}

/// Logs the average FPS to the console every `FPS_LOG_INTERVAL_FRAMES` frames.
fn log_fps(frame_count: &mut u32, last_log_time: &mut Instant) {
    if *frame_count >= FPS_LOG_INTERVAL_FRAMES {
        let elapsed_time = last_log_time.elapsed().as_secs_f32();
        let fps = *frame_count as f32 / elapsed_time;
        println!("Average FPS over last {} frames: {:.2}", *frame_count, fps);

        // Reset counter and timer
        *frame_count = 0;
        *last_log_time = Instant::now();
    }
}


/// Handles player interactions with items and blocks (grabbing, dropping, throwing).
fn process_interactions(player: &mut Player, items: &mut [Item], blocks: &mut [Block]) {
    let space_pressed = is_key_pressed(KeyCode::Space);

    match player.held_object {
        Some(HeldObject::Item(idx)) => {
            let item = &mut items[idx];
            if space_pressed {
                item.state = ItemState::Thrown;
                item.on_ground = false;
                let dir = if player.facing_right { 1.0 } else { -1.0 };
                item.velocity = vec2(dir, -1.0).normalize() * ITEM_THROW_SPEED;
                player.held_object = None;
            } else {
                // Keep item hooked to player
                item.position.y = player.position.y;
                item.position.x = if player.facing_right {
                    player.position.x + player.size.x
                } else {
                    player.position.x - item.size.x
                };
            }
        }
        Some(HeldObject::Block(idx)) => {
            let block = &mut blocks[idx];
            if space_pressed {
                block.state = BlockState::Idle;
                block.on_ground = false;
                player.held_object = None;
            } else {
                // Keep block hooked to player
                block.position.y = player.position.y - BLOCK_OFFSET;
                block.position.x = if player.facing_right {
                    player.position.x + player.size.x
                } else {
                    player.position.x - block.size.x
                };
            }
        }
        None => {
            // Try to grab an object
            if space_pressed {
                let player_rect = player.rect();
                // Prioritize grabbing items
                for (i, item) in items.iter_mut().enumerate() {
                    if item.state == ItemState::Idle && player_rect.overlaps(&item.rect()) {
                        item.state = ItemState::Hooked;
                        item.velocity = Vec2::ZERO;
                        player.held_object = Some(HeldObject::Item(i));
                        return; // Exit after grabbing one object
                    }
                }
                // If no item was grabbed, try to grab a block
                for (i, block) in blocks.iter_mut().enumerate() {
                    // Player cannot grab a block they are standing on.
                    let player_is_on_block = player.on_ground
                        && player.rect().bottom() >= block.rect().top()
                        && player.rect().bottom() <= block.rect().top() + 1.0 // Tolerance
                        && player_rect.overlaps(&block.rect());

                    if !player_is_on_block
                        && block.state == BlockState::Idle
                        && player_rect.overlaps(&block.rect())
                    {
                        block.state = BlockState::Hooked;
                        block.velocity = Vec2::ZERO;
                        player.held_object = Some(HeldObject::Block(i));
                        return; // Exit after grabbing one object
                    }
                }
            }
        }
    }
}
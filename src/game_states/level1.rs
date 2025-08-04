use std::time::Instant;

use macroquad::prelude::*;

use crate::{
    blocks::BlockState,
    constants::BLOCK_OFFSET,
    game::Game,
    game_states::GameState,
    items::ItemState,
    physics,
    player::HeldObject,
};

use crate::constants::BACKGROUND_COLOR;

const FPS_LOG_INTERVAL_FRAMES: u32 = 1000;

pub async fn run_level1(game: &mut Game) {
    let mut frame_count = 0;
    let mut last_log_time = Instant::now();

    let dt = get_frame_time();

    // Update
    update(game, dt);

    // Draw
    draw(game);

    // Log FPS
    frame_count += 1;
    log_fps(&mut frame_count, &mut last_log_time);
}

/// Updates the game state for the current frame.
fn update(game: &mut Game, dt: f32) {
    update_player_and_collisions(game, dt);
    update_world_objects(game, dt);
    update_baddies_and_collisions(game, dt);

    game.camera.update(&game.player);

    // --- Player vs. Baddie Collision ---
    for baddie in &game.baddies {
        if game.player.rect().overlaps(&baddie.rect()) {
            game.gamestate = GameState::GameOver;
        }
    }

    // --- Game Over Condition ---
    // Check for collision between the player and any thrown item.
    for item in &game.level.items {
        if item.state == ItemState::Thrown && game.player.rect().overlaps(&item.rect()) {
            game.gamestate = GameState::GameOver;
        }
    }
}

/// Handles the player's movement, interactions, and physics collisions.
fn update_player_and_collisions(game: &mut Game, dt: f32) {
    game.player.update(dt);
    // Player interactions can modify items and blocks, so it needs mutable access.
    game.player
        .process_interactions(&mut game.level.items, &mut game.level.blocks);

    // --- Borrowing Strategy for Collision Detection ---
    // To satisfy the borrow checker, we structure the update logic to avoid simultaneous
    // mutable and immutable borrows of `self.level.blocks`.

    // Destructure level components into immutable slices for collision checks that don't require mutation.
    let (platforms, ground, left_wall, right_wall, ceiling) = (
        game.level.platforms.as_slice(),
        &game.level.ground,
        &game.level.left_wall,
        &game.level.right_wall,
        &game.level.ceiling,
    );

    // Create an immutable borrow of blocks to pass to functions that only need to read block data.
    let blocks = game.level.blocks.as_slice();

    // Player collisions are resolved first, using the immutable block slice.
    physics::resolve_player_collisions(
        &mut game.player,
        platforms,
        &game.level.items,
        blocks,
        ground,
        left_wall,
        right_wall,
        ceiling,
    );
}

/// Handles the updates and physics for all non-character objects in the world (items and blocks).
fn update_world_objects(game: &mut Game, dt: f32) {
    // Destructure level components into immutable slices for collision checks that don't require mutation.
    let (platforms, ground, left_wall, right_wall, _ceiling) = (
        game.level.platforms.as_slice(),
        &game.level.ground,
        &game.level.left_wall,
        &game.level.right_wall,
        &game.level.ceiling,
    );

    // Create an immutable borrow of blocks to pass to functions that only need to read block data.
    let blocks = game.level.blocks.as_slice();

    // Update items, which also use the immutable block slice for collision checks.
    for (i, item) in game.level.items.iter_mut().enumerate() {
        if item.state != ItemState::Hooked {
            if !item.on_ground {
                item.update(dt);
                physics::resolve_item_collisions(
                    item, platforms, blocks, ground, left_wall, right_wall,
                );
            }
        } else {
            let mut is_held = false;
            if let Some(HeldObject::Item(id)) = game.player.held_object {
                if id == i {
                    is_held = true;
                }
            }
            if !is_held {
                for baddie in &game.baddies {
                    if let Some(id) = baddie.held_item_id {
                        if id == i {
                            is_held = true;
                            break;
                        }
                    }
                }
            }
            if !is_held {
                item.state = ItemState::Idle;
            }
        }
    }

    // --- Handling Mutable Borrows for Block-on-Block Collisions ---
    // The immutable borrow of `blocks` is no longer needed, so we can now create mutable borrows.
    // To resolve collisions between blocks, we need to mutate a block while comparing it against
    // other blocks. A standard `iter_mut` would violate the borrow checker (one mutable borrow
    // and multiple immutable borrows at the same time).
    // The solution is to use `split_at_mut`, which divides the slice into two mutable parts,
    // allowing us to safely mutate the current block while accessing the others.
    for i in 0..game.level.blocks.len() {
        let (blocks_before, blocks_after_with_current) = game.level.blocks.split_at_mut(i);
        let (block_slice, blocks_after) = blocks_after_with_current.split_at_mut(1);
        let block = &mut block_slice[0];

        if block.state != BlockState::Hooked {
            if !block.on_ground {
                block.update(dt);
                physics::resolve_block_collisions(
                    block,
                    platforms,
                    blocks_before, // All blocks before the current one
                    blocks_after,  // All blocks after the current one
                    ground,
                    left_wall,
                    right_wall,
                );
            }
        } else {
            if game.player.held_object.is_none() {
                block.state = BlockState::Idle;
            }
        }
    }
}

/// Handles baddie movement, interactions, and collisions, including their interactions with thrown items.
fn update_baddies_and_collisions(game: &mut Game, dt: f32) {
    // Destructure level components into immutable slices for collision checks that don't require mutation.
    let (platforms, ground, left_wall, right_wall, ceiling) = (
        game.level.platforms.as_slice(),
        &game.level.ground,
        &game.level.left_wall,
        &game.level.right_wall,
        &game.level.ceiling,
    );

    // --- Baddie Updates ---
    // After all block mutations are done, we can safely create a new immutable borrow
    // of the entire `blocks` slice to check for baddie collisions.
    for baddie in game.baddies.iter_mut() {
        baddie.update(dt);
        baddie.process_interactions(&mut game.level.items, game.player.position);
        physics::resolve_baddie_collisions(
            baddie,
            platforms,
            &mut game.level.blocks,
            &mut game.level.items,
            ground,
            left_wall,
            right_wall,
            ceiling,
        );
    }

    // Update block positions for baddies that are holding them
    for baddie in &game.baddies {
        if let Some(block_id) = baddie.grabbed_block_id {
            if let Some(block) = game.level.blocks.get_mut(block_id) {
                block.position = baddie.position;
                block.position.y -= BLOCK_OFFSET;
                if baddie.facing_right {
                    block.position.x += baddie.size.x;
                } else {
                    block.position.x -= block.size.x;
                }
            }
        }
    }

    // --- Baddie vs. Thrown Item Collisions ---
    // When a thrown item hits a baddie, remove both.
    let mut baddies_hit_mask = vec![false; game.baddies.len()];
    let mut items_hit_mask = vec![false; game.level.items.len()];
    let mut items_to_drop: Vec<usize> = Vec::new();

    for (item_idx, item) in game.level.items.iter().enumerate() {
        if item.state == ItemState::Thrown {
            for (baddie_idx, baddie) in game.baddies.iter().enumerate() {
                // Check if the baddie hasn't already been marked for removal by another item
                if !baddies_hit_mask[baddie_idx] && baddie.rect().overlaps(&item.rect()) {
                    baddies_hit_mask[baddie_idx] = true;
                    items_hit_mask[item_idx] = true;

                    // If the baddie was holding a block, drop it.
                    if let Some(block_id) = baddie.grabbed_block_id {
                        if let Some(block) = game.level.blocks.get_mut(block_id) {
                            block.state = BlockState::Idle;
                        }
                    }

                    // If the baddie was holding an item, drop it.
                    if let Some(item_id) = baddie.held_item_id {
                        items_to_drop.push(item_id);
                    }

                    // An item is consumed upon hitting a baddie and cannot hit another in the same frame.
                    break;
                }
            }
        }
    }

    for item_id in items_to_drop {
        if let Some(item) = game.level.items.get_mut(item_id) {
            item.state = ItemState::Idle;
        }
    }

    // Remove baddies that were hit using the mask.
    let mut i = 0;
    game.baddies.retain(|_| {
        let keep = !baddies_hit_mask[i];
        i += 1;
        keep
    });

    // Remove items that hit a baddie using the mask.
    i = 0;
    game.level.items.retain(|_| {
        let keep = !items_hit_mask[i];
        i += 1;
        keep
    });
}

/// Draws the game world.
fn draw(game: &Game) {
    clear_background(BACKGROUND_COLOR);

    set_camera(&macroquad::prelude::Camera2D {
        target: vec2(
            game.camera.rect.x + game.camera.rect.w / 2.,
            game.camera.rect.y + game.camera.rect.h / 2.,
        ),
        zoom: vec2(1. / game.camera.rect.w, 1. / game.camera.rect.h),
        ..Default::default()
    });

    game.level.draw();
    game.player.draw();
    for baddie in game.baddies.iter() {
        baddie.draw();
    }

    set_default_camera();
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
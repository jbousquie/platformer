//! Game Module
//!
//! This module contains the main game loop and game state management.

use crate::baddies::Baddie;
use crate::blocks::{Block, BlockState};
use crate::camera::Camera;

use crate::constants::{
    BACKGROUND_COLOR, BLOCK_OFFSET, ITEM_THROW_SPEED, MAX_BADDIES,
};
use crate::gamestate::GameState;
use crate::items::{Item, ItemState};
use crate::level::{Level, LEVEL_HEIGHT, LEVEL_WIDTH};
use crate::physics;
use crate::player::{HeldObject, Player};
use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use std::time::Instant;

const FPS_LOG_INTERVAL_FRAMES: u32 = 1000;

/// Represents the main game state.
pub struct Game {
    gamestate: GameState,
    player: Player,
    level: Level,
    camera: Camera,
    baddies: Vec<Baddie>,
}

impl Game {
    /// Creates a new game instance.
    async fn new() -> Self {
        let player = Player::new();
        let level = Level::new().await;
        let camera = Camera::new();
        let mut baddies = Vec::new();

        for _ in 0..MAX_BADDIES {
            let x = thread_rng().gen_range(0.0..LEVEL_WIDTH);
            let y = LEVEL_HEIGHT / 2.0;
            baddies.push(Baddie::new(vec2(x, y)));
        }

        Self {
            gamestate: GameState::Intro,
            player,
            level,
            camera,
            baddies,
        }
    }

    /// Runs the main game loop.
    async fn run(&mut self) {
        loop {
            match self.gamestate {
                GameState::Intro => {
                    self.run_intro().await;
                }
                GameState::Level1 => {
                    self.run_level1().await;
                }
                GameState::GameOver => {
                    self.run_game_over().await;
                }
            }
            next_frame().await
        }
    }

    async fn run_intro(&mut self) {
        clear_background(BLACK);
        draw_text(
            "PLATFORMER",
            screen_width() / 2. - 150.,
            screen_height() / 2. - 40.,
            50.,
            WHITE,
        );
        draw_text(
            "Press ENTER to start",
            screen_width() / 2. - 130.,
            screen_height() / 2. + 20.,
            30.,
            WHITE,
        );

        if is_key_pressed(KeyCode::Enter) {
            self.gamestate = GameState::Level1;
        }
    }

    async fn run_game_over(&mut self) {
        clear_background(BLACK);
        draw_text(
            "GAME OVER",
            screen_width() / 2. - 150.,
            screen_height() / 2. - 40.,
            50.,
            WHITE,
        );
        draw_text(
            "Press ENTER to restart",
            screen_width() / 2. - 160.,
            screen_height() / 2. + 20.,
            30.,
            WHITE,
        );

        if is_key_pressed(KeyCode::Enter) {
            *self = Game::new().await;
            self.gamestate = GameState::Level1;
        }
    }

    async fn run_level1(&mut self) {
        let mut frame_count = 0;
        let mut last_log_time = Instant::now();

        let dt = get_frame_time();

        // Update
        self.update(dt);

        // Draw
        self.draw();

        // Log FPS
        frame_count += 1;
        log_fps(&mut frame_count, &mut last_log_time);
    }

    /// Updates the game state for the current frame.
    fn update(&mut self, dt: f32) {
        self.player.update(dt);
        // Player interactions can modify items and blocks, so it needs mutable access.
        process_interactions(
            &mut self.player,
            &mut self.level.items,
            &mut self.level.blocks,
        );

        // --- Borrowing Strategy for Collision Detection ---
        // To satisfy the borrow checker, we structure the update logic to avoid simultaneous
        // mutable and immutable borrows of `self.level.blocks`.

        // Destructure level components into immutable slices for collision checks that don't require mutation.
        let (platforms, items, ground, left_wall, right_wall, ceiling) = (
            self.level.platforms.as_slice(),
            self.level.items.as_slice(),
            &self.level.ground,
            &self.level.left_wall,
            &self.level.right_wall,
            &self.level.ceiling,
        );

        // Create an immutable borrow of blocks to pass to functions that only need to read block data.
        let blocks = self.level.blocks.as_slice();

        // Player collisions are resolved first, using the immutable block slice.
        physics::resolve_player_collisions(
            &mut self.player,
            platforms,
            items,
            blocks,
            ground,
            left_wall,
            right_wall,
            ceiling,
        );

        // Update items, which also use the immutable block slice for collision checks.
        for item in self.level.items.iter_mut() {
            if item.state != ItemState::Hooked {
                if !item.on_ground {
                    item.update(dt);
                    physics::resolve_item_collisions(
                        item, platforms, blocks, ground, left_wall, right_wall,
                    );
                }
            } else {
                if self.player.held_object.is_none() {
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
        for i in 0..self.level.blocks.len() {
            let (blocks_before, blocks_after_with_current) = self.level.blocks.split_at_mut(i);
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
                if self.player.held_object.is_none() {
                    block.state = BlockState::Idle;
                }
            }
        }

        // --- Baddie Updates ---
        // After all block mutations are done, we can safely create a new immutable borrow
        // of the entire `blocks` slice to check for baddie collisions.
        let blocks = self.level.blocks.as_slice();
        for baddie in self.baddies.iter_mut() {
            baddie.update(dt);
            physics::resolve_baddie_collisions(
                baddie, platforms, blocks, ground, left_wall, right_wall,
            );
        }

        // --- Baddie vs. Thrown Item Collisions ---
        // When a thrown item hits a baddie, remove both.
        let mut baddies_hit_mask = vec![false; self.baddies.len()];
        let mut items_hit_mask = vec![false; self.level.items.len()];

        for (item_idx, item) in self.level.items.iter().enumerate() {
            if item.state == ItemState::Thrown {
                for (baddie_idx, baddie) in self.baddies.iter().enumerate() {
                    // Check if the baddie hasn't already been marked for removal by another item
                    if !baddies_hit_mask[baddie_idx] && baddie.rect().overlaps(&item.rect()) {
                        baddies_hit_mask[baddie_idx] = true;
                        items_hit_mask[item_idx] = true;
                        // An item is consumed upon hitting a baddie and cannot hit another in the same frame.
                        break;
                    }
                }
            }
        }

        // Remove baddies that were hit using the mask.
        let mut i = 0;
        self.baddies.retain(|_| {
            let keep = !baddies_hit_mask[i];
            i += 1;
            keep
        });

        // Remove items that hit a baddie using the mask.
        i = 0;
        self.level.items.retain(|_| {
            let keep = !items_hit_mask[i];
            i += 1;
            keep
        });

        self.camera.update(&self.player);

        // --- Player vs. Baddie Collision ---
        for baddie in &self.baddies {
            if self.player.rect().overlaps(&baddie.rect()) {
                self.gamestate = GameState::GameOver;
            }
        }

        // --- Game Over Condition ---
        // Check for collision between the player and any thrown item.
        for item in &self.level.items {
            if item.state == ItemState::Thrown && self.player.rect().overlaps(&item.rect()) {
                self.gamestate = GameState::GameOver;
            }
        }
    }

    /// Draws the game world.
    fn draw(&self) {
        clear_background(BACKGROUND_COLOR);

        set_camera(&macroquad::prelude::Camera2D {
            target: vec2(
                self.camera.rect.x + self.camera.rect.w / 2.,
                self.camera.rect.y + self.camera.rect.h / 2.,
            ),
            zoom: vec2(1. / self.camera.rect.w, 1. / self.camera.rect.h),
            ..Default::default()
        });

        self.level.draw();
        self.player.draw();
        for baddie in self.baddies.iter() {
            baddie.draw();
        }

        set_default_camera();
    }
}

/// Runs the main game loop.
pub async fn run() {
    let mut game = Game::new().await;
    game.run().await;
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
                item.velocity = player.velocity + vec2(dir, -1.0).normalize() * ITEM_THROW_SPEED;
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
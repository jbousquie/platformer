//! Player Module
//!
//! This module defines the player's behavior and properties.

use crate::{
    blocks::{Block, BlockState},
    constants::{
        BLOCK_OFFSET, GRAVITY, GROUND_HEIGHT, ITEM_THROW_SPEED, JUMP_FORCE, PLAYER_COLOR,
        PLAYER_SIZE, PLAYER_SPAWN_X, PLAYER_SPEED,
    },
    items::{Item, ItemState},
    level::LEVEL_HEIGHT,
};
use macroquad::prelude::*;

/// Represents the different states the player can be in.
pub enum PlayerState {
    Idle,
    Run,
    Jump,
    Fall,
}

/// Represents the object a player is holding.
#[derive(PartialEq)]
pub enum HeldObject {
    Item(usize),
    Block(usize),
}

/// Represents the player character in the game.
pub struct Player {
    pub position: Vec2,
    pub size: Vec2,
    pub velocity: Vec2,
    pub on_ground: bool,
    pub state: PlayerState,
    pub facing_right: bool,
    pub held_object: Option<HeldObject>,
}

impl Player {
    /// Creates a new player instance with default values.
    pub fn new() -> Self {
        Self {
            position: vec2(PLAYER_SPAWN_X, LEVEL_HEIGHT - GROUND_HEIGHT - PLAYER_SIZE),
            size: vec2(PLAYER_SIZE, PLAYER_SIZE),
            velocity: Vec2::new(0., 0.),
            on_ground: false,
            state: PlayerState::Idle,
            facing_right: true,
            held_object: None,
        }
    }

    /// Returns the player's bounding box as a `Rect`.
    pub fn rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.size.x, self.size.y)
    }

    /// Updates the player's state, including position, velocity, and state, based on input and physics.
    pub fn update(&mut self, dt: f32) {
        // Apply gravity
        self.velocity.y += GRAVITY * dt;

        // Handle input
        if is_key_down(KeyCode::Right) {
            self.velocity.x = PLAYER_SPEED;
            self.facing_right = true;
        } else if is_key_down(KeyCode::Left) {
            self.velocity.x = -PLAYER_SPEED;
            self.facing_right = false;
        } else {
            self.velocity.x = 0.;
        }

        if is_key_pressed(KeyCode::Up) && self.on_ground {
            self.velocity.y = -JUMP_FORCE;
            self.on_ground = false;
        }

        // Update position
        self.position += self.velocity * dt;

        // Update state
        if self.on_ground {
            if self.velocity.x.abs() > 0.1 {
                self.state = PlayerState::Run;
            } else {
                self.state = PlayerState::Idle;
            }
        } else {
            if self.velocity.y < 0. {
                self.state = PlayerState::Jump;
            } else {
                self.state = PlayerState::Fall;
            }
        }
    }

    /// Draws the player on the screen.
    pub fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            PLAYER_COLOR,
        );
    }

    /// Handles player interactions with items and blocks (grabbing, dropping, throwing).
    pub fn process_interactions(&mut self, items: &mut [Item], blocks: &mut [Block]) {
        let space_pressed = is_key_pressed(KeyCode::Space);

        match self.held_object {
            Some(HeldObject::Item(idx)) => {
                let item = &mut items[idx];
                if space_pressed {
                    item.state = ItemState::Thrown;
                    item.on_ground = false;
                    let dir = if self.facing_right { 1.0 } else { -1.0 };
                    item.velocity = self.velocity + vec2(dir, -1.0).normalize() * ITEM_THROW_SPEED;
                    self.held_object = None;
                } else {
                    // Keep item hooked to player
                    item.position.y = self.position.y;
                    item.position.x = if self.facing_right {
                        self.position.x + self.size.x
                    } else {
                        self.position.x - item.size.x
                    };
                }
            }
            Some(HeldObject::Block(idx)) => {
                let block = &mut blocks[idx];
                if space_pressed {
                    block.state = BlockState::Idle;
                    block.on_ground = false;
                    self.held_object = None;
                } else {
                    // Keep block hooked to player
                    block.position.y = self.position.y - BLOCK_OFFSET;
                    block.position.x = if self.facing_right {
                        self.position.x + self.size.x
                    } else {
                        self.position.x - block.size.x
                    };
                }
            }
            None => {
                // Try to grab an object
                if space_pressed {
                    let player_rect = self.rect();
                    // Prioritize grabbing items
                    for (i, item) in items.iter_mut().enumerate() {
                        if item.state == ItemState::Idle && player_rect.overlaps(&item.rect()) {
                            item.state = ItemState::Hooked;
                            item.velocity = Vec2::ZERO;
                            self.held_object = Some(HeldObject::Item(i));
                            return; // Exit after grabbing one object
                        }
                    }
                    // If no item was grabbed, try to grab a block
                    for (i, block) in blocks.iter_mut().enumerate() {
                        // Player cannot grab a block they are standing on.
                        let player_is_on_block = self.on_ground
                            && self.rect().bottom() >= block.rect().top()
                            && self.rect().bottom() <= block.rect().top() + 1.0
                            // Tolerance
                            && player_rect.overlaps(&block.rect());

                        if !player_is_on_block
                            && block.state == BlockState::Idle
                            && player_rect.overlaps(&block.rect())
                        {
                            block.state = BlockState::Hooked;
                            block.velocity = Vec2::ZERO;
                            self.held_object = Some(HeldObject::Block(i));
                            return; // Exit after grabbing one object
                        }
                    }
                }
            }
        }
    }
}
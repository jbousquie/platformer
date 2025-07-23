use macroquad::prelude::*;
use crate::constants::{PLAYER_SIZE, PLAYER_SPEED, JUMP_FORCE, GRAVITY, GROUND_HEIGHT};
use crate::level::LEVEL_HEIGHT;

pub struct Player {
    pub rect: Rect,
    pub velocity: Vec2,
    pub on_ground: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                100.,
                LEVEL_HEIGHT - GROUND_HEIGHT - PLAYER_SIZE,
                PLAYER_SIZE,
                PLAYER_SIZE,
            ),
            velocity: Vec2::new(0., 0.),
            on_ground: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Apply gravity
        self.velocity.y += GRAVITY * dt;

        // Handle input
        if is_key_down(KeyCode::Right) {
            self.velocity.x = PLAYER_SPEED;
        } else if is_key_down(KeyCode::Left) {
            self.velocity.x = -PLAYER_SPEED;
        } else {
            self.velocity.x = 0.;
        }

        if is_key_pressed(KeyCode::Up) && self.on_ground {
            self.velocity.y = -JUMP_FORCE;
            self.on_ground = false;
        }

        // Update position
        self.rect.x += self.velocity.x * dt;
        self.rect.y += self.velocity.y * dt;
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }
}
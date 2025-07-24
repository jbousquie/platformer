//! Game Module
//!
//! This module contains the main game loop and game state management.

use crate::camera::Camera;
use crate::level::Level;
use crate::physics;
use crate::player::Player;
use macroquad::prelude::*;

/// Runs the main game loop.
pub async fn run() {
    let mut player = Player::new();
    let level = Level::new();
    let mut camera = Camera::new();

    loop {
        let dt = get_frame_time();

        // Update
        player.update(dt);
        physics::resolve_collisions(&mut player, &level);
        camera.update(&player);

        // Draw
        clear_background(BLACK);

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

        next_frame().await
    }
}

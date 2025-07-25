//! Main Module
//!
//! This is the entry point of the platformer game.

mod constants;
mod camera;
mod physics;
mod game;
mod level;
mod player;
mod items;
mod blocks;

/// Configures the game window.
fn window_conf() -> macroquad::prelude::Conf {
    macroquad::prelude::Conf {
        window_title: "Platformer".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

/// The entry point of the application.
#[macroquad::main(window_conf)]
async fn main() {
    game::run().await;
}
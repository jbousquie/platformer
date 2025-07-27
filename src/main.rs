//! Main Module
//!
//! This is the entry point of the platformer game.

mod baddies;
mod blocks;
mod camera;
mod constants;
mod game;
mod items;
mod level;
mod physics;
mod player;

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

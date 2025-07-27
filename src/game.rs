//! Game Module
//!
//! This module contains the main game loop and game state management.

use crate::baddies::Baddie;
use crate::camera::Camera;
use crate::constants::MAX_BADDIES;
use crate::game_states::{GameState, self};
use crate::level::{Level, LEVEL_HEIGHT, LEVEL_WIDTH};
use crate::player::Player;
use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;

/// Represents the main game state.
pub struct Game {
    pub gamestate: GameState,
    pub player: Player,
    pub level: Level,
    pub camera: Camera,
    pub baddies: Vec<Baddie>,
}

impl Game {
    /// Creates a new game instance.
    pub async fn new() -> Self {
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
                    game_states::intro::run_intro(self).await;
                }
                GameState::Level1 => {
                    game_states::level1::run_level1(self).await;
                }
                GameState::GameOver => {
                    game_states::game_over::run_game_over(self).await;
                }
            }
            next_frame().await
        }
    }
}

/// Runs the main game loop.
pub async fn run() {
    let mut game = Game::new().await;
    game.run().await;
}

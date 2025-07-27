use macroquad::prelude::*;

use crate::game::Game;
use crate::game_states::GameState;

pub async fn run_intro(game: &mut Game) {
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
        game.gamestate = GameState::Level1;
    }
}
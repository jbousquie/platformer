use macroquad::prelude::*;

use crate::game::Game;
use crate::game_states::GameState;

pub async fn run_game_over(game: &mut Game) {
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
        *game = Game::new().await;
        game.gamestate = GameState::Level1;
    }
}
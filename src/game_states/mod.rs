pub mod game_over;
pub mod intro;
pub mod level1;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameState {
    Intro,
    Level1,
    GameOver,
}
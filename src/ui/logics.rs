use macroquad::color::BLACK;
use macroquad::prelude::{clear_background, next_frame};
use crate::board::Board;
use crate::ui::rendering;

/// used for updating the game logics
/// gets called in the game loop
pub async fn update(board: &mut Board) {
    todo!("implement piece moving logics");
}

/// used for rendering the current state of the game
/// gets called in the game loop
pub async fn render(board: &Board) {
    clear_background(BLACK);

    rendering::draw_board(board);

    next_frame().await;
}

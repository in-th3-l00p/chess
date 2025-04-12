use macroquad::color::BLACK;
use macroquad::prelude::{clear_background, next_frame};
use crate::board::Board;
use crate::ui::rendering;
use crate::ui::rendering::pieces::Textures;

/// used for updating the game logics
/// gets called in the game loop
pub async fn update(board: &mut Board) {
    todo!("implement piece moving logics");
}

/// used for rendering the current state of the game
/// gets called in the game loop
pub async fn render(textures: &Textures, board: &Board) {
    clear_background(BLACK);

    rendering::draw_board(textures, board);

    next_frame().await;
}

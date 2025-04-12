use macroquad::color::BLACK;
use macroquad::prelude::{clear_background, next_frame};
use crate::board::Board;
use crate::ui::board;

pub async fn update(board: &mut Board) {
    todo!("implement piece moving logics");
}

pub async fn render(board: &Board) {
    clear_background(BLACK);

    board::draw_board(board);

    next_frame().await;
}

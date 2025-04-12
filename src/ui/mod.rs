mod board;
pub mod constants;

use macroquad::color::BLACK;
use macroquad::window::{clear_background, next_frame, Conf};
use crate::board::Board;

pub fn window_conf() -> Conf {
    Conf {
        window_title: String::from(constants::window::WINDOW_TITLE),
        window_width: constants::window::WINDOW_WIDTH,
        window_height: constants::window::WINDOW_HEIGHT,
        window_resizable: constants::window::WINDOW_RESIZABLE,
        ..Default::default()
    }
}

pub async fn run() {
    let board = Board::new();
    loop {
        update(&board).await;
    }
}

async fn update(board: &Board) {
    clear_background(BLACK);

    board::draw_board(board);

    next_frame().await;
}

pub mod pieces;
pub mod utils;

use macroquad::color::Color;
use macroquad::shapes::draw_rectangle;
use crate::board::Board;
use crate::evaluation::evaluate;
use crate::ui::constants;
use crate::ui::rendering::pieces::Textures;
use crate::ui::rendering::utils::draw_board_square;

fn draw_square(x: i32, y: i32) {
    draw_board_square(
        (x, y),
       if (x + y) % 2 == 0 {
           constants::ui::WHITE
       } else {
           constants::ui::BLACK
       }
    )
}

pub fn draw_board(textures: &Textures, board: &Board) {
    for x in 0..8 {
        for y in 0..8{
            draw_square(x, y);
            if let Some(piece) = board.get_piece((x, y)) {
                pieces::draw_piece(textures, x, y, piece);
            }
        }
    }
}

fn sigmoid(x: i32) -> f32 {
    1.0 / (1.0 + (-x as f32 / 200.0).exp())
}

pub fn draw_eval_bar(board: &Board) {
    let evaluation = sigmoid(evaluate(board));
    let white_height: f32 = evaluation * constants::window::WINDOW_HEIGHT as f32;
    draw_rectangle(
        0., 0.,
        constants::ui::EVAL_BAR_WIDTH as f32,
        constants::window::WINDOW_HEIGHT as f32 - white_height,
        Color::from_rgba(0, 0, 0, 255)
    );
    draw_rectangle(
        0., constants::window::WINDOW_HEIGHT as f32 - white_height,
        constants::ui::EVAL_BAR_WIDTH as f32,
        white_height,
        Color::from_rgba(255, 255, 255, 255)
    );
}
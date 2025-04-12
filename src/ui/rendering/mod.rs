pub mod pieces;

use crate::board::piece::Piece;
use crate::board::Board;
use macroquad::prelude::Color;
use macroquad::shapes::draw_rectangle;

fn draw_square(x: i32, y: i32) {
    draw_rectangle(
        (x * super::constants::board::SQUARE_SIZE) as f32,
        (y * super::constants::board::SQUARE_SIZE) as f32,
       super::constants::board::SQUARE_SIZE as f32,
       super::constants::board::SQUARE_SIZE as f32,
       if (x + y) % 2 == 0 {
           super::constants::board::WHITE
       } else {
           super::constants::board::BLACK
       }
    )
}

fn draw_piece(x: i32, y: i32, piece: Piece) {
    draw_rectangle(
        (x * super::constants::board::SQUARE_SIZE) as f32,
        (y * super::constants::board::SQUARE_SIZE) as f32,
        super::constants::board::SQUARE_SIZE as f32,
        super::constants::board::SQUARE_SIZE as f32,
        Color::from_rgba(255, 255, 255, 255),
    )
}

pub fn draw_board(board: &Board) {
    for x in 0..8 {
        for y in 0..8 {
            draw_square(x, y);
            if let Some(piece) = board.get_piece(x as usize, y as usize) {
                draw_piece(x, y, piece);
            }
        }
    }
}
pub mod pieces;
pub mod utils;

use crate::board::Board;
use crate::ui::rendering::pieces::Textures;
use crate::ui::rendering::utils::draw_board_square;

fn draw_square(x: usize, y: usize) {
    draw_board_square(
        (x, y),
       if (x + y) % 2 == 0 {
           super::constants::board::WHITE
       } else {
           super::constants::board::BLACK
       }
    )
}

pub fn draw_board(textures: &Textures, board: &Board) {
    for x in 0usize..8usize {
        for y in 0usize..8usize {
            draw_square(x, y);
            if let Some(piece) = board.get_piece(x, y) {
                pieces::draw_piece(textures, x, y, piece);
            }
        }
    }
}
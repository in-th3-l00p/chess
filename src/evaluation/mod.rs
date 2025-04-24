mod constants;

use crate::board::Board;
use crate::board::color::Color;

fn calculate_material(board: &Board) -> (i32, i32) {
    let mut white_material = 0;
    let mut black_material = 0;

    for x in 0..8 {
        for y in 0..8 {
            if let Some(piece) = board.get_piece((x, y)) {
                match piece.color {
                    Color::White => white_material += piece.get_worth(),
                    Color::Black => black_material += piece.get_worth(),
                }
            }
        }
    }

    (white_material, black_material)
}

pub fn evaluate(board: &Board) -> i32 {
    let (white_material, black_material) = calculate_material(board);
    white_material - black_material
}
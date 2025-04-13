use crate::board::Board;
use crate::board::color::Color;
use crate::move_generation::utils::insert_pos;

pub fn generate(board: &Board, moves: &mut Vec<(usize, usize)>, pos: &(i32, i32), color: &Color, has_moved: &bool) {
    let delta = if let Color::White = color { 1 } else { -1 };
    insert_pos(board, moves, (pos.0 + delta, pos.1 + delta));
    if !has_moved {
        insert_pos(board, moves, (pos.0 + 2 * delta, pos.1 + 2 * delta));
    }
}
use crate::board::Board;
use crate::board::color::Color;
use crate::move_generation::{generate_moves, BoardMove};

pub mod evaluation;
pub mod searching;

pub fn get_move(board: &Board, color: Color) -> Option<BoardMove> {
    let mut max = i32::MIN;
    let mut max_i: usize = 0;
    let moves = generate_moves(board, color);
    if moves.len() == 0 {
        None
    } else {
        for i in moves.iter().enumerate() {
            let mut new_board = board.clone();
            new_board.make_move(i.1.clone());
            let evaluation = -searching::negamax::search(&mut new_board, color.inverse());
            if evaluation > max {
                max = evaluation;
                max_i = i.0;
            }
        }
        moves.get(max_i).cloned()
    }
}
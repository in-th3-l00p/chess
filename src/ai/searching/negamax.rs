use crate::ai::evaluation::evaluate;
use crate::ai::constants;
use crate::board::Board;
use crate::board::color::Color;

fn negamax(board: &mut Board, depth: i32, color: Color) -> i32 {
    if depth == 0 {
        evaluate(board)
    } else {
        let mut max_score = i32::MIN + 1;
        for potential_move in crate::move_generation::generate_moves(board, color) {
            let mut new_board = board.clone();
            new_board.make_move(potential_move);
            max_score = max_score.max(negamax(&mut new_board, depth, color.inverse()));
        }
        max_score
    }
}

pub fn search(board: &mut Board, color: Color) -> i32 {
    negamax(board, constants::SEARCH_DEPTH - 1, color.inverse())
}

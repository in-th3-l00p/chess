use crate::board::Board;
use crate::board::pieces::color::Color;
use crate::board::generation::generate_moves;
use crate::board::moving::BoardMove;

pub mod evaluation;
pub mod searching;
mod constants;

pub fn get_move(board: &Board, color: Color) -> Option<BoardMove> {
    let mut max = i32::MIN + 1;
    let mut max_i: usize = 0;
    let moves = generate_moves(board, color);
    if moves.len() == 0 {
        None
    } else {
        for i in moves.iter().enumerate() {
            let mut new_board = board.clone();
            new_board.make_move(i.1.clone());

            let evaluation = -searching::search(
                &mut new_board,
                constants::SEARCH_DEPTH - 1,
                color.inverse(),
                i32::MIN + 1, i32::MAX
            );
            if evaluation > max {
                max = evaluation;
                max_i = i.0;
            }
        }
        moves.get(max_i).cloned()
    }
}
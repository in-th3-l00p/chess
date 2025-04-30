use crate::board::pieces::color::Color;
use crate::board::Board;
use crate::board::generation::generate_moves;

pub fn perft(board: &Board, depth: i32) -> usize {
    let mut moves = generate_moves(board, Color::White);
    moves.append(&mut generate_moves(board, Color::Black));
    if depth == 1 {
        return moves.len();
    }

    let mut nodes = 0usize;
    for potential_move in moves {
        let mut new_board = board.clone();
        new_board.make_move(potential_move);
        nodes += perft(&new_board, depth - 1);
    }
    nodes
}
use crate::board::piece::Piece;
use crate::board::Board;
use crate::move_generation::Move;
use crate::move_generation::utils::add_move;

const KNIGHT_OFFSETS: [(i32, i32); 8] = [
    (2, 1), (1, 2),
    (-2, 1), (-1, 2),
    (2, -1), (1, -2),
    (-2, -1), (-1, -2)
];

pub fn generate(
    board: &Board,
    moves: &mut Vec<Move>,
    pos: &(i32, i32),
    piece: &Piece
) {
    for offset in KNIGHT_OFFSETS {
        add_move(board, moves, pos.clone(), (pos.0 + offset.0, pos.1 + offset.1), piece);
    }
}

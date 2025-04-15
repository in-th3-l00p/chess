use crate::board::Board;
use crate::board::piece::Piece;
use crate::move_generation::utils::generate_continious;

const BISHOP_OFFSETS: [(i32, i32); 4] = [
    (1, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
];

pub fn generate(
    board: &Board,
    moves: &mut Vec<(i32, i32)>,
    pos: &(i32, i32),
    piece: &Piece
) {
    generate_continious(board, moves, pos, piece, &BISHOP_OFFSETS);
}

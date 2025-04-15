use crate::board::Board;
use crate::board::piece::Piece;
use crate::move_generation::utils::generate_continious;

const ROOK_OFFSETS: [(i32, i32); 4] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
];

pub fn generate(
    board: &Board,
    moves: &mut Vec<(i32, i32)>,
    pos: &(i32, i32),
    piece: &Piece
) {
    generate_continious(board, moves, pos, piece, &ROOK_OFFSETS);
}

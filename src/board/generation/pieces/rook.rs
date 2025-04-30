use crate::board::Board;
use crate::board::pieces::Piece;
use crate::board::moving::BoardMove;
use crate::board::generation::utils::generate_continuous;

const ROOK_OFFSETS: [(i32, i32); 4] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
];

pub fn generate(
    board: &Board,
    moves: &mut Vec<BoardMove>,
    pos: (i32, i32),
    piece: &Piece
) {
    generate_continuous(board, moves, pos, piece, &ROOK_OFFSETS);
}

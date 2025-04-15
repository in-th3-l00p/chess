use crate::board::piece::Piece;
use crate::board::Board;

const KNIGHT_OFFSETS: [(i32, i32); 8] = [
    (2, 1), (1, 2),
    (-2, 1), (-1, 2),
    (2, -1), (1, -2),
    (-2, -1), (-1, -2)
];

pub fn generate(
    board: &Board,
    moves: &mut Vec<(i32, i32)>,
    pos: &(i32, i32),
    piece: &Piece,
) {
    for offset in KNIGHT_OFFSETS {
        moves.push((pos.0 + offset.0, pos.1 + offset.1));
    }
}

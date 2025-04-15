use crate::board::piece::Piece;
use crate::board::Board;
use crate::move_generation::utils::add_move;

const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
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
    for direction in DIRECTIONS.iter() {
        add_move(
            board,
            moves,
            (pos.0 + direction.0, pos.1 + direction.1),
            piece
        );
    }
}
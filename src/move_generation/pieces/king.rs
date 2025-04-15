use crate::board::piece::Piece;
use crate::board::Board;

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
        let next_pos = (pos.0 + direction.0, pos.1 + direction.1);
        if let Some(capture) = board.get_piece(next_pos) {
            if capture.color != piece.color {
                moves.push(next_pos);
            }
        } else {
            moves.push(next_pos);
        }
    }
}
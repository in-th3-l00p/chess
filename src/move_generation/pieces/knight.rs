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
    piece: &Piece
) {
    for offset in KNIGHT_OFFSETS {
        let new_pos = (pos.0 + offset.0, pos.1 + offset.1);
        if let Some(capture) = board.get_piece(new_pos) {
            if capture.color != piece.color {
                moves.push(new_pos);
            }
        } else {
            moves.push(new_pos);
        }
    }
}

use crate::board::Board;
use crate::board::piece::Piece;

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
    for offsets in BISHOP_OFFSETS.iter() {
        let mut current_pos = pos.clone();
        current_pos.0 += offsets.0;
        current_pos.1 += offsets.1;

        while
            board.get_piece(current_pos).is_none() &&
            board.is_in_bounds(current_pos)
        {
            moves.push((current_pos.0, current_pos.1));
            current_pos.0 += offsets.0;
            current_pos.1 += offsets.1;
        }

        if board.is_in_bounds(current_pos) {
            if let Some(capture) = board.get_piece(current_pos) {
                if capture.color != piece.color {
                    moves.push(current_pos)
                }
            }
        }
    }
}

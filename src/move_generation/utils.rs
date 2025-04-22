use crate::board::Board;
use crate::board::piece::Piece;
use crate::move_generation::Move;

pub fn generate_continuous(
    board: &Board,
    moves: &mut Vec<Move>,
    pos: &(i32, i32),
    piece: &Piece,
    offsets: &[(i32, i32)]
) {
    for offset in offsets.iter() {
        let mut current_pos = pos.clone();
        current_pos.0 += offset.0;
        current_pos.1 += offset.1;

        while
        board.get_piece(current_pos).is_none() &&
            board.is_in_bounds(current_pos)
        {
            moves.push(Move::new(pos.clone(), (current_pos.0, current_pos.1), None));
            current_pos.0 += offset.0;
            current_pos.1 += offset.1;
        }

        if board.is_in_bounds(current_pos) {
            if let Some(capture) = board.get_piece(current_pos) {
                if capture.color != piece.color {
                    moves.push(Move::new(pos.clone(), current_pos, None))
                }
            }
        }
    }
}

pub fn add_move(
    board: &Board,
    moves: &mut Vec<Move>,
    current_pos: (i32, i32),
    added_pos: (i32, i32),
    piece: &Piece
) {
    if let Some(capture) = board.get_piece(added_pos) {
        if capture.color != piece.color {
            moves.push(Move::new(current_pos, added_pos, None));
        }
    } else {
        moves.push(Move::new(current_pos, added_pos, None));
    }
}
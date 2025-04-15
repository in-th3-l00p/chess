use crate::board::color::Color;
use crate::board::piece::PieceType::Pawn;
use crate::board::piece::Piece;
use crate::board::{Board, BoardMove};

fn check_en_passant(
    board: &Board,
    moves: &mut Vec<(i32, i32)>,
    pos: &(i32, i32),
    piece: &Piece,
    forward_delta: &i32
) {
    if let Some(last_move) = board.get_last_move() {
        let en_passant_delta =
            if
                *last_move == BoardMove::new(
                    (pos.0 + 1, pos.1 + forward_delta * 2),
                    (pos.0 + 1, pos.1)
                ) { 1 }
            else if
                *last_move == BoardMove::new(
                    (pos.0 - 1, pos.1 + forward_delta * 2),
                    (pos.0 - 1, pos.1)
                ) { -1 }
            else { 0 };

        if en_passant_delta != 0 {
            if let Some(en_passant) = board.get_piece(
                (pos.0 + en_passant_delta, pos.1)
            ) {
                if en_passant.color != piece.color {
                    if let Pawn { .. } = en_passant.piece_type {
                        moves.push((
                            pos.0 + en_passant_delta,
                            pos.1 + forward_delta
                        ));
                    }
                }
            }
        }
    }
}

pub fn generate(
    board: &Board,
    moves: &mut Vec<(i32, i32)>,
    pos: &(i32, i32),
    piece: &Piece,
) {
    let delta = if let Color::White = piece.color { -1 } else { 1 };

    // single push
    if board.get_piece((pos.0, pos.1 + delta)).is_none() {
        moves.push((pos.0, pos.1 + delta));
    }

    // captures
    if let Some(right_capture) = board.get_piece((pos.0 + 1, pos.1 + delta)) {
        if right_capture.color != piece.color {
            moves.push((pos.0 + 1, pos.1 + delta));
        }
    }
    if let Some(left_capture) = board.get_piece((pos.0 - 1, pos.1 + delta)) {
        if left_capture.color != piece.color {
            moves.push((pos.0 - 1, pos.1 + delta));
        }
    }

    // double push
    if let Pawn { has_moved, .. } = piece.piece_type {
        if
            !has_moved &&
            board.get_piece((pos.0, pos.1 + delta)).is_none() &&
            board.get_piece((pos.0, pos.1 + 2 * delta)).is_none()
        {
            moves.push((pos.0, pos.1 + 2 * delta));
        }
    }

    // en passant
    match piece.color {
        Color::White => {
            if pos.1 == 3 {
                check_en_passant(board, moves, pos, piece, &delta);
            }
        },
        Color::Black => {
            if pos.1 == 4 {
                check_en_passant(board, moves, pos, piece, &delta);
            }
        }
    }
}
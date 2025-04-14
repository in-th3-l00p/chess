use crate::board::Board;
use crate::board::color::Color;
use crate::board::piece::{Piece, PieceType};

pub fn generate(
    board: &Board,
    moves: &mut Vec<(i32, i32)>,
    pos: &(i32, i32),
    piece: &Piece,
) {
    let delta = if let Color::White = piece.color { -1 } else { 1 };
    if board.get_piece((pos.0, pos.1 + delta)).is_none() {
        moves.push((pos.0, pos.1 + delta));
    }
    if board.get_piece((pos.0 + 1, pos.1 + delta)).is_some() {
        moves.push((pos.0 + 1, pos.1 + delta));
    }
    if board.get_piece((pos.0 - 1, pos.1 + delta)).is_some() {
        moves.push((pos.0 - 1, pos.1 + delta));
    }
    if let PieceType::Pawn { has_moved, .. } = piece.piece_type {
        if
            !has_moved &&
            board.get_piece((pos.0, pos.1 + delta)).is_none() &&
            board.get_piece((pos.0, pos.1 + 2 * delta)).is_none()
        {
            moves.push((pos.0, pos.1 + 2 * delta));
        }
    }
}
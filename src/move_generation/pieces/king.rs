use crate::board::piece::{Piece, PieceType};
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

    // casteling
    if let PieceType::King { has_moved, .. } = piece.piece_type {
        if !has_moved {
            if board.get_piece((5, pos.1)).is_none() &&
                board.get_piece((6, pos.1)).is_none()
            {
                if let Some(potential_rook) = board.get_piece((7, pos.1)) {
                    if let PieceType::Rook { has_moved } = potential_rook.piece_type {
                        if !has_moved {
                            moves.push((6, pos.1));
                        }
                    }
                }
            }

            // queen side
            if board.get_piece((3, pos.1)).is_none() &&
                board.get_piece((2, pos.1)).is_none() &&
                board.get_piece((1, pos.1)).is_none()
            {
                if let Some(potential_rook) = board.get_piece((0, pos.1)) {
                    if let PieceType::Rook { has_moved } = potential_rook.piece_type {
                        if !has_moved {
                            moves.push((2, pos.1));
                        }
                    }
                }
            }
        }
    }
}
use crate::board::piece::{Piece, PieceType};
use crate::board::Board;
use crate::board::color::Color;
use crate::move_generation::{generate_piece_unchecked_moves, BoardMove};
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

fn check_touching(board: &Board, color: Color, positions: [(i32, i32); 3]) -> bool {
    for y in 0..8 {
        for x in  0..8 {
            if let Some(piece) = board.get_piece((x, y)) {
                if piece.color == color {
                    match piece.piece_type {
                        PieceType::King { .. } => {
                            for direction in DIRECTIONS.iter() {
                                for i in 0..3 {
                                    if
                                        x + direction.0 == positions[i].0 &&
                                        y + direction.1 == positions[i].1
                                    {
                                        return true;
                                    }
                                }
                            }
                        },
                        _ => {
                            if generate_piece_unchecked_moves(board, (x, y))
                                .iter()
                                .any(|potential_break_move| {
                                    for i in 0..3 {
                                        if
                                            potential_break_move.to.0 == positions[i].0 &&
                                            potential_break_move.to.1 == positions[i].1
                                        {
                                            return true;
                                        }
                                    }
                                    return false;
                                })
                            {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

pub fn generate(
    board: &Board,
    moves: &mut Vec<BoardMove>,
    pos: (i32, i32),
    piece: &Piece
) {
    for direction in DIRECTIONS.iter() {
        add_move(
            board,
            moves,
            pos,
            (pos.0 + direction.0, pos.1 + direction.1),
            piece
        );
    }

    // casteling
    if let PieceType::King { has_moved, .. } = piece.piece_type {
        if !has_moved {
            // king side
            if board.get_piece((5, pos.1)).is_none() &&
                board.get_piece((6, pos.1)).is_none()
            {
                if let Some(potential_rook) = board.get_piece((7, pos.1)) {
                    if let PieceType::Rook { has_moved } = potential_rook.piece_type {
                        if !has_moved {
                            if !check_touching(
                                board,
                                piece.color.inverse(),
                                [(5, pos.1), (6, pos.1), (-100, -100)]
                            ) {
                                moves.push(BoardMove::new(pos, (6, pos.1), None));
                            }
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
                            if !check_touching(
                                board,
                                piece.color.inverse(),
                                [(3, pos.1), (2, pos.1), (1, pos.1)]
                            ) {
                                moves.push(BoardMove::new(pos, (2, pos.1), None));
                            }
                        }
                    }
                }
            }
        }
    }
}
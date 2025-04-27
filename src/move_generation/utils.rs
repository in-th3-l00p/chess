use crate::board::Board;
use crate::board::color::Color;
use crate::board::piece::{Piece, PieceType};
use crate::move_generation::{generate_unchecked_moves_by_color, BoardMove};

pub fn generate_continuous(
    board: &Board,
    moves: &mut Vec<BoardMove>,
    pos: (i32, i32),
    piece: &Piece,
    offsets: &[(i32, i32)]
) {
    for offset in offsets.iter() {
        let mut current_pos = pos;
        current_pos.0 += offset.0;
        current_pos.1 += offset.1;

        while
        board.get_piece(current_pos).is_none() &&
            board.is_in_bounds(current_pos)
        {
            moves.push(BoardMove::new(pos, (current_pos.0, current_pos.1), None));
            current_pos.0 += offset.0;
            current_pos.1 += offset.1;
        }

        if board.is_in_bounds(current_pos) {
            if let Some(capture) = board.get_piece(current_pos) {
                if capture.color != piece.color {
                    moves.push(BoardMove::new(pos, current_pos, None))
                }
            }
        }
    }
}

pub fn add_move(
    board: &Board,
    moves: &mut Vec<BoardMove>,
    current_pos: (i32, i32),
    added_pos: (i32, i32),
    piece: &Piece
) {
    if board.is_in_bounds(added_pos) {
        if let Some(capture) = board.get_piece(added_pos) {
            if capture.color != piece.color {
                moves.push(BoardMove::new(current_pos, added_pos, None));
            }
        } else {
            moves.push(BoardMove::new(current_pos, added_pos, None));
        }
    }
}

pub fn is_in_check(board: &Board, color: Color) -> bool {
    let mut king_x = 0;
    let mut king_y = 0;
    while king_y < 8 {
        king_x = 0;
        while king_x < 8 {
            if let Some(possible_piece) = board.get_piece((king_x, king_y)) {
                if let PieceType::King { .. } = possible_piece.piece_type {
                    if possible_piece.color == color {
                        break
                    }
                }
            }
            king_x += 1;
        }
        if let Some(possible_piece) = board.get_piece((king_x, king_y)) {
            if let PieceType::King { .. } = possible_piece.piece_type {
                if possible_piece.color == color {
                    break
                }
            }
        }
        king_y += 1;
    }

    generate_unchecked_moves_by_color(board, color.inverse())
        .iter()
        .any(|possible_capture| {
            possible_capture.to.0 == king_x &&
            possible_capture.to.1 == king_y
        })
}
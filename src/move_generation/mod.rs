mod utils;
mod pieces;

use pieces::{bishop, knight, pawn, queen, rook};
use crate::board::piece::PieceType;
use crate::board::Board;
use crate::board::color::Color;
use crate::move_generation::pieces::king;
use crate::move_generation::utils::is_in_check;

#[derive(Clone, Eq, PartialEq)]
pub struct BoardMove {
    pub from: (i32, i32),
    pub to: (i32, i32),
    pub promote: Option<PieceType>,
}

impl BoardMove {
    pub fn new(
        from: (i32, i32),
        to: (i32, i32),
        promote: Option<PieceType>
    ) -> BoardMove {
        BoardMove { from, to, promote }
    }
}

fn generate_piece_unchecked_moves(board: &Board, pos: (i32, i32)) -> Vec<BoardMove> {
    let mut moves = Vec::new();
    let piece = board.get_piece(pos);
    if let Some(piece) = piece {
        match piece.piece_type {
            PieceType::Pawn{..} =>
                pawn::generate(board, &mut moves, pos, &piece),
            PieceType::Knight{..} =>
                knight::generate(board, &mut moves, pos, &piece),
            PieceType::Bishop{..} =>
                bishop::generate(board, &mut moves, pos, &piece),
            PieceType::Rook{..} =>
                rook::generate(board, &mut moves, pos, &piece),
            PieceType::Queen{..} =>
                queen::generate(board, &mut moves, pos, &piece),
            PieceType::King{..} =>
                king::generate(board, &mut moves, pos, &piece)
        }
    }
    moves
}

fn generate_moves_by_color(board: &Board, color: Color) -> Vec<BoardMove> {
    let mut moves = Vec::new();
    for x in 0..8 {
        for y in 0..8 {
            if let Some(piece) = board.get_piece((x, y)) {
                if piece.color == color {
                    moves.append(&mut generate_piece_unchecked_moves(board, (x, y)));
                }
            }
        }
    }
    moves
}

pub fn generate_moves(board: &Board, color: Color) -> Vec<BoardMove> {
    generate_moves_by_color(board, color)
        .into_iter()
        .filter(|possible_move| {
            let mut new_board = board.clone();
            new_board.make_move((*possible_move).clone());
            return !is_in_check(&new_board, color);
        })
        .collect()
}

pub fn generate_piece_moves(board: &Board, pos: (i32, i32)) -> Vec<BoardMove> {
    if let Some(piece) = board.get_piece(pos) {
        generate_moves(board, piece.color)
            .into_iter()
            .filter(|possible_move| {
                possible_move.from.0 == pos.0 &&
                possible_move.from.1 == pos.1
            })
            .collect()
    } else {
        vec!()
    }
}
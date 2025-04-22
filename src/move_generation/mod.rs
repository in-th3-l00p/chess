mod utils;
mod pieces;

use pieces::{bishop, knight, pawn, queen, rook};
use crate::board::piece::PieceType;
use crate::board::Board;
use crate::move_generation::pieces::king;

pub struct Move {
    pub to: (i32, i32),
    pub promote: Option<PieceType>,
}

impl Move {
    pub fn new(to: (i32, i32), promote: Option<PieceType>) -> Move {
        Move { to, promote }
    }
}

pub fn generate_possible_moves(board: &Board, pos: (i32, i32)) -> Vec<Move> {
    let mut moves = Vec::new();
    let piece = board.get_piece(pos);
    if let Some(piece) = piece {
        match piece.piece_type {
            PieceType::Pawn{..} =>
                pawn::generate(board, &mut moves, &pos, &piece),
            PieceType::Knight{..} =>
                knight::generate(board, &mut moves, &pos, &piece),
            PieceType::Bishop{..} =>
                bishop::generate(board, &mut moves, &pos, &piece),
            PieceType::Rook{..} =>
                rook::generate(board, &mut moves, &pos, &piece),
            PieceType::Queen{..} =>
                queen::generate(board, &mut moves, &pos, &piece),
            PieceType::King{..} =>
                king::generate(board, &mut moves, &pos, &piece)
        }
    }
    moves
}
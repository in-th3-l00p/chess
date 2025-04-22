mod utils;
mod pieces;

use pieces::{bishop, knight, pawn, queen, rook};
use crate::board::piece::PieceType;
use crate::board::Board;
use crate::move_generation::pieces::king;

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

pub fn generate_possible_moves(board: &Board, pos: (i32, i32)) -> Vec<BoardMove> {
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
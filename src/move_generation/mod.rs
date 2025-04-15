mod pawn;
mod knight;
mod bishop;
mod utils;
mod rook;
mod queen;

use crate::board::piece::PieceType;
use crate::board::Board;

pub fn generate_possible_moves(board: &Board, pos: (i32, i32)) -> Vec<(i32, i32)> {
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
            _ => {},
        }
    }
    moves
}
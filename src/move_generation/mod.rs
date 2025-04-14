mod pawn;

use crate::board::piece::PieceType;
use crate::board::Board;

pub fn generate_possible_moves(board: &Board, pos: (i32, i32)) -> Vec<(i32, i32)> {
    let mut moves = Vec::new();
    let piece = board.get_piece(pos);
    let pos = (pos.0, pos.1); // casting
    if let Some(piece) = piece {
        match piece.piece_type {
            PieceType::Pawn{ ..} =>
                pawn::generate(board, &mut moves, &pos, &piece),
            _ => {}
        }
    }
    moves
}
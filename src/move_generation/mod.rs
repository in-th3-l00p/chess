mod utils;
mod pawn;

use crate::board::piece::Piece;
use crate::board::Board;

pub fn generate_possible_moves(board: &Board, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();
    let piece = board.get_piece(pos);
    let pos = (pos.0 as i32, pos.1 as i32); // casting
    if let Some(piece) = piece {
        match piece {
            Piece::Pawn{color, has_moved, ..} =>
                pawn::generate(board, &mut moves, &pos, &color, &has_moved),
            _ => {}
        }
        moves
    } else {
        moves
    }
}
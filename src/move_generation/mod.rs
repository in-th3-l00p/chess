use crate::board::Board;
use crate::board::color::Color;
use crate::board::piece::Piece;

fn check_valid(board: &Board, pos: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < 8 && pos.1 >= 0 && pos.1 < 8 &&
    board.get_piece(pos.0 as usize, pos.1 as usize).is_none()
}

fn insert_pos(board: &Board, moves: &mut Vec<(usize, usize)>, pos: (i32, i32)) {
    if check_valid(board, pos) {
        moves.push((pos.0 as usize, pos.1 as usize));
    }
}

pub fn generate(board: &Board, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();
    let piece = board.get_piece(pos.0, pos.1);
    let pos = (pos.0 as i32, pos.1 as i32); // casting
    if let Some(piece) = piece {
        match piece {
            Piece::Pawn{color, has_moved, ..} => {
                let delta = if let Color::White = color { 1 } else { -1 };
                insert_pos(board, &mut moves, (pos.0 + delta, pos.1 + delta));
                if !has_moved {
                    insert_pos(board, &mut moves, (pos.0 + 2 * delta, pos.1 + 2 * delta));
                }
            }
            _ => {}
        }
        moves
    } else {
        moves
    }
}
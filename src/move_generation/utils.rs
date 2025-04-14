use crate::board::Board;

fn check_valid(board: &Board, pos: (i32, i32)) -> bool {
    board.get_piece(pos).is_none()
}

pub fn insert_pos(board: &Board, moves: &mut Vec<(i32, i32)>, pos: (i32, i32)) {
    if check_valid(board, pos) {
        moves.push(pos);
    }
}

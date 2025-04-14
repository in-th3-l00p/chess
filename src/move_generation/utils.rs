use crate::board::Board;

fn check_valid(board: &Board, pos: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < 8 && pos.1 >= 0 && pos.1 < 8 &&
        board.get_piece((pos.0 as usize, pos.1 as usize)).is_none()
}

pub fn insert_pos(board: &Board, moves: &mut Vec<(usize, usize)>, pos: (i32, i32)) {
    if check_valid(board, pos) {
        moves.push((pos.0 as usize, pos.1 as usize));
    }
}

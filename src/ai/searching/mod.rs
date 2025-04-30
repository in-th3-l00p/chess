use crate::ai::evaluation::evaluate;
use crate::board::Board;
use crate::board::pieces::color::Color;

pub fn search(
    board: &mut Board,
    depth: i32,
    color: Color,
    mut alpha: i32,
    beta: i32
) -> i32 {
    if depth == 0 {
        if let Color::White = color {
            evaluate(board)
        } else {
            -evaluate(board)
        }
    } else {
        let mut best_value = i32::MIN + 1;
        for potential_move in crate::board::generation::generate_moves(board, color) {
            let mut new_board = board.clone();
            new_board.make_move(potential_move);
            let score = -search(
                &mut new_board,
                depth - 1,
                color.inverse(),
                -beta,
                -alpha
            );
            if score > best_value {
                best_value = score;
                if score > alpha {
                    alpha = score;
                }
            }

            if score >= beta {
                return beta;
            }
        }
        best_value
    }
}